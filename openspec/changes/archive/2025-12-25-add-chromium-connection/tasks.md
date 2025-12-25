# Tasks: Add Chromium Connection with Wait System and Navigation

## 1. Project Setup

- [x] 1.1 Create `crates/` directory structure
- [x] 1.2 Create `rustright-cdp` crate with Cargo.toml (tokio, tokio-tungstenite, serde, serde_json, thiserror)
- [x] 1.3 Create `rustright-core` crate with Cargo.toml (tokio, thiserror, depends on rustright-cdp)
- [x] 1.4 Update workspace Cargo.toml to include both crates
- [x] 1.5 Add workspace-level clippy pedantic lints configuration

## 2. CDP Transport Layer (rustright-cdp)

- [x] 2.1 Create `error.rs` with `CdpError` enum (thiserror)
- [x] 2.2 Create `transport.rs` with `CdpMessage` request/response types
- [x] 2.3 Create `connection.rs` with `CdpConnection` (WebSocket wrapper)
- [x] 2.4 Implement message ID generation (atomic counter per connection)
- [x] 2.5 Implement `send_command<T>()` with typed response deserialization
- [x] 2.6 Implement event subscription system (broadcast channel for CDP events)
- [x] 2.7 Write unit tests for message serialization/deserialization

## 3. CDP Protocol Types (rustright-cdp)

- [x] 3.1 Create `protocol/target.rs` - Target domain types (createBrowserContext, createTarget, attachToTarget, etc.)
- [x] 3.2 Create `protocol/page.rs` - Page domain types (navigate, loadEventFired, domContentEventFired, frameNavigated)
- [x] 3.3 Create `protocol/network.rs` - Network domain types (enable, requestWillBeSent, loadingFinished, loadingFailed)
- [x] 3.4 Create `protocol/runtime.rs` - Runtime domain types (evaluate, for future JS execution)
- [x] 3.5 Write integration test connecting to real Chromium and listing targets

## 4. Browser Launching (rustright-core)

- [x] 4.1 Create `browser/error.rs` with `BrowserError` enum
- [x] 4.2 Define `BrowserLauncher` port trait
- [x] 4.3 Create `ChromiumLauncher` adapter in rustright-cdp
- [x] 4.4 Implement Chromium path detection (common paths + env var)
- [x] 4.5 Implement process spawning with `--remote-debugging-port=0`
- [x] 4.6 Parse WebSocket URL from Chromium stderr output
- [x] 4.7 Implement `Browser::launch()` builder pattern
- [x] 4.8 Implement `Browser::connect(ws_url)` for existing instances
- [x] 4.9 Implement `Browser::close()` (close WebSocket, terminate process if launched)
- [x] 4.10 Write integration test: launch browser, verify connection, close

## 5. Context and Page Management (rustright-core)

- [x] 5.1 Create `context/error.rs` with `ContextError` enum
- [x] 5.2 Implement `BrowserContext` type with CDP session
- [x] 5.3 Implement `Browser::new_context()` using Target.createBrowserContext
- [x] 5.4 Create `page/error.rs` with `PageError` enum
- [x] 5.5 Implement `Page` type with dedicated CDP session
- [x] 5.6 Implement `BrowserContext::new_page()` using Target.createTarget + attachToTarget
- [x] 5.7 Implement `Page::close()` using Target.closeTarget
- [x] 5.8 Implement `BrowserContext::close()` (close all pages, dispose context)
- [x] 5.9 Write integration test: create context, create page, close all

## 6. Wait System (rustright-core)

- [x] 6.1 Create `wait/error.rs` with `WaitError` enum (timeout, cancelled)
- [x] 6.2 Create `wait/load_state.rs` with `DocumentLoadState` enum (Commit, DomContentLoaded, Load, NetworkIdle)
- [x] 6.3 Implement `LoadStateWaiter` that listens for CDP page/network events
- [x] 6.4 Implement network activity tracking for NetworkIdle detection
- [x] 6.5 Implement configurable timeout (default 30 seconds like Playwright)
- [x] 6.6 Write unit tests for load state transitions

## 7. Navigation (rustright-core)

- [x] 7.1 Create `page/navigation.rs` with navigation types
- [x] 7.2 Implement `GotoBuilder` with `wait_until()`, `timeout()`, `referer()` options
- [x] 7.3 Implement `Page::goto()` that returns `GotoBuilder`
- [x] 7.4 Implement navigation execution: Page.navigate + wait for load state
- [x] 7.5 Implement navigation response capture (status code, headers)
- [x] 7.6 Handle navigation errors (net::ERR_*, timeout, etc.)
- [x] 7.7 Write integration test: navigate to example.com, verify load states work

## 8. Integration & Polish

- [x] 8.1 Create `examples/basic_navigation.rs` demonstrating full flow
- [x] 8.2 Add documentation comments to all public APIs
- [x] 8.3 Run clippy with pedantic lints and fix all warnings
- [x] 8.4 Verify all tests pass with `cargo test --workspace`
- [x] 8.5 Test with Chromium from Nix flake
