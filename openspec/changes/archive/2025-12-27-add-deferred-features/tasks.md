# Tasks: Add Deferred Features

## 1. Event System Infrastructure

- [x] 1.1 Create `EventHandler<T>` trait for typed async callbacks
- [x] 1.2 Create `HandlerId` type for handler identification
- [x] 1.3 Create `EventEmitter<T>` for managing handler collections
- [x] 1.4 Implement `context.on_page(handler)` for new page events
- [x] 1.5 Implement `context.off_page(handler_id)` for handler removal
- [x] 1.6 Implement `context.on_close(handler)` for close events
- [x] 1.7 Implement `context.wait_for_page(action)` helper
- [x] 1.8 Emit close event before context cleanup
- [x] 1.9 Add tests for event system

## 2. Popup Handling

- [x] 2.1 Add `Target.targetCreated` event handling
- [x] 2.2 Detect popup windows vs regular targets
- [x] 2.3 Create popup Page with shared context
- [x] 2.4 Implement `page.on_popup(handler)` event
- [x] 2.5 Implement `page.wait_for_popup(action)` helper
- [x] 2.6 Implement `page.opener()` for popup pages
- [x] 2.7 Add tests for popup handling

## 3. Context-Level Routing

- [x] 3.1 Create route handler registry for BrowserContext
- [x] 3.2 Implement `context.route(pattern, handler)`
- [x] 3.3 Implement `context.unroute(pattern)`
- [x] 3.4 Implement `context.unroute_all()`
- [x] 3.5 Apply context routes to existing pages on registration
- [x] 3.6 Apply context routes to new pages on creation
- [x] 3.7 Ensure page routes take precedence over context routes
- [x] 3.8 Add tests for context routing

## 4. WebSocket Monitoring

- [x] 4.1 Add `Network.webSocketCreated` event handling
- [x] 4.2 Add `Network.webSocketClosed` event handling
- [x] 4.3 Add `Network.webSocketFrameSent` event handling
- [x] 4.4 Add `Network.webSocketFrameReceived` event handling
- [x] 4.5 Create `WebSocket` struct with URL and state
- [x] 4.6 Create `WebSocketFrame` struct for message data
- [x] 4.7 Implement `page.on_websocket(handler)` event
- [x] 4.8 Implement `websocket.url()` accessor
- [x] 4.9 Implement `websocket.on_framesent(handler)`
- [x] 4.10 Implement `websocket.on_framereceived(handler)`
- [x] 4.11 Implement `websocket.on_close(handler)`
- [x] 4.12 Add tests for WebSocket monitoring

## 5. Exposed Functions

- [x] 5.1 Add `Runtime.addBinding` CDP command
- [x] 5.2 Add `Runtime.bindingCalled` event handling
- [x] 5.3 Create binding callback registry
- [x] 5.4 Implement `page.expose_function(name, callback)`
- [x] 5.5 Implement `context.expose_function(name, callback)`
- [x] 5.6 Handle async function callbacks
- [x] 5.7 Re-bind functions after navigation
- [x] 5.8 Support JSON-serializable arguments and return values
- [x] 5.9 Add tests for exposed functions

## 6. HAR Replay

- [x] 6.1 Create HAR file parser with serde
- [x] 6.2 Implement HAR entry matching by URL
- [x] 6.3 Implement HAR entry matching by method
- [x] 6.4 Implement HAR entry matching by post data
- [x] 6.5 Create `HarOptions` builder for configuration
- [x] 6.6 Implement `page.route_from_har(path)`
- [x] 6.7 Implement `page.route_from_har_with_options(path, options)`
- [x] 6.8 Implement `context.route_from_har(path)`
- [x] 6.9 Implement URL filter option
- [x] 6.10 Implement strict mode (fail if no match)
- [x] 6.11 Implement update mode (record missing entries)
- [x] 6.12 Implement timing preservation option
- [x] 6.13 Add tests for HAR replay

## 7. Storage State Advanced

