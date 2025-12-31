## 1. Update AriaSnapshot Struct
- [x] 1.1 Add `ref` field to `AriaSnapshot` struct with `Option<String>` type
- [x] 1.2 Update `AriaSnapshot::to_yaml()` to include ref in output (e.g., `[ref=e123]`)
- [x] 1.3 Update `AriaSnapshot::from_yaml()` to parse ref attribute
- [x] 1.4 Add unit tests for ref serialization/deserialization

## 2. Capture backendNodeId in JavaScript Snapshot
- [x] 2.1 Modify `aria_snapshot_js()` to call back for `backendNodeId` for each element
- [x] 2.2 Format refs as `e{backendNodeId}` in the JavaScript return value
- [x] 2.3 Ensure dynamically created elements get valid refs

## 3. Add Ref Resolution API
- [x] 3.1 Add `Page::element_from_ref(ref: &str) -> Result<ElementHandle>` method
- [x] 3.2 Add `Page::locator_from_ref(ref: &str) -> Locator` method for auto-waiting behavior
- [x] 3.3 Implement CDP `DOM.resolveNode` call using parsed `backendNodeId`

## 4. Integration Tests
- [x] 4.1 Test snapshot includes refs for all interactive elements
- [x] 4.2 Test snapshot includes refs for non-interactive elements (headings, text)
- [x] 4.3 Test ref resolution and subsequent click action
- [x] 4.4 Test ref resolution for dynamically created elements
- [x] 4.5 Test error handling for invalid/stale refs

## 5. Documentation
- [x] 5.1 Update AriaSnapshot struct documentation
- [x] 5.2 Add examples showing ref-based interaction workflow
