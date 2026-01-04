# Design: Fix Iframe Element Ref Resolution

## Context

The `aria_snapshot_with_frames()` method captures accessibility snapshots from all frames and stitches them together. However, element refs from child frames cannot be resolved because the backendNodeId mappings are discarded during capture.

This is a critical bug for MCP server integrations (like viewpoint-mcp) that rely on ref-based element interaction.

## Goals

- Enable `page.locator_from_ref()` to resolve refs from any frame captured by `aria_snapshot_with_frames()`
- Maintain backward compatibility with existing API
- Minimal changes to Frame internals

## Non-Goals

- Changing the ref format
- Modifying how refs are generated
- Adding new public APIs

## Decision: Return Refs from Frame Snapshot Capture

### Problem

`Frame.aria_snapshot_with_options()` calls `apply_refs_to_snapshot()` which returns a `HashMap<String, BackendNodeId>` mapping refs to their backend node IDs. Currently this mapping is discarded:

```rust
// Current code - mappings discarded
let _ = apply_refs_to_snapshot(&mut snapshot, &ref_map, ...);
```

### Solution

Modify `Frame.aria_snapshot_with_options()` to return both the snapshot AND the ref mappings. The Page then collects these mappings and stores them in its ref_map.

**Option A (Chosen): Return tuple from internal method**
```rust
// Frame returns snapshot + ref mappings
pub(super) async fn capture_snapshot_with_refs(
    &self,
    options: SnapshotOptions,
) -> Result<(AriaSnapshot, HashMap<String, BackendNodeId>), PageError>
```

**Option B: New return type struct**
- Create `SnapshotWithRefs { snapshot, ref_mappings }`
- More explicit but adds a new type

**Option C: Pass Page's ref_map to Frame**
- Frame writes directly to Page's ref_map
- Rejected: Increases coupling, requires Arc<RwLock> threading

### Implementation

1. **Modify `Frame.capture_snapshot_with_refs()`** to return the ref mappings:

```rust
// In page/frame/aria.rs
pub(super) async fn capture_snapshot_with_refs(
    &self,
    options: SnapshotOptions,
) -> Result<(AriaSnapshot, HashMap<String, BackendNodeId>), PageError> {
    // ... existing capture code ...
    
    let ref_mappings = apply_refs_to_snapshot(
        &mut snapshot,
        &ref_map,
        self.context_index,
        self.page_index,
        self.frame_index,
    );
    
    Ok((snapshot, ref_mappings))
}
```

2. **Update public `Frame.aria_snapshot_with_options()`** to discard mappings (maintains API):

```rust
pub async fn aria_snapshot_with_options(
    &self,
    options: SnapshotOptions,
) -> Result<AriaSnapshot, PageError> {
    let (snapshot, _) = self.capture_snapshot_with_refs(options).await?;
    Ok(snapshot)
}
```

3. **Update `Page.aria_snapshot_with_frames()`** to collect and store mappings:

```rust
// In page/aria_snapshot/mod.rs
pub async fn aria_snapshot_with_frames(&self) -> Result<AriaSnapshot, PageError> {
    // ... capture main frame ...
    
    // Capture child frames in parallel
    let frame_futures: FuturesUnordered<_> = child_frames
        .iter()
        .map(|frame| {
            async move {
                match frame.capture_snapshot_with_refs(opts).await {
                    Ok((snapshot, ref_mappings)) => Some((frame_id, snapshot, ref_mappings)),
                    Err(e) => None,
                }
            }
        })
        .collect();
    
    // Collect results and store ref mappings
    for result in results.into_iter().flatten() {
        let (frame_id, snapshot, ref_mappings) = result;
        
        // Store child frame refs in Page's ref_map
        for (ref_str, backend_node_id) in ref_mappings {
            self.store_ref_mapping(ref_str, backend_node_id);
        }
        
        // ... stitch snapshot ...
    }
}
```

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| Internal API change | Low | Only affects `capture_snapshot_with_refs`, not public API |
| Memory for ref storage | Low | Refs are small; typical pages have <1000 elements |
| Cross-frame ref conflicts | None | Ref format includes frame index (f0, f1, etc.) |

## Testing

1. Integration test: Click button inside iframe after `aria_snapshot_with_frames()`
2. Integration test: Type into input inside nested iframe
3. Integration test: Verify refs from different frames don't conflict
4. Unit test: Verify ref mappings are returned from Frame capture
