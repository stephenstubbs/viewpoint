## Completed: JavaScript Macro Refactoring

### 1. viewpoint-core Locator Module Refactoring

- [x] 1.1 Refactor `page/locator/queries/mod.rs` - convert `format!` JS to `js!` macro
- [x] 1.2 Refactor `page/locator/evaluation/mod.rs` - convert `format!` JS to `js!` macro
- [x] 1.3 Refactor `page/locator/files/mod.rs` - convert `format!` JS to `js!` macro
- [x] 1.4 Refactor `page/locator/select/mod.rs` - convert `format!` JS to `js!` macro and helper functions
- [x] 1.5 Refactor `page/locator/helpers/mod.rs` - convert `format!` JS to `js!` macro
- [x] 1.6 Refactor `page/locator/aria_snapshot_impl.rs` - convert JS code variables to `js!` macro
- [x] 1.7 Refactor `page/locator/debug/mod.rs` - convert JS code to `js!` macro

### 2. viewpoint-core Other Modules Refactoring

- [x] 2.1 Refactor `page/screenshot_element/mod.rs` - convert JS code to `js!` macro

### 3. viewpoint-test Refactoring

- [x] 3.1 Refactor `expect/locator_helpers.rs` - convert `format!` JS to `js!` macro
  - Added `viewpoint-js` and `viewpoint-js-core` dependencies to Cargo.toml
  - Removed unused `js_string_literal` function

### 4. Bug Fix During Implementation

- [x] 4.1 Fixed `js!` macro token handling - added space before `#` and `@` interpolation markers when following an identifier

---

## Completed: Additional JavaScript Macro Refactoring

### 5. frame_locator Module

- [x] 5.1 `page/frame_locator/mod.rs` - converted to `js!` macro

### 6. binding Module

- [x] 6.1 `page/binding/mod.rs` - converted to `js!` macro

### 7. storage Modules

- [x] 7.1 `context/storage/mod.rs` - converted to `js!` macro
- [x] 7.2 `context/storage_restore/mod.rs` - converted to `js!` macro
- [x] 7.3 `context/types/storage/mod.rs` - converted to `js!` macro

### 8. aria_snapshot Helpers

- [x] 8.1 `page/aria_snapshot/cdp_helpers.rs` - lines 137, 164 use `format!` for property access JS
  - Also fixed `page/frame/aria.rs` - lines 319, 346 same pattern

---

## Completed: File Size Refactoring (>500 lines)

Per project.md: "Maximum 500 lines per file — refactor into smaller modules if exceeded"

### 9. Split Large Files

- [x] 9.1 `page/locator/evaluation/mod.rs` - 717 lines → split into submodules
  - Split into: evaluate.rs (209), evaluate_all.rs (204), element_handle.rs (138), scroll.rs (152), bounding_box.rs (54)
- [x] 9.2 `page/locator/files/mod.rs` - 706 lines → split into submodules
  - Split into: set_input_files.rs (298), set_input_files_buffer.rs (418)
- [x] 9.3 `page/locator/queries/mod.rs` - 572 lines → split into submodules
  - Split into: state.rs (126), text.rs (231), attributes.rs (148), helpers.rs (93)

---

## Completed: Inline Test Module Refactoring

Per project.md: "No inline tests (`#[cfg(test)] mod tests` blocks)" - tests should be in `tests/` folder modules

### 10. Move Inline Tests to Folder Modules

**Analysis:** Most files already use folder modules (`mod tests;`). Only 3 files had inline tests:

**viewpoint-core crate:**
- [x] 10.1 `page/ref_resolution/mod.rs` - converted inline tests to `tests/mod.rs`
- [x] 10.2 `context/types/proxy.rs` - converted to folder module `proxy/mod.rs` with `tests/mod.rs`
- [x] 10.3 `wait/navigation_waiter/mod.rs` - converted inline tests to `tests/mod.rs`

**Already using folder modules (no changes needed):**
- viewpoint-cdp: All test modules already use `mod tests;` pattern
- viewpoint-core: All other test modules already use `mod tests;` pattern  
- viewpoint-js: All test modules already use `mod tests;` pattern
- viewpoint-test: All test modules already use `mod tests;` pattern

---

## Validation

- [x] Run `cargo test --workspace` - all tests pass
- [x] Run `cargo test --workspace --features integration` - all 254 integration tests pass
- [x] Run `cargo clippy --workspace` - no warnings

## Session 2 Validation (File Size + Inline Tests Refactoring)

- [x] Run `cargo test --workspace` - all tests pass
- [x] Run `cargo clippy --workspace` - no warnings
- [x] Run `cargo test --workspace --features integration` - all integration tests pass
