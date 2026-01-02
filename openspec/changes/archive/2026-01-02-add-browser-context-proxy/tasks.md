## 1. Type Definitions

- [x] 1.1 Create `ProxyConfig` type in `context/types/` (or re-export from `api/options`)
  - server: String (required) - proxy URL like "http://proxy:8080" or "socks5://proxy:1080"
  - username: Option<String> - authentication username
  - password: Option<String> - authentication password
  - bypass: Option<String> - comma-separated list of domains to bypass

## 2. Builder Methods

- [x] 2.1 Add `proxy` field to `ContextOptions` struct
- [x] 2.2 Add `proxy()` method to `ContextOptionsBuilder`
- [x] 2.3 Add `proxy()` method to `NewContextBuilder` (browser/context_builder.rs)

## 3. CDP Integration

- [x] 3.1 Wire proxy configuration to browser context creation
  - Used Target.createBrowserContext with proxy_server and proxy_bypass_list params
- [x] 3.2 Handle proxy authentication via Fetch.authRequired events
  - Extended AuthHandler to support both HTTP and proxy authentication
  - Added ProxyCredentials type and wiring through page factory

## 4. Testing

- [x] 4.1 Add unit tests for ProxyConfig builder methods
- [x] 4.2 Add integration test with mock proxy server (if feasible)
  - Added proxy_tests.rs with 6 integration tests covering:
    - Simple proxy configuration
    - Proxy with authentication credentials
    - SOCKS5 proxy
    - Proxy with bypass list
    - Full proxy configuration
    - Multiple contexts with different proxies
- [x] 4.3 Document proxy usage in code examples
  - Added doc examples on ProxyConfig type and builder methods

## 5. Documentation

- [x] 5.1 Update context builder documentation with proxy examples
  - Added examples to ProxyConfig, ContextOptionsBuilder.proxy(), and NewContextBuilder.proxy()
