//! Page-level ARIA accessibility snapshot methods.
//!
//! This module provides methods for capturing accessibility snapshots that span
//! multiple frames, stitching together the accessibility trees from each frame
//! into a complete representation of the page.
//!
//! # Frame Boundary Handling
//!
//! When capturing aria snapshots, iframes are marked as frame boundaries with
//! `is_frame: true`. The `aria_snapshot_with_frames()` method captures snapshots
//! from all frames and stitches them together at the iframe boundaries.
//!
//! # Cross-Origin Limitations
//!
//! Due to browser security restrictions:
//! - Same-origin iframes: Content is fully captured and stitched
//! - Cross-origin iframes: Marked as boundaries with `is_frame: true` but content
//!   may be limited or empty depending on CDP permissions
//!
//! # Example
//!
//! ```no_run
//! use viewpoint_core::Page;
//!
//! # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
//! // Capture complete accessibility tree including iframes
//! let snapshot = page.aria_snapshot_with_frames().await?;
//! println!("{}", snapshot);
//!
//! // The snapshot will include all frame content stitched together
//! // Iframes are represented with their content inline
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;

use tracing::{debug, instrument, trace, warn};
use viewpoint_cdp::protocol::dom::{BackendNodeId, DescribeNodeParams, DescribeNodeResult};
use viewpoint_js::js;

use super::locator::aria_js::aria_snapshot_with_refs_js;
use super::locator::AriaSnapshot;
use super::ref_resolution::format_ref;
use super::Page;
use crate::error::PageError;

impl Page {
    /// Capture an ARIA accessibility snapshot of the entire page including all frames.
    ///
    /// This method captures the accessibility tree of the main frame and all child
    /// frames (iframes), then stitches them together into a single tree. Frame
    /// boundaries in the main frame snapshot are replaced with the actual content
    /// from the corresponding frames.
    ///
    /// # Frame Content Stitching
    ///
    /// The method works by:
    /// 1. Capturing the main frame's aria snapshot (which marks iframes as boundaries)
    /// 2. Getting the frame tree from CDP
    /// 3. For each child frame, capturing its aria snapshot
    /// 4. Stitching child frame content into the parent snapshot at iframe boundaries
    ///
    /// # Cross-Origin Frames
    ///
    /// For cross-origin frames, CDP may still be able to capture content through
    /// out-of-process iframe (OOPIF) handling. However, some content may be
    /// inaccessible due to browser security policies. In such cases, the frame
    /// boundary will remain with `is_frame: true` but may have limited or no children.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.aria_snapshot_with_frames().await?;
    ///
    /// // The snapshot YAML output will show frame content inline:
    /// // - document "Main Page"
    /// //   - heading "Title"
    /// //   - iframe "Widget Frame" [frame-boundary]
    /// //     - document "Widget"
    /// //       - button "Click me"
    /// println!("{}", snapshot);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - Frame tree retrieval fails
    /// - Snapshot capture fails for the main frame
    #[instrument(level = "debug", skip(self), fields(target_id = %self.target_id))]
    pub async fn aria_snapshot_with_frames(&self) -> Result<AriaSnapshot, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        // Get the main frame snapshot first
        let main_frame = self.main_frame().await?;
        let mut root_snapshot = main_frame.aria_snapshot().await?;

        // Get all frames
        let frames = self.frames().await?;

        // Build a map of frame URL/name to captured snapshots
        let mut frame_snapshots: HashMap<String, AriaSnapshot> = HashMap::new();

        for frame in &frames {
            if !frame.is_main() {
                // Capture snapshot for this frame
                match frame.aria_snapshot().await {
                    Ok(snapshot) => {
                        let url = frame.url();
                        if !url.is_empty() && url != "about:blank" {
                            frame_snapshots.insert(url.clone(), snapshot.clone());
                        }
                        let name = frame.name();
                        if !name.is_empty() {
                            frame_snapshots.insert(name.clone(), snapshot.clone());
                        }
                        // Also store by frame ID
                        frame_snapshots.insert(frame.id().to_string(), snapshot);
                    }
                    Err(e) => {
                        warn!(
                            error = %e,
                            frame_id = %frame.id(),
                            frame_url = %frame.url(),
                            "Failed to capture frame snapshot, skipping"
                        );
                    }
                }
            }
        }

        // Stitch frame content into the snapshot
        stitch_frame_content(&mut root_snapshot, &frame_snapshots, 0);

