# Change: Add Deferred Features

## Why

During the initial Playwright feature parity implementation, several advanced features were deferred because they required additional infrastructure or represented optional functionality. These features are now ready to be implemented to complete the Viewpoint API.

The deferred items fall into these categories:
1. **Event System** - Context and page-level event handlers (on/off pattern)
2. **WebSocket Monitoring** - Track WebSocket connections and messages
3. **HAR Replay** - route_from_har() for replaying recorded network traffic
4. **Exposed Functions** - Bidirectional JS-to-Rust function calls
5. **Popup Handling** - Track and interact with popup windows
6. **Aria Accessibility** - Accessibility tree snapshots and assertions
7. **Soft Assertions** - Collect multiple assertion failures without stopping
8. **Storage State Advanced** - localStorage and IndexedDB support
9. **Context Routing** - BrowserContext-level network interception
10. **API Cookie Sync** - Bidirectional cookie sharing between browser and API context

## What Changes

### New Capabilities

1. **Event System Infrastructure**
   - `context.on('page')` / `context.off('page')` - new page events
   - `context.on('close')` / `context.off('close')` - context close events
   - `context.wait_for_page(action)` - wait for new page during action
   - Event handler type system for typed callbacks

2. **WebSocket Monitoring**
   - `page.on('websocket')` - WebSocket connection events
   - `websocket.url()` - connection URL
   - `websocket.on('framesent')` - outgoing message events
   - `websocket.on('framereceived')` - incoming message events
   - `websocket.on('close')` - connection close events

3. **HAR Replay**
   - `page.route_from_har(path)` - replay from HAR file
   - `context.route_from_har(path)` - context-level HAR replay
   - URL filtering and strict mode options
   - Update mode for recording missing entries

4. **Exposed Functions**
   - `page.expose_function(name, callback)` - expose Rust function to JS
   - `context.expose_function(name, callback)` - context-wide exposure
   - Async function support
   - Automatic rebinding across navigations

5. **Popup Handling**
   - `page.on('popup')` - popup window events
   - `page.wait_for_popup(action)` - wait for popup during action
   - `page.opener()` - get opener page reference
   - Popup pages share context with opener

6. **Aria Accessibility**
   - `locator.aria_snapshot()` - get accessibility tree
   - `expect(locator).to_match_aria_snapshot(yaml)` - assert accessibility
   - YAML-like snapshot format matching Playwright
   - Regex support in snapshots

7. **Soft Assertions**
   - `expect.soft(locator)` - non-failing assertions
   - Collect all failures and report at end
   - Integration with test harness

8. **Storage State Advanced**
   - `storage_state().indexed_db(true)` - include IndexedDB
   - localStorage collection from all origins
   - localStorage/IndexedDB restoration on context create

9. **Context Routing**
   - `context.route(pattern, handler)` - context-level interception
   - `context.unroute(pattern)` / `context.unroute_all()`
   - Routes apply to all pages in context
   - New pages inherit context routes

10. **API Cookie Sync**
    - Sync cookies from browser context to API context
    - Sync cookies from API responses back to browser
    - Proper domain/path handling

### Additional Improvements

- `locator.highlight()` - visual debugging helper
- `locator.evaluate()` / `locator.evaluate_all()` - element-scoped JS
- `locator.element_handle()` - get underlying element handle
- `context.add_init_script()` - context-wide init scripts
- Screenshot element masking
- Timeout wiring from context to pages
- HTTP credentials auth challenge handling

## Impact

- **New specs**: `event-system`, `websocket-monitoring`, `exposed-functions`, `popup-handling`
- **Modified specs**: `har-support`, `advanced-assertions`, `storage-state`, `network-routing`, `api-request-context`, `context-lifecycle`, `http-credentials`
- **Affected code**:
  - `viewpoint-core/src/context/` - event system, routing
  - `viewpoint-core/src/page/` - popups, websocket, exposed functions
  - `viewpoint-core/src/network/` - HAR replay, context routing
  - `viewpoint-test/src/expect/` - soft assertions, aria assertions
- **Breaking changes**: None (all additive)
- **Dependencies**: Event system should be implemented first as foundation
