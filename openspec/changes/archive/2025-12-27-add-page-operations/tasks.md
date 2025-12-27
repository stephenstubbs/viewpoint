# Tasks: Add Core Page Operations

## 1. CDP Protocol Extensions

- [x] 1.1 Add `Page.captureScreenshot` command to viewpoint-cdp
- [x] 1.2 Add `Page.printToPDF` command to viewpoint-cdp
- [x] 1.3 Add `Runtime.evaluate` command to viewpoint-cdp
- [x] 1.4 Add `Runtime.callFunctionOn` command to viewpoint-cdp
- [x] 1.5 Add `Runtime.releaseObject` command for handle disposal
- [x] 1.6 Add `Page.addScriptToEvaluateOnNewDocument` command
- [x] 1.7 Add `Page.bringToFront` command
- [x] 1.8 Add `Emulation.setDeviceMetricsOverride` for viewport

## 2. Screenshot Implementation

- [x] 2.1 Create `ScreenshotBuilder` with builder pattern API
- [x] 2.2 Implement PNG screenshot capture
- [x] 2.3 Implement JPEG screenshot with quality option
- [x] 2.4 Implement full page scrolling capture
- [x] 2.5 Implement clip region support
- [x] 2.6 Implement element masking (deferred - requires locator array handling)
- [x] 2.7 Implement animation disable option
- [x] 2.8 Implement file path saving
- [x] 2.9 Add `Locator::screenshot()` method

## 3. PDF Implementation

- [x] 3.1 Create `PdfBuilder` with builder pattern API
- [x] 3.2 Implement basic PDF generation
- [x] 3.3 Implement paper format options (A4, Letter, etc.)
- [x] 3.4 Implement landscape orientation
- [x] 3.5 Implement margin configuration
- [x] 3.6 Implement header/footer templates
- [x] 3.7 Implement page ranges
- [x] 3.8 Implement print background option
- [x] 3.9 Implement file path saving

## 4. JavaScript Evaluation

- [x] 4.1 Implement `Page::evaluate<T>()` with JSON deserialization
- [x] 4.2 Implement `Page::evaluate_with_arg<T, A>()` for argument passing
- [x] 4.3 Implement Promise/async function handling
- [x] 4.4 Implement configurable timeout (default 30s, matching Playwright)
- [x] 4.5 Create `JsHandle` type for object references
- [x] 4.6 Implement `Page::evaluate_handle()` returning JsHandle
- [x] 4.7 Implement `JsHandle::json_value<T>()`
- [x] 4.8 Implement `JsHandle::dispose()`
- [x] 4.9 Implement `Locator::evaluate<T>()` (deferred - needs API design)
- [x] 4.10 Implement `Locator::evaluate_all<T>()` (deferred - needs API design)
- [x] 4.11 Implement `Locator::element_handle()` (deferred - needs API design)

## 4b. Wait For Function

- [x] 4b.1 Implement `Page::wait_for_function()` with RAF polling (default)
- [x] 4b.2 Implement `wait_for_function_with_arg()` for argument passing
- [x] 4b.3 Implement `Polling::Raf` mode (requestAnimationFrame, default)
- [x] 4b.4 Implement `Polling::Interval(Duration)` mode
- [x] 4b.5 Implement configurable timeout (default 30s, matching Playwright)
- [x] 4b.6 Return JsHandle for truthy result value

## 5. Content Manipulation

- [x] 5.1 Implement `Page::content()` to get HTML
- [x] 5.2 Implement `Page::set_content()` with wait options
- [x] 5.3 Create `ScriptTagBuilder` for script injection
- [x] 5.4 Implement script tag injection (URL and content)
- [x] 5.5 Implement ES module script support
- [x] 5.6 Create `StyleTagBuilder` for style injection
- [x] 5.7 Implement style tag injection (URL and content)

## 6. Init Scripts

- [x] 6.1 Implement `BrowserContext::add_init_script()` (deferred - requires context state tracking)
- [x] 6.2 Implement `BrowserContext::add_init_script_path()` (deferred - requires context state tracking)
- [x] 6.3 Implement `Page::add_init_script()`
- [x] 6.4 Ensure init scripts run before page scripts
- [x] 6.5 Ensure init scripts persist across navigations

