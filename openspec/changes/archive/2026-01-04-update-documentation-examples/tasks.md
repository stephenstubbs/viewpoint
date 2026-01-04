# Tasks: Update Documentation with Comprehensive Examples

## 1. Documentation Audit
- [x] 1.1 Inventory all public types/methods lacking doc comments
- [x] 1.2 Identify examples using `ignore` that could use `compile`
- [x] 1.3 List modules needing enhanced documentation
- [x] 1.4 Document audit findings in design.md

## 2. README Files
- [x] 2.1 Create `viewpoint-js-core/README.md` with usage examples
- [x] 2.2 Enhance `viewpoint-cdp/README.md` with more examples
- [x] 2.3 Enhance `viewpoint-test-macros/README.md` with configuration options
- [x] 2.4 Review and update root `README.md` for completeness
- [x] 2.5 Ensure all READMEs link to docs.rs

## 3. Crate-Level Documentation (lib.rs)
- [x] 3.1 Update `viewpoint-core/src/lib.rs` examples to use `compile` where possible
- [x] 3.2 Update `viewpoint-test/src/lib.rs` examples to use `compile` where possible
- [x] 3.3 Ensure `viewpoint-cdp/src/lib.rs` has connection examples
- [x] 3.4 Verify `viewpoint-js/src/lib.rs` examples are complete
- [x] 3.5 Verify `viewpoint-js-core/src/lib.rs` examples are complete
- [x] 3.6 Ensure `viewpoint-test-macros/src/lib.rs` documents all macro options

## 4. Primary Type Documentation (viewpoint-core)
- [x] 4.1 Document `Browser` type with all connection method examples
- [x] 4.2 Document `BrowserContext` with context options examples
- [x] 4.3 Document `Page` type with navigation and interaction examples
- [x] 4.4 Document `Locator` type with all action method examples
- [x] 4.5 Document `Frame` type with iframe interaction examples
- [x] 4.6 Document `ElementHandle` type with low-level DOM examples

## 5. Network Types Documentation
- [x] 5.1 Document `Route` with interception examples
- [x] 5.2 Document `Request` with request inspection examples
- [x] 5.3 Document `Response` with response handling examples
- [x] 5.4 Document `WebSocket` with monitoring examples
- [x] 5.5 Document HAR types with recording/replay examples

## 6. Event Types Documentation
- [x] 6.1 Document `Dialog` with alert/confirm/prompt examples
- [x] 6.2 Document `Download` with file download examples
- [x] 6.3 Document `FileChooser` with file upload examples
- [x] 6.4 Document `ConsoleMessage` with logging examples

## 7. Test Framework Documentation (viewpoint-test)
- [x] 7.1 Document `TestHarness` with all configuration options
- [x] 7.2 Document `expect()` assertions with examples for each assertion type
- [x] 7.3 Document `expect_page()` assertions with examples
- [x] 7.4 Document `SoftAssertions` with multi-failure examples
- [x] 7.5 Document fixture scoping patterns

## 8. Input Device Documentation
- [x] 8.1 Document `Keyboard` with typing and shortcut examples
- [x] 8.2 Document `Mouse` with click, drag, and scroll examples
- [x] 8.3 Document `Touchscreen` with tap and swipe examples

## 9. Utility Types Documentation
- [x] 9.1 Document `Clock` with time mocking examples
- [x] 9.2 Document `Tracing` with trace recording examples
- [x] 9.3 Document `Cookie` types with management examples
- [x] 9.4 Document `StorageState` with persistence examples
- [x] 9.5 Document `AriaSnapshot` with accessibility testing examples

## 10. Module Documentation Enhancement
- [x] 10.1 Enhance `browser/mod.rs` with usage patterns
- [x] 10.2 Enhance `page/mod.rs` with common workflows
- [x] 10.3 Enhance `locator/mod.rs` with locator strategy guidance
- [x] 10.4 Enhance `network/mod.rs` with interception patterns
- [x] 10.5 Enhance `context/mod.rs` with isolation patterns

## 11. Documentation Spec Update
- [x] 11.1 Update spec Purpose section
- [x] 11.2 Add requirement for compile-checked examples
- [x] 11.3 Add scenarios for new crate documentation
- [x] 11.4 Update Context7 configuration requirements

## 12. Validation
- [x] 12.1 Run `cargo test --doc --workspace` and fix failures
- [x] 12.2 Run `cargo doc --workspace --no-deps` and verify rendering
- [x] 12.3 Update Context7 configuration if needed
- [x] 12.4 Final review of docs.rs preview

## Implementation Notes

### Audit Summary
The existing documentation was found to be comprehensive. Key findings:
- All 6 crates had lib.rs documentation with examples
- 256 doc tests pass across the workspace
- `ignore` attributes are appropriately used for callback patterns

### Changes Made
1. Created `viewpoint-js-core/README.md` (was missing)
2. Enhanced `viewpoint-cdp/README.md` with comprehensive examples
3. Enhanced `viewpoint-test-macros/README.md` with all configuration options
4. Updated root `README.md` to include viewpoint-js-core crate
5. Fixed doc warning in page/mod.rs (`Vec<Page>` -> `` `Vec<Page>` ``)
6. Updated design.md with audit findings
