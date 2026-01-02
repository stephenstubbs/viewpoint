//! Ref resolution helpers for ARIA snapshots.
//!
//! This module contains helpers for resolving element references
//! and applying them to snapshot trees.

use std::collections::HashMap;

use viewpoint_cdp::protocol::dom::BackendNodeId;

use crate::page::locator::AriaSnapshot;
use crate::page::ref_resolution::format_ref;

/// Recursively apply refs to the snapshot tree based on element indices.
///
/// This function is used by both Page and Frame implementations to resolve
/// element references after capturing an aria snapshot with element indices.
pub(crate) fn apply_refs_to_snapshot(
    snapshot: &mut AriaSnapshot,
    ref_map: &HashMap<usize, BackendNodeId>,
) {
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
