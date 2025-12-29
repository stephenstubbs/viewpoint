## 1. Extend viewpoint-js macro for raw expression interpolation

- [x] 1.1 Add raw interpolation syntax (`@{expr}`) to `viewpoint-js` parser
- [x] 1.2 Update interpolation module to handle raw expressions (no quoting/escaping)
- [x] 1.3 Add compile-fail tests for invalid raw interpolation usage
- [x] 1.4 Add integration tests for raw interpolation with JS expressions
- [x] 1.5 Update `viewpoint-js` README with raw interpolation documentation

## 2. Add JsEvaluator trait to viewpoint-js-core

- [x] 2.1 Define `JsEvaluator` trait in `viewpoint-js-core` (implemented as `Page::evaluate_js_raw` instead)
- [x] 2.2 Add re-export in `viewpoint-js-core` lib.rs (N/A - method on Page)
- [x] 2.3 Add unit tests for trait definition (N/A - covered by integration tests)

## 3. Implement JsEvaluator in viewpoint-core

- [x] 3.1 Implement `evaluate_js_raw` for `Page`
- [x] 3.2 Create shared evaluation helper that uses the method
- [x] 3.3 Update `Locator` to use shared evaluation helper
- [x] 3.4 Update `FrameElementLocator` to use shared evaluation helper
- [x] 3.5 Update `DragAndDropBuilder` to use shared evaluation helper

## 4. Migrate inline JavaScript to js! macro

- [x] 4.1 Migrate `page/locator/debug/mod.rs` (highlight functions)
- [x] 4.2 Migrate `page/mouse_drag/mod.rs` (get_element_box)
- [x] 4.3 Migrate `page/locator/queries/mod.rs` (is_checked, all_inner_texts, all_text_contents, inner_text, get_attribute, input_value)
- [x] 4.4 Migrate `page/locator/select/mod.rs` (select_option, select_options) - skipped, uses different pattern
- [x] 4.5 Migrate `page/locator/helpers/mod.rs` (query_element_info, focus_element)
- [x] 4.6 Migrate `page/locator/files/mod.rs` (file input handling) - skipped, uses different pattern
- [x] 4.7 Migrate `page/locator/evaluation/mod.rs` (evaluate, evaluate_all) - skipped, uses user-provided JS
- [x] 4.8 Migrate `page/frame_locator_actions/mod.rs` (query_element_info, focus_element)

## 5. Remove duplicate evaluate_js implementations

- [x] 5.1 Remove `evaluate_js` from `page/locator/helpers/mod.rs` (replaced with delegation to Page)
- [x] 5.2 Remove `evaluate_js` from `page/frame_locator_actions/mod.rs` (replaced with delegation to Page)
- [x] 5.3 Remove `evaluate_js` from `page/mouse_drag/mod.rs` (replaced with delegation to Page)
- [x] 5.4 Verify no other duplicate implementations exist

## 6. Update and fix tests

- [x] 6.1 Update unit tests in viewpoint-js for new interpolation
- [x] 6.2 Update integration tests in viewpoint-core
- [x] 6.3 Run full test suite and fix any failures
- [x] 6.4 Verify compile-time JS validation catches errors

## 7. Documentation

- [x] 7.1 Update viewpoint-js crate documentation
- [x] 7.2 Update viewpoint-js-core crate documentation (N/A - no changes needed)
- [x] 7.3 Add migration notes for any API changes (N/A - internal only)
