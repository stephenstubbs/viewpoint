# viewpoint-cdp

Low-level Chrome DevTools Protocol (CDP) implementation over WebSocket for Rust.

This crate provides the foundational CDP communication layer for the [Viewpoint](https://github.com/stephenstubbs/viewpoint) browser automation framework.

## Features

- WebSocket-based CDP connection
- Async message sending and receiving
- Event subscription support
- Protocol domain implementations (Page, Runtime, Network, Input, Target)

## Usage

This crate is primarily used internally by `viewpoint-core`. For browser automation, use `viewpoint-test` instead.

```rust
use viewpoint_cdp::{Connection, CdpError};

// Connect to a running Chrome instance
let connection = Connection::new("ws://localhost:9222/devtools/page/...").await?;

// Send CDP commands
let result = connection.send("Page.navigate", json!({"url": "https://example.com"})).await?;
```

## License

MIT
