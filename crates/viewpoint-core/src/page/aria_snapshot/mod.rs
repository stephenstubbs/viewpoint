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

use tracing::{debug, instrument, warn};

use super::locator::AriaSnapshot;
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
        stitch_frame_content(&mut root_snapshot, &frame_snapshots);

        Ok(root_snapshot)
    }

    /// Capture an ARIA accessibility snapshot of just the main frame.
    ///
    /// This is a convenience method equivalent to calling `main_frame().await?.aria_snapshot().await`.
    /// Unlike `aria_snapshot_with_frames()`, this does NOT stitch in iframe content -
    /// iframes are left as boundaries with `is_frame: true`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Quick snapshot without frame content
    /// let snapshot = page.aria_snapshot().await?;
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

        let main_frame = self.main_frame().await?;
        main_frame.aria_snapshot().await
    }
}

/// Recursively stitch frame content into aria snapshot at iframe boundaries.
///
/// This function traverses the snapshot tree looking for nodes with `is_frame: true`.
/// When found, it attempts to find the corresponding frame snapshot and adds that
/// content as children of the iframe node.
fn stitch_frame_content(snapshot: &mut AriaSnapshot, frame_snapshots: &HashMap<String, AriaSnapshot>) {
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
                "Stitching frame content into snapshot"
            );

            // Add the frame's content as children of this iframe node
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
        stitch_frame_content(child, frame_snapshots);
    }
}
