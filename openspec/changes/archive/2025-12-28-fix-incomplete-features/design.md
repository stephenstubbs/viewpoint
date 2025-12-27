# Design: Fix Incomplete Feature Implementations

## Context

The Viewpoint browser automation framework has several features that are partially implemented. The infrastructure exists (types, handlers, CDP protocol bindings) but the wiring between CDP events and user-facing handlers is incomplete. This was discovered during integration testing.

### Current State

1. **HTTP Authentication**
   - `AuthHandler` exists in `network/auth.rs` with full credential handling logic
   - `RouteHandlerRegistry` can handle `Fetch.authRequired` events
   - BUT: Pages don't enable Fetch with `handleAuthRequests: true` when context has credentials
   - BUT: Page-level fetch isn't started when context credentials are present

2. **Dialog Handling**
   - `Dialog` type exists with `accept()`, `dismiss()`, `accept_with_text()` methods
   - `PageEventManager.handle_dialog_event()` exists and routes to user handlers
   - BUT: `Page.javascriptDialogOpening` events are not subscribed to
   - BUT: No call to `handle_dialog_event()` from the event listener

3. **Route Interception**
   - `Route`, `FulfillBuilder`, `ContinueBuilder` types exist with full CDP command support
   - `RouteHandlerRegistry` stores handlers and processes `Fetch.requestPaused`
   - BUT: Handler invocation may not be reaching user closures correctly
   - BUT: Context-level routes may not propagate to pages correctly

## Goals

- Make HTTP authentication work for browser page navigation
- Make dialog events fire and allow users to accept/dismiss dialogs
- Make route interception work for page-initiated requests
- Un-ignore the 8 currently skipped integration tests

## Non-Goals

- Adding new features beyond what's already specified
- Performance optimization
- Refactoring the overall architecture

## Decisions

### Decision 1: Dialog Event Wiring

Add `Page.javascriptDialogOpening` handling to `PageEventManager.start_event_listener()`.

**Why**: This matches how other events (console, pageerror, frame events) are already handled. The `handle_dialog_event` method already exists and does the right thing.

**Implementation**:
```rust
// In start_event_listener match:
"Page.javascriptDialogOpening" => {
    if let Some(params) = &event.params {
        if let Ok(dialog_event) = serde_json::from_value::<JavascriptDialogOpeningEvent>(params.clone()) {
            // Call handle_dialog_event on the event manager
        }
    }
}
```

**Challenge**: The event listener runs in a spawned task without access to `self`. Need to pass the dialog handler reference or use a channel.

**Solution**: Pass the dialog_handler Arc and wait_for_dialog_tx Arc into the spawned task, similar to how console_handler is already passed.

### Decision 2: HTTP Auth Wiring

When a context has HTTP credentials, ensure pages enable Fetch with auth handling.

**Why**: The credentials are stored but never used because Fetch.enable isn't called with the right parameters.

**Implementation**:
1. In `Page::new()` or `Page::with_opener()`, check if context has credentials
2. If so, call `route_registry.set_http_credentials(creds)` 
3. Ensure `Fetch.enable` is called with `handleAuthRequests: true`

**Alternative considered**: Wire auth at context level instead of page level. Rejected because CDP Fetch domain is session-specific (per page).

### Decision 3: Route Handler Verification

Trace through `Fetch.requestPaused` handling to ensure user handlers are invoked.

**Why**: The code exists but tests show it doesn't work. Need to verify:
1. `Fetch.enable` is called when routes are registered
2. `Fetch.requestPaused` events reach the handler registry
3. User closures are actually invoked
4. `Fetch.continueRequest` / `fulfillRequest` / `failRequest` is sent

**Implementation**: Add tracing and step through the code path. Fix any gaps found.

## Risks / Trade-offs

### Risk: Breaking existing tests
**Mitigation**: Run full test suite after each change. The 115 currently passing tests should remain passing.

### Risk: CDP event ordering issues
**Mitigation**: Follow the same patterns used for console/frame events which work correctly.

### Trade-off: Complexity in event listener
The event listener is getting large with many match arms. Acceptable for now; refactor later if needed.

## Migration Plan

No migration needed. These are bug fixes to make existing APIs work as specified.

## Open Questions - RESOLVED

### Question 1: Is `Page.enable` called for dialogs?

**RESOLVED: YES** - `Page.enable` is already called in `context/mod.rs` line 335 during page setup.

```rust
// In BrowserContext::new_page() - line 335
self.connection
    .send_command::<(), serde_json::Value>("Page.enable", None, Some(session_id))
    .await?;
```

The missing piece is that `Page.javascriptDialogOpening` events are not handled in `start_event_listener()`.

### Question 2: How do credentials flow from context to pages?

**RESOLVED: THEY DON'T** - This is the bug.

Investigation found:
- `RouteHandlerRegistry::with_context_routes()` (handler.rs lines 98-113) only receives `context_routes`, NOT credentials
- `RouteHandlerRegistry::set_http_credentials()` exists (line 196) but is never called from context
- The `AuthHandler` is created fresh in `with_context_routes()` without any credentials

**Fix needed**: Pass credentials from `BrowserContext` when creating page's `RouteHandlerRegistry`, and call `set_http_credentials()`.

## Investigation Findings

### Dialog Events - Root Cause Identified

The `start_event_listener()` method (events.rs lines 170-327) handles these events:
- ✅ `Runtime.consoleAPICalled`
- ✅ `Runtime.exceptionThrown`  
- ✅ `Page.frameAttached`
- ✅ `Page.frameNavigated`
- ✅ `Page.frameDetached`
- ❌ `Page.javascriptDialogOpening` - **MISSING**

The `handle_dialog_event()` method exists (lines 534-571) and works correctly, but it's **never called** because the event listener doesn't handle the CDP event.

**Fix**: Add `Page.javascriptDialogOpening` match arm and pass `dialog_handler` + `wait_for_dialog_tx` into the spawned task.

### HTTP Auth - Root Cause Identified

The wiring is broken at page creation time:
1. `BrowserContext` stores credentials in `self.options.http_credentials`
2. When creating a page, `Page::with_context_routes()` is called
3. `RouteHandlerRegistry::with_context_routes()` creates a fresh `AuthHandler` with NO credentials
4. Credentials are never passed through

**Fix**: Modify `with_context_routes()` to accept optional credentials, OR add a method to set credentials after creation.

### Route Interception - Status

The infrastructure appears complete:
- `start_fetch_listener()` is called in all Page constructors (lines 135, 191, 272)
- `Fetch.requestPaused` handling exists (handler.rs lines 152-168)
- `ensure_fetch_enabled()` is called when routes are registered

Route tests may be failing due to timing issues or because Fetch isn't enabled before navigation. Will verify after fixing other issues.
