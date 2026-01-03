# Tasks

## 1. Select Module (select/mod.rs)

- [x] 1.1 Add `select_option_by_backend_id` helper method
- [x] 1.2 Add `select_options_by_backend_id` helper method
- [x] 1.3 Update `select_option_internal` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 1.4 Update `select_options_internal` to check for `Selector::Ref` and `Selector::BackendNodeId`

## 2. Files Module (files/mod.rs)

- [x] 2.1 Add `set_input_files_by_backend_id` helper method (or refactor to use shared pattern)
- [x] 2.2 Add `set_input_files_from_buffer_by_backend_id` helper method
- [x] 2.3 Update `set_input_files` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 2.4 Update `set_input_files_from_buffer` to check for `Selector::Ref` and `Selector::BackendNodeId`

## 3. Evaluation Module (evaluation/mod.rs)

- [x] 3.1 Add `evaluate_by_backend_id` helper method
- [x] 3.2 Add `evaluate_all_by_backend_id` helper method
- [x] 3.3 Add `element_handle_by_backend_id` helper method
- [x] 3.4 Add `scroll_into_view_by_backend_id` helper method
- [x] 3.5 Update `evaluate` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 3.6 Update `evaluate_all` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 3.7 Update `element_handle` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 3.8 Update `scroll_into_view_if_needed` to check for `Selector::Ref` and `Selector::BackendNodeId`

## 4. Debug Module (debug/mod.rs)

- [x] 4.1 Add `highlight_by_backend_id` helper method
- [x] 4.2 Update `highlight_for` to check for `Selector::Ref` and `Selector::BackendNodeId`

## 5. Queries Module (queries/mod.rs)

- [x] 5.1 Add `is_checked_by_backend_id` helper method
- [x] 5.2 Add `all_inner_texts_by_backend_id` helper method
- [x] 5.3 Add `all_text_contents_by_backend_id` helper method
- [x] 5.4 Add `inner_text_by_backend_id` helper method
- [x] 5.5 Add `get_attribute_by_backend_id` helper method
- [x] 5.6 Add `input_value_by_backend_id` helper method
- [x] 5.7 Update `is_checked` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 5.8 Update `all_inner_texts` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 5.9 Update `all_text_contents` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 5.10 Update `inner_text` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 5.11 Update `get_attribute` to check for `Selector::Ref` and `Selector::BackendNodeId`
- [x] 5.12 Update `input_value` to check for `Selector::Ref` and `Selector::BackendNodeId`

## 6. Aria Snapshot Module (aria_snapshot_impl.rs)

- [x] 6.1 Add `aria_snapshot_by_backend_id` helper method
- [x] 6.2 Update `aria_snapshot` to check for `Selector::Ref` and `Selector::BackendNodeId`

## 7. Screenshot Element Module (screenshot_element/mod.rs)

- [x] 7.1 Add `get_element_bounding_box_by_backend_id` helper method
- [x] 7.2 Update `get_element_bounding_box` to check for `Selector::Ref` and `Selector::BackendNodeId`

## 8. Viewpoint-Test Locator Helpers (locator_helpers.rs)

- [x] 8.1 Add ref resolution helper for test assertions
- [x] 8.2 Update `get_input_value` to handle `Selector::Ref` and `Selector::BackendNodeId`
- [x] 8.3 Update `get_selected_values` to handle `Selector::Ref` and `Selector::BackendNodeId`
- [x] 8.4 Update `get_attribute` to handle `Selector::Ref` and `Selector::BackendNodeId`
- [x] 8.5 Update `is_enabled` to handle `Selector::Ref` and `Selector::BackendNodeId`

## 9. Integration Tests

- [x] 9.1 Add test: select option via ref from aria snapshot
- [x] 9.2 Add test: select multiple options via ref
- [x] 9.3 Add test: set_input_files via ref
- [x] 9.4 Add test: evaluate on element via ref
- [x] 9.5 Add test: element_handle via ref
- [x] 9.6 Add test: scroll_into_view via ref
- [x] 9.7 Add test: highlight via ref
- [x] 9.8 Add test: query methods (is_checked, inner_text, get_attribute, input_value) via ref
- [x] 9.9 Add test: aria_snapshot on locator via ref
- [x] 9.10 Add test: element screenshot via ref
- [x] 9.11 Add test: all_inner_texts and all_text_contents via ref
- [x] 9.12 Add test: bounding_box via ref

## 10. Validation

- [x] 10.1 Run `cargo test --workspace` to verify unit tests pass
- [x] 10.2 Run `cargo test --workspace --features integration` to verify integration tests pass
- [x] 10.3 Run `cargo clippy --workspace` to verify no new errors (warnings exist but are cosmetic)
