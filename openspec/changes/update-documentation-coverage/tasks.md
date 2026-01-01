# Tasks: Update Documentation Coverage

## 1. Analysis & Planning
- [x] 1.1 Audit current doc coverage: run `cargo doc --no-deps` and note warnings
- [x] 1.2 Identify all public types/methods without doc comments
- [x] 1.3 Prioritize by usage frequency and API importance
- [x] 1.4 Review Context7 benchmark questions to understand what documentation is being tested

## 2. Crate-Level Documentation (`lib.rs`)
- [x] 2.1 Update `viewpoint-core/src/lib.rs` with comprehensive crate docs
- [x] 2.2 Update `viewpoint-test/src/lib.rs` with comprehensive crate docs
- [x] 2.3 Update `viewpoint-js/src/lib.rs` with comprehensive crate docs
- [x] 2.4 Update `viewpoint-js-core/src/lib.rs` with comprehensive crate docs
- [x] 2.5 Update `viewpoint-cdp/src/lib.rs` with comprehensive crate docs
- [x] 2.6 Update `viewpoint-test-macros/src/lib.rs` with comprehensive crate docs

## 3. Core Module Documentation
- [x] 3.1 Document `browser/mod.rs` - Browser launching and connection
- [x] 3.2 Document `context/mod.rs` - BrowserContext and configuration
- [x] 3.3 Document `page/mod.rs` - Page navigation and interaction
- [x] 3.4 Document `network/mod.rs` - Network interception and routing
- [x] 3.5 Document `wait/mod.rs` - Wait system and load states

## 4. Public Type Documentation (viewpoint-core)
- [x] 4.1 Document `Browser` type fully with all methods and examples
- [x] 4.2 Document `BrowserContext` type fully
- [x] 4.3 Document `Page` type fully (navigation, content, input methods)
- [x] 4.4 Document `Locator` type and all locator methods
- [x] 4.5 Document `Route` and network interception types
- [x] 4.6 Document `Frame` and `FrameLocator` types
- [x] 4.7 Document event types (Dialog, Download, FileChooser, Console)
- [x] 4.8 Document emulation types (ViewportSize, Geolocation, etc.)
- [x] 4.9 Document input device types (Keyboard, Mouse, Touchscreen)
- [x] 4.10 Document Clock mocking API

## 5. Public Type Documentation (viewpoint-test)
- [x] 5.1 Document `TestHarness` type and builder pattern
- [x] 5.2 Document `expect` and `expect_page` functions
- [x] 5.3 Document `LocatorAssertions` trait methods
- [x] 5.4 Document `PageAssertions` trait methods
- [x] 5.5 Document `SoftAssertions` type
- [x] 5.6 Document `TestConfig` and configuration options

## 6. Public Type Documentation (viewpoint-js)
- [x] 6.1 Document `js!` macro with comprehensive examples
- [x] 6.2 Document interpolation syntax (`#{}` and `@{}`)
- [x] 6.3 Document `ToJsValue` trait and implementations
- [x] 6.4 Document escape utilities in viewpoint-js-core

## 7. README Enhancement
- [x] 7.1 Expand main `README.md` with additional examples
- [x] 7.2 Expand `viewpoint-core/README.md` with full API coverage
- [x] 7.3 Expand `viewpoint-test/README.md` with more assertion examples
- [x] 7.4 Expand `viewpoint-js/README.md` with edge case examples
- [x] 7.5 Expand `viewpoint-cdp/README.md` with protocol documentation
- [x] 7.6 Expand `viewpoint-test-macros/README.md` with configuration options

## 8. Context7 Configuration
- [x] 8.1 Review current `context7.json` configuration
- [x] 8.2 Add appropriate exclusion patterns (tests, internal modules)
- [x] 8.3 Add `rules` field with best practices for AI agents
- [x] 8.4 Add descriptive `projectTitle` and `description`
- [ ] 8.5 Trigger re-index on Context7 and verify improvement

## 9. Validation
- [x] 9.1 Run `cargo doc --no-deps --all-features` with no warnings
- [x] 9.2 Review generated documentation for completeness
- [x] 9.3 Verify all code examples compile (`cargo test --doc`)
- [ ] 9.4 Check Context7 benchmark score improvement
