# Change: Fix locator operations to support Selector::Ref

## Why

Multiple locator operations fail when used with locators created from snapshot refs (`page.locator_from_ref(ref)`). When a user captures an accessibility snapshot, gets an element's ref, and attempts to interact with or query that element, the operations throw a JavaScript error: "Ref selectors must be resolved via Page ref map".

This is a bug introduced when `Selector::Ref` support was added - core locator operations (click, fill, hover, focus, etc.) were updated to handle refs via CDP resolution, but several other operations in different modules were missed.

## What Changes

### viewpoint-core/src/page/locator/select/mod.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `select_option_internal`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `select_options_internal`

### viewpoint-core/src/page/locator/files/mod.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `set_input_files`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `set_input_files_from_buffer`

### viewpoint-core/src/page/locator/evaluation/mod.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `evaluate`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `evaluate_all`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `element_handle`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `scroll_into_view_if_needed`

### viewpoint-core/src/page/locator/debug/mod.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `highlight_for`

### viewpoint-core/src/page/locator/queries/mod.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `is_checked`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `all_inner_texts`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `all_text_contents`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `inner_text`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `get_attribute`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `input_value`

### viewpoint-core/src/page/locator/aria_snapshot_impl.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `aria_snapshot`

### viewpoint-core/src/page/screenshot_element/mod.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `get_element_bounding_box`

### viewpoint-test/src/expect/locator_helpers.rs
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `get_input_value`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `get_selected_values`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `get_attribute`
- Add `Selector::Ref` and `Selector::BackendNodeId` handling to `is_enabled`

## Impact

- Affected specs:
  - `test-actions` (Select Option Action)
  - `file-uploads` (Set Input Files)
  - `javascript-evaluation` (Locator Evaluate)
  - `advanced-locators` (Locator Queries, Aria Snapshot, Highlight, Element Ref Resolution)
- Affected code: Multiple files in viewpoint-core and viewpoint-test crates
- No breaking changes - this is a bug fix that enables existing ref-based functionality to work correctly
