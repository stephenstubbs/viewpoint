//! Ref resolution helpers for ARIA snapshots.
//!
//! This module contains helpers for resolving element references
//! and applying them to snapshot trees.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use viewpoint_cdp::protocol::dom::BackendNodeId;

use crate::page::locator::AriaSnapshot;
use crate::page::ref_resolution::format_ref;

/// Recursively apply refs to the snapshot tree based on element indices.
///
/// This function is used by both Page and Frame implementations to resolve
/// element references after capturing an aria snapshot with element indices.
///
/// The refs are generated in the format `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}`.
///
/// Returns a map of ref strings to their corresponding backendNodeIds, which
/// should be stored by the caller for later ref resolution.
pub(crate) fn apply_refs_to_snapshot(
    snapshot: &mut AriaSnapshot,
    index_to_backend_id: &HashMap<usize, BackendNodeId>,
    context_index: usize,
    page_index: usize,
    frame_index: usize,
) -> HashMap<String, BackendNodeId> {
    let counter = AtomicUsize::new(1);
    let mut ref_to_backend_id = HashMap::new();
    apply_refs_recursive(
        snapshot,
        index_to_backend_id,
        context_index,
        page_index,
        frame_index,
        &counter,
        &mut ref_to_backend_id,
    );
    ref_to_backend_id
}

/// Internal recursive helper for applying refs.
fn apply_refs_recursive(
    snapshot: &mut AriaSnapshot,
    index_to_backend_id: &HashMap<usize, BackendNodeId>,
    context_index: usize,
    page_index: usize,
    frame_index: usize,
    counter: &AtomicUsize,
    ref_to_backend_id: &mut HashMap<String, BackendNodeId>,
) {
    // Apply ref if this node has an element_index
    if let Some(index) = snapshot.element_index {
        if let Some(&backend_node_id) = index_to_backend_id.get(&index) {
            let element_counter = counter.fetch_add(1, Ordering::SeqCst);
            let ref_str = format_ref(context_index, page_index, frame_index, element_counter);
            snapshot.node_ref = Some(ref_str.clone());
            ref_to_backend_id.insert(ref_str, backend_node_id);
        }
        // Clear the element_index now that we've processed it
        snapshot.element_index = None;
    }

    // Recursively process children
    for child in &mut snapshot.children {
        apply_refs_recursive(
            child,
            index_to_backend_id,
            context_index,
            page_index,
            frame_index,
            counter,
            ref_to_backend_id,
        );
    }
}
