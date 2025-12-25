# Tasks: Add Test Framework

## 1. Foundation

- [x] 1.1 Create `crates/rustright-test/` crate with runtime dependencies
- [x] 1.2 Create `crates/rustright-test-macros/` crate with proc-macro setup
- [x] 1.3 Add workspace members to root `Cargo.toml`
- [x] 1.4 Set up crate re-exports and public API structure

## 2. TestHarness (in rustright-test) - PRIMARY API

- [x] 2.1 Define `TestHarness` struct with ownership tracking (owns_browser, owns_context flags)
- [x] 2.2 Implement `TestHarness::new()` with default configuration (test-scoped)
- [x] 2.3 Implement `TestHarness::from_browser()` for module-scoped browser sharing
- [x] 2.4 Implement `TestHarness::from_context()` for shared context scenarios
- [x] 2.5 Implement `TestHarnessBuilder` for configuration (headless, timeout)
- [x] 2.6 Implement `.page()`, `.context()`, `.browser()` accessors
- [x] 2.7 Implement `.new_page()` for creating additional pages
- [x] 2.8 Implement `Drop` for automatic cleanup (respecting ownership)
- [x] 2.9 Implement `.close()` for explicit async cleanup (respecting ownership)
- [x] 2.10 Add `TestConfig` struct for configuration options
- [x] 2.11 Write tests for harness lifecycle (all scoping levels)

## 3. Locator System (in rustright-core)

- [x] 3.1 Define `Selector` enum (Css, Text, Role, TestId, Label, Placeholder, Chained)
- [x] 3.2 Define `Locator` struct with page reference and selector
- [x] 3.3 Define `LocatorOptions` for timeout configuration
- [x] 3.4 Implement `Page::locator()` for CSS selectors
- [x] 3.5 Implement `Page::get_by_text()` for text locators
- [x] 3.6 Implement `Page::get_by_role()` for ARIA role locators
- [x] 3.7 Implement `Page::get_by_test_id()` for test ID locators
- [x] 3.8 Implement `Page::get_by_label()` for form label locators
- [x] 3.9 Implement `Page::get_by_placeholder()` for placeholder locators
- [x] 3.10 Implement locator chaining (`.locator()` on Locator)
- [x] 3.11 Implement `.first()`, `.last()`, `.nth()` selection
- [x] 3.12 Add CDP commands for element querying (Runtime.evaluate, DOM methods)
- [x] 3.13 Write unit tests for locator creation and chaining

## 4. Actions System (in rustright-core)

- [x] 4.1 Add CDP commands for input simulation (Input.dispatchMouseEvent, Input.dispatchKeyEvent)
- [x] 4.2 Implement auto-waiting logic for actionability checks
- [x] 4.3 Implement `Locator::click()` with options builder
- [x] 4.4 Implement `Locator::dblclick()`
- [x] 4.5 Implement `Locator::fill()` for input fields
- [x] 4.6 Implement `Locator::type_text()` for character-by-character input
- [x] 4.7 Implement `Locator::press()` for key presses
- [x] 4.8 Implement `Locator::select_option()` for dropdowns
- [x] 4.9 Implement `Locator::check()` and `Locator::uncheck()`
- [x] 4.10 Implement `Locator::hover()`
- [x] 4.11 Implement `Locator::focus()`
- [x] 4.12 Implement `Locator::clear()`
- [x] 4.13 Write integration tests for actions against real browser

## 5. Assertions System (in rustright-test)

- [x] 5.1 Define `AssertionError` type with expected/actual info
- [x] 5.2 Implement `expect()` function returning assertion builders
- [x] 5.3 Implement `LocatorAssertions` struct
- [x] 5.4 Implement `.to_be_visible()` and `.to_be_hidden()`
- [x] 5.5 Implement `.to_have_text()` and `.to_contain_text()`
- [x] 5.6 Implement `.to_have_attribute()`
- [x] 5.7 Implement `.to_have_class()`
- [x] 5.8 Implement `.to_be_enabled()` and `.to_be_disabled()`
- [x] 5.9 Implement `.to_be_checked()`
- [x] 5.10 Implement `PageAssertions` struct
- [x] 5.11 Implement `.to_have_url()` and `.to_have_url_containing()`
- [x] 5.12 Implement `.to_have_title()`
- [x] 5.13 Add assertion timeout configuration
- [x] 5.14 Write tests for assertion success and failure cases

## 6. Test Macro (in rustright-test-macros) - OPTIONAL

- [x] 6.1 Parse `#[rustright::test]` attribute and function signature
- [x] 6.2 Detect fixture parameters (Page, BrowserContext, Browser)
- [x] 6.3 Generate TestHarness setup code based on parameters
- [x] 6.4 Generate cleanup code with proper error handling
- [x] 6.5 Support `#[rustright::test(timeout = ...)]` option
- [x] 6.6 Support `#[rustright::test(headless = ...)]` option
- [x] 6.7 Support `#[rustright::test(scope = "browser", browser = "fn_name")]` for module-scoped browser
- [x] 6.8 Support `#[rustright::test(scope = "context", context = "fn_name")]` for shared context
- [x] 6.9 Emit compile error when scope requires source function but none provided
- [x] 6.10 Write macro expansion tests for all scoping options

## 7. Integration & Documentation

- [x] 7.1 Create example test using TestHarness in `crates/rustright-test/examples/`
- [x] 7.2 Create example test using proc macro in `crates/rustright-test/examples/`
- [x] 7.3 Add doc comments with examples to public API
- [x] 7.4 Write end-to-end test using the full framework
- [x] 7.5 Update workspace README with test framework usage

---

## Summary

**Completed**: 70/70 tasks (100%)

The test framework is fully functional with:
- TestHarness for explicit test setup
- Locator system with multiple selector types
- Action methods (click, fill, type, check, hover, focus, select_option, etc.)
- Assertion API (expect/expect_page with visibility, text, URL, title checks)
- Proc macro for convenient test writing
- Comprehensive test coverage (18 harness tests, 29 core tests, 8 E2E tests)
- Full documentation in workspace README
