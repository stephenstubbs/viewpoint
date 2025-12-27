# Change: Enhance Unit and Integration Tests

## Prerequisites

**This proposal depends on:** `add-js-validation-macro` (COMPLETED), `refactor-module-conventions` (COMPLETED)

The `viewpoint-js` crate (providing the `js!` macro) and the module restructuring have been completed.

## Why

Viewpoint is a testing framework that will be used to test other projects. Following the recent `refactor-module-conventions` change:

1. **Module structure has changed** - All modules now use folder structure (`module/mod.rs`)
2. **Test organization has changed** - Tests should be in `tests/` subdirectories within each module
3. **Integration tests are now feature-gated** - Using `#![cfg(feature = "integration")]`
4. **Many previously "completed" tasks need re-verification** - The project structure changed significantly

The current test suite needs verification and updates to ensure:
- All tests follow the new code-quality standards
- No inline `#[cfg(test)] mod tests` blocks remain
- All features have proper test coverage
- Integration tests are properly gated

## What Changes

### Phase A: Verify and Update Test Organization (NEW - Required First)

1. **Audit existing inline tests** - Identify all `#[cfg(test)] mod tests` blocks that need migration
2. **Migrate tests to external directories** - Move inline tests to `<module>/tests/mod.rs` structure
3. **Verify integration feature gating** - Ensure all browser-requiring tests use `#![cfg(feature = "integration")]`
4. **Verify folder module compliance** - All modules should follow folder structure

### Phase B: Re-verify Previously Completed Tasks

Many tasks were marked complete before the module restructuring. These need re-verification:

1. **viewpoint-js tests** - Verify tests are in correct locations
2. **viewpoint-cdp tests** - Verify unit tests migrated to external test directories
3. **viewpoint-core tests** - Verify all unit tests use new structure
4. **viewpoint-test tests** - Verify test framework tests follow conventions

### Phase C: Complete Remaining Test Coverage

The following test areas still have gaps:

1. **Network event capture tests** (8.3-8.7, 8.9-8.10)
2. **Context integration tests** (9.4-9.7)
3. **Frame integration tests** (10.2, 10.4-10.5)
4. **Dialog/Download tests** (11.1-11.6)
5. **Advanced feature tests** (12.2, 12.6-12.8)
6. **viewpoint-test unit/integration tests** (13.x, 14.x)
7. **viewpoint-test-macros tests** (15.x)
8. **Cross-crate integration tests** (16.x)

## Current State Assessment

Based on current audit:
- **35 inline test modules** still use `#[cfg(test)] mod tests` pattern
- **Integration tests** are properly feature-gated
- **Folder modules** are implemented but tests not fully migrated
- **83 of 130 tasks** were previously marked complete

## Impact

- **Affected crates**: All 5 workspace crates
- **Structural changes**: Test files moved to external directories
- **No functional changes**: Tests remain the same, just reorganized
- **CI implications**: `cargo test` runs unit tests only, `cargo test --features integration` runs all

## Success Criteria

1. **Zero inline test modules** - All `#[cfg(test)] mod tests` blocks migrated
2. **Folder module compliance** - All modules use folder structure per code-quality spec
3. **Integration tests gated** - All browser tests require `--features integration`
4. **All spec scenarios tested** - Each spec requirement has corresponding tests
5. **All tests pass** - `cargo test --workspace --features integration` succeeds
6. **All JavaScript uses js! macro** - No raw string literals for JavaScript code
