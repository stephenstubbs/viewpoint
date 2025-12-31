# Tasks

## 1. Update Frame aria_snapshot Implementation

- [x] 1.1 Import `aria_snapshot_with_refs_js` in `page/frame/mod.rs`
- [x] 1.2 Update `Frame.aria_snapshot()` to use `aria_snapshot_with_refs_js()` instead of `aria_snapshot_js()`
- [x] 1.3 Change `return_by_value` to `false` to get RemoteObjects
- [x] 1.4 Extract the snapshot and elements array from the result
- [x] 1.5 Iterate over elements array and call `DOM.describeNode` for each to get `backendNodeId`
- [x] 1.6 Build a ref map from element index to `backendNodeId`
- [x] 1.7 Apply refs to the snapshot tree using `apply_refs_to_snapshot()` (import or duplicate)
- [x] 1.8 Release RemoteObjects to free memory

## 2. Helper Functions

- [x] 2.1 Add helper method `Frame.get_property_value()` for getting property values
- [x] 2.2 Add helper method `Frame.get_property_object()` for getting property as RemoteObject
- [x] 2.3 Add helper method `Frame.get_array_element()` for array access
- [x] 2.4 Add helper method `Frame.describe_node()` for getting backendNodeId
- [x] 2.5 Add helper method `Frame.release_object()` for cleanup
- [x] 2.6 Import or expose `apply_refs_to_snapshot()` for use in Frame

## 3. Verification

- [x] 3.1 Run unit tests: `cargo test --workspace`
- [x] 3.2 Run integration tests: `cargo test --workspace --features integration`
- [x] 3.3 Verify `test_aria_snapshot_with_frames` includes refs in output
- [x] 3.4 Verify `test_aria_snapshot_includes_refs_for_buttons` still passes
- [x] 3.5 Add new integration test for `Frame.aria_snapshot()` with refs if not covered
