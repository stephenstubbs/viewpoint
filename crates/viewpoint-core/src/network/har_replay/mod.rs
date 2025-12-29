//! HAR replay functionality.
//!
//! This module provides the ability to replay network requests from HAR files,
//! mocking API responses during test execution.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, trace, warn};

use crate::error::NetworkError;
use crate::network::{Route, UrlPattern};

use super::har::{Har, HarEntry};

/// Options for HAR replay.
#[derive(Debug, Clone)]
pub struct HarReplayOptions {
    /// URL pattern to filter which requests use HAR replay.
    /// Only URLs matching this pattern will be replayed from the HAR.
    pub url_filter: Option<UrlPattern>,
    /// If true, requests that don't match any HAR entry will fail.
    /// If false, unmatched requests continue normally.
    pub strict: bool,
    /// If true, unmatched requests will be recorded to update the HAR file.
    pub update: bool,
    /// How to handle content in update mode.
    pub update_content: UpdateContentMode,
    /// How to handle timings in update mode.
    pub update_timings: TimingMode,
    /// If true, simulate the original timing delays from the HAR.
    pub use_original_timing: bool,
}

impl Default for HarReplayOptions {
    fn default() -> Self {
        Self {
            url_filter: None,
            strict: false,
            update: false,
            update_content: UpdateContentMode::Embed,
            update_timings: TimingMode::Placeholder,
            use_original_timing: false,
        }
    }
}

impl HarReplayOptions {
    /// Create new default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set URL filter pattern.
    #[must_use]
    pub fn url<M: Into<UrlPattern>>(mut self, pattern: M) -> Self {
        self.url_filter = Some(pattern.into());
        self
    }

    /// Enable strict mode (fail if no match found).
    #[must_use]
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Enable update mode (record missing entries).
    #[must_use]
    pub fn update(mut self, update: bool) -> Self {
        self.update = update;
        self
    }

    /// Set how to handle content in update mode.
    #[must_use]
    pub fn update_content(mut self, mode: UpdateContentMode) -> Self {
        self.update_content = mode;
        self
    }

    /// Set how to handle timings in update mode.
    #[must_use]
    pub fn update_timings(mut self, mode: TimingMode) -> Self {
        self.update_timings = mode;
        self
    }

    /// Enable timing simulation from HAR entries.
    #[must_use]
    pub fn use_original_timing(mut self, use_timing: bool) -> Self {
        self.use_original_timing = use_timing;
        self
    }
}

/// How to handle response content in update mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateContentMode {
    /// Embed content directly in the HAR file.
    Embed,
    /// Store large content as separate files.
    Attach,
    /// Don't record response body.
    Omit,
}

/// How to handle timing information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingMode {
    /// Use placeholder values (0 or -1) for consistent git diffs.
    Placeholder,
    /// Record actual timing values.
    Actual,
}

/// HAR replay handler that matches requests against HAR entries.
pub struct HarReplayHandler {
    /// The loaded HAR data.
    har: Har,
    /// Options for replay behavior.
    options: HarReplayOptions,
    /// Path to the HAR file (for update mode).
    har_path: Option<std::path::PathBuf>,
    /// New entries recorded during update mode.
    new_entries: Arc<RwLock<Vec<HarEntry>>>,
}

impl HarReplayHandler {
    /// Create a new HAR replay handler from a file path.
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, NetworkError> {
        let path = path.as_ref();
        debug!("Loading HAR from: {}", path.display());

        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| NetworkError::IoError(format!("Failed to read HAR file: {e}")))?;

        let har: Har = serde_json::from_str(&content)
            .map_err(|e| NetworkError::HarError(format!("Failed to parse HAR: {e}")))?;

        debug!("Loaded HAR with {} entries", har.log.entries.len());

