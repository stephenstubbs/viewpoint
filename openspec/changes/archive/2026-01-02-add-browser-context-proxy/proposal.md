# Change: Add Proxy Configuration for Browser Contexts

## Why

The viewpoint-mcp MCP server needs to support proxy configuration when creating browser contexts. This is a standard browser automation feature that allows routing browser traffic through proxy servers (e.g., for testing geo-restricted content, corporate proxies, or privacy proxies).

Currently, viewpoint has `ProxyConfig` in the API request context (`api/options/mod.rs`) but not for browser contexts. Playwright supports proxy configuration at the browser context level, and viewpoint should provide equivalent functionality.

This is a **downstream dependency** for viewpoint-mcp's `browser_context_create` tool which accepts proxy parameters but cannot wire them through to viewpoint-core.

## What Changes

- Add `ProxyConfig` type to context options (can reuse existing type from `api/options`)
- Add `proxy()` builder method to `ContextOptionsBuilder`
- Add `proxy()` builder method to `NewContextBuilder`
- Wire proxy configuration through to Chromium via CDP's `Fetch.enable` or browser launch args

## Impact

- Affected specs: `context-lifecycle`
- Affected code:
  - `crates/viewpoint-core/src/context/types/options/mod.rs` - Add proxy field and builder method
  - `crates/viewpoint-core/src/browser/context_builder.rs` - Add proxy builder method
  - `crates/viewpoint-core/src/context/construction/mod.rs` - Wire proxy to CDP
- Downstream: Unblocks viewpoint-mcp `browser_context_create` proxy support
