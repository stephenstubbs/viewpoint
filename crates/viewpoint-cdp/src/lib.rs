//! # Viewpoint CDP - Chrome DevTools Protocol Client
//!
//! Low-level Chrome DevTools Protocol (CDP) implementation over WebSocket,
//! providing the foundational transport layer for Viewpoint browser automation.
//!
//! This crate handles:
//! - WebSocket connection management to Chrome/Chromium browsers
//! - CDP message serialization and deserialization
//! - Command/response handling with async/await
//! - Event subscription and streaming
//! - Session management for multiple targets (pages, workers)
//!
//! ## Features
//!
//! - **Async WebSocket**: Non-blocking WebSocket communication with Chromium
//! - **Type-safe Protocol**: Strongly-typed CDP domains (Page, Runtime, Network, etc.)
//! - **Event Streaming**: Subscribe to CDP events with async streams
//! - **Session Multiplexing**: Handle multiple page sessions over a single connection
//! - **Error Handling**: Comprehensive error types for CDP and transport errors
//!
//! ## Quick Start
//!
//! ```no_run
//! use viewpoint_cdp::{CdpConnection, protocol::target_domain::GetTargetsParams};
//!
//! # async fn example() -> Result<(), viewpoint_cdp::CdpError> {
//! // Connect to a running Chrome instance
//! let conn = CdpConnection::connect("ws://localhost:9222/devtools/browser/...").await?;
//!
//! // Send a CDP command
//! let result: viewpoint_cdp::protocol::target_domain::GetTargetsResult =
//!     conn.send_command("Target.getTargets", Some(GetTargetsParams::default()), None).await?;
//!
//! for target in result.target_infos {
//!     println!("Target: {} - {}", target.target_type, target.url);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Discovering Chrome WebSocket URL
//!
//! Chrome exposes a JSON API for discovering the WebSocket URL:
//!
//! ```no_run
//! use viewpoint_cdp::{discover_websocket_url, CdpConnectionOptions};
//!
//! # async fn example() -> Result<(), viewpoint_cdp::CdpError> {
//! // Get WebSocket URL from HTTP endpoint
//! let options = CdpConnectionOptions::default();
//! let ws_url = discover_websocket_url("http://localhost:9222", &options).await?;
//! println!("WebSocket URL: {}", ws_url);
//! # Ok(())
//! # }
//! ```
//!
//! ## Sending Commands
//!
//! Commands are sent with optional session IDs for page-specific operations:
//!
//! ```no_run
//! use viewpoint_cdp::CdpConnection;
//! use viewpoint_cdp::protocol::page::NavigateParams;
//!
//! # async fn example(conn: &CdpConnection, session_id: &str) -> Result<(), viewpoint_cdp::CdpError> {
//! // Browser-level command (no session)
//! let version: viewpoint_cdp::BrowserVersion = conn.send_command(
//!     "Browser.getVersion",
//!     None::<()>,
//!     None  // No session ID for browser-level commands
//! ).await?;
//!
//! // Page-level command (with session)
//! let result: viewpoint_cdp::protocol::page::NavigateResult = conn.send_command(
//!     "Page.navigate",
//!     Some(NavigateParams {
//!         url: "https://example.com".to_string(),
//!         referrer: None,
//!         transition_type: None,
//!         frame_id: None,
//!     }),
//!     Some(session_id)  // Target a specific page
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Subscribing to Events
//!
//! Subscribe to CDP events using the event channel:
//!
//! ```no_run
//! use viewpoint_cdp::{CdpConnection, CdpEvent};
//!
//! # async fn example(conn: &CdpConnection) -> Result<(), viewpoint_cdp::CdpError> {
//! let mut events = conn.subscribe_events();
//!
//! // Process events in a loop
//! while let Ok(event) = events.recv().await {
//!     match &event.method[..] {
//!         "Page.loadEventFired" => {
//!             println!("Page loaded!");
//!         }
//!         "Network.requestWillBeSent" => {
//!             println!("Network request: {:?}", event.params);
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Connection Options
//!
//! Configure connection behavior with options:
//!
//! ```no_run
//! use viewpoint_cdp::{CdpConnection, CdpConnectionOptions};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), viewpoint_cdp::CdpError> {
//! let options = CdpConnectionOptions::new()
//!     .timeout(Duration::from_secs(30));
//!
//! let conn = CdpConnection::connect_with_options(
//!     "ws://localhost:9222/devtools/browser/...",
//!     &options
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Protocol Domains
//!
//! The [`protocol`] module contains typed definitions for CDP domains:
//!
//! - `target_domain` - Target management (pages, workers, service workers)
//! - `page` - Page navigation and lifecycle
//! - `runtime` - JavaScript execution
//! - `network` - Network monitoring and interception
//! - `fetch` - Network request interception
//! - `dom` - DOM inspection and manipulation
//! - `input` - Input device simulation
//! - `emulation` - Device and media emulation
//! - And many more...
//!
//! ## Error Handling
//!
//! The [`CdpError`] type covers all possible errors:
//!
//! ```no_run
//! use viewpoint_cdp::{CdpConnection, CdpError};
//!
//! # async fn example() -> Result<(), CdpError> {
//! let result = CdpConnection::connect("ws://invalid:9999/...").await;
//!
//! match result {
//!     Ok(conn) => println!("Connected!"),
//!     Err(CdpError::ConnectionFailed(e)) => println!("Connection error: {}", e),
//!     Err(CdpError::Protocol { code, message }) => {
//!         println!("CDP error {}: {}", code, message);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Module Organization
//!
//! - [`connection`] - WebSocket connection management
//! - [`transport`] - Message types and serialization
//! - [`protocol`] - CDP domain type definitions
//! - [`error`] - Error types

pub mod connection;
pub mod error;
pub mod protocol;
pub mod transport;

pub use connection::{BrowserVersion, CdpConnection, CdpConnectionOptions, discover_websocket_url};
pub use error::CdpError;
pub use transport::{CdpEvent, CdpMessage, CdpRequest, CdpResponse};