        Ok(Self {
            har,
            options: HarReplayOptions::default(),
            har_path: Some(path.to_path_buf()),
            new_entries: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Create a new HAR replay handler from a HAR struct.
    pub fn from_har(har: Har) -> Self {
        Self {
            har,
            options: HarReplayOptions::default(),
            har_path: None,
            new_entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Set options for the handler.
    pub fn with_options(mut self, options: HarReplayOptions) -> Self {
        self.options = options;
        self
    }

    /// Find a matching HAR entry for the given request.
    pub fn find_entry(
        &self,
        url: &str,
        method: &str,
        post_data: Option<&str>,
    ) -> Option<&HarEntry> {
        // First, check if URL matches the filter
        if let Some(ref filter) = self.options.url_filter {
            if !filter.matches(url) {
                trace!("URL {} doesn't match filter, skipping HAR lookup", url);
                return None;
            }
        }

        // Find matching entries
        for entry in &self.har.log.entries {
            if self.entry_matches(entry, url, method, post_data) {
                debug!("Found HAR match for {} {}", method, url);
                return Some(entry);
            }
        }

        debug!("No HAR match found for {} {}", method, url);
        None
    }

    /// Check if a HAR entry matches the request.
    fn entry_matches(
        &self,
        entry: &HarEntry,
        url: &str,
        method: &str,
        post_data: Option<&str>,
    ) -> bool {
        // Match URL
        if !self.url_matches(&entry.request.url, url) {
            return false;
        }

        // Match method
        if entry.request.method.to_uppercase() != method.to_uppercase() {
            return false;
        }

        // Match POST data if present
        if let Some(request_post_data) = post_data {
            if let Some(ref har_post_data) = entry.request.post_data {
                if !self.post_data_matches(&har_post_data.text, request_post_data) {
                    return false;
                }
            }
        }

        true
    }

    /// Check if URLs match (handles query string variations).
    fn url_matches(&self, har_url: &str, request_url: &str) -> bool {
        // Parse both URLs
        let har_parsed = url::Url::parse(har_url);
        let request_parsed = url::Url::parse(request_url);

        match (har_parsed, request_parsed) {
            (Ok(har), Ok(req)) => {
                // Compare scheme, host, and path
                if har.scheme() != req.scheme() {
                    return false;
                }
                if har.host_str() != req.host_str() {
                    return false;
                }
                if har.path() != req.path() {
                    return false;
                }

                // For query parameters, check that all HAR params are present
                // (request may have additional params)
                let har_params: HashMap<_, _> = har.query_pairs().collect();
                let req_params: HashMap<_, _> = req.query_pairs().collect();

                for (key, value) in &har_params {
                    if req_params.get(key) != Some(value) {
                        return false;
                    }
                }

                true
            }
            _ => {
                // Fallback to exact string match
                har_url == request_url
            }
        }
    }

    /// Check if POST data matches.
    fn post_data_matches(&self, har_post_data: &str, request_post_data: &str) -> bool {
        // Try parsing as JSON for semantic comparison
        let har_json: Result<serde_json::Value, _> = serde_json::from_str(har_post_data);
        let req_json: Result<serde_json::Value, _> = serde_json::from_str(request_post_data);

        match (har_json, req_json) {
            (Ok(har), Ok(req)) => har == req,
            _ => {
                // Fallback to string comparison
                har_post_data == request_post_data
            }
        }
    }

    /// Build a response from a HAR entry.
    pub fn build_response(&self, entry: &HarEntry) -> HarResponseData {
        let response = &entry.response;

        HarResponseData {
            status: response.status as u16,
            status_text: response.status_text.clone(),
            headers: response
                .headers
                .iter()
                .map(|h| (h.name.clone(), h.value.clone()))
                .collect(),
            body: response.content.text.clone(),
            timing_ms: if self.options.use_original_timing {
                Some(entry.time as u64)
            } else {
                None
            },
        }
    }

    /// Get the options.
    pub fn options(&self) -> &HarReplayOptions {
        &self.options
    }

    /// Get the HAR data.
    pub fn har(&self) -> &Har {
        &self.har
    }

    /// Record a new entry (for update mode).
    pub async fn record_entry(&self, entry: HarEntry) {
        let mut entries = self.new_entries.write().await;
        entries.push(entry);
    }

    /// Save updated HAR file (for update mode).
    pub async fn save_updates(&self) -> Result<(), NetworkError> {
        if !self.options.update {
            return Ok(());
        }

        let path = self
            .har_path
            .as_ref()
            .ok_or_else(|| NetworkError::HarError("No HAR path set for updates".to_string()))?;

        let new_entries = self.new_entries.read().await;
        if new_entries.is_empty() {
            return Ok(());
        }

        // Create updated HAR
        let mut updated_har = self.har.clone();
        for entry in new_entries.iter() {
            updated_har.log.entries.push(entry.clone());
        }

        // Write to file
        let content = serde_json::to_string_pretty(&updated_har)
            .map_err(|e| NetworkError::HarError(format!("Failed to serialize HAR: {e}")))?;

        tokio::fs::write(path, content)
            .await
            .map_err(|e| NetworkError::IoError(format!("Failed to write HAR: {e}")))?;

        debug!("Saved {} new entries to HAR", new_entries.len());
        Ok(())
    }
}

/// Response data extracted from a HAR entry.
#[derive(Debug, Clone)]
pub struct HarResponseData {
    /// HTTP status code.
    pub status: u16,
    /// HTTP status text.
    pub status_text: String,
    /// Response headers.
    pub headers: Vec<(String, String)>,
    /// Response body (if available).
    pub body: Option<String>,
    /// Timing to simulate (in milliseconds).
    pub timing_ms: Option<u64>,
}

/// Create a route handler that replays from HAR.
pub fn create_har_route_handler(
    handler: Arc<HarReplayHandler>,
) -> impl Fn(
    Route,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), NetworkError>> + Send>>
+ Send
+ Sync
+ Clone
+ 'static {
    move |route: Route| {
        let handler = handler.clone();
        Box::pin(async move {
            let request = route.request();
            let url = request.url();
            let method = request.method();
            let post_data = request.post_data();

            // Find matching entry
            if let Some(entry) = handler.find_entry(url, method, post_data) {
                let response_data = handler.build_response(entry);

                // Simulate timing if requested
                if let Some(timing_ms) = response_data.timing_ms {
                    tokio::time::sleep(std::time::Duration::from_millis(timing_ms)).await;
                }

                // Build and send response
                let mut builder = route.fulfill().status(response_data.status);

                // Add headers
                for (name, value) in response_data.headers {
                    builder = builder.header(&name, &value);
                }

                // Add body
                if let Some(body) = response_data.body {
                    builder = builder.body(body);
                }

                builder.send().await
            } else if handler.options().strict {
                // Strict mode: fail on no match
                warn!("HAR strict mode: no match for {} {}", method, url);
                route.abort().await
            } else {
                // Non-strict mode: continue request normally
                route.continue_().await
            }
        })
    }
}

#[cfg(test)]
mod tests;
