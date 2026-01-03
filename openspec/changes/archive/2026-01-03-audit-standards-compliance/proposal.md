# Change: Audit Project Standards Compliance

## Why
The codebase has accumulated technical debt that violates the established code quality standards defined in `openspec/specs/code-quality/spec.md`. An audit reveals 10 files exceeding the 500-line limit, 19 clippy warnings, and 11 files with crate-level lint suppressions. These issues need to be addressed to maintain code quality and developer experience.

## What Changes
- Refactor 10 source/test files exceeding 500 lines into smaller, focused modules
- Fix 19 clippy warnings (mostly `single_char_pattern` and `uninlined_format_args`)
- Remove or justify 11 crate-level `#![allow(...)]` suppressions
- No new features; purely code quality improvements

## Impact
- Affected specs: `code-quality`
- Affected code:
  - `crates/viewpoint-core/src/page/mod.rs` (706 lines)
  - `crates/viewpoint-core/src/browser/mod.rs` (553 lines)
  - `crates/viewpoint-core/src/network/websocket/mod.rs` (531 lines)
  - `crates/viewpoint-core/src/network/handler/mod.rs` (520 lines)
  - `crates/viewpoint-core/src/page/locator/mod.rs` (510 lines)
  - `crates/viewpoint-core/src/context/mod.rs` (506 lines)
  - `crates/viewpoint-core/tests/aria_snapshot_ref_tests.rs` (930 lines)
  - `crates/viewpoint-core/tests/aria_snapshot_basic_tests.rs` (674 lines)
  - `crates/viewpoint-core/tests/locator_tests.rs` (555 lines)
  - `crates/viewpoint-test/src/expect/locator.rs` (500 lines)
  - 11 files with crate-level allow suppressions

## Audit Summary

### Compliant Areas
- Zero compiler warnings with `cargo check` 
- Integration tests properly gated with `#![cfg(feature = "integration")]`
- No inline test blocks (`#[cfg(test)] mod tests {}`)
- Workspace dependencies centralized in root `Cargo.toml`
- Folder module structure generally followed

### Violations Found

#### 1. File Size Violations (10 files over 500 lines)

| File | Lines | Priority |
|------|-------|----------|
| `tests/aria_snapshot_ref_tests.rs` | 930 | High |
| `src/page/mod.rs` | 706 | High |
| `tests/aria_snapshot_basic_tests.rs` | 674 | High |
| `tests/locator_tests.rs` | 555 | Medium |
| `src/browser/mod.rs` | 553 | High |
| `src/network/websocket/mod.rs` | 531 | Medium |
| `src/network/handler/mod.rs` | 520 | Medium |
| `src/page/locator/mod.rs` | 510 | Medium |
| `src/context/mod.rs` | 506 | Medium |
| `src/expect/locator.rs` | 500 | Low (borderline) |

#### 2. Clippy Warnings (19 total)

- `single_char_pattern`: 12 instances in test files (use `'p'` instead of `"p"`)
- `uninlined_format_args`: 7 instances (use `{var}` instead of `"{}", var`)

#### 3. Crate-Level Allow Suppressions (11 files)

| File | Suppressions |
|------|--------------|
| `viewpoint-core/src/network/response/tests/mod.rs` | `float_cmp`, `unreadable_literal` |
| `viewpoint-cdp/tests/integration_tests.rs` | `uninlined_format_args` |
| `viewpoint-core/tests/clock_tests.rs` | `float_cmp`, `unreadable_literal` |
| `viewpoint-core/tests/device_emulation_tests.rs` | `float_cmp`, `assertions_on_constants` |
| `viewpoint-core/tests/device_emulation_context_tests.rs` | `assertions_on_constants`, `float_cmp` |
| `viewpoint-core/tests/har_tests.rs` | `float_cmp` |
| `viewpoint-core/src/context/types/tests/mod.rs` | `float_cmp` |
| `viewpoint-core/src/devices/tests/mod.rs` | `float_cmp`, `assertions_on_constants` |
| `viewpoint-core/src/page/locator/selector/tests/mod.rs` | `uninlined_format_args` |
| `viewpoint-core/src/page/locator/aria_role/tests/mod.rs` | `uninlined_format_args` |
| `viewpoint-core/src/page/locator/aria/tests/mod.rs` | `uninlined_format_args` |

**Note**: `float_cmp` suppressions in test files are justified for testing exact floating point values. These should be converted to item-level suppressions with comments.
