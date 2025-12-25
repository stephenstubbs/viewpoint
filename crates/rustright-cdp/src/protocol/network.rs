//! Network domain types.
//!
//! The Network domain allows tracking network activities of the page.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique request identifier.
pub type RequestId = String;

/// Unique loader identifier.
pub type LoaderId = String;

/// Unique frame identifier.
pub type FrameId = String;

/// HTTP request data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Request URL.
    pub url: String,
    /// HTTP request method.
    pub method: String,
    /// HTTP request headers.
    pub headers: HashMap<String, String>,
    /// HTTP POST request data.
    pub post_data: Option<String>,
    /// Whether the request has POST data.
    pub has_post_data: Option<bool>,
    /// Request body mixed content type.
    pub mixed_content_type: Option<String>,
    /// The referrer policy of the request.
    pub referrer_policy: Option<String>,
    /// Whether is loaded via link preload.
    pub is_link_preload: Option<bool>,
}

/// HTTP response data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Response URL.
    pub url: String,
    /// HTTP response status code.
    pub status: u32,
    /// HTTP response status text.
    pub status_text: String,
    /// HTTP response headers.
    pub headers: HashMap<String, String>,
    /// HTTP response headers text.
    pub headers_text: Option<String>,
    /// Resource mimeType.
    pub mime_type: String,
    /// Refined HTTP request headers that were actually transmitted over the network.
    pub request_headers: Option<HashMap<String, String>>,
    /// HTTP request headers text.
    pub request_headers_text: Option<String>,
    /// Whether the response was served from disk cache.
    pub from_disk_cache: Option<bool>,
    /// Whether the response was served from the prefetch cache.
    pub from_prefetch_cache: Option<bool>,
    /// Whether the response was served from `ServiceWorker`.
    pub from_service_worker: Option<bool>,
    /// Total number of bytes received.
    pub encoded_data_length: Option<f64>,
    /// Protocol for the request.
    pub protocol: Option<String>,
    /// Security state.
    pub security_state: Option<String>,
}

/// Parameters for Network.enable.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// Buffer size in bytes to use for storing network data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_total_buffer_size: Option<i64>,
    /// Per-resource buffer size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resource_buffer_size: Option<i64>,
    /// Max post data size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_post_data_size: Option<i64>,
}

/// Event: Network.requestWillBeSent
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestWillBeSentEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Loader identifier.
    pub loader_id: LoaderId,
    /// URL of the document this request is loaded for.
    pub document_url: String,
    /// Request data.
    pub request: Request,
    /// Timestamp.
    pub timestamp: f64,
    /// Timestamp.
    pub wall_time: f64,
    /// Request initiator.
    pub initiator: RequestInitiator,
    /// Frame identifier.
    pub frame_id: Option<FrameId>,
    /// Whether this request is a navigation request.
    pub has_user_gesture: Option<bool>,
    /// Type of the request.
    #[serde(rename = "type")]
    pub resource_type: Option<String>,
}

/// Request initiator information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestInitiator {
    /// Type of initiator.
    #[serde(rename = "type")]
    pub initiator_type: String,
    /// Initiator URL.
    pub url: Option<String>,
    /// Initiator line number.
    pub line_number: Option<f64>,
    /// Initiator column number.
    pub column_number: Option<f64>,
}

/// Event: Network.responseReceived
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseReceivedEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Loader identifier.
    pub loader_id: LoaderId,
    /// Timestamp.
    pub timestamp: f64,
    /// Resource type.
    #[serde(rename = "type")]
    pub resource_type: String,
    /// Response data.
    pub response: Response,
    /// Frame identifier.
    pub frame_id: Option<FrameId>,
}

/// Event: Network.loadingFinished
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadingFinishedEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Timestamp.
    pub timestamp: f64,
    /// Total number of bytes received.
    pub encoded_data_length: f64,
}

/// Event: Network.loadingFailed
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadingFailedEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Timestamp.
    pub timestamp: f64,
    /// Resource type.
    #[serde(rename = "type")]
    pub resource_type: String,
    /// User friendly error message.
    pub error_text: String,
    /// True if loading was canceled.
    pub canceled: Option<bool>,
    /// The reason why loading was blocked.
    pub blocked_reason: Option<String>,
}

/// Event: Network.requestServedFromCache
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestServedFromCacheEvent {
    /// Request identifier.
    pub request_id: RequestId,
}
