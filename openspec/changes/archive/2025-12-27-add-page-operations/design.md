# Design: Core Page Operations

## Context

Adding page-level operations (screenshots, PDFs, JavaScript evaluation, content manipulation) requires extending both the CDP protocol layer and the core page API. These features are foundational for many testing scenarios and will be used by subsequent proposals.

## Goals

- Provide Playwright-compatible API for page operations
- Maintain async/await ergonomics throughout
- Support builder patterns for complex options
- Handle binary data (images, PDFs) efficiently in Rust

## Non-Goals

- Video recording (separate proposal: Tracing & Debugging)
- HAR recording (separate proposal: Network Interception)

## Decisions

### Decision 1: Screenshot Return Type

**Choice**: Return `Vec<u8>` for screenshot/PDF data, with optional path parameter for direct file saving.

**Rationale**: 
- Rust has excellent binary handling via `Vec<u8>`
- Matches Playwright's Buffer return type semantically
- Optional `path` parameter provides convenience for file saving
- Caller can use `tokio::fs` for async file operations if needed

**Alternatives considered**:
- Returning a custom `Screenshot` type - adds complexity without clear benefit
- Always requiring a path - less flexible for in-memory operations

### Decision 2: JavaScript Evaluation Architecture

**Choice**: Use CDP's `Runtime.evaluate` and `Runtime.callFunctionOn` with JSON serialization for arguments and return values.

**Rationale**:
- CDP Runtime domain is well-documented and stable
- JSON serialization handles most use cases
- Complex objects can be handled via `evaluateHandle()` returning a handle ID

**Implementation**:
```
Page::evaluate<T: DeserializeOwned>(script: &str) -> Result<T>
Page::evaluate_with_arg<T, A>(script: &str, arg: A) -> Result<T>
Page::evaluate_handle(script: &str) -> Result<JsHandle>
```

### Decision 3: Builder Pattern for Options

**Choice**: Use builder pattern for screenshot and PDF options, similar to navigation.

**Example**:
```rust
page.screenshot()
    .full_page(true)
    .format(ScreenshotFormat::Png)
    .path("screenshot.png")
    .capture()
    .await?;

page.pdf()
    .format(PaperFormat::A4)
    .landscape(true)
    .path("document.pdf")
    .generate()
    .await?;
```

**Rationale**:
- Consistent with existing navigation builder pattern
- Many optional parameters make builders more ergonomic than option structs
- Method chaining is idiomatic Rust

### Decision 4: Content Encoding

**Choice**: Use UTF-8 strings for HTML content, Base64 for binary script/style content.

**Rationale**:
- HTML is inherently text-based
- CDP uses Base64 for binary data
- Rust's String type handles UTF-8 natively

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| Screenshot | Page.captureScreenshot | Page |
| PDF | Page.printToPDF | Page |
| Evaluate | Runtime.evaluate | Runtime |
| Evaluate Handle | Runtime.callFunctionOn | Runtime |
| Get Content | Runtime.evaluate (document.documentElement.outerHTML) | Runtime |
| Set Content | Page.setDocumentContent or navigate to data: URL | Page |
| Add Script Tag | Runtime.evaluate (DOM manipulation) | Runtime |
| Add Style Tag | Runtime.evaluate (DOM manipulation) | Runtime |
| Add Init Script | Page.addScriptToEvaluateOnNewDocument | Page |
| Get Title | Runtime.evaluate (document.title) | Runtime |
| Viewport | Emulation.setDeviceMetricsOverride | Emulation |
| Bring to Front | Page.bringToFront | Page |

## Risks / Trade-offs

### Risk: Large Screenshot/PDF Memory Usage

**Mitigation**: 
- Document memory implications in API docs
- Consider streaming for very large captures in future
- Provide `path` option to write directly to disk

### Risk: JavaScript Evaluation Security

**Mitigation**:
- Document that evaluate runs in page context with full access
- This is expected behavior matching Playwright
- Users control what scripts are executed

### Risk: Init Scripts Timing

**Mitigation**:
- Use CDP's `Page.addScriptToEvaluateOnNewDocument` which guarantees execution before page scripts
- Document that init scripts persist across navigations

## Resolved Questions

### Q1: Should `evaluate` support timeout configuration?

**Answer**: Yes, with Playwright-compatible defaults.

Playwright uses the page's default timeout (30 seconds) for evaluation operations, configurable via `page.setDefaultTimeout()`. We will:
- Default to 30 seconds (matching Playwright)
- Allow per-call timeout override via builder pattern
- Respect page-level default timeout settings

```rust
// Uses default 30s timeout
page.evaluate::<i32>("longRunning()").await?;

// Custom timeout
page.evaluate::<i32>("longRunning()").timeout(Duration::from_secs(60)).await?;
```

### Q2: Should we add `waitForFunction` in this proposal?

**Answer**: Yes, include it in this proposal.

`waitForFunction` is closely related to `evaluate` - it repeatedly evaluates a function until it returns a truthy value. Including it here keeps all JavaScript evaluation functionality together.

```rust
// Wait for condition with polling
page.wait_for_function("() => document.querySelector('.loaded')").await?;

// With timeout and polling interval
page.wait_for_function("() => window.ready")
    .timeout(Duration::from_secs(10))
    .polling(Polling::Interval(Duration::from_millis(100)))
    .await?;
```
