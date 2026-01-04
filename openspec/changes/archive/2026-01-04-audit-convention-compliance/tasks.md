# Tasks: Audit Convention Compliance

## 1. File Size Audit
- [x] 1.1 Scan all `.rs` files in `crates/` (excluding `target/`)
- [x] 1.2 Identify files exceeding 500 lines
- [x] 1.3 Categorize violations by severity (500-600: minor, 600-800: moderate, 800+: major)
- [x] 1.4 Document each violation with file path and line count

## 2. Module Structure Audit
- [x] 2.1 Verify all non-trivial modules use folder structure (`mod.rs` pattern)
- [x] 2.2 Check for single `.rs` files that should be folder modules
- [x] 2.3 Verify each folder module has appropriate structure (mod.rs, error.rs if needed, tests/)

## 3. Test Organization Audit
- [x] 3.1 Verify no inline `#[cfg(test)] mod tests { ... }` blocks exist
- [x] 3.2 Confirm all unit tests use `#[cfg(test)] mod tests;` (external module reference)
- [x] 3.3 Verify test modules are in `tests/` subdirectories
- [x] 3.4 Check integration tests use `#![cfg(feature = "integration")]` gate

## 4. Naming Convention Audit
- [x] 4.1 Verify error types follow `{Module}Error` naming
- [x] 4.2 Check for Result type aliases where appropriate

## 5. JavaScript Code Audit
- [x] 5.1 Search for raw JavaScript strings that should use `js!` macro
- [x] 5.2 Identify any `format!` usage for building JavaScript
- [x] 5.3 Verify `viewpoint_js_core` utilities used for dynamic JS string building

## 6. Report Generation
- [x] 6.1 Compile all findings into categorized report
- [x] 6.2 Prioritize findings by impact and effort
- [x] 6.3 Create remediation recommendations

## 7. Remediation: Fix Inline Test Block (Priority 1 - Required)
- [x] 7.1 Create `viewpoint-cdp/src/protocol/target_domain/tests/` directory
- [x] 7.2 Create `tests/mod.rs` with extracted test code
- [x] 7.3 Remove inline test block from `target_domain/mod.rs`
- [x] 7.4 Add `#[cfg(test)] mod tests;` reference
- [x] 7.5 Verify tests pass: `cargo test -p viewpoint-cdp`

## 8. Remediation: Split Large Integration Tests (Priority 2 - Recommended)
- [x] 8.1 Create shared test utilities module for common helpers
- [x] 8.2 Split `locator_ref_operations_tests.rs` (792 lines) into 3 files
- [x] 8.3 Split `aria_snapshot_ref_frame_tests.rs` (638 lines) into 2 files
- [x] 8.4 Split `context_tests.rs` (522 lines) into 3 files
- [x] 8.5 Verify all tests compile and pass
