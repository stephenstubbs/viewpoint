# Design: Deferred Features

## Context

This proposal consolidates all deferred items from the initial Playwright feature parity implementation. These features were deferred because they require foundational infrastructure (like an event system) or represent advanced functionality that wasn't critical for initial release.

## Goals

- Implement all remaining Playwright API features
- Create reusable event system infrastructure
- Enable advanced testing patterns (soft assertions, aria testing)
- Complete network interception capabilities

## Non-Goals

- Multi-browser support (remains Chromium-only)
- Breaking API changes
- Performance optimization (unless required)

## Decisions

### Decision 1: Event System Architecture

**Choice**: Use typed event handlers with async callbacks stored in the context/page.

```rust
// Type-safe event handler registration
context.on_page(|page: Page| async move {
    println!("New page: {}", page.url().await?);
    Ok(())
}).await?;

// Remove specific handler
let handler_id = context.on_page(handler).await?;
context.off_page(handler_id).await?;

// Wait for event with action
let popup = page.wait_for_popup(|| async {
    page.click("button.open-popup").await
}).await?;
```

**Rationale**:
- Type safety prevents runtime errors
- Async handlers allow awaiting in callbacks
- Handler IDs enable selective removal
- Matches Playwright's event pattern

### Decision 2: WebSocket Implementation

**Choice**: Create WebSocket type that wraps CDP Network.webSocketCreated events.

```rust
page.on_websocket(|ws: WebSocket| async move {
    println!("WebSocket opened: {}", ws.url());
    
    ws.on_framesent(|frame| async move {
        println!("Sent: {:?}", frame.payload());
        Ok(())
    }).await?;
    
    ws.on_framereceived(|frame| async move {
        println!("Received: {:?}", frame.payload());
        Ok(())
    }).await?;
    
    Ok(())
}).await?;
```

**Rationale**:
- Mirrors Playwright's WebSocket API
- Events are per-socket, not global
- Supports binary and text frames

### Decision 3: HAR Replay Strategy

**Choice**: Parse HAR at route registration, match requests during interception.

```rust
// Basic HAR replay
page.route_from_har("api.har").await?;

// With options
page.route_from_har_with_options("api.har", HarOptions::new()
    .url("**/api/**")
    .update(true)  // Record missing entries
    .update_content(UpdateMode::Embed)
).await?;
```

**Rationale**:
- HAR parsing happens once at registration
- Matching uses existing route infrastructure
- Update mode enables HAR maintenance

### Decision 4: Exposed Function Callbacks

**Choice**: Use `Runtime.addBinding` with a message queue for Rust callbacks.

```rust
page.expose_function("compute", |args: Vec<serde_json::Value>| async move {
    let x: i64 = serde_json::from_value(args[0].clone())?;
    let y: i64 = serde_json::from_value(args[1].clone())?;
    Ok(serde_json::to_value(x + y)?)
}).await?;

// In JavaScript:
// const result = await window.compute(1, 2);  // returns 3
```

**Rationale**:
- CDP's addBinding provides the JS->Rust bridge
- Message queue handles async execution
- Serde provides type conversion

### Decision 5: Aria Snapshot Format

**Choice**: Use YAML-like format matching Playwright for compatibility.

```rust
expect(locator).to_match_aria_snapshot(r#"
- navigation:
  - link "Home"
  - link "About"
  - link /Contact.*/
"#).await?;
```

**Rationale**:
- Matches Playwright's format for familiarity
- Regex support via `/pattern/` syntax
- Hierarchical structure reflects accessibility tree

### Decision 6: Soft Assertions Collection

**Choice**: Thread-local collection with test harness integration.

```rust
// Soft assertions don't fail immediately
expect.soft(locator1).to_have_text("A").await?;
expect.soft(locator2).to_have_text("B").await?;
expect.soft(locator3).to_have_text("C").await?;

// Check all at once
expect.soft_assertions_passed()?;  // Fails if any soft assertion failed
```

**Rationale**:
- Thread-local storage is test-runner agnostic
- Explicit check allows seeing all failures
- Integrates with viewpoint-test harness

### Decision 7: Context Routing Implementation

**Choice**: Register routes at browser level, propagate to all pages.

