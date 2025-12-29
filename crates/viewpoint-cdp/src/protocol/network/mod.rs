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

/// Monotonically increasing time in seconds since an arbitrary point in the past.
pub type MonotonicTime = f64;

/// UTC time in seconds, counted from January 1, 1970.
pub type TimeSinceEpoch = f64;

/// Resource type as it was perceived by the rendering engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ResourceType {
    /// Document resource.
    Document,
    /// Stylesheet resource.
    Stylesheet,
    /// Image resource.
    Image,
    /// Media resource.
    Media,
    /// Font resource.
    Font,
    /// Script resource.
    Script,
    /// Text track resource.
    TextTrack,
    /// `XMLHttpRequest` resource.
    XHR,
    /// Fetch API resource.
    Fetch,
    /// Prefetch resource.
    Prefetch,
    /// `EventSource` resource.
    EventSource,
    /// WebSocket resource.
    WebSocket,
    /// Manifest resource.
    Manifest,
    /// Signed exchange resource.
    SignedExchange,
    /// Ping resource.
    Ping,
    /// CSP violation report.
    CSPViolationReport,
    /// Preflight request.
    Preflight,
    /// Other resource type.
    #[default]
    Other,
}

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
    /// Priority of the resource request.
    pub initial_priority: Option<String>,
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
    /// Security details for HTTPS responses.
    pub security_details: Option<SecurityDetails>,
    /// Remote IP address.
    #[serde(rename = "remoteIPAddress")]
    pub remote_ip_address: Option<String>,
    /// Remote port.
    pub remote_port: Option<i32>,
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
    #[serde(default)]
    pub document_url: Option<String>,
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
    /// Redirect response data. Present only if this request was triggered by a redirect.
    pub redirect_response: Option<Response>,
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

/// Timing information for the request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTiming {
    /// Timing's requestTime is a baseline in seconds.
    pub request_time: f64,
    /// Started resolving proxy.
    pub proxy_start: f64,
    /// Finished resolving proxy.
    pub proxy_end: f64,
    /// Started DNS address resolve.
    pub dns_start: f64,
    /// Finished DNS address resolve.
    pub dns_end: f64,
    /// Started connecting to the remote host.
    pub connect_start: f64,
    /// Connected to the remote host.
    pub connect_end: f64,
    /// Started SSL handshake.
    pub ssl_start: f64,
    /// Finished SSL handshake.
    pub ssl_end: f64,
    /// Started sending request.
    pub send_start: f64,
    /// Finished sending request.
    pub send_end: f64,
    /// Started receiving response headers.
    pub receive_headers_start: Option<f64>,
    /// Finished receiving response headers.
    pub receive_headers_end: Option<f64>,
}

/// Security details about a request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDetails {
    /// Protocol name (e.g. "TLS 1.2" or "QUIC").
    pub protocol: String,
    /// Key Exchange used by the connection.
    pub key_exchange: String,
    /// (EC)DH group used by the connection, if applicable.
    pub key_exchange_group: Option<String>,
    /// Cipher name.
    pub cipher: String,
    /// TLS MAC.
    pub mac: Option<String>,
    /// Certificate subject name.
    pub subject_name: String,
    /// Subject Alternative Name (SAN) DNS names and IP addresses.
    pub san_list: Vec<String>,
    /// Name of the issuing CA.
    pub issuer: String,
    /// Certificate valid from date.
    pub valid_from: TimeSinceEpoch,
    /// Certificate valid to (expiration) date.
    pub valid_to: TimeSinceEpoch,
}

/// Parameters for Network.getResponseBody.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyParams {
    /// Identifier of the network request to get content for.
    pub request_id: RequestId,
}

/// Result for Network.getResponseBody.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyResult {
    /// Response body.
    pub body: String,
    /// True, if content was sent as base64.
    pub base64_encoded: bool,
}

/// Parameters for Network.setExtraHTTPHeaders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExtraHTTPHeadersParams {
    /// Map with extra HTTP headers.
    pub headers: HashMap<String, String>,
}

/// Parameters for Network.setCacheDisabled.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCacheDisabledParams {
    /// Cache disabled state.
    pub cache_disabled: bool,
}

/// Parameters for Network.setBypassServiceWorker.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassServiceWorkerParams {
    /// Bypass service worker and load from network.
    pub bypass: bool,
}

// =============================================================================
// Network Conditions
// =============================================================================

/// Parameters for Network.emulateNetworkConditions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsParams {
    /// True to emulate internet disconnection.
    pub offline: bool,
    /// Minimum latency from request sent to response headers received (ms).
    pub latency: f64,
    /// Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub download_throughput: f64,
    /// Maximal aggregated upload throughput (bytes/sec). -1 disables upload throttling.
    pub upload_throughput: f64,
    /// Connection type if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<ConnectionType>,
}

impl EmulateNetworkConditionsParams {
    /// Create params for offline mode.
    pub fn offline() -> Self {
        Self {
            offline: true,
            latency: 0.0,
            download_throughput: -1.0,
            upload_throughput: -1.0,
            connection_type: None,
        }
    }

    /// Create params for online mode (no throttling).
    pub fn online() -> Self {
        Self {
            offline: false,
            latency: 0.0,
            download_throughput: -1.0,
            upload_throughput: -1.0,
            connection_type: None,
        }
    }
}

/// Connection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionType {
    /// No connection.
    None,
    /// Cellular 2G.
    Cellular2g,
    /// Cellular 3G.
    Cellular3g,
    /// Cellular 4G.
    Cellular4g,
    /// Bluetooth.
    Bluetooth,
    /// Ethernet.
    Ethernet,
    /// `WiFi`.
    Wifi,
    /// `WiMAX`.
    Wimax,
    /// Other.
    Other,
}
