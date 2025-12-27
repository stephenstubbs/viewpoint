# Change: Refactor codebase to match project.md module conventions

## Why

The codebase currently violates multiple conventions defined in `project.md`:
1. **129+ single `.rs` files** that should be folder modules with `mod.rs`
2. **35+ inline test blocks** (`#[cfg(test)] mod tests`) that should be in separate `tests/` folders
3. **1 file over 500 lines** (`keyboard/keys.rs` at 1092 lines)
4. **Missing `integration` feature flag** - No crate defines this feature, and no integration test uses `#![cfg(feature = "integration")]` as required by `project.md`

These violations make the codebase inconsistent with documented standards, harder to maintain, and create friction for contributors expecting the documented structure.

## What Changes

### Folder Module Refactoring
- Convert all single `.rs` files to folder modules with `mod.rs`
- Applies to all crates: viewpoint-cdp (20 files), viewpoint-core (102 files), viewpoint-js (3 files), viewpoint-test (3 files), viewpoint-test-macros (1 file)

### Inline Test Extraction
- Move all `#[cfg(test)] mod tests` blocks to proper `tests/` subfolders within each module
- Replace with `#[cfg(test)] mod tests;` declarations referencing external test modules
- Applies to: viewpoint-cdp (4 files), viewpoint-core (27 files), viewpoint-js (3 files), viewpoint-test (1 file)

### File Size Reduction
- Refactor `viewpoint-core/src/page/keyboard/keys.rs` (1092 lines) into smaller modules
- Monitor and preemptively split files approaching 500 lines (14 files at 400-500 lines)

### Integration Test Feature Flag
- Add `integration = []` feature to Cargo.toml for: viewpoint-cdp, viewpoint-core, viewpoint-test
- Add `#![cfg(feature = "integration")]` to all integration test files that require Chromium
- Update test documentation to use `cargo test --features integration`

## Impact

- **Affected specs**: `code-quality` (MODIFIED)
- **Affected code**: All crates in the workspace
- **Breaking changes**: None (internal refactoring only, public API unchanged)
- **Risk**: Low - structural refactoring with no behavior changes
