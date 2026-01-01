# Design: Snapshot Performance Optimization

## Context

The ARIA snapshot system captures accessibility trees from web pages. Performance is critical because:
- MCP integrations call `aria_snapshot()` frequently for agent interactions
- Multi-frame pages can have 100s of elements requiring backend node ID resolution
- Each `DOM.describeNode` CDP call is a separate round-trip (~1-5ms each)

Current bottleneck analysis:
```
Single Frame Snapshot (100 elements):
├── JS DOM traversal: ~5-10ms (fast)
├── Return snapshot + elements array: ~1ms
└── CDP describe_node x100: ~100-500ms (SLOW - sequential)
    └── Each call: 1-5ms round-trip

Multi-Frame Snapshot (5 frames, 50 elements each):
├── Frame 1: ~50-250ms
├── Frame 2: ~50-250ms (waits for Frame 1)
├── ...sequential...
└── Frame 5: ~50-250ms
Total: ~250-1250ms
```

Target performance:
```
Single Frame Snapshot (100 elements):
├── JS DOM traversal: ~5-10ms
├── Return snapshot + elements array: ~1ms
└── CDP describe_node (parallel, 50 concurrent): ~10-50ms
Total: ~16-61ms (10-20x improvement)

Multi-Frame Snapshot (5 frames parallel):
├── All frames concurrent: ~50-250ms (limited by slowest frame)
Total: ~50-250ms (5x improvement)
```

## Goals / Non-Goals

### Goals
- **10x+ improvement** for single-frame snapshot node resolution
- **Linear scaling** with number of frames (instead of multiplicative)
- **Non-breaking API** - existing code works unchanged
- **Configurable concurrency** for different environments (slow CI vs fast local)

### Non-Goals
- Changing the JavaScript traversal algorithm (already efficient)
- Supporting non-Chromium browsers (future work)
- Caching snapshots between calls (invalidation is complex)

## Decisions

### Decision 1: Use FuturesUnordered for Parallel CDP Calls

**What**: Replace sequential `for` loop with `FuturesUnordered` stream for `DOM.describeNode` calls.

**Why**: 
- Native Tokio support, no new dependencies
- Automatic backpressure handling
- Easy concurrency limit via `buffer_unordered(N)`

**Alternative considered**: Thread pool with blocking calls
- Rejected: CDP connection is async, would require awkward blocking wrappers

**Implementation**:
```rust
use futures::stream::{FuturesUnordered, StreamExt};

// Instead of:
for i in 0..element_count {
    let node_id = self.describe_node(&element_id).await?;
    ref_map.insert(i, node_id);
}

// Use:
let futures: FuturesUnordered<_> = (0..element_count)
    .map(|i| async move {
        let element_id = get_array_element(&elements_object_id, i).await?;
        let node_id = describe_node(&element_id).await?;
        Ok::<_, PageError>((i, node_id))
    })
    .collect();

let ref_map: HashMap<usize, BackendNodeId> = futures
    .buffer_unordered(50) // Limit concurrency
    .filter_map(|r| async { r.ok() })
    .collect()
    .await;
```

### Decision 2: Batch Array Element Access

**What**: Get all element object IDs in a single CDP call instead of one per element.

**Why**: Reduces N CDP calls to 1 for element array access.

**Implementation**: Use `Runtime.getProperties` to get all array elements at once:
```rust
let properties = connection.send_command(
    "Runtime.getProperties",
    Some(json!({
        "objectId": elements_object_id,
        "ownProperties": true,
        "generatePreview": false
    })),
    session_id
).await?;

// properties.result contains all array indices as properties
// Filter to numeric indices, extract object IDs
```

### Decision 3: Parallel Frame Capture for Multi-Frame Snapshots

**What**: Capture all child frame snapshots concurrently using `futures::future::join_all`.

**Why**: Frames are independent, no ordering dependency.

**Consideration**: Cross-origin frames may have different CDP sessions. Ensure session isolation.

