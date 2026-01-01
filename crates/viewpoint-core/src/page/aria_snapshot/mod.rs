//! Page-level ARIA accessibility snapshot methods.
//!
//! This module provides methods for capturing accessibility snapshots that span
//! multiple frames, stitching together the accessibility trees from each frame
//! into a complete representation of the page.
//!
//! # Performance
//!
//! Snapshot capture is optimized for performance through:
//! - **Parallel node resolution**: Multiple `DOM.describeNode` CDP calls are executed
//!   concurrently (up to 50 by default) instead of sequentially
//! - **Batch array access**: Element object IDs are retrieved in a single CDP call
//!   using `Runtime.getProperties` instead of N individual calls
//! - **Parallel frame capture**: Multi-frame snapshots capture all child frames
//!   concurrently instead of sequentially
//!
//! These optimizations can provide 10-20x performance improvement for large DOMs.
//!
//! # Configuration
//!
//! Use [`SnapshotOptions`] to tune snapshot behavior:
//!
//! ```no_run
//! use viewpoint_core::{Page, SnapshotOptions};
//!
//! # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
//! // Default options (include refs, 50 concurrent CDP calls)
//! let snapshot = page.aria_snapshot().await?;
//!
//! // Skip ref resolution for maximum performance
//! let options = SnapshotOptions::default().include_refs(false);
//! let snapshot = page.aria_snapshot_with_options(options).await?;
//!
//! // Increase concurrency for fast networks
//! let options = SnapshotOptions::default().max_concurrency(100);
//! let snapshot = page.aria_snapshot_with_options(options).await?;
//! # Ok(())
//! # }
//! ```
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

use futures::stream::{FuturesUnordered, StreamExt};
use tracing::{debug, instrument, trace, warn};
use viewpoint_cdp::protocol::dom::{BackendNodeId, DescribeNodeParams, DescribeNodeResult};
use viewpoint_js::js;

use super::locator::aria_js::aria_snapshot_with_refs_js;
use super::locator::AriaSnapshot;
use super::ref_resolution::format_ref;
use super::Page;
use crate::error::PageError;

/// Default maximum number of concurrent CDP calls for node resolution.
pub const DEFAULT_MAX_CONCURRENCY: usize = 50;

/// Configuration options for ARIA snapshot capture.
///
/// Use this struct to tune snapshot performance and behavior.
///
/// # Example
///
/// ```no_run
/// use viewpoint_core::SnapshotOptions;
///
/// // Default options
/// let options = SnapshotOptions::default();
///
/// // Skip ref resolution for faster snapshots
/// let options = SnapshotOptions::default().include_refs(false);
///
/// // Increase concurrency for fast networks
/// let options = SnapshotOptions::default().max_concurrency(100);
/// ```
#[derive(Debug, Clone)]
pub struct SnapshotOptions {
    /// Maximum number of concurrent CDP calls for node resolution.
    ///
    /// Higher values improve performance but may overwhelm slow connections.
    /// Default: 50
    max_concurrency: usize,

    /// Whether to include element refs (backendNodeIds) in the snapshot.
    ///
    /// Set to `false` to skip ref resolution for maximum performance when
    /// you only need the accessibility tree structure.
    /// Default: true
    include_refs: bool,
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            max_concurrency: DEFAULT_MAX_CONCURRENCY,
            include_refs: true,
        }
    }
}

impl SnapshotOptions {
    /// Set the maximum number of concurrent CDP calls for node resolution.
    ///
    /// Higher values improve performance but may overwhelm slow connections.
    /// Default: 50
    #[must_use]
    pub fn max_concurrency(mut self, max: usize) -> Self {
        self.max_concurrency = max;
        self
    }

    /// Set whether to include element refs (backendNodeIds) in the snapshot.
    ///
    /// Set to `false` to skip ref resolution for maximum performance when
    /// you only need the accessibility tree structure.
    /// Default: true
    #[must_use]
    pub fn include_refs(mut self, include: bool) -> Self {
        self.include_refs = include;
        self
    }

    /// Get the maximum concurrency setting.
    pub fn get_max_concurrency(&self) -> usize {
        self.max_concurrency
    }

    /// Get whether refs should be included.
    pub fn get_include_refs(&self) -> bool {
        self.include_refs
    }
}