        Ok(root_snapshot)
    }

    /// Capture an ARIA accessibility snapshot of just the main frame.
    ///
    /// This is a convenience method equivalent to calling `main_frame().await?.aria_snapshot().await`.
    /// Unlike `aria_snapshot_with_frames()`, this does NOT stitch in iframe content -
    /// iframes are left as boundaries with `is_frame: true`.
    ///
    /// # Node References
    ///
    /// The snapshot includes `node_ref` on each element (format: `e{backendNodeId}`).
    /// These refs can be used with `element_from_ref()` or `locator_from_ref()` to
    /// interact with elements discovered in the snapshot.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Quick snapshot without frame content
    /// let snapshot = page.aria_snapshot().await?;
    ///
    /// // Each element has a ref for interaction
    /// if let Some(ref node_ref) = snapshot.node_ref {
    ///     let locator = page.locator_from_ref(node_ref);
    ///     locator.click().await?;
    /// }
    ///
    /// // Check if there are frame boundaries to expand
    /// if !snapshot.iframe_refs.is_empty() {
    ///     println!("Page has {} frames that can be expanded", snapshot.iframe_refs.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - Snapshot capture fails
    #[instrument(level = "debug", skip(self), fields(target_id = %self.target_id))]
    pub async fn aria_snapshot(&self) -> Result<AriaSnapshot, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        // Capture snapshot with element collection for ref resolution
        self.capture_snapshot_with_refs().await
    }

    /// Internal method to capture a snapshot with refs resolved.
    ///
    /// This uses a two-phase approach:
    /// 1. JS traversal collects the snapshot and element references
    /// 2. CDP calls resolve each element to its backendNodeId
    #[instrument(level = "debug", skip(self), fields(target_id = %self.target_id))]
    async fn capture_snapshot_with_refs(&self) -> Result<AriaSnapshot, PageError> {
        let snapshot_fn = aria_snapshot_with_refs_js();

        // Evaluate the JS function to get snapshot and element array
        // We return by value for the snapshot, but need remote objects for elements
        let js_code = js! {
            (function() {
                const getSnapshotWithRefs = @{snapshot_fn};
                return getSnapshotWithRefs(document.body);
            })()
        };

        // First, evaluate to get the result as a RemoteObject (not by value)
        // so we can access the elements array
        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js_code,
                    object_group: Some("viewpoint-snapshot".to_string()),
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(false), // Get RemoteObject, not value
                    await_promise: Some(false),
                }),
                Some(self.session_id()),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(PageError::EvaluationFailed(exception.text));
        }

        let result_object_id = result.result.object_id.ok_or_else(|| {
            PageError::EvaluationFailed("No object ID from snapshot evaluation".to_string())
        })?;

        // Get the snapshot property (by value)
        let snapshot_value = self.get_property_value(&result_object_id, "snapshot").await?;
        
        // Parse the snapshot
        let mut snapshot: AriaSnapshot = serde_json::from_value(snapshot_value).map_err(|e| {
            PageError::EvaluationFailed(format!("Failed to parse aria snapshot: {e}"))
        })?;

        // Get the elements array as a RemoteObject
        let elements_result = self.get_property_object(&result_object_id, "elements").await?;
        
        if let Some(elements_object_id) = elements_result {
            // Get the length of the elements array
            let length_value = self.get_property_value(&elements_object_id, "length").await?;
            let element_count = length_value.as_u64().unwrap_or(0) as usize;
            
            debug!(element_count = element_count, "Resolving element refs");

            // Build a map of element index -> backendNodeId
            let mut ref_map: HashMap<usize, BackendNodeId> = HashMap::new();

            for i in 0..element_count {
                // Get the element at index i
                if let Ok(Some(element_object_id)) = self.get_array_element(&elements_object_id, i).await {
                    // Get the backendNodeId for this element
                    match self.describe_node(&element_object_id).await {
                        Ok(backend_node_id) => {
                            ref_map.insert(i, backend_node_id);
                            trace!(index = i, backend_node_id = backend_node_id, "Resolved element ref");
                        }
                        Err(e) => {
                            debug!(index = i, error = %e, "Failed to get backendNodeId for element");
                        }
                    }
                }
            }

            // Apply refs to the snapshot tree
            apply_refs_to_snapshot(&mut snapshot, &ref_map);

            // Release the elements array to free memory
            let _ = self.release_object(&elements_object_id).await;
        }

        // Release the result object
        let _ = self.release_object(&result_object_id).await;

        Ok(snapshot)
    }

    /// Get a property value from a RemoteObject by name.
    async fn get_property_value(
        &self,
        object_id: &str,
        property: &str,
    ) -> Result<serde_json::Value, PageError> {
        #[derive(Debug, serde::Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
        }

        let result: CallResult = self
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": format!("function() {{ return this.{}; }}", property),
                    "returnByValue": true
                })),
                Some(self.session_id()),
            )
            .await?;

        Ok(result.result.value.unwrap_or(serde_json::Value::Null))
    }

    /// Get a property as a RemoteObject from a RemoteObject by name.
    async fn get_property_object(
        &self,
        object_id: &str,
        property: &str,
    ) -> Result<Option<String>, PageError> {
        #[derive(Debug, serde::Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
        }

        let result: CallResult = self
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": format!("function() {{ return this.{}; }}", property),
                    "returnByValue": false
                })),
                Some(self.session_id()),
            )
            .await?;

        Ok(result.result.object_id)
    }

    /// Get an element from an array by index.
    async fn get_array_element(
        &self,
        array_object_id: &str,
        index: usize,
    ) -> Result<Option<String>, PageError> {
        #[derive(Debug, serde::Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
        }

        let result: CallResult = self
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": array_object_id,
                    "functionDeclaration": format!("function() {{ return this[{}]; }}", index),
                    "returnByValue": false
                })),
                Some(self.session_id()),
            )
            .await?;

        Ok(result.result.object_id)
    }

    /// Get the backendNodeId for an element by its object ID.
    async fn describe_node(&self, object_id: &str) -> Result<BackendNodeId, PageError> {
        let result: DescribeNodeResult = self
            .connection()
            .send_command(
                "DOM.describeNode",
                Some(DescribeNodeParams {
                    node_id: None,
                    backend_node_id: None,
                    object_id: Some(object_id.to_string()),
                    depth: Some(0),
                    pierce: None,
                }),
                Some(self.session_id()),
            )
            .await?;

        Ok(result.node.backend_node_id)
    }

    /// Release a RemoteObject by its object ID.
    async fn release_object(&self, object_id: &str) -> Result<(), PageError> {
        let _: serde_json::Value = self
            .connection()
            .send_command(
                "Runtime.releaseObject",
                Some(serde_json::json!({
                    "objectId": object_id
                })),
                Some(self.session_id()),
            )
            .await?;

        Ok(())
    }
}

