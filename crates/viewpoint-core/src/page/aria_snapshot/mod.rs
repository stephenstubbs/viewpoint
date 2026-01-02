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

mod cdp_helpers;
mod frame_stitching;
mod options;
mod ref_resolution;

use std::collections::HashMap;

use futures::stream::{FuturesUnordered, StreamExt};
use tracing::{debug, instrument};
use viewpoint_js::js;

use self::frame_stitching::stitch_frame_content;
pub use self::options::SnapshotOptions;
pub(crate) use self::ref_resolution::apply_refs_to_snapshot;
use super::Page;
use super::locator::AriaSnapshot;
use super::locator::aria_js::aria_snapshot_with_refs_js;
use crate::error::PageError;

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
                            tracing::warn!(
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
                let element_object_ids = self.get_all_array_elements(&elements_object_id).await?;
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
}
