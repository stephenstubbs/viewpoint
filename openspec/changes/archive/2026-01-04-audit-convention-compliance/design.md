## Context
This audit examines the Viewpoint codebase for compliance with conventions defined in `project.md` and the `code-quality` spec. The goal is to identify deviations and create a remediation plan.

## Audit Methodology

### File Size Check
- Command: `find ./crates -name "*.rs" -type f ! -path "*/target/*" -exec wc -l {} \;`
- Threshold: 500 lines (source files), with flexibility for test files

### Module Structure Check
- Verify folder module pattern (`mod.rs` in directories)
- Check for single `.rs` files that should be folder modules

### Test Organization Check
- Search for inline test blocks: `#[cfg(test)] mod tests {`
- Verify external test module pattern: `#[cfg(test)] mod tests;`
- Confirm `tests/` subdirectories exist for unit tests

### JavaScript Code Check
- Verify `js!` macro usage for compile-time validated JavaScript
- Check `viewpoint_js_core` utilities for dynamic JS building

## Audit Findings

### 1. File Size Violations

| File | Lines | Severity | Notes |
|------|-------|----------|-------|
| `viewpoint-core/tests/locator_ref_operations_tests.rs` | 792 | Major | Integration test - consider splitting by feature |
| `viewpoint-core/tests/aria_snapshot_ref_frame_tests.rs` | 638 | Moderate | Integration test - consider splitting |
| `viewpoint-core/tests/context_tests.rs` | 522 | Minor | Integration test - slightly over limit |

**Analysis**: All violations are in integration test files, not source code. The `code-quality` spec states test files "SHOULD NOT exceed 500 lines" (not SHALL), making these recommendations rather than requirements.

### 2. Module Structure

**Status**: COMPLIANT

All non-trivial modules follow the folder module pattern with `mod.rs`. The project correctly uses:
- Folder modules for all substantial modules
- `mod.rs` for public exports
- `tests/` subdirectories for unit tests

### 3. Test Organization

**Finding**: One inline test block exists.

| File | Issue |
|------|-------|
| `viewpoint-cdp/src/protocol/target_domain/mod.rs` | Contains inline `#[cfg(test)] mod tests { ... }` block instead of external `mod tests;` |

**Remediation**: 
1. Create `viewpoint-cdp/src/protocol/target_domain/tests/mod.rs`
2. Move test code to the new file with `use super::*;`
3. Replace inline block with `#[cfg(test)] mod tests;`

### 4. Naming Conventions

**Status**: COMPLIANT

- Error types follow `{Module}Error` pattern: `CdpError`, `CoreError`, `BrowserError`, `TestError`, etc.
- All error types use `thiserror` derive
- Error types are consolidated in `error/mod.rs` modules

### 5. JavaScript Code

**Status**: COMPLIANT

- The `js!` macro is used extensively (59 usages found)
- Dynamic JavaScript building uses `viewpoint_js_core` utilities:
  - `js_string_literal()` for proper string escaping
  - `escape_js_contents_single()` for CSS in JS strings
  - `css_attr_value()` for CSS attribute value escaping

The `format!` usage in `selector/mod.rs` is acceptable because it's building JavaScript expressions dynamically at runtime with proper escaping via the core utilities.

### 6. Integration Test Feature Flags

**Status**: COMPLIANT

All integration tests properly use `#![cfg(feature = "integration")]` gate.

## Remediation Completed

### Priority 1: Inline Test Block - FIXED

Extracted the inline test block from `viewpoint-cdp/src/protocol/target_domain/mod.rs`:
- Created `viewpoint-cdp/src/protocol/target_domain/tests/mod.rs` with 3 tests
- Replaced inline block with `#[cfg(test)] mod tests;`
- All tests pass

### Priority 2: Test File Splitting - COMPLETED

Split the oversized integration test files into smaller, focused modules:

#### locator_ref_operations_tests.rs (792 lines) → 3 files
| New File | Lines | Tests |
|----------|-------|-------|
| `locator_ref_action_tests.rs` | 315 | select_option, scroll, highlight, files |
| `locator_ref_element_tests.rs` | 234 | evaluate, element_handle, screenshot, bounding_box |
| `locator_ref_query_tests.rs` | 213 | is_checked, inner_text, get_attribute, aria_snapshot |

#### aria_snapshot_ref_frame_tests.rs (638 lines) → 2 files
| New File | Lines | Tests |
|----------|-------|-------|
| `aria_snapshot_ref_page_tests.rs` | 139 | page vs frame snapshot resolution |
| `aria_snapshot_ref_iframe_tests.rs` | 420 | iframe ref resolution, click, type, nested |

#### context_tests.rs (522 lines) → 3 files
| New File | Lines | Tests |
|----------|-------|-------|
| `context_cookie_tests.rs` | 176 | cookies, permissions, geolocation |
| `context_config_tests.rs` | 194 | timeout, headers, offline, emulation |
| `context_storage_tests.rs` | 167 | storage state lifecycle |

#### Shared Test Utilities
Enhanced `common/mod.rs` (86 lines) with shared helpers:
- `find_ref_by_role()` - find element refs by ARIA role
- `find_button_ref()` - convenience helper for buttons
- `find_textbox_ref()` - convenience helper for textboxes
- `collect_refs_by_role()` - collect all refs for a given role

## Final Status

| Category | Before | After |
|----------|--------|-------|
| File Size Violations | 3 files | 0 files |
| Inline Test Blocks | 1 block | 0 blocks |
| Module Structure | Compliant | Compliant |
| Naming Conventions | Compliant | Compliant |
| JavaScript Code | Compliant | Compliant |

**All audit findings have been remediated. The codebase is now fully compliant with project conventions.**

## Open Questions
None - audit and remediation complete.
