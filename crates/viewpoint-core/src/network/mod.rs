//! # Network Interception and Monitoring
//!
//! This module provides comprehensive network capabilities including request
//! interception, response mocking, event monitoring, and HAR recording/replay.
//!
//! ## Features
//!
//! - **Request Interception**: Intercept and modify outgoing requests
//! - **Response Mocking**: Return custom responses without hitting the server
//! - **Request Blocking**: Block requests matching specific patterns
//! - **Network Events**: Monitor requests, responses, and failures
//! - **HAR Recording**: Record network traffic for debugging
//! - **HAR Replay**: Replay recorded traffic for testing
//! - **WebSocket Monitoring**: Track WebSocket connections and messages
//!
//! ## Request Interception
//!
//! Use [`Route`] to intercept and handle network requests:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Mock an API response
//! page.route("**/api/users", |route| async move {
//!     route.fulfill()
//!         .status(200)
//!         .content_type("application/json")
//!         .body(r#"[{"id": 1, "name": "Alice"}]"#)
//!         .fulfill()
//!         .await
//! }).await?;
//!
//! // Block images
//! page.route("**/*.{png,jpg,jpeg,gif,webp}", |route| async move {
//!     route.abort().await
//! }).await?;
//!
//! // Modify request headers
//! page.route("**/api/**", |route| async move {
//!     route.continue_route()
//!         .header("Authorization", "Bearer token123")
//!         .continue_route()
//!         .await
//! }).await?;
//!
//! // Modify POST body
//! page.route("**/api/submit", |route| async move {
//!     route.continue_route()
//!         .post_data(r#"{"modified": true}"#)
//!         .continue_route()
//!         .await
//! }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## URL Patterns
//!
//! Routes support glob patterns for matching URLs:
//!
//! - `**` - Match any path segments
//! - `*` - Match any characters except `/`
//! - `?` - Match a single character
//!
//! ```ignore
//! use viewpoint_core::{Browser, Route};
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Match all API endpoints
//! page.route("**/api/**", |route| async move {
//!     route.continue_route().continue_route().await
//! }).await?;
//!
//! // Match specific file types
//! page.route("**/*.{js,css}", |route| async move {
//!     route.continue_route().continue_route().await
//! }).await?;
//!
//! // Match exact URL
//! page.route("https://example.com/login", |route| async move {
//!     route.continue_route().continue_route().await
//! }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Network Events
//!
//! Monitor network activity with event listeners:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Wait for a specific request
//! let request = page.wait_for_request("**/api/data")
//!     .wait()
//!     .await?;
//! println!("Request URL: {}", request.url());
//!
//! // Wait for a specific response
//! let response = page.wait_for_response("**/api/data")
//!     .wait()
//!     .await?;
//! println!("Response status: {}", response.status());
//! # Ok(())
//! # }
//! ```
//!
//! ## HAR Recording
//!
//! Record network traffic for debugging or test fixtures:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! // Start HAR recording
//! context.route_from_har()
//!     .record_path("recording.har")
//!     .build()
//!     .await?;
//!
//! // ... navigate and interact ...
//!
//! // HAR is automatically saved when context closes
//! # Ok(())
//! # }
//! ```
//!
//! ## HAR Replay
//!
//! Replay recorded traffic for deterministic tests:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! // Replay from HAR file
//! context.route_from_har()
//!     .path("recording.har")
//!     .build()
//!     .await?;
//!
//! // Now requests will be served from the HAR file
//! let page = context.new_page().await?;
//! page.goto("https://example.com").goto().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## WebSocket Monitoring
//!
//! Monitor WebSocket connections:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Listen for WebSocket connections
//! page.on_websocket(|ws| async move {
//!     println!("WebSocket connected: {}", ws.url());
//!     
//!     // Listen for messages
//!     ws.on_frame(|frame| async move {
//!         println!("Frame: {:?}", frame.payload());
//!         Ok(())
//!     }).await;
//!     
//!     Ok(())
//! }).await;
//! # Ok(())
//! # }
//! ```

pub mod auth;
pub mod events;
pub(crate) mod handler;
mod handler_fetch;
mod handler_request;
pub mod har;
pub mod har_recorder;
pub mod har_replay;
mod har_types;
mod request;
mod response;
mod route;
mod route_builders;
mod route_fetch;
mod types;
pub mod websocket;

pub use events::{
    NetworkEvent, NetworkEventListener, RequestEvent, RequestFailedEvent, RequestFinishedEvent,
    ResponseEvent, WaitForRequestBuilder, WaitForResponseBuilder,
};
pub use handler::RouteHandlerRegistry;
pub use har::{Har, HarEntry, HarPage, HarRequest, HarResponse, HarTimings};
pub use har_recorder::{HarRecorder, HarRecordingBuilder, HarRecordingOptions};
pub use har_replay::{
    HarReplayHandler, HarReplayOptions, HarResponseData, TimingMode, UpdateContentMode,
};
pub use request::{Request, RequestSizes, RequestTiming};
pub use response::{RemoteAddress, Response, SecurityDetails};
pub use route::{Route, RouteAction, RouteHandler};
pub use route_builders::{ContinueBuilder, FulfillBuilder};
pub use route_fetch::{FetchBuilder, FetchedResponse};
pub use types::{AbortError, ResourceType, UrlMatcher, UrlPattern};
pub use websocket::{WebSocket, WebSocketFrame, WebSocketManager};

// Re-export CDP types that are used directly
pub use viewpoint_cdp::protocol::fetch::HeaderEntry;
