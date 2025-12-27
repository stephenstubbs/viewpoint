# Change: Complete Test Coverage

## Why

Viewpoint is a testing framework that must itself be rigorously tested. An audit of the test suite against the specification revealed several gaps where spec scenarios lack corresponding integration tests. These gaps represent potential regressions and reduce confidence in the framework's reliability.

The missing test coverage falls into these categories:

1. **Dialog handling** - Alert, confirm, prompt, and beforeunload dialogs have specs but no integration tests
2. **Download handling** - Download events, paths, save_as, and cancel have specs but no integration tests
3. **PDF generation** - PDF generation with various options has specs but no integration tests
4. **Media emulation** - Color scheme, reduced motion, forced colors have specs but no integration tests
5. **Device emulation** - Device descriptors, viewport, touch emulation have specs but limited tests
6. **Test framework macros** - The `#[viewpoint::test]` macro lacks fixture injection tests
7. **Error recovery** - Timeout behavior and error handling paths need coverage

Following the project's code-quality and test-requirements specs, all tests must:
- Use real Chromium (integration feature gate)
- Follow external test directory structure
- Use the `js!` macro for JavaScript
- Cover both success and failure paths

## What Changes

### New Integration Test Files

1. **dialog_tests.rs** - Complete coverage of dialogs spec
   - Alert accept/dismiss
   - Confirm accept/dismiss with return value
   - Prompt with text input
   - Beforeunload handling
   - Auto-dismiss behavior

2. **download_tests.rs** - Complete coverage of downloads spec
   - Download event capture
   - Suggested filename access
   - Download path waiting
   - Save to custom location
   - Cancel in-progress download
   - Failure detection

3. **pdf_tests.rs** - Complete coverage of PDF generation spec
   - Default PDF generation
   - Custom paper size (A4, Letter)
   - Landscape orientation
   - Margins configuration
   - Header/footer templates
   - Page ranges
   - Background graphics
   - Save to file

4. **emulation_tests.rs** - Complete coverage of media/device emulation specs
   - Media type (print/screen)
   - Color scheme (dark/light)
   - Reduced motion
   - Forced colors
   - Vision deficiency
   - Device descriptors
   - Viewport/scale factor
   - Touch emulation
   - Locale/timezone

5. **macro_tests.rs** (viewpoint-test-macros) - Test macro functionality
   - Basic fixture injection
   - Multiple fixture parameters
   - Configuration attributes
   - Scope parameters
   - Compile-fail cases for invalid usage

### Enhanced Existing Tests

6. **Timeout behavior tests** - Add to harness_tests.rs or new file
   - Operation timeout expiry
   - Custom timeout configuration
   - Timeout error messages

7. **Error recovery tests** - Add to e2e_tests.rs or new file
   - Navigation failure recovery
   - Element not found handling
   - Network error handling
   - Graceful degradation

## Impact

- **New files**: 5 integration test files (~300-400 lines each)
- **Modified files**: Possible additions to existing test files
- **Affected crates**: viewpoint-core, viewpoint-test, viewpoint-test-macros
- **Breaking changes**: None (test-only additions)
- **Dependencies**: Real Chromium for all integration tests

## Success Criteria

1. **Every spec scenario has a test** - Cross-reference specs to verify coverage
2. **All tests pass** - `cargo test --workspace --features integration` succeeds
3. **File size limits respected** - No test file exceeds 500 lines
4. **Project conventions followed** - Feature gates, js! macro, external test directories
5. **Both paths tested** - Success and failure scenarios covered
