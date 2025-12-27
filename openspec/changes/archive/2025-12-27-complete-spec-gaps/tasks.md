# Tasks: Complete Spec Gaps

## 1. Navigation Response Status/Headers

- [x] 1.1 Add `status: Option<u16>` and `headers: Option<HashMap<String, String>>` to `NavigationResponse`
- [x] 1.2 Extend `NavigationWaiter` to track main document response via `Network.responseReceived`
- [x] 1.3 Populate response status/headers when navigation completes
- [x] 1.4 Add `response.status()` and `response.headers()` accessor methods
- [x] 1.5 Add tests for navigation response data

## 2. Route Fetch Implementation

- [x] 2.1 Modify `route.fetch()` to use `Fetch.continueRequest` with `interceptResponse: true`
- [x] 2.2 Handle `Fetch.requestPaused` with response data
- [x] 2.3 Call `Fetch.getResponseBody` to retrieve response body
- [x] 2.4 Populate `FetchedResponse` with actual status, headers, body
- [x] 2.5 Support `route.fetch()` with request modifiers (headers, URL, etc.)
- [x] 2.6 Add timeout support to route fetch
- [x] 2.7 Add tests for route fetch functionality

## 3. Storage State Restoration

- [x] 3.1 Generate localStorage restoration script from `StorageState.origins`
- [x] 3.2 Apply localStorage script via `add_init_script` during context creation
- [x] 3.3 Implement IndexedDB restoration via JavaScript evaluation
- [x] 3.4 Apply IndexedDB restoration on first page navigation in context
- [x] 3.5 Add tests for localStorage restoration
- [x] 3.6 Add tests for IndexedDB restoration

## 4. HTTP Credentials Authentication

- [x] 4.1 Ensure `Fetch.enable` called with `handleAuthRequests: true` when credentials set
- [x] 4.2 Wire `Fetch.authRequired` event to auth handler
- [x] 4.3 Call `Fetch.continueWithAuth` with stored credentials on auth challenge
- [x] 4.4 Support Basic authentication scheme
- [x] 4.5 Support Digest authentication scheme
- [x] 4.6 Add tests for HTTP authentication

## 5. Context Route Propagation

- [x] 5.1 Store context routes in `BrowserContext.route_registry`
- [x] 5.2 In `new_page()`, apply context routes to new page's handler registry
- [x] 5.3 Ensure page routes take precedence over context routes
- [x] 5.4 Add tests for context route propagation to new pages

## 6. Advanced Assertions

- [x] 6.1 Add `to_have_count_greater_than(n)` assertion to `LocatorAssertions`
- [x] 6.2 Add `to_have_count_less_than(n)` assertion
- [x] 6.3 Add `to_have_count_at_least(n)` and `to_have_count_at_most(n)` assertions
- [x] 6.4 Add `to_match_aria_snapshot(expected)` assertion
- [x] 6.5 Generate meaningful diff on ARIA snapshot mismatch
- [x] 6.6 Add soft assertion variants for new assertions
- [x] 6.7 Add tests for count comparison assertions
- [x] 6.8 Add tests for ARIA snapshot assertions

## 7. Frame Events

- [x] 7.1 Add `on_frameattached(handler)` method to `Page`
- [x] 7.2 Add `on_framenavigated(handler)` method to `Page`
- [x] 7.3 Add `on_framedetached(handler)` method to `Page`
- [x] 7.4 Subscribe to CDP `Page.frameAttached` event in page event loop
- [x] 7.5 Subscribe to CDP `Page.frameNavigated` event
- [x] 7.6 Subscribe to CDP `Page.frameDetached` event
- [x] 7.7 Add `frame.child_frames()` method
- [x] 7.8 Add `frame.parent_frame()` method
- [x] 7.9 Add tests for frame events

## 8. Action Builders

- [x] 8.1 Add `ClickBuilder` with fluent API for click options
- [x] 8.2 Add `.position(x, y)` to `ClickBuilder` for offset from center
- [x] 8.3 Add `.button(MouseButton)` to `ClickBuilder` for right/middle click
- [x] 8.4 Add `.modifiers(&[Modifier])` to `ClickBuilder` for Shift/Ctrl/Alt
- [x] 8.5 Add `.force(true)` to `ClickBuilder` to skip actionability checks
- [x] 8.6 Add `.delay(Duration)` to type_text for character-by-character delay
- [x] 8.7 Add `.position(x, y)` to hover action
- [x] 8.8 Add tests for click builder options
- [x] 8.9 Add tests for type delay

## 9. Custom Test ID Attribute

- [x] 9.1 Add `set_test_id_attribute(name)` to `BrowserContext`
- [x] 9.2 Store custom test ID attribute in context state
- [x] 9.3 Update `get_by_test_id()` selector generation to use configured attribute
- [x] 9.4 Default to `data-testid` if not configured
- [x] 9.5 Add tests for custom test ID attribute

## 10. Tracing Snapshots

- [x] 10.1 Implement DOM snapshot capture when `snapshots(true)` is set
- [x] 10.2 Use `DOMSnapshot.captureSnapshot` CDP command
- [x] 10.3 Store snapshots in trace data
- [x] 10.4 Implement source file collection when `sources(true)` is set
- [x] 10.5 Add tests for trace with snapshots
- [x] 10.6 Add tests for trace with sources

## Dependencies

Implementation order based on dependencies:

1. **Navigation Response (1)** - Independent, foundational
2. **Route Fetch (2)** - Independent, foundational
3. **Storage State (3)** - Independent
4. **HTTP Credentials (4)** - Independent
5. **Context Route Propagation (5)** - Independent
6. **Advanced Assertions (6)** - Independent
7. **Frame Events (7)** - Independent
8. **Action Builders (8)** - Independent
9. **Custom Test ID (9)** - Independent
10. **Tracing Snapshots (10)** - Independent

## Parallelizable Work

All sections can be implemented in parallel as they affect different areas:
- Network: 1, 2, 4, 5
- Storage: 3
- Testing: 6, 8, 9
- Page: 7
- Tracing: 10
