//! HAR (HTTP Archive) recording.
//!
//! This module provides functionality to record network traffic to HAR format
//! during browser automation.

// Allow dead code for HAR recording scaffolding (spec: har-support)

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use tracing::{debug, trace};

use super::har::{Har, HarEntry, HarPage, HarRequest, HarResponse, HarTimings};
use crate::error::NetworkError;

/// Options for HAR recording.
#[derive(Debug, Clone)]
pub struct HarRecordingOptions {
    /// Path where HAR will be saved.
    pub path: PathBuf,
    /// URL pattern to filter (glob pattern). If set, only matching URLs are recorded.
    pub url_filter: Option<String>,
    /// Whether to omit response content.
    pub omit_content: bool,
    /// Content types to include (empty means all).
    pub content_types: Vec<String>,
    /// Maximum body size in bytes (0 means unlimited).
    pub max_body_size: usize,
}

impl Default for HarRecordingOptions {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            url_filter: None,
            omit_content: false,
            content_types: Vec::new(),
            max_body_size: 0,
        }
    }
}

/// Builder for HAR recording options.
#[derive(Debug)]
pub struct HarRecordingBuilder {
    options: HarRecordingOptions,
}

impl HarRecordingBuilder {
    /// Create a new builder with the output path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            options: HarRecordingOptions {
                path: path.into(),
                ..Default::default()
            },
        }
    }

    /// Set URL filter (glob pattern).
    #[must_use]
    pub fn url_filter(mut self, pattern: impl Into<String>) -> Self {
        self.options.url_filter = Some(pattern.into());
        self
    }

    /// Omit response content from HAR.
    #[must_use]
    pub fn omit_content(mut self, omit: bool) -> Self {
        self.options.omit_content = omit;
        self
    }

    /// Filter by content types.
    #[must_use]
    pub fn content_types(mut self, types: Vec<String>) -> Self {
        self.options.content_types = types;
        self
    }

    /// Set maximum body size.
    #[must_use]
    pub fn max_body_size(mut self, size: usize) -> Self {
        self.options.max_body_size = size;
        self
    }

    /// Get the options.
    pub fn build(self) -> HarRecordingOptions {
        self.options
    }
}

/// A pending request being recorded.
#[derive(Debug, Clone)]
struct PendingHarRequest {
    /// Request ID from CDP.
    request_id: String,
    /// Start time.
    started_at: DateTime<Utc>,
    /// Request data.
    request: HarRequest,
    /// Page reference.
    page_ref: Option<String>,
    /// Resource type.
    resource_type: String,
    /// Frame ID.
    frame_id: String,
    /// Timing data.
    timing: Option<HarTimings>,
}

/// HAR recorder that captures network traffic.
#[derive(Debug)]
pub struct HarRecorder {
    /// Recording options.
    options: HarRecordingOptions,
    /// The HAR being built.
    har: RwLock<Har>,
    /// Pending requests (not yet complete).
    pending_requests: RwLock<HashMap<String, PendingHarRequest>>,
    /// Current page ID.
    current_page_id: RwLock<Option<String>>,
    /// Whether recording is active.
    is_recording: RwLock<bool>,
    /// URL matcher for filtering.
    url_matcher: Option<glob::Pattern>,
}

impl HarRecorder {
    /// Create a new HAR recorder.
    pub fn new(options: HarRecordingOptions) -> Result<Self, NetworkError> {
        let url_matcher =
            if let Some(ref pattern) = options.url_filter {
                Some(glob::Pattern::new(pattern).map_err(|e| {
                    NetworkError::InvalidResponse(format!("Invalid URL pattern: {e}"))
                })?)
            } else {
                None
            };

        Ok(Self {
            options,
            har: RwLock::new(Har::new("viewpoint", env!("CARGO_PKG_VERSION"))),
            pending_requests: RwLock::new(HashMap::new()),
            current_page_id: RwLock::new(None),
            is_recording: RwLock::new(true),
            url_matcher,
        })
    }

    /// Check if a URL should be recorded based on the filter.
    fn should_record_url(&self, url: &str) -> bool {
        match &self.url_matcher {
            Some(pattern) => pattern.matches(url),
            None => true,
        }
    }

    /// Check if content type should be included.
    fn should_include_content(&self, mime_type: &str) -> bool {
        if self.options.omit_content {
            return false;
        }
        if self.options.content_types.is_empty() {
            return true;
        }
        self.options
            .content_types
            .iter()
            .any(|t| mime_type.contains(t))
    }

    /// Start recording a new page.
    pub async fn start_page(&self, page_id: &str, title: &str) {
        let mut har = self.har.write().await;
        let page = HarPage::new(
            page_id,
            title,
            &Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        );
        har.add_page(page);

        let mut current = self.current_page_id.write().await;
        *current = Some(page_id.to_string());

        debug!(page_id = %page_id, title = %title, "Started recording page");
    }

