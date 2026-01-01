# Tasks: Fix ARIA Accessible Name Extraction

## 1. Analysis
- [x] 1.1 Review W3C Accessible Name Computation specification
- [x] 1.2 Identify all roles that allow "name from content"
- [x] 1.3 Document current vs expected behavior

## 2. Implementation
- [x] 2.1 Update `getAccessibleName()` in `snapshot_basic.rs` to use text content as fallback for elements with roles that allow name from content
- [x] 2.2 Update `getAccessibleName()` in `snapshot_with_refs.rs` with the same fix
- [x] 2.3 Add list of roles that allow name from content (headings, listitem, cell, option, etc.)

## 3. Testing
- [x] 3.1 Add unit tests for accessible name extraction from headings
- [x] 3.2 Add unit tests for accessible name extraction from list items
- [x] 3.3 Add unit tests for accessible name extraction from table cells
- [x] 3.4 Verify existing ARIA snapshot tests still pass

## 4. Validation
- [x] 4.1 Run `cargo test` to verify all tests pass
- [x] 4.2 Run `cargo doc --no-deps` to ensure docs build
- [x] 4.3 Manual verification with real web page (verified through integration tests)
