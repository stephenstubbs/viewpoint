## Context

Viewpoint needs two key capabilities to function as an MCP server like Playwright:

1. **Connect to remote browser endpoints**: Given an HTTP URL like `http://localhost:9222`, discover the WebSocket endpoint and connect. This is how Playwright's `connectOverCDP` works.

2. **Access existing browser tabs**: When connecting to an already-running browser, enumerate and work with tabs that were already open before connection.

Chrome DevTools Protocol (CDP) exposes an HTTP endpoint at the debugging port that returns browser metadata including the WebSocket URL. The endpoint `/json/version` returns the WebSocket URL for the browser-level connection.

## Goals / Non-Goals

**Goals:**
- Connect to browsers via HTTP endpoint URL (auto-discover WebSocket)
- Enumerate existing browser contexts via `browser.contexts()`
- Access existing pages/tabs in those contexts
- Support connection options (timeout, headers)

**Non-Goals:**
- Playwright protocol connection (only CDP is supported for Chromium)
- Network exposure/forwarding features
- Multi-browser support (Firefox, WebKit) - Chromium only for now

## Decisions

### Decision 1: Use HTTP endpoint discovery for `connect_over_cdp`

When given an HTTP URL like `http://localhost:9222`, make a request to `/json/version` to get the `webSocketDebuggerUrl`. This is the standard CDP discovery mechanism.

**Alternatives considered:**
- Require users to always provide the full WebSocket URL: Less ergonomic, requires users to manually discover the URL
- Parse multiple endpoints: Complexity not needed; `/json/version` is sufficient

### Decision 2: Return existing contexts from `Browser::contexts()`

Use `Target.getBrowserContexts` CDP command to enumerate existing contexts. The default context (empty string ID) represents the browser's main profile.

**Behavior:**
- For launched browsers: Returns contexts we created
- For connected browsers: Returns all contexts including the default one

### Decision 3: Wrap existing contexts as `BrowserContext` instances

When connecting to a browser with existing contexts/tabs, we need to wrap them in our `BrowserContext` type. The key difference is:
- Contexts we create: `owned = true`, dispose on close
- Existing contexts: `owned = false`, don't dispose on close (browser keeps running)

### Decision 4: Builder pattern for connection options

```rust
Browser::connect_over_cdp("http://localhost:9222")
    .timeout(Duration::from_secs(30))
    .header("Authorization", "Bearer token")
    .connect()
    .await?
```

This matches the existing `Browser::launch()` builder pattern.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| HTTP client dependency | Use `reqwest` with minimal features (already common in Rust ecosystem) |
| Existing tabs may have state we can't track | Document that connecting to existing tabs has limitations |
| Default context ID varies | Use empty string check, documented in CDP |

## Migration Plan

No breaking changes. New methods are additive:
- `Browser::connect_over_cdp()` - new method
- `Browser::contexts()` - new method
- `Browser::connect()` keeps working as-is

## Open Questions

None - the design follows established Playwright patterns for CDP connection.