```rust
// Context routes apply to all pages
context.route("**/api/**", |route| async move {
    route.fulfill().json(&mock_data).await
}).await?;

// New pages automatically inherit routes
let page = context.new_page().await?;
// page already has the route registered
```

**Rationale**:
- Uses Browser.setDownloadBehavior pattern
- Routes stored in context, applied on page creation
- Existing pages updated when routes added

## Implementation Order

The features have dependencies that dictate implementation order:

1. **Event System** (foundation for many features)
2. **Popup Handling** (depends on event system)
3. **Context Routing** (depends on existing route infrastructure)
4. **WebSocket Monitoring** (depends on event system)
5. **Exposed Functions** (independent but complex)
6. **HAR Replay** (depends on context routing)
7. **Storage State Advanced** (independent)
8. **API Cookie Sync** (independent)
9. **Aria Accessibility** (independent)
10. **Soft Assertions** (independent)

## CDP Commands Required

| Feature | CDP Command/Event | Domain |
|---------|------------------|--------|
| Event system | Target.attachedToTarget | Target |
| WebSocket | Network.webSocketCreated/Closed/FrameSent/FrameReceived | Network |
| HAR replay | Uses existing Fetch.requestPaused | Fetch |
| Exposed functions | Runtime.addBinding, Runtime.bindingCalled | Runtime |
| Popup handling | Target.targetCreated, Target.attachedToTarget | Target |
| Aria snapshots | Accessibility.getFullAXTree | Accessibility |
| Context routing | Fetch.enable at browser level | Fetch |
| HTTP auth | Fetch.authRequired, Fetch.continueWithAuth | Fetch |

## Risks / Trade-offs

### Risk: Event Handler Memory Leaks

**Mitigation**:
- Weak references where possible
- Clear handlers on context/page close
- Document cleanup requirements

### Risk: Exposed Function Callback Complexity

**Mitigation**:
- Limit to JSON-serializable arguments
- Timeout on callback execution
- Clear error messages for type mismatches

### Risk: HAR File Size

**Mitigation**:
- Lazy body loading option
- Content omission for large responses
- Streaming parser for large files

## Resolved Questions

### Question 1: Should soft assertions integrate with standard Rust test frameworks?

**Decision**: No - viewpoint-test integration only.

Soft assertions will only work automatically within the `#[viewpoint_test]` macro, which will check soft assertions at test end. Users of plain `#[tokio::test]` or `#[test]` must manually call `soft_assertions_passed()` to check for failures.

**Rationale**:
- Simpler implementation without custom panic hooks or Drop guards
- Matches Playwright's model (which has its own test runner)
- Users of viewpoint-test get automatic checking
- Users of other frameworks can still use soft assertions with explicit checks
- Can add broader integration later if demand arises

### Question 2: Should exposed functions support streaming responses?

**Decision**: No - single response only.

Exposed functions will return a single JSON value. Streaming responses are not supported.

```rust
page.expose_function("compute", |args: Vec<serde_json::Value>| async move {
    // Return a single JSON value
    Ok(serde_json::to_value(result)?)
}).await?;
```

**Rationale**:
- Matches Playwright's current behavior
- Covers 99% of use cases
- Simpler CDP interaction (single Runtime.evaluate response)
- Streaming can be simulated with multiple function calls from JS if needed
- Avoids complex ongoing communication protocol on top of CDP
- Can add streaming as a separate feature later if needed

### Question 3: Should HAR update mode preserve timing information?

**Decision**: Configurable, with placeholder timings as the default.

```rust
page.route_from_har_with_options("api.har", HarOptions::new()
    .update(true)
    .update_timings(TimingMode::Placeholder)  // Default
    // or .update_timings(TimingMode::Actual)
).await?;
```

When `TimingMode::Placeholder` (default):
- New HAR entries use `0` or `-1` for all timing values
- Produces clean, consistent git diffs
- Works identically in CI and local environments

When `TimingMode::Actual`:
- Records real timing values from the test run
- HAR reflects actual network performance
- Useful for performance analysis use cases

**Rationale**:
- Most users want HAR for mocking, not performance analysis
- Placeholder timings avoid noisy git diffs from timing variations
- Users who need real timings can opt-in explicitly
- Default behavior is predictable and CI-friendly
- Flexible enough for power users with specific needs
