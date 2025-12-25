# Change: Add Chromium Connection with Wait System and Navigation

## Why

RustRight needs foundational browser automation capabilities to function as a Playwright clone. The initial connection to Chromium via CDP, navigation with configurable wait states, and Playwright-compatible auto-waiting logic are the critical building blocks that all other features depend on. Optimizing for test completion speed requires efficient async operations and minimal overhead in the CDP communication layer.

## What Changes

- **NEW**: `rustright-cdp` crate - Low-level Chrome DevTools Protocol implementation over WebSocket
- **NEW**: `rustright-core` crate - Core domain types (Browser, Context, Page) with hexagonal architecture
- **NEW**: Browser launching - Spawn Chromium process with `--remote-debugging-port` and connect via CDP
- **NEW**: CDP connection - Connect to existing Chromium instance via WebSocket
- **NEW**: Builder pattern API - `Browser::launch().headless(true).launch().await?`
- **NEW**: Navigation - `page.goto()` with `wait_until` options (load, domcontentloaded, networkidle, commit)
- **NEW**: Wait system - Playwright-compatible wait states and auto-waiting infrastructure
- **NEW**: Page/Context lifecycle - Create contexts, pages, and manage their lifecycle

## Impact

- Affected specs: None (greenfield - creates new capabilities)
- Affected code: Creates new workspace crates `rustright-cdp` and `rustright-core`
- New dependencies: `tokio`, `tokio-tungstenite`, `serde`, `serde_json`, `thiserror`

## Performance Considerations

- Use `tokio-tungstenite` for zero-copy WebSocket frames where possible
- Pre-allocate CDP message buffers to reduce allocations
- Use `serde_json::RawValue` for CDP responses to defer parsing until needed
- Maintain persistent WebSocket connection (no reconnection overhead per command)
- Parallel CDP session support for concurrent page operations