impl Page {
    /// Capture an ARIA accessibility snapshot of the entire page including all frames.
    ///
    /// This method captures the accessibility tree of the main frame and all child
    /// frames (iframes), then stitches them together into a single tree. Frame
    /// boundaries in the main frame snapshot are replaced with the actual content
    /// from the corresponding frames.
    ///
    /// # Performance
    ///
    /// Child frame snapshots are captured in parallel for improved performance.
    /// For pages with many iframes, this can significantly reduce capture time.
    ///
    /// # Frame Content Stitching
    ///
    /// The method works by:
    /// 1. Capturing the main frame's aria snapshot (which marks iframes as boundaries)
    /// 2. Getting the frame tree from CDP
    /// 3. For each child frame, capturing its aria snapshot (in parallel)
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
        self.aria_snapshot_with_frames_and_options(SnapshotOptions::default())
            .await
    }

    /// Capture an ARIA accessibility snapshot of the entire page including all frames,
    /// with custom options.
    ///
    /// See [`aria_snapshot_with_frames`](Self::aria_snapshot_with_frames) for details.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Page, SnapshotOptions};
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Skip ref resolution for faster capture
    /// let options = SnapshotOptions::default().include_refs(false);
    /// let snapshot = page.aria_snapshot_with_frames_and_options(options).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self, options), fields(target_id = %self.target_id))]
    pub async fn aria_snapshot_with_frames_and_options(
        &self,
        options: SnapshotOptions,
    ) -> Result<AriaSnapshot, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        // Get the main frame snapshot first
        let main_frame = self.main_frame().await?;
        let mut root_snapshot = main_frame
            .aria_snapshot_with_options(options.clone())
            .await?;

        // Get all frames
        let frames = self.frames().await?;

        // Filter to non-main frames
        let child_frames: Vec<_> = frames.iter().filter(|f| !f.is_main()).collect();

        if child_frames.is_empty() {
            return Ok(root_snapshot);
        }

        debug!(
            frame_count = child_frames.len(),
            "Capturing child frame snapshots in parallel"
        );

        // Capture all child frame snapshots in parallel
        let frame_futures: FuturesUnordered<_> = child_frames
            .iter()
            .map(|frame| {
                let frame_id = frame.id().to_string();
                let frame_url = frame.url().clone();
                let frame_name = frame.name().clone();
                let opts = options.clone();
                async move {
                    match frame.aria_snapshot_with_options(opts).await {
                        Ok(snapshot) => Some((frame_id, frame_url, frame_name, snapshot)),
                        Err(e) => {
                            warn!(
                                error = %e,
                                frame_id = %frame_id,
                                frame_url = %frame_url,
                                "Failed to capture frame snapshot, skipping"
                            );
                            None
                        }
                    }
                }
            })
            .collect();

        // Collect results
        let results: Vec<_> = frame_futures.collect().await;

        // Build a map of frame URL/name to captured snapshots
        let mut frame_snapshots: HashMap<String, AriaSnapshot> = HashMap::new();

        for result in results.into_iter().flatten() {
            let (frame_id, frame_url, frame_name, snapshot) = result;

            if !frame_url.is_empty() && frame_url != "about:blank" {
                frame_snapshots.insert(frame_url, snapshot.clone());
            }
            if !frame_name.is_empty() {
                frame_snapshots.insert(frame_name, snapshot.clone());
            }
            // Also store by frame ID
            frame_snapshots.insert(frame_id, snapshot);
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
        self.aria_snapshot_with_options(SnapshotOptions::default())
            .await
    }

    /// Capture an ARIA accessibility snapshot with custom options.
    ///
    /// See [`aria_snapshot`](Self::aria_snapshot) for details.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Page, SnapshotOptions};
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Skip ref resolution for maximum performance
    /// let options = SnapshotOptions::default().include_refs(false);
    /// let snapshot = page.aria_snapshot_with_options(options).await?;
    ///
    /// // Increase concurrency for fast networks
    /// let options = SnapshotOptions::default().max_concurrency(100);
    /// let snapshot = page.aria_snapshot_with_options(options).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self, options), fields(target_id = %self.target_id))]
    pub async fn aria_snapshot_with_options(
        &self,
        options: SnapshotOptions,
    ) -> Result<AriaSnapshot, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        // Capture snapshot with element collection for ref resolution
        self.capture_snapshot_with_refs(options).await
    }

    /// Internal method to capture a snapshot with refs resolved.
    ///
    /// This uses a two-phase approach:
    /// 1. JS traversal collects the snapshot and element references
    /// 2. CDP calls resolve each element to its backendNodeId (in parallel)
    ///
    /// # Performance Optimizations
    ///
    /// - Uses `Runtime.getProperties` to batch-fetch all array element object IDs
    /// - Uses `FuturesUnordered` to resolve node IDs in parallel
    /// - Configurable concurrency limit to avoid overwhelming the browser
    #[instrument(level = "debug", skip(self, options), fields(target_id = %self.target_id))]
    async fn capture_snapshot_with_refs(
        &self,
        options: SnapshotOptions,
    ) -> Result<AriaSnapshot, PageError> {
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
        let snapshot_value = self
            .get_property_value(&result_object_id, "snapshot")
            .await?;

        // Parse the snapshot
        let mut snapshot: AriaSnapshot = serde_json::from_value(snapshot_value).map_err(|e| {
            PageError::EvaluationFailed(format!("Failed to parse aria snapshot: {e}"))
        })?;

        // Only resolve refs if requested
        if options.include_refs {
            // Get the elements array as a RemoteObject
            let elements_result = self
                .get_property_object(&result_object_id, "elements")
                .await?;

            if let Some(elements_object_id) = elements_result {
                // Batch-fetch all array element object IDs using Runtime.getProperties
                let element_object_ids = self
                    .get_all_array_elements(&elements_object_id)
                    .await?;
                let element_count = element_object_ids.len();

                debug!(
                    element_count = element_count,
                    max_concurrency = options.max_concurrency,
                    "Resolving element refs in parallel"
                );

                // Resolve all node IDs in parallel with concurrency limit
                let ref_map = self
                    .resolve_node_ids_parallel(element_object_ids, options.max_concurrency)
                    .await;

                debug!(
                    resolved_count = ref_map.len(),
                    total_count = element_count,
                    "Completed parallel ref resolution"
                );

                // Apply refs to the snapshot tree
                apply_refs_to_snapshot(&mut snapshot, &ref_map);

                // Release the elements array to free memory
                let _ = self.release_object(&elements_object_id).await;
            }
        }

        // Release the result object
        let _ = self.release_object(&result_object_id).await;

        Ok(snapshot)
    }

    /// Batch-fetch all array element object IDs using `Runtime.getProperties`.
    ///
    /// This replaces N individual `get_array_element()` calls with a single CDP call,
    /// significantly reducing round-trips for large arrays.
    async fn get_all_array_elements(
        &self,
        array_object_id: &str,
    ) -> Result<Vec<(usize, String)>, PageError> {
        #[derive(Debug, serde::Deserialize)]
        struct PropertyDescriptor {
            name: String,
            value: Option<viewpoint_cdp::protocol::runtime::RemoteObject>,
        }

        #[derive(Debug, serde::Deserialize)]
        struct GetPropertiesResult {
            result: Vec<PropertyDescriptor>,
        }

        let result: GetPropertiesResult = self
            .connection()
            .send_command(
                "Runtime.getProperties",
                Some(serde_json::json!({
                    "objectId": array_object_id,
                    "ownProperties": true,
                    "generatePreview": false
                })),
                Some(self.session_id()),
            )
            .await?;

        // Filter to numeric indices and extract object IDs
        let mut elements: Vec<(usize, String)> = Vec::new();

        for prop in result.result {
            // Parse numeric indices (array elements)
            if let Ok(index) = prop.name.parse::<usize>() {
                if let Some(value) = prop.value {
                    if let Some(object_id) = value.object_id {
                        elements.push((index, object_id));
                    }
                }
            }
        }

        // Sort by index to maintain order
        elements.sort_by_key(|(index, _)| *index);

        trace!(element_count = elements.len(), "Batch-fetched array elements");

        Ok(elements)
    }

    /// Resolve node IDs in parallel with a concurrency limit.
    ///
    /// Uses chunked processing with `FuturesUnordered` to limit concurrency
    /// and avoid overwhelming the browser's CDP connection.
    async fn resolve_node_ids_parallel(
        &self,
        element_object_ids: Vec<(usize, String)>,
        max_concurrency: usize,
    ) -> HashMap<usize, BackendNodeId> {
        let mut ref_map = HashMap::new();

        // Process in chunks to limit concurrency
        for chunk in element_object_ids.chunks(max_concurrency) {
            let futures: FuturesUnordered<_> = chunk
                .iter()
                .map(|(index, object_id)| {
                    let index = *index;
                    let object_id = object_id.clone();
                    async move {
                        match self.describe_node(&object_id).await {
                            Ok(backend_node_id) => {
                                trace!(
                                    index = index,
                                    backend_node_id = backend_node_id,
                                    "Resolved element ref"
                                );
                                Some((index, backend_node_id))
                            }
                            Err(e) => {
                                debug!(index = index, error = %e, "Failed to get backendNodeId for element");
                                None
                            }
                        }
                    }
                })
                .collect();

            // Collect all results from this chunk
            let results: Vec<_> = futures.collect().await;
            for result in results.into_iter().flatten() {
                ref_map.insert(result.0, result.1);
            }
        }

        ref_map
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
///
/// This function is used by both Page and Frame implementations to resolve
/// element references after capturing an aria snapshot with element indices.
pub(crate) fn apply_refs_to_snapshot(snapshot: &mut AriaSnapshot, ref_map: &HashMap<usize, BackendNodeId>) {
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
