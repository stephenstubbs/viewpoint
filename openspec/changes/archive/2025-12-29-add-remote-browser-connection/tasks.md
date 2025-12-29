## 1. CDP Layer Updates

- [x] 1.1 Add `reqwest` dependency to `viewpoint-cdp` for HTTP endpoint discovery
- [x] 1.2 Create `CdpConnection::connect_via_http(endpoint_url)` that fetches `/json/version` and extracts `webSocketDebuggerUrl`
- [x] 1.3 Add connection options struct (`CdpConnectionOptions`) with timeout and headers fields
- [x] 1.4 Update `CdpConnection::connect()` to accept optional `CdpConnectionOptions`

## 2. Browser Connection Builder

- [x] 2.1 Create `ConnectOverCdpBuilder` struct in `browser/mod.rs` or new `browser/connector/mod.rs`
- [x] 2.2 Implement builder methods: `timeout()`, `header()`, `headers()`
- [x] 2.3 Implement `connect()` async method that uses CDP layer

## 3. Browser::connect_over_cdp Implementation

- [x] 3.1 Add `Browser::connect_over_cdp(endpoint_url: &str) -> ConnectOverCdpBuilder`
- [x] 3.2 Handle both HTTP and WebSocket URL inputs (auto-detect based on scheme)
- [x] 3.3 Add error types: `BrowserError::EndpointDiscoveryFailed`, `BrowserError::InvalidEndpointUrl`

## 4. Browser::contexts Implementation

- [x] 4.1 Add `GetBrowserContextsParams` type to `viewpoint-cdp` target_domain if missing
- [x] 4.2 Implement `Browser::contexts() -> Result<Vec<BrowserContext>, BrowserError>`
- [x] 4.3 Use `Target.getBrowserContexts` CDP command
- [x] 4.4 Handle default context (empty string ID) appropriately
- [x] 4.5 Track context ownership (owned vs connected-to)

## 5. BrowserContext Wrapping for Existing Contexts

- [x] 5.1 Add `owned` field to `BrowserContext` struct
- [x] 5.2 Create `BrowserContext::from_existing()` constructor for connected contexts
- [x] 5.3 Modify `BrowserContext::close()` to skip dispose for non-owned contexts
- [x] 5.4 Ensure `context.pages()` works for existing contexts

## 6. Integration Tests

- [x] 6.1 Test `connect_over_cdp` with HTTP endpoint URL
- [x] 6.2 Test `connect_over_cdp` with WebSocket URL (should also work)
- [x] 6.3 Test `browser.contexts()` returns existing contexts
- [x] 6.4 Test accessing pages in existing contexts
- [x] 6.5 Test connection timeout handling
- [x] 6.6 Test invalid endpoint URL error handling

## 7. Documentation

- [x] 7.1 Update `Browser` rustdoc with `connect_over_cdp` examples
- [x] 7.2 Document behavior differences between launched vs connected browsers
- [x] 7.3 Add example showing MCP-like usage pattern
