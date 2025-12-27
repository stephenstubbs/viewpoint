# Tasks: Enhance Unit and Integration Tests

## Phase A: Verify and Update Test Organization (NEW - Required First)

### A.1: Audit and Migrate Inline Tests

- [x] A.1.1 List all files with `#[cfg(test)] mod tests` blocks
- [x] A.1.2 Create `tests/` subdirectory for each module with inline tests
- [x] A.1.3 Migrate inline tests to external test files
- [x] A.1.4 Update source files to use `#[cfg(test)] mod tests;` declaration
- [x] A.1.5 Verify all migrated tests pass

### A.2: Verify Integration Test Gating

- [x] A.2.1 Verify `viewpoint-cdp/tests/integration_tests.rs` has `#![cfg(feature = "integration")]`
- [x] A.2.2 Verify all `viewpoint-core/tests/*.rs` browser tests have feature gate
- [x] A.2.3 Verify `viewpoint-test/tests/*.rs` browser tests have feature gate
- [x] A.2.4 Verify `cargo test` (without features) runs only unit tests
- [x] A.2.5 Verify `cargo test --features integration` runs all tests

### A.3: Verify Folder Module Compliance

- [x] A.3.1 Verify all modules use folder structure (`module/mod.rs`)
- [x] A.3.2 Verify no single-file modules remain (except lib.rs entry points)
- [x] A.3.3 Document any exceptions with justification

## Phase B: Re-verify Previously Completed Tasks

### B.1: viewpoint-js Tests (Phase 0 re-verification)

- [x] B.1.1 Verify js! macro unit tests exist and pass
- [x] B.1.2 Verify ToJsValue tests exist and pass
- [x] B.1.3 Verify interpolation tests exist and pass
- [x] B.1.4 Verify compile-fail tests work with trybuild
- [x] B.1.5 Verify tests follow external test directory structure

### B.2: viewpoint-cdp Tests (Phases 1-2 re-verification)

- [x] B.2.1 Verify connection state machine tests exist
- [x] B.2.2 Verify CDP message parsing tests exist
- [x] B.2.3 Verify error type tests cover all variants
- [x] B.2.4 Verify integration tests exist and are feature-gated
- [x] B.2.5 Verify tests follow external test directory structure

### B.3: viewpoint-core Unit Tests (Phases 3-4 re-verification)

- [x] B.3.1 Verify network type tests exist (URL patterns, glob)
- [x] B.3.2 Verify HAR format tests exist
- [x] B.3.3 Verify selector parsing tests exist
- [x] B.3.4 Verify page state tests exist
- [x] B.3.5 Verify tests follow external test directory structure

### B.4: viewpoint-core Integration Tests (Phases 5-7 re-verification)

- [x] B.4.1 Verify navigation tests exist and are feature-gated
- [x] B.4.2 Verify locator tests exist and are feature-gated
- [x] B.4.3 Verify action tests exist and are feature-gated
- [x] B.4.4 Verify all tests use js! macro for JavaScript

### B.5: viewpoint-test Tests (Phase 4 re-verification)

- [x] B.5.1 Verify soft assertion tests exist
- [x] B.5.2 Verify tests follow external test directory structure

## Phase C: Complete Remaining Test Coverage

Note: Many of these tests already exist in the current test suite. The existing coverage includes:
- Network tests: route abort, fulfill, continue, multi-route handling
- Context tests: cookies, context isolation, HTTP credentials
- Frame tests: frame access, iframe handling, frame locators
- Clock tests: time mocking, timer control
- E2E tests: form interactions, assertions, page workflows

### C.1: Network Event Tests (8.3-8.7, 8.9-8.10)

- [x] C.1.1 Add request event capture tests (in network_tests.rs)
- [x] C.1.2 Add response event capture tests (in network_tests.rs)
- [x] C.1.3 Add request failure event tests (in network_tests.rs - route abort)
- [x] C.1.4 Add HAR recording tests during navigation (in har_tests.rs)
- [x] C.1.5 Add HAR filtering tests (URL patterns, content types) (in har_tests.rs)
- [x] C.1.6 Add extra HTTP headers tests (in network_tests.rs)
- [ ] C.1.7 Add offline mode tests

### C.2: Context Integration Tests (9.4-9.7)

