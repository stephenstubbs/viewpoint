//! Network interception and monitoring.
//!
//! This module provides types for intercepting and modifying network requests,
//! monitoring network activity, and replaying network traffic from HAR files.

pub mod auth;
pub mod events;
pub mod har;
pub mod har_recorder;
pub mod har_replay;
mod har_types;
pub(crate) mod handler;
mod handler_fetch;
mod handler_request;
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
