## Phase 1: Dialog Event Handling

**Root Cause**: `Page.javascriptDialogOpening` events are not handled in `start_event_listener()`. The `handle_dialog_event()` method exists but is never invoked.

**Note**: `Page.enable` is already called (context/mod.rs:335) - no change needed there.

- [x] 1.1 Check if `JavascriptDialogOpeningEvent` type exists in viewpoint-cdp, add if missing
- [x] 1.2 Pass `dialog_handler` Arc into `start_event_listener` spawned task
- [x] 1.3 Pass `wait_for_dialog_tx` Arc into `start_event_listener` spawned task
- [x] 1.4 Add match arm for `Page.javascriptDialogOpening` in event listener (events.rs ~line 192)
- [x] 1.5 Create Dialog from event params and dispatch to handler/waiter
- [x] 1.6 Un-ignore and verify `test_alert_dialog` passes
- [x] 1.7 Un-ignore and verify `test_confirm_dialog` passes
- [x] 1.8 Un-ignore and verify `test_prompt_dialog` passes

## Phase 2: HTTP Authentication

**Root Cause**: `RouteHandlerRegistry::with_context_routes()` creates a fresh `AuthHandler` without credentials. The `BrowserContext`'s credentials are never passed to pages.

**Key files**:
- `context/mod.rs` - has credentials in `self.options.http_credentials`
- `network/handler.rs:98-113` - `with_context_routes()` doesn't accept credentials
- `network/handler.rs:196` - `set_http_credentials()` exists but never called from context

- [x] 2.1 Add optional `HttpCredentials` parameter to `RouteHandlerRegistry::with_context_routes()`
- [x] 2.2 In `Page::with_context_routes()`, pass context credentials to registry
- [x] 2.3 Call `set_http_credentials()` if credentials are provided
- [x] 2.4 Verify `Fetch.enable` is called with `handleAuthRequests: true` (already happens via `set_http_credentials`)
- [x] 2.5 Un-ignore and verify `test_http_basic_auth` passes
- [x] 2.6 Un-ignore and verify `test_http_auth_wrong_credentials` passes

## Phase 3: Route Interception

**Root Cause**: Routes added after page creation didn't work because Fetch domain wasn't enabled on existing pages when context.route() was called.

**Solution**: 
- `ContextRouteRegistry` now tracks weak references to page `RouteHandlerRegistry` instances
- When a route is added via `context.route()`, it synchronously enables Fetch on all registered page registries before returning
- Pages register their registry with the context during `with_context_routes()`

**Key files modified**:
- `context/routing.rs` - Added `page_registries: RwLock<Vec<Weak<RouteHandlerRegistry>>>`, `register_page_registry()`, and `enable_fetch_on_all_pages()`
- `network/handler.rs` - Added `ensure_fetch_enabled_public()` wrapper method
- `page/mod.rs` - Made `with_context_routes()` async, added call to `context_routes.register_page_registry()`
- `context/mod.rs` - Added `.await` to `with_context_routes()` calls

- [x] 3.1 Un-ignore `test_route_abort` and run with tracing to diagnose
- [x] 3.2 Verify `Fetch.enable` is called BEFORE navigation (timing issue check)
- [x] 3.3 Add debug logging to `handle_request()` to confirm events arrive
- [x] 3.4 Verify handler matching logic finds registered handlers
- [x] 3.5 Un-ignore and verify `test_route_abort` passes
- [x] 3.6 Un-ignore and verify `test_route_fulfill_custom` passes
- [x] 3.7 Un-ignore and verify `test_route_continue_with_headers` passes

## Phase 4: Verification

- [x] 4.1 Run full test suite, ensure no regressions
- [x] 4.2 Verify all 8 previously ignored tests now pass
- [x] 4.3 Update test coverage summary in tasks.md

## Test Results

All 8 previously-ignored tests now pass:
```
test test_alert_dialog ... ok
test test_confirm_dialog ... ok
test test_prompt_dialog ... ok
test test_http_basic_auth ... ok
test test_http_auth_wrong_credentials ... ok
test test_route_abort ... ok
test test_route_fulfill_custom ... ok
test test_route_continue_with_headers ... ok
```
