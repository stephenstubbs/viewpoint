# viewpoint-cdp

[![Crates.io](https://img.shields.io/crates/v/viewpoint-cdp.svg)](https://crates.io/crates/viewpoint-cdp)
[![Documentation](https://docs.rs/viewpoint-cdp/badge.svg)](https://docs.rs/viewpoint-cdp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Low-level Chrome DevTools Protocol (CDP) implementation over WebSocket for Rust.

This crate provides the foundational CDP communication layer for the [Viewpoint](https://github.com/user/viewpoint) browser automation framework. It handles the WebSocket connection, message serialization, and protocol domain definitions.

## Features

- **Async WebSocket**: Non-blocking WebSocket communication with Chromium
- **Type-safe Protocol**: Strongly-typed CDP domains (Page, Runtime, Network, etc.)
- **Event Streaming**: Subscribe to CDP events with async channels
- **Session Multiplexing**: Handle multiple page sessions over a single connection
- **Error Handling**: Comprehensive error types for CDP and transport errors

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
viewpoint-cdp = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use viewpoint_cdp::{CdpConnection, protocol::target_domain::GetTargetsParams};

#[tokio::main]
async fn main() -> Result<(), viewpoint_cdp::CdpError> {
    // Connect to a running Chrome instance
    let conn = CdpConnection::connect("ws://localhost:9222/devtools/browser/...").await?;

    // Send a CDP command
    let result: viewpoint_cdp::protocol::target_domain::GetTargetsResult =
        conn.send_command("Target.getTargets", Some(GetTargetsParams::default()), None).await?;

    for target in result.target_infos {
        println!("Target: {} - {}", target.target_type, target.url);
    }
    Ok(())
}
```

## Discovering Chrome WebSocket URL

Chrome exposes a JSON API for discovering the WebSocket URL:

```rust
use viewpoint_cdp::{discover_websocket_url, CdpConnectionOptions};

#[tokio::main]
async fn main() -> Result<(), viewpoint_cdp::CdpError> {
    let options = CdpConnectionOptions::default();
    let ws_url = discover_websocket_url("http://localhost:9222", &options).await?;
    println!("WebSocket URL: {}", ws_url);
    Ok(())
}
```

## Sending Commands

Commands are sent with optional session IDs for page-specific operations:

```rust
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::page::NavigateParams;

async fn example(conn: &CdpConnection, session_id: &str) -> Result<(), viewpoint_cdp::CdpError> {
    // Browser-level command (no session)
    let version: viewpoint_cdp::BrowserVersion = conn.send_command(
        "Browser.getVersion",
        None::<()>,
        None  // No session ID for browser-level commands
    ).await?;

    // Page-level command (with session)
    let result: viewpoint_cdp::protocol::page::NavigateResult = conn.send_command(
        "Page.navigate",
        Some(NavigateParams {
            url: "https://example.com".to_string(),
            referrer: None,
            transition_type: None,
            frame_id: None,
        }),
        Some(session_id)  // Target a specific page
    ).await?;
    Ok(())
}
```

## Subscribing to Events

Subscribe to CDP events using the event channel:

```rust
use viewpoint_cdp::CdpConnection;

async fn example(conn: &CdpConnection) -> Result<(), viewpoint_cdp::CdpError> {
    let mut events = conn.subscribe_events();

    // Process events in a loop
    while let Ok(event) = events.recv().await {
        match &event.method[..] {
            "Page.loadEventFired" => {
                println!("Page loaded!");
            }
            "Network.requestWillBeSent" => {
                println!("Network request: {:?}", event.params);
            }
            _ => {}
        }
    }
    Ok(())
}
```

## Connection Options

Configure connection behavior with options:

```rust
use viewpoint_cdp::{CdpConnection, CdpConnectionOptions};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), viewpoint_cdp::CdpError> {
    let options = CdpConnectionOptions::new()
        .timeout(Duration::from_secs(30));

    let conn = CdpConnection::connect_with_options(
        "ws://localhost:9222/devtools/browser/...",
        &options
    ).await?;
    Ok(())
}
```

## Protocol Domains

The `protocol` module contains typed definitions for CDP domains:

- `target_domain` - Target management (pages, workers, service workers)
- `page` - Page navigation and lifecycle
- `runtime` - JavaScript execution
- `network` - Network monitoring and interception
- `fetch` - Network request interception
- `dom` - DOM inspection and manipulation
- `input` - Input device simulation
- `emulation` - Device and media emulation
- And many more...

## Error Handling

The `CdpError` type covers all possible errors:

```rust
use viewpoint_cdp::{CdpConnection, CdpError};

async fn example() -> Result<(), CdpError> {
    let result = CdpConnection::connect("ws://invalid:9999/...").await;

    match result {
        Ok(_conn) => println!("Connected!"),
        Err(CdpError::ConnectionFailed(e)) => println!("Connection error: {}", e),
        Err(CdpError::Protocol { code, message }) => {
            println!("CDP error {}: {}", code, message);
        }
        Err(e) => println!("Other error: {}", e),
    }
    Ok(())
}
```

## When to Use This Crate

This crate is primarily used internally by `viewpoint-core`. For browser automation, use `viewpoint-test` or `viewpoint-core` instead, which provide a higher-level, more ergonomic API.

Use this crate directly if you need:
- Low-level CDP access
- Custom CDP domain implementations
- Direct WebSocket communication with Chrome

## License

MIT
