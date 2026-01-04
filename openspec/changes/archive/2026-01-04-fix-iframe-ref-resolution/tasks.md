# Tasks: Fix Iframe Element Ref Resolution

## 1. Implementation

- [x] 1.1 Modify `Frame.capture_snapshot_with_refs()` to return `(AriaSnapshot, HashMap<String, BackendNodeId>)`
- [x] 1.2 Update `Frame.aria_snapshot_with_options()` to extract snapshot from tuple (maintains public API)
- [x] 1.3 Update `Page.aria_snapshot_with_frames()` to collect ref mappings from child frame captures
- [x] 1.4 Store child frame ref mappings in Page's ref_map via `store_ref_mapping()`

## 2. Testing

- [x] 2.1 Add integration test: click element inside iframe after `aria_snapshot_with_frames()`
- [x] 2.2 Add integration test: type into input inside iframe after `aria_snapshot_with_frames()`
- [x] 2.3 Add integration test: interact with elements in nested iframes (iframe within iframe)
- [x] 2.4 Add integration test: verify refs from different frames have correct frame index (f0, f1, f2)
- [x] 2.5 Run full integration test suite to verify no regressions

## 3. Documentation

- [x] 3.1 Update doc comments on `aria_snapshot_with_frames()` to mention iframe ref resolution
- [x] 3.2 Add example showing iframe element interaction via refs

## Dependencies

- All tasks in section 1 must complete before section 2
- Task 2.5 depends on all other 2.x tasks

## Validation

```bash
# Run unit tests
cargo test --workspace

# Run integration tests (requires Chromium)
cargo test --workspace --features integration
```
