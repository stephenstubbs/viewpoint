# Change: Add Network Interception

## Why

Network interception is essential for modern web testing:
- **API Mocking**: Test UI without hitting real backends
- **Response Modification**: Test error handling, edge cases
- **Request Blocking**: Block ads, analytics, or specific resources
- **Network Monitoring**: Capture requests/responses for debugging
- **HAR Recording**: Record and replay network traffic for reproducible tests

This is proposal 2 of 11 in the Playwright feature parity series (Chromium only).

## What Changes

### New Capabilities

1. **Request Routing** - Intercept and handle network requests
   - `page.route()` / `context.route()` - register route handlers
   - `route.fulfill()` - respond with custom data
   - `route.continue()` - continue with optional modifications
   - `route.abort()` - abort the request
   - `route.fallback()` - pass to next handler
   - `route.fetch()` - fetch and modify response
   - URL matching via glob patterns and regex

2. **Network Events** - Monitor network activity
   - `page.on('request')` - emitted when request is issued
   - `page.on('response')` - emitted when response is received
   - `page.on('requestfinished')` - emitted when request completes
   - `page.on('requestfailed')` - emitted when request fails
   - `page.wait_for_request()` - wait for specific request
   - `page.wait_for_response()` - wait for specific response

3. **Request Object** - Access request details
   - URL, method, headers, post data
   - Resource type, timing information
   - Redirect chain, frame association
   - Request sizes

4. **Response Object** - Access response details
   - Status code, status text, headers
   - Body as text, JSON, or bytes
   - Security details, server address
   - Associated request

5. **HAR Support** - Record and replay network traffic
   - `context.route_from_har()` - replay from HAR file
   - `page.route_from_har()` - page-level HAR replay
   - Record HAR during test execution
   - Update HAR files with new responses

6. **WebSocket Interception** - Monitor WebSocket connections
   - `page.on('websocket')` - WebSocket opened event
   - Frame sent/received events
   - WebSocket close event

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1 | Core Page Operations | Created |
| **2** | **Network Interception** (this) | **Current** |
| 3 | Input Devices | Pending |
| 4 | Browser Context Features | Pending |
| 5-11 | ... | Pending |

## Impact

- **New specs**: `network-routing`, `network-events`, `har-support`
- **Affected code**: 
  - `viewpoint-cdp/src/protocol/network.rs` - expand Network domain
  - `viewpoint-cdp/src/protocol/fetch.rs` - new Fetch domain for interception
  - `viewpoint-core/src/page/` - add routing and event methods
  - `viewpoint-core/src/network/` - new module for Request, Response, Route types
- **Breaking changes**: None
- **Dependencies**: None (can be implemented in parallel with proposal 1)
