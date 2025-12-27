//! Network request types.

// Allow dead code for request builder setters (spec: network-events)
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use viewpoint_cdp::CdpConnection;

use super::types::ResourceType;

/// A network request.
///
/// This type represents an HTTP request and provides access to request details
/// such as URL, method, headers, and body.
#[derive(Debug, Clone)]
pub struct Request {
    /// Request URL.
    pub(crate) url: String,
    /// HTTP method.
    pub(crate) method: String,
    /// Request headers.
    pub(crate) headers: HashMap<String, String>,
    /// POST data (if any).
    pub(crate) post_data: Option<String>,
    /// Resource type.
    pub(crate) resource_type: ResourceType,
    /// Frame ID.
    pub(crate) frame_id: String,
    /// Whether this is a navigation request.
    pub(crate) is_navigation: bool,
    /// CDP connection for fetching additional data.
    pub(crate) connection: Option<Arc<CdpConnection>>,
    /// Session ID for CDP commands.
    pub(crate) session_id: Option<String>,
    /// Network request ID.
    pub(crate) request_id: Option<String>,
    /// The request that this request was redirected from (previous in chain).
    pub(crate) redirected_from: Option<Box<Request>>,
    /// The request that this request redirected to (next in chain).
    /// Note: This is wrapped in Arc to allow updating after creation.
    pub(crate) redirected_to: Option<Box<Request>>,
    /// Request timing information.
    pub(crate) timing: Option<RequestTiming>,
    /// Failure text if the request failed.
    pub(crate) failure_text: Option<String>,
}

impl Request {
    /// Create a new request from CDP request data.
    pub(crate) fn from_cdp(
        cdp_request: viewpoint_cdp::protocol::network::Request,
        resource_type: viewpoint_cdp::protocol::network::ResourceType,
        frame_id: String,
        connection: Option<Arc<CdpConnection>>,
        session_id: Option<String>,
        request_id: Option<String>,
    ) -> Self {
        Self {
            url: cdp_request.url,
            method: cdp_request.method,
            headers: cdp_request.headers,
            post_data: cdp_request.post_data,
            resource_type: resource_type.into(),
            frame_id,
            is_navigation: false, // Will be set separately
            connection,
            session_id,
            request_id,
            redirected_from: None,
            redirected_to: None,
            timing: None,
            failure_text: None,
        }
    }

    /// Get the request URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the HTTP method.
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Get the request headers.
    ///
    /// Note: Headers are case-insensitive but the map preserves the original case.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get a header value by name (case-insensitive).
    pub fn header_value(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// Get all headers asynchronously.
    ///
    /// This may fetch additional headers that weren't available synchronously.
    pub async fn all_headers(&self) -> HashMap<String, String> {
        // For now, just return the cached headers
        // In the future, we could fetch security headers via CDP
        self.headers.clone()
    }

    /// Get the POST data.
    pub fn post_data(&self) -> Option<&str> {
        self.post_data.as_deref()
    }

    /// Get the POST data as bytes.
    pub fn post_data_buffer(&self) -> Option<Vec<u8>> {
        self.post_data.as_ref().map(|s| s.as_bytes().to_vec())
    }

    /// Parse the POST data as JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if the data is not valid JSON or doesn't match type T.
    pub fn post_data_json<T: serde::de::DeserializeOwned>(&self) -> Result<Option<T>, serde_json::Error> {
        match &self.post_data {
            Some(data) => serde_json::from_str(data).map(Some),
            None => Ok(None),
        }
    }

    /// Get the resource type.
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Get the frame ID.
    pub fn frame_id(&self) -> &str {
        &self.frame_id
    }

    /// Check if this is a navigation request.
    pub fn is_navigation_request(&self) -> bool {
        self.is_navigation
    }

    /// Get the request that caused this redirect, if any.
    pub fn redirected_from(&self) -> Option<&Request> {
        self.redirected_from.as_deref()
    }

    /// Get the request that this request redirected to, if any.
    pub fn redirected_to(&self) -> Option<&Request> {
        self.redirected_to.as_deref()
    }

    /// Get request timing information.
    pub fn timing(&self) -> Option<&RequestTiming> {
        self.timing.as_ref()
    }

    /// Get request size information.
    ///
    /// Returns request body size and headers size.
    pub async fn sizes(&self) -> RequestSizes {
        let body_size = self.post_data.as_ref().map_or(0, std::string::String::len);
        let headers_size = self
            .headers
            .iter()
            .map(|(k, v)| k.len() + v.len() + 4) // ": " and "\r\n"
            .sum();

        RequestSizes {
            request_body_size: body_size,
            request_headers_size: headers_size,
        }
    }

    /// Get the failure reason if the request failed.
    pub fn failure(&self) -> Option<&str> {
        self.failure_text.as_deref()
    }

    /// Set the navigation flag.
    pub(crate) fn set_is_navigation(&mut self, is_navigation: bool) {
        self.is_navigation = is_navigation;
    }

    /// Set the redirect chain (previous request that redirected to this one).
    pub(crate) fn set_redirected_from(&mut self, from: Request) {
        self.redirected_from = Some(Box::new(from));
    }

    /// Set the redirect target (next request in the redirect chain).
    pub(crate) fn set_redirected_to(&mut self, to: Request) {
        self.redirected_to = Some(Box::new(to));
    }

    /// Set timing information.
    pub(crate) fn set_timing(&mut self, timing: RequestTiming) {
        self.timing = Some(timing);
    }

    /// Set the failure text.
    pub(crate) fn set_failure_text(&mut self, text: String) {
        self.failure_text = Some(text);
    }
}

/// Request timing information.
#[derive(Debug, Clone)]
pub struct RequestTiming {
    /// Request start time in milliseconds.
    pub start_time: f64,
    /// Time spent resolving proxy.
    pub proxy_start: f64,
    pub proxy_end: f64,
    /// Time spent resolving DNS.
    pub dns_start: f64,
    pub dns_end: f64,
    /// Time spent connecting.
    pub connect_start: f64,
    pub connect_end: f64,
    /// Time spent in SSL handshake.
    pub ssl_start: f64,
    pub ssl_end: f64,
    /// Time sending the request.
    pub send_start: f64,
    pub send_end: f64,
    /// Time receiving response headers.
    pub receive_headers_start: f64,
    pub receive_headers_end: f64,
}

impl From<viewpoint_cdp::protocol::network::ResourceTiming> for RequestTiming {
    fn from(timing: viewpoint_cdp::protocol::network::ResourceTiming) -> Self {
        Self {
            start_time: timing.request_time * 1000.0,
            proxy_start: timing.proxy_start,
            proxy_end: timing.proxy_end,
            dns_start: timing.dns_start,
            dns_end: timing.dns_end,
            connect_start: timing.connect_start,
            connect_end: timing.connect_end,
            ssl_start: timing.ssl_start,
            ssl_end: timing.ssl_end,
            send_start: timing.send_start,
            send_end: timing.send_end,
            receive_headers_start: timing.receive_headers_start.unwrap_or(0.0),
            receive_headers_end: timing.receive_headers_end.unwrap_or(0.0),
        }
    }
}

/// Request size information.
#[derive(Debug, Clone, Copy)]
pub struct RequestSizes {
    /// Size of the request body in bytes.
    pub request_body_size: usize,
    /// Size of the request headers in bytes.
    pub request_headers_size: usize,
}

#[cfg(test)]
mod tests;