**Implementation**:
```rust
let frame_futures: Vec<_> = frames
    .iter()
    .filter(|f| !f.is_main())
    .map(|frame| async {
        let snapshot = frame.aria_snapshot().await?;
        Ok::<_, PageError>((frame.id().to_string(), snapshot))
    })
    .collect();

let frame_results = futures::future::join_all(frame_futures).await;
let frame_snapshots: HashMap<String, AriaSnapshot> = frame_results
    .into_iter()
    .filter_map(|r| r.ok())
    .collect();
```

### Decision 4: Add SnapshotOptions for Configuration

**What**: Optional configuration struct for tuning behavior.

**Why**: Different environments have different optimal concurrency limits.

**Implementation**:
```rust
#[derive(Debug, Clone, Default)]
pub struct SnapshotOptions {
    /// Max concurrent CDP calls for node resolution (default: 50)
    pub max_concurrency: Option<usize>,
    /// Whether to include refs (default: true)
    pub include_refs: bool,
}

impl Page {
    pub async fn aria_snapshot(&self) -> Result<AriaSnapshot, PageError> {
        self.aria_snapshot_with_options(SnapshotOptions::default()).await
    }
    
    pub async fn aria_snapshot_with_options(
        &self, 
        options: SnapshotOptions
    ) -> Result<AriaSnapshot, PageError> {
        // ...
    }
}
```

## Risks / Trade-offs

### Risk 1: CDP Connection Saturation
- **Risk**: Too many concurrent calls could overwhelm CDP connection
- **Mitigation**: Default concurrency limit of 50, configurable via `SnapshotOptions`
- **Monitoring**: Add tracing spans for concurrent call count

### Risk 2: Browser Memory Pressure
- **Risk**: Holding many RemoteObject handles simultaneously
- **Mitigation**: Release object handles as soon as node ID is resolved
- **Implementation**: Use `releaseObjectGroup` after batch completion

### Risk 3: Error Handling Complexity
- **Risk**: Partial failures in concurrent operations harder to debug
- **Mitigation**: Continue on individual failures, log warnings, collect successful results
- **Semantics**: Match current behavior where single element failures don't abort snapshot

## Migration Plan

1. **Phase 1**: Add `FuturesUnordered` parallel node resolution (no API change)
2. **Phase 2**: Add batch array element access (no API change)
3. **Phase 3**: Add parallel frame capture (no API change)
4. **Phase 4**: Add `SnapshotOptions` for configuration (additive API)

All phases maintain backward compatibility. No breaking changes.

## Resolved Questions

### 1. Optimal Concurrency Limit

**Question**: What is the optimal default concurrency limit for CDP calls?

**Decision**: **50 concurrent calls** as the default.

**Rationale**:
- CDP WebSocket is single-threaded on the browser side, but can queue messages
- 50 provides good parallelism without overwhelming the message queue
- Empirical testing with Playwright shows similar concurrency patterns work well
- Configurable via `SnapshotOptions::max_concurrency` for edge cases
- Add `tracing` metrics to measure actual performance, allowing future tuning based on real data

### 2. Include Refs Option

**Question**: Should we expose an `include_refs: false` option to skip ref resolution?

**Decision**: **Yes**, add `include_refs: bool` to `SnapshotOptions` (default: `true`).

**Rationale**:
- Some use cases only need the accessibility tree structure (e.g., visual diffing, accessibility audits)
- Ref resolution is the most expensive part (~90% of snapshot time for large DOMs)
- Skipping refs provides an escape hatch for performance-critical paths
- Default `true` maintains backward compatibility

### 3. CDP Pipelining

**Question**: Can we pipeline CDP requests without waiting for responses to further improve throughput?

**Decision**: **Defer to future work**. The current concurrent approach is sufficient.

**Rationale**:
- `FuturesUnordered` with `buffer_unordered(50)` already achieves effective pipelining
- True pipelining would require CDP connection changes (complex)
- Expected 10-20x improvement from current approach meets performance goals
- Can revisit if benchmarks show CDP round-trip is still the bottleneck after this change