/// Recursively apply refs to the snapshot tree based on element indices.
fn apply_refs_to_snapshot(snapshot: &mut AriaSnapshot, ref_map: &HashMap<usize, BackendNodeId>) {
    // Apply ref if this node has an element_index
    if let Some(index) = snapshot.element_index {
        if let Some(&backend_node_id) = ref_map.get(&index) {
            snapshot.node_ref = Some(format_ref(backend_node_id));
        }
        // Clear the element_index now that we've processed it
        snapshot.element_index = None;
    }

    // Recursively process children
    for child in &mut snapshot.children {
        apply_refs_to_snapshot(child, ref_map);
    }
}

/// Recursively stitch frame content into aria snapshot at iframe boundaries.
///
/// This function traverses the snapshot tree looking for nodes with `is_frame: true`.
/// When found, it attempts to find the corresponding frame snapshot and adds that
/// content as children of the iframe node.
fn stitch_frame_content(
    snapshot: &mut AriaSnapshot,
    frame_snapshots: &HashMap<String, AriaSnapshot>,
    depth: usize,
) {
    // Prevent infinite recursion - max depth of 10 nested frames
    const MAX_DEPTH: usize = 10;
    if depth > MAX_DEPTH {
        warn!(
            depth = depth,
            "Max frame nesting depth exceeded, stopping recursion"
        );
        return;
    }

    // If this is a frame boundary, try to get its content
    if snapshot.is_frame == Some(true) {
        // Try to find the matching frame snapshot
        let frame_snapshot = snapshot
            .frame_url
            .as_ref()
            .and_then(|url| frame_snapshots.get(url))
            .or_else(|| {
                snapshot
                    .frame_name
                    .as_ref()
                    .and_then(|name| frame_snapshots.get(name))
            });

        if let Some(frame_content) = frame_snapshot {
            debug!(
                frame_url = ?snapshot.frame_url,
                frame_name = ?snapshot.frame_name,
                depth = depth,
                "Stitching frame content into snapshot"
            );

            // Add the frame's content as children of this iframe node
            // Clear is_frame to prevent re-processing this boundary
            snapshot.is_frame = Some(false);
            snapshot.children = vec![frame_content.clone()];
        } else {
            debug!(
                frame_url = ?snapshot.frame_url,
                frame_name = ?snapshot.frame_name,
                "No matching frame snapshot found for iframe boundary"
            );
        }
    }

    // Recursively process children
    for child in &mut snapshot.children {
        stitch_frame_content(child, frame_snapshots, depth + 1);
    }
}
