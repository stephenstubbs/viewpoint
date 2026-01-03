# Change: Refactor Codebase to Follow project.md Conventions

## Why

The codebase has several violations of the project.md conventions that need to be addressed:

1. **JavaScript Convention Violations**: Significant inline JavaScript constructed using `format!` with raw string literals, bypassing the compile-time syntax validation provided by the `js!` macro. This violates: "Always use `viewpoint_js::js!` macro for inline JavaScript code."

2. **File Size Violations**: Multiple files exceed the 500-line limit. This violates: "Maximum 500 lines per file — refactor into smaller modules if exceeded"

3. **Inline Test Module Violations**: 40 files have inline `#[cfg(test)] mod tests` blocks instead of folder modules. This violates: "No inline tests (`#[cfg(test)] mod tests` blocks)"

## What Changes

### JavaScript Macro Refactoring (Partially Complete)

**Completed:**
- `page/locator/evaluation/mod.rs`
- `page/locator/files/mod.rs`
- `page/locator/queries/mod.rs`
- `page/locator/select/mod.rs`
- `page/locator/helpers/mod.rs`
- `page/locator/aria_snapshot_impl.rs`
- `page/locator/debug/mod.rs`
- `page/screenshot_element/mod.rs`
- `expect/locator_helpers.rs` (viewpoint-test)

**Pending:**
- `page/frame_locator/mod.rs` - format! JS for frame access
- `page/binding/mod.rs` - format! JS for function exposure
- `context/storage/mod.rs` - format! JS for IndexedDB collection
- `context/storage_restore/mod.rs` - format! JS for restoration
- `context/types/storage/mod.rs` - format! JS for init scripts
- `page/aria_snapshot/cdp_helpers.rs` - format! JS for property access

### File Size Refactoring

Files over 500 lines that need to be split:
- `page/locator/evaluation/mod.rs` - 717 lines
- `page/locator/files/mod.rs` - 706 lines
- `page/locator/queries/mod.rs` - 572 lines

### Inline Test Module Refactoring

40 files across viewpoint-cdp and viewpoint-core need inline tests moved to `tests/` folder modules:
- viewpoint-cdp: 5 files
- viewpoint-core: 35 files

## Impact

- Affected specs: `js-validation` (modifying internal usage requirement)
- Affected code: ~60 files across `viewpoint-core`, `viewpoint-cdp`, and `viewpoint-test` crates
- No breaking changes: This is an internal refactoring
- Improved compile-time error detection for JavaScript code
- Better code organization following project conventions