- [x] C.2.1 Add storage state tests (save, restore) (in context_tests.rs)
- [x] C.2.2 Add context isolation tests (cookies don't leak) (in context_tests.rs)
- [x] C.2.3 Add HTTP credentials tests (in context_tests.rs)
- [x] C.2.4 Add timezone/locale tests (in context_tests.rs)

### C.3: Frame Integration Tests (10.2, 10.4-10.5)

- [x] C.3.1 Add iframe navigation tests (in frame_tests.rs)
- [x] C.3.2 Add cross-frame element access tests (in frame_tests.rs)
- [x] C.3.3 Add frame detachment handling tests (in frame_tests.rs)

### C.4: Dialog/Download Tests (11.1-11.6)

- [ ] C.4.1 Add alert dialog tests (accept, dismiss)
- [ ] C.4.2 Add confirm dialog tests
- [ ] C.4.3 Add prompt dialog tests (with input)
- [ ] C.4.4 Add beforeunload dialog tests
- [ ] C.4.5 Add download initiation tests
- [ ] C.4.6 Add download path/state tests

### C.5: Advanced Feature Tests (12.2, 12.6-12.8)

- [ ] C.5.1 Add PDF generation tests
- [x] C.5.2 Add clock mocking tests (in clock_tests.rs)
- [ ] C.5.3 Add media emulation tests
- [ ] C.5.4 Add device emulation tests

### C.6: viewpoint-test Unit Tests (13.x)

- [x] C.6.1 Add locator assertion unit tests (all conditions) (in e2e_tests.rs)
- [x] C.6.2 Add page assertion unit tests (in e2e_tests.rs)
- [x] C.6.3 Add soft assertion accumulation tests (in soft_tests.rs)
- [ ] C.6.4 Add timeout behavior tests
- [x] C.6.5 Add negation (not()) tests (in e2e_tests.rs)
- [x] C.6.6 Add assertion error message tests (in soft_tests.rs)

### C.7: viewpoint-test Integration Tests (14.x)

- [x] C.7.1 Add to_be_visible assertion tests with real elements (in e2e_tests.rs)
- [x] C.7.2 Add to_be_hidden assertion tests (in e2e_tests.rs)
- [x] C.7.3 Add to_have_text assertion tests (exact, partial) (in e2e_tests.rs)
- [x] C.7.4 Add to_be_checked assertion tests (in e2e_tests.rs)
- [x] C.7.5 Add to_be_enabled/disabled assertion tests (in e2e_tests.rs)
- [x] C.7.6 Add to_have_attribute assertion tests (in e2e_tests.rs)
- [x] C.7.7 Add to_have_value assertion tests (in e2e_tests.rs)
- [x] C.7.8 Add to_have_count assertion tests (in e2e_tests.rs)
- [x] C.7.9 Add page URL/title assertion tests (in e2e_tests.rs)
- [x] C.7.10 Add test harness lifecycle tests (in harness_tests.rs)

### C.8: viewpoint-test-macros Tests (15.x)

- [ ] C.8.1 Add macro expansion unit tests
- [ ] C.8.2 Add fixture injection integration tests
- [ ] C.8.3 Add error case tests (invalid macro usage)

### C.9: Cross-Crate Integration Tests (16.x)

- [x] C.9.1 Add complete E2E workflow tests (in e2e_tests.rs)
- [x] C.9.2 Add multi-page workflow tests (in harness_tests.rs)
- [x] C.9.3 Add network + assertions combined tests (in e2e_tests.rs)
- [ ] C.9.4 Add error recovery workflow tests

## Phase D: Final Verification

### D.1: Code Quality Compliance

- [x] D.1.1 Run `cargo check --workspace` - zero warnings
- [x] D.1.2 Run `cargo clippy --workspace` - zero warnings
- [x] D.1.3 Verify no inline `#[cfg(test)] mod tests` blocks remain
- [x] D.1.4 Verify all modules use folder structure

### D.2: Test Coverage Verification

- [x] D.2.1 Run `cargo test --workspace` - all unit tests pass
- [ ] D.2.2 Run `cargo test --workspace --features integration` - all tests pass (requires Chromium)
- [x] D.2.3 Verify no raw JavaScript strings remain in tests

### D.3: Documentation

- [ ] D.3.1 Update README with test running instructions
- [ ] D.3.2 Document test organization structure
- [ ] D.3.3 Archive change proposal when complete

## Summary

### Completed Work

1. **Migrated 29 inline test modules** to external `tests/mod.rs` directories:
   - viewpoint-cdp: 2 modules (transport, protocol/browser)
   - viewpoint-core: 27 modules (network/*, context/*, page/*, api/*, wait/*, devices)

2. **Verified integration test gating**:
   - All browser-requiring tests use `#![cfg(feature = "integration")]`
   - Unit tests run without feature flag
   - Pure type tests (api_request_tests.rs, network_types_tests.rs) correctly ungated

3. **Verified folder module compliance**:
   - All top-level modules use folder structure
   - Sub-modules within folders are acceptable per code-quality spec

4. **Re-verified existing test coverage**:
   - viewpoint-js: js! macro, ToJsValue, interpolation, compile-fail tests
   - viewpoint-cdp: connection, error, transport, CDP message tests
   - viewpoint-core: network, page, context, navigation, locator, HAR tests
   - viewpoint-test: soft assertions, harness lifecycle, E2E tests

### Test Statistics

After migration:
- Total unit tests: ~180 passing
- Doc tests: ~62 (some ignored for requiring runtime)
- Integration tests: Feature-gated, require Chromium
- All `cargo check` and `cargo clippy` pass with zero warnings
