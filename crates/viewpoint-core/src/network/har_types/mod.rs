//! HAR (HTTP Archive) format data types.
//!
//! This module provides type definitions for the HAR file format.
//! Implementation methods are in the `har` module.

use serde::{Deserialize, Serialize};

/// HAR file structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Har {
    /// HAR log.
    pub log: HarLog,
}

/// HAR log structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarLog {
    /// HAR version (always "1.2").
    pub version: String,
    /// Creator tool information.
    pub creator: HarCreator,
    /// Browser information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<HarCreator>,
    /// Pages in the HAR.
    #[serde(default)]
    pub pages: Vec<HarPage>,
    /// Network entries.
    pub entries: Vec<HarEntry>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Creator/browser information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarCreator {
    /// Name of the creator/browser.
    pub name: String,
    /// Version of the creator/browser.
    pub version: String,
}

/// HAR page entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarPage {
    /// Start timestamp (ISO 8601).
    pub started_date_time: String,
    /// Page ID (unique identifier).
    pub id: String,
    /// Page title.
    pub title: String,
    /// Page timing information.
    pub page_timings: HarPageTimings,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR page timing information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarPageTimings {
    /// Time until content is loaded (ms, -1 if unknown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_content_load: Option<f64>,
    /// Time until page is loaded (ms, -1 if unknown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_load: Option<f64>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR entry (request/response pair).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarEntry {
    /// Reference to the parent page (pageref).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageref: Option<String>,
    /// Start timestamp (ISO 8601).
    pub started_date_time: String,
    /// Total time in milliseconds.
    pub time: f64,
    /// Request details.
    pub request: HarRequest,
    /// Response details.
    pub response: HarResponse,
    /// Cache details.
    pub cache: HarCache,
    /// Timing details.
    pub timings: HarTimings,
    /// Server IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ip_address: Option<String>,
    /// Connection ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR request details.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarRequest {
    /// HTTP method.
    pub method: String,
    /// Request URL.
    pub url: String,
    /// HTTP version.
    pub http_version: String,
    /// Request cookies.
    pub cookies: Vec<HarCookie>,
    /// Request headers.
    pub headers: Vec<HarHeader>,
    /// Query string parameters.
    pub query_string: Vec<HarQueryParam>,
    /// POST data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_data: Option<HarPostData>,
    /// Header size in bytes (-1 if unknown).
    pub headers_size: i64,
    /// Body size in bytes (-1 if unknown).
    pub body_size: i64,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR response details.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarResponse {
    /// HTTP status code.
    pub status: i32,
    /// HTTP status text.
    pub status_text: String,
    /// HTTP version.
    pub http_version: String,
    /// Response cookies.
    pub cookies: Vec<HarCookie>,
    /// Response headers.
    pub headers: Vec<HarHeader>,
    /// Response content.
    pub content: HarContent,
    /// Redirect URL.
    pub redirect_url: String,
    /// Header size in bytes (-1 if unknown).
    pub headers_size: i64,
    /// Body size in bytes (-1 if unknown).
    pub body_size: i64,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR header.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarHeader {
    /// Header name.
    pub name: String,
    /// Header value.
    pub value: String,
}

/// HAR cookie.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarCookie {
    /// Cookie name.
    pub name: String,
    /// Cookie value.
    pub value: String,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Cookie domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Cookie expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// HTTP only flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    /// Secure flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR query parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarQueryParam {
    /// Parameter name.
    pub name: String,
    /// Parameter value.
    pub value: String,
}

/// HAR POST data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarPostData {
    /// MIME type.
    pub mime_type: String,
    /// Text content.
    pub text: String,
    /// Form parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<HarParam>>,
}

/// HAR form parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarParam {
    /// Parameter name.
    pub name: String,
    /// Parameter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// File name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// HAR content.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarContent {
    /// Content size in bytes.
    pub size: i64,
    /// Compression ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<i64>,
    /// MIME type.
    pub mime_type: String,
    /// Text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Encoding (e.g., "base64").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR cache information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarCache {
    /// Cache state before request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_request: Option<HarCacheEntry>,
    /// Cache state after response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_request: Option<HarCacheEntry>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// HAR cache entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarCacheEntry {
    /// Expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Last access timestamp.
    pub last_access: String,
    /// ETag.
    pub e_tag: String,
    /// Hit count.
    pub hit_count: i32,
}

/// HAR timing information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HarTimings {
    /// Time waiting in queue (ms, -1 if unknown).
    #[serde(default = "default_timing_f64")]
    pub blocked: f64,
    /// DNS resolution time (ms, -1 if unknown).
    #[serde(default = "default_timing_f64")]
    pub dns: f64,
    /// TCP connection time (ms, -1 if unknown).
    #[serde(default = "default_timing_f64")]
    pub connect: f64,
    /// Time to send request (ms, -1 if unknown).
    #[serde(default = "default_timing_f64")]
    pub send: f64,
    /// Time waiting for response (ms, -1 if unknown).
    #[serde(default = "default_timing_f64")]
    pub wait: f64,
    /// Time to receive response (ms, -1 if unknown).
    #[serde(default = "default_timing_f64")]
    pub receive: f64,
    /// SSL/TLS negotiation time (ms, -1 if unknown).
    #[serde(default = "default_timing_f64")]
    pub ssl: f64,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

fn default_timing_f64() -> f64 {
    -1.0
}