    /// Record a request.
    pub async fn record_request(
        &self,
        request_id: &str,
        url: &str,
        method: &str,
        headers: &HashMap<String, String>,
        post_data: Option<&str>,
        resource_type: &str,
        frame_id: &str,
    ) {
        if !*self.is_recording.read().await {
            return;
        }

        if !self.should_record_url(url) {
            trace!(url = %url, "Skipping request - URL filter");
            return;
        }

        let mut request = HarRequest::new(method, url);
        request.set_headers(headers);
        request.parse_query_string();

        // Get content type for POST data
        let content_type = headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
            .map(|(_, v)| v.as_str());

        request.set_post_data(post_data, content_type);

        let page_ref = self.current_page_id.read().await.clone();

        let pending = PendingHarRequest {
            request_id: request_id.to_string(),
            started_at: Utc::now(),
            request,
            page_ref,
            resource_type: resource_type.to_string(),
            frame_id: frame_id.to_string(),
            timing: None,
        };

        let mut pending_requests = self.pending_requests.write().await;
        pending_requests.insert(request_id.to_string(), pending);

        trace!(request_id = %request_id, url = %url, "Recorded request");
    }

    /// Record timing information for a request.
    pub async fn record_timing(
        &self,
        request_id: &str,
        dns_start: f64,
        dns_end: f64,
        connect_start: f64,
        connect_end: f64,
        ssl_start: f64,
        ssl_end: f64,
        send_start: f64,
        send_end: f64,
        receive_headers_end: f64,
    ) {
        let mut pending_requests = self.pending_requests.write().await;
        if let Some(pending) = pending_requests.get_mut(request_id) {
            pending.timing = Some(HarTimings::from_resource_timing(
                dns_start,
                dns_end,
                connect_start,
                connect_end,
                ssl_start,
                ssl_end,
                send_start,
                send_end,
                receive_headers_end,
            ));
            trace!(request_id = %request_id, "Recorded timing");
        }
    }

    /// Record a response and complete the entry.
    pub async fn record_response(
        &self,
        request_id: &str,
        status: i32,
        status_text: &str,
        headers: &HashMap<String, String>,
        mime_type: &str,
        body: Option<&[u8]>,
        server_ip: Option<&str>,
    ) {
        let mut pending_requests = self.pending_requests.write().await;

        let pending = if let Some(p) = pending_requests.remove(request_id) {
            p
        } else {
            trace!(request_id = %request_id, "No pending request for response");
            return;
        };

        let started_date_time = pending
            .started_at
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let mut entry = HarEntry::new(&started_date_time);
        entry.pageref = pending.page_ref;
        entry.set_request(pending.request);

        let mut response = HarResponse::new(status, status_text);
        response.set_headers(headers);

        // Handle body content
        if self.should_include_content(mime_type) {
            let body_text = body.map(|b| {
                let mut data = b;

                // Truncate if needed
                if self.options.max_body_size > 0 && data.len() > self.options.max_body_size {
                    data = &data[..self.options.max_body_size];
                }

                // Try to decode as UTF-8
                if let Ok(text) = std::str::from_utf8(data) {
                    text.to_string()
                } else {
                    // Base64 encode binary content
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD.encode(data)
                }
            });

            let encoding = body.and_then(|b| {
                if std::str::from_utf8(b).is_err() {
                    Some("base64")
                } else {
                    None
                }
            });

            response.set_content(body_text.as_deref(), mime_type, encoding);
        }

        entry.set_response(response);

        // Set timing if available
        if let Some(mut timing) = pending.timing {
            // Calculate receive time
            let elapsed = Utc::now()
                .signed_duration_since(pending.started_at)
                .num_milliseconds() as f64;
            timing.receive = elapsed - timing.total();
            if timing.receive < 0.0 {
                timing.receive = 0.0;
            }
            entry.set_timings(timing);
        }

        // Set server IP
        entry.server_ip_address = server_ip.map(std::string::ToString::to_string);

        // Add entry to HAR
        let mut har = self.har.write().await;
        har.add_entry(entry);

        trace!(request_id = %request_id, status = %status, "Recorded response");
    }

    /// Record a failed request.
    pub async fn record_failure(&self, request_id: &str, error_text: &str) {
        let mut pending_requests = self.pending_requests.write().await;

        let pending = match pending_requests.remove(request_id) {
            Some(p) => p,
            None => return,
        };

        let started_date_time = pending
            .started_at
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let mut entry = HarEntry::new(&started_date_time);
        entry.pageref = pending.page_ref;
        entry.set_request(pending.request);
        entry.set_response(HarResponse::error(error_text));

        if let Some(timing) = pending.timing {
            entry.set_timings(timing);
        }

        let mut har = self.har.write().await;
        har.add_entry(entry);

        trace!(request_id = %request_id, error = %error_text, "Recorded request failure");
    }

    /// Save the HAR to file.
    pub async fn save(&self) -> Result<PathBuf, NetworkError> {
        let har = self.har.read().await;
        let path = &self.options.path;

        let json = serde_json::to_string_pretty(&*har)
            .map_err(|e| NetworkError::InvalidResponse(format!("Failed to serialize HAR: {e}")))?;

        tokio::fs::write(path, json)
            .await
            .map_err(|e| NetworkError::IoError(format!("Failed to write HAR file: {e}")))?;

        debug!(path = %path.display(), "Saved HAR file");
        Ok(path.clone())
    }

    /// Stop recording.
    pub async fn stop(&self) {
        let mut is_recording = self.is_recording.write().await;
        *is_recording = false;
    }

    /// Get the current HAR (for inspection).
    pub async fn get_har(&self) -> Har {
        self.har.read().await.clone()
    }

    /// Get the output path.
    pub fn path(&self) -> &PathBuf {
        &self.options.path
    }
}

#[cfg(test)]
mod tests;
