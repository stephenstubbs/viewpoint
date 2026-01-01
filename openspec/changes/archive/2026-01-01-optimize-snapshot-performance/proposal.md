# Change: Optimize ARIA Snapshot Performance Using Parallelization

## Why

ARIA snapshot capture is a critical path operation for accessibility testing and MCP integrations. The current implementation has significant performance bottlenecks:

1. **Sequential CDP calls**: For each element in the snapshot, we call `DOM.describeNode` sequentially (O(n) round trips to browser)
2. **Sequential frame processing**: Multi-frame snapshots capture each frame one-by-one
3. **Synchronous JS traversal**: The JavaScript DOM traversal is recursive and doesn't leverage browser parallelism

For pages with 100+ elements, this results in noticeable latency (100+ sequential CDP round-trips).

## What Changes

### Rust-Side Parallelization
- **Batch CDP calls**: Use `futures::stream::FuturesUnordered` to send multiple `DOM.describeNode` requests concurrently
- **Configurable concurrency limit**: Allow tuning the max concurrent CDP calls (default ~50) to avoid overwhelming the browser
- **Parallel frame capture**: Capture multiple frame snapshots concurrently using `join_all`

### JavaScript-Side Optimization
- **Batched node ID resolution**: Collect all elements first, then resolve backend node IDs in a single batched CDP call using `DOM.getNodesForSubtreeByStyle` or `Runtime.getProperties` with bulk array access
- **Optimized traversal**: Use non-recursive iteration with an explicit stack to reduce call stack overhead for deep trees

### API Changes (Non-Breaking)
- Add optional `SnapshotOptions` parameter for tuning performance characteristics
- Maintain backward compatibility - existing API signatures unchanged

## Impact

- Affected specs: `advanced-locators` (Aria Snapshot, Frame Aria Snapshot, Multi-Frame Aria Snapshot requirements)
- Affected code:
  - `crates/viewpoint-core/src/page/aria_snapshot/mod.rs`
  - `crates/viewpoint-core/src/page/frame/aria.rs`
  - `crates/viewpoint-core/src/page/locator/aria_js/snapshot_with_refs.rs`
  - `crates/viewpoint-cdp/src/connection/mod.rs` (potential batch command support)
