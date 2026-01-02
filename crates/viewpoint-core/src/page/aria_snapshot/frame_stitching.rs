//! Frame content stitching for ARIA snapshots.
//!
//! This module handles stitching content from child frames into
//! the parent snapshot at iframe boundaries.

use std::collections::HashMap;

use tracing::{debug, warn};

use crate::page::locator::AriaSnapshot;

/// Recursively stitch frame content into aria snapshot at iframe boundaries.
///
/// This function traverses the snapshot tree looking for nodes with `is_frame: true`.
/// When found, it attempts to find the corresponding frame snapshot and adds that
/// content as children of the iframe node.
pub(crate) fn stitch_frame_content(
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
