# Change: Fix Incomplete Feature Implementations

## Why

Several features specified in the specs have partial implementations that don't fully work:

1. **HTTP Authentication**: Credentials are stored but `Fetch.authRequired` events are not connected to page navigation flows. The auth handler exists but pages don't receive auth challenges.

2. **Dialog Event Handling**: The `handle_dialog_event` method exists but `Page.javascriptDialogOpening` CDP events are not routed to it. Dialogs cause pages to hang.

3. **Route Interception**: Route handlers are registered and `Fetch.requestPaused` events are processed, but the handlers don't reliably receive routed requests for page-initiated requests.

These gaps were discovered during integration testing in the `enhance-integration-tests` proposal.

## Investigation Findings (2024-12-28)

### Dialog Events - Root Cause Found
- **`Page.enable` IS called** (context/mod.rs:335) ✓
- **`Page.javascriptDialogOpening` NOT handled** in `start_event_listener()` (events.rs:170-327)
- The `handle_dialog_event()` method exists (events.rs:534-571) but is **never invoked**
- **Fix**: Add the missing event match arm and pass handler refs into spawned task

### HTTP Auth - Root Cause Found  
- `BrowserContext` stores credentials in `self.options.http_credentials`
- `RouteHandlerRegistry::with_context_routes()` (handler.rs:98-113) creates fresh `AuthHandler` with NO credentials
- `set_http_credentials()` exists (handler.rs:196) but is **never called from context**
- **Fix**: Pass credentials through `with_context_routes()` or call `set_http_credentials()` after creation

### Route Interception - Infrastructure Complete
- `start_fetch_listener()` called in all Page constructors (lines 135, 191, 272)
- `Fetch.requestPaused` handling exists (handler.rs:152-168)
- May be timing issues - needs verification with tracing

## What Changes

### Dialog Handling (Phase 1)
- Add `Page.javascriptDialogOpening` match arm to `start_event_listener()` in events.rs
- Pass `dialog_handler` and `wait_for_dialog_tx` Arcs into the spawned event listener task
- Create `Dialog` from event params and dispatch to handler or waiter
- **No change needed** to `Page.enable` - already called

### HTTP Credentials (Phase 2)
- Modify `RouteHandlerRegistry::with_context_routes()` to accept optional `HttpCredentials`
- In `Page::with_context_routes()`, pass context credentials to the registry
- Call `set_http_credentials()` which triggers `Fetch.enable` with `handleAuthRequests: true`

### Route Interception (Phase 3)
- Verify with tracing that events flow correctly
- Fix any timing issues (Fetch.enable must happen before navigation)
- May need no code changes if issue is test-specific

## Impact

- **Affected specs**: `http-credentials`, `dialogs`, `network-routing`
- **Affected code**:
  - `crates/viewpoint-core/src/page/events.rs` - Add dialog event handling
  - `crates/viewpoint-core/src/page/mod.rs` - Enable Page domain for dialogs
  - `crates/viewpoint-core/src/context/mod.rs` - Wire credentials to pages
  - `crates/viewpoint-core/src/network/handler.rs` - Fix route handler invocation
- **Tests**: Will un-ignore 8 integration tests that are currently skipped
