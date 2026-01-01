# Tasks: Optimize Snapshot Performance

## 1. Parallel Node Resolution (Core Performance Gain)

- [x] 1.1 Add `futures` crate dependency to viewpoint-core Cargo.toml (for `FuturesUnordered`)
- [x] 1.2 Refactor `Page::capture_snapshot_with_refs()` to use parallel describe_node calls
  - Replace sequential loop with `FuturesUnordered`
  - Add concurrency limit (default 50)
  - Preserve error handling semantics (continue on individual failures)
- [x] 1.3 Refactor `Frame::capture_snapshot_with_refs()` with same parallel pattern
- [x] 1.4 Add tracing spans for parallel operation metrics

## 2. Batch Array Element Access

- [x] 2.1 Create `get_all_array_elements()` helper using `Runtime.getProperties`
- [x] 2.2 Replace N individual `get_array_element()` calls with single batch call
- [x] 2.3 Update both Page and Frame implementations

## 3. Parallel Frame Capture

- [x] 3.1 Refactor `Page::aria_snapshot_with_frames()` to capture child frames in parallel
  - Use `futures::future::join_all` for concurrent frame snapshots
  - Collect results and filter errors
- [x] 3.2 Ensure proper error logging for individual frame failures

## 4. Configuration Options

- [x] 4.1 Create `SnapshotOptions` struct in `page/aria_snapshot/mod.rs`
  - `max_concurrency: Option<usize>` (default 50)
  - `include_refs: bool` (default true)
- [x] 4.2 Add `aria_snapshot_with_options()` method to Page
- [x] 4.3 Add `aria_snapshot_with_options()` method to Frame
- [x] 4.4 Document new options in module docs

## 5. Testing & Validation

- [x] 5.1 Add integration test for large DOM snapshot performance
  - Generate page with 100+ elements
  - Verify snapshot completes within reasonable time (<500ms)
- [x] 5.2 Add integration test for multi-frame parallel capture
  - Page with 5+ iframes
  - Verify all frame content captured correctly
- [x] 5.3 Add test for `include_refs: false` option
- [x] 5.4 Verify existing snapshot tests still pass

## 6. Documentation

- [x] 6.1 Update aria_snapshot module docs with performance notes
- [x] 6.2 Add examples showing SnapshotOptions usage

## Dependencies

- Task 1 must complete before Task 2 (batch access builds on parallel infrastructure)
- Task 1 must complete before Task 3 (same reason)
- Tasks 2 and 3 can run in parallel
- Task 4 can run in parallel with 2 and 3
- Task 5 depends on all implementation tasks
- Task 6 depends on Task 4 (needs final API to document)

## Parallelizable Work

Tasks that can be worked on simultaneously:
- After Task 1 completes: Tasks 2, 3, and 4 can all proceed in parallel
- Task 5.1 and 5.2 can run concurrently