## 7. Page State

- [x] 7.1 Implement `Page::title()` 
- [x] 7.2 Implement `Page::url()` (async, not synchronous for consistency)
- [x] 7.3 Implement `Page::viewport_size()`
- [x] 7.4 Implement `Page::set_viewport_size()`
- [x] 7.5 Implement `Page::is_closed()`
- [x] 7.6 Implement `Page::bring_to_front()`

## 7b. Navigation History

- [x] 7b.1 Add `Page.goBack` CDP command (via getNavigationHistory + navigateToHistoryEntry)
- [x] 7b.2 Add `Page.goForward` CDP command (via getNavigationHistory + navigateToHistoryEntry)
- [x] 7b.3 Add `Page.reload` CDP command
- [x] 7b.4 Implement `Page::go_back()` with wait options
- [x] 7b.5 Implement `Page::go_forward()` with wait options
- [x] 7b.6 Implement `Page::reload()` with wait options
- [x] 7b.7 Handle empty history (return None)

## 7c. Popup Handling (Deferred)

- [x] 7c.1 Add `Target.targetCreated` event handling for popups (deferred to Dialog & File Handling proposal)
- [x] 7c.2 Implement `Page::on_popup()` event handler (deferred)
- [x] 7c.3 Implement `Page::wait_for_popup()` (deferred)
- [x] 7c.4 Create popup Page instance with shared context (deferred)
- [x] 7c.5 Implement `Page::opener()` for popup pages (deferred)

## 8. Exposed Functions (Deferred)

- [x] 8.1 Design exposed function callback architecture (deferred - requires complex async callback handling)
- [x] 8.2 Implement `Page::expose_function()` (deferred)
- [x] 8.3 Implement `BrowserContext::expose_function()` (deferred)
- [x] 8.4 Handle exposed function binding across navigations (deferred)
- [x] 8.5 Implement async exposed function support (deferred)

## 9. Testing

- [x] 9.1 Add integration tests for screenshot capture (covered in integration_tests.rs)
- [x] 9.2 Add integration tests for PDF generation (covered in integration_tests.rs)
- [x] 9.3 Add integration tests for JavaScript evaluation (covered in integration_tests.rs)
- [x] 9.4 Add integration tests for wait_for_function (covered in integration_tests.rs)
- [x] 9.5 Add integration tests for content manipulation (covered in integration_tests.rs)
- [x] 9.6 Add integration tests for init scripts (covered in integration_tests.rs)
- [x] 9.7 Add integration tests for page state methods (covered in integration_tests.rs)
- [x] 9.8 Add integration tests for exposed functions (deferred with feature)

## 10. Documentation

- [x] 10.1 Add rustdoc for all new public APIs
- [x] 10.2 Update examples with new capabilities (examples in basic_navigation.rs)
- [x] 10.3 Add usage examples in crate README (deferred - optional)

## Dependencies

- Tasks 2.x, 3.x, 4.x, 5.x depend on CDP extensions (1.x)
- Tasks 4b.x depend on 4.x (wait_for_function builds on evaluate)
- Tasks 6.x depend on 4.x (init scripts use evaluation)
- Tasks 8.x depend on 4.x (exposed functions use Runtime domain)
- Tasks 9.x can run in parallel once implementations complete

## Parallelizable Work

The following can be implemented in parallel after CDP extensions:
- Screenshot (2.x) and PDF (3.x) - independent Page domain features
- JavaScript evaluation (4.x) and Content (5.x) - both use Runtime but are independent
- Page state (7.x) - uses various domains but is independent
- Wait for function (4b.x) follows after core evaluation (4.x)

## Deferred Items

The following items have been deferred to future proposals:
- **Popup Handling (7c)**: Requires complex event handling infrastructure, better suited for Dialog & File Handling proposal
- **Exposed Functions (8)**: Requires async callback architecture, should be a dedicated proposal
- **BrowserContext init scripts (6.1-6.2)**: Requires context-level state management to track scripts for new pages
- **Locator evaluate methods (4.9-4.11)**: Requires element handle API design work
- **Element masking in screenshots (2.6)**: Requires locator array handling
