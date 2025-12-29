# Change: Add Remote Browser Connection Capabilities

## Why

To build an MCP (Model Context Protocol) server similar to Playwright's, Viewpoint needs to connect to browsers that are already running - both locally-running browsers with existing tabs and remote browser endpoints accessed over the network. The current implementation only supports launching new browsers or connecting via a known WebSocket URL; it cannot discover and attach to existing browser tabs or resolve HTTP endpoints to WebSocket URLs automatically.

## What Changes

- **ADDED**: `Browser::connect_over_cdp(endpoint_url)` - Connect using HTTP endpoint URL (e.g., `http://localhost:9222`) which auto-discovers the WebSocket URL, similar to Playwright's `connectOverCDP`
- **ADDED**: `Browser::contexts()` - List all existing browser contexts when connecting to a running browser
- **ADDED**: `BrowserContext` wrapping for default/existing contexts discovered via CDP
- **ADDED**: Connection options (timeout, custom headers for WebSocket connection)
- **MODIFIED**: `Browser::connect(ws_url)` to support connection builder pattern with options

## Impact

- **Affected specs**: `browser-connection`
- **Affected code**: 
  - `crates/viewpoint-core/src/browser/mod.rs` - Add `connect_over_cdp`, `contexts` methods
  - `crates/viewpoint-core/src/browser/launcher/mod.rs` - Add connection builder
  - `crates/viewpoint-cdp/src/lib.rs` - May need HTTP client for endpoint discovery
- **Dependencies**: May need `reqwest` or similar HTTP client for fetching `/json/version` endpoint