- [x] 7.1 Implement localStorage collection via JS evaluation per origin
- [x] 7.2 Implement IndexedDB snapshot via JS evaluation
- [x] 7.3 Add `storage_state().indexed_db(true)` option
- [x] 7.4 Implement localStorage restoration on context create
- [x] 7.5 Implement IndexedDB restoration on context create
- [x] 7.6 Add tests for advanced storage state

## 8. API Cookie Sync

- [x] 8.1 Implement sync cookies from browser to API context
- [x] 8.2 Implement sync cookies from API responses to browser
- [x] 8.3 Handle cookie domain/path matching correctly
- [x] 8.4 Add automatic sync on API request
- [x] 8.5 Add tests for cookie synchronization

## 9. Aria Accessibility

- [x] 9.1 Add CDP `Accessibility.getFullAXTree` command
- [x] 9.2 Create aria snapshot format (YAML-like)
- [x] 9.3 Implement accessibility tree traversal
- [x] 9.4 Implement `locator.aria_snapshot()` method
- [x] 9.5 Create snapshot comparison logic
- [x] 9.6 Support regex patterns in snapshots
- [x] 9.7 Implement `to_match_aria_snapshot()` assertion
- [x] 9.8 Generate helpful diff on failure
- [x] 9.9 Add tests for aria snapshots

## 10. Soft Assertions

- [x] 10.1 Create soft assertion context (thread-local storage)
- [x] 10.2 Create `SoftAssertionError` collection type
- [x] 10.3 Implement `expect.soft(locator)` method
- [x] 10.4 Collect failures without throwing
- [x] 10.5 Implement `expect.soft_assertions_passed()` check
- [x] 10.6 Integrate with viewpoint-test harness
- [x] 10.7 Add tests for soft assertions

## 11. Locator Enhancements

- [x] 11.1 Implement `locator.highlight()` visual debugging
- [x] 11.2 Implement `locator.evaluate(expression)` method
- [x] 11.3 Implement `locator.evaluate_all(expression)` method
- [x] 11.4 Implement `locator.element_handle()` method
- [x] 11.5 Implement screenshot element masking
- [x] 11.6 Add tests for locator enhancements

## 12. Context Enhancements

- [x] 12.1 Implement `context.add_init_script(script)`
- [x] 12.2 Implement `context.add_init_script_path(path)`
- [x] 12.3 Apply init scripts to new pages
- [x] 12.4 Wire context timeout to page operations
- [x] 12.5 Implement page-level timeout override
- [x] 12.6 Add tests for context enhancements

## 13. HTTP Credentials Auth

- [x] 13.1 Add `Fetch.authRequired` event handling
- [x] 13.2 Add `Fetch.continueWithAuth` command
- [x] 13.3 Wire http_credentials to auth challenges
- [x] 13.4 Support Basic and Digest authentication
- [x] 13.5 Add tests for HTTP authentication

## 14. Documentation

- [x] 14.1 Document event system patterns
- [x] 14.2 Document WebSocket monitoring
- [x] 14.3 Document HAR replay usage
- [x] 14.4 Document exposed functions
- [x] 14.5 Document popup handling
- [x] 14.6 Document aria accessibility testing
- [x] 14.7 Document soft assertions
- [x] 14.8 Update README with new features

## Dependencies

Implementation order based on dependencies:

1. **Event System (1)** - Foundation for popups, websocket, etc.
2. **Popup Handling (2)** - Depends on event system
3. **Context Routing (3)** - Depends on existing route infrastructure
4. **WebSocket Monitoring (4)** - Depends on event system
5. **Exposed Functions (5)** - Independent but complex
6. **HAR Replay (6)** - Depends on context routing
7. **Storage State (7)** - Independent
8. **API Cookie Sync (8)** - Independent
9. **Aria Accessibility (9)** - Independent
10. **Soft Assertions (10)** - Independent
11. **Locator Enhancements (11)** - Independent
12. **Context Enhancements (12)** - Depends on event system for some items
13. **HTTP Auth (13)** - Independent

## Parallelizable Work

After event system is complete:
- Popup Handling and WebSocket Monitoring (both need events)
- Context Routing and Exposed Functions (independent)
- Storage State, API Cookie Sync, Aria, Soft Assertions (all independent)
- Locator and Context Enhancements (mostly independent)
