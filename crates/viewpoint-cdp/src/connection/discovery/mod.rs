//! CDP endpoint discovery via HTTP.
//!
//! Chrome DevTools Protocol exposes an HTTP endpoint that returns browser metadata
//! including the WebSocket URL. This module handles discovering the WebSocket URL
//! from an HTTP endpoint.

use std::collections::HashMap;
use std::time::Duration;

use serde::Deserialize;
use tracing::{debug, info, instrument};
use url::Url;

use crate::error::CdpError;

/// Default timeout for HTTP endpoint discovery.
const DEFAULT_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(30);

/// Response from the `/json/version` endpoint.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserVersion {
    /// Browser name and version.
    pub browser: Option<String>,
    /// Protocol version.
    pub protocol_version: Option<String>,
    /// User agent string.
    pub user_agent: Option<String>,
    /// V8 version.
    #[serde(rename = "V8-Version")]
    pub v8_version: Option<String>,
    /// WebKit version.
    pub webkit_version: Option<String>,
    /// The WebSocket URL for browser-level CDP connection.
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: Option<String>,
}

/// Options for CDP connection.
#[derive(Debug, Clone, Default)]
pub struct CdpConnectionOptions {
    /// Timeout for the connection attempt.
    pub timeout: Option<Duration>,
    /// Custom headers to include in the WebSocket upgrade request.
    pub headers: HashMap<String, String>,
}

impl CdpConnectionOptions {
    /// Create new connection options with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the connection timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Add a custom header.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add multiple custom headers.
    #[must_use]
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }
}

/// Discover the WebSocket URL from an HTTP endpoint.
///
/// Given a URL like `http://localhost:9222`, this function fetches `/json/version`
/// to get the `webSocketDebuggerUrl`.
///
/// # Arguments
///
/// * `endpoint_url` - The HTTP endpoint URL (e.g., `http://localhost:9222`)
/// * `options` - Connection options including timeout and headers
///
/// # Errors
///
/// Returns an error if:
/// - The URL is invalid
/// - The HTTP request fails
/// - The response doesn't contain a WebSocket URL
#[instrument(level = "info", skip(options))]
pub async fn discover_websocket_url(
    endpoint_url: &str,
    options: &CdpConnectionOptions,
) -> Result<String, CdpError> {
    // Parse and validate the URL
    let base_url = Url::parse(endpoint_url)
        .map_err(|e| CdpError::InvalidEndpointUrl(format!("{endpoint_url}: {e}")))?;

    // Check if it's already a WebSocket URL
    if base_url.scheme() == "ws" || base_url.scheme() == "wss" {
        debug!("URL is already a WebSocket URL, returning as-is");
        return Ok(endpoint_url.to_string());
    }

    // Ensure it's an HTTP URL
    if base_url.scheme() != "http" && base_url.scheme() != "https" {
        return Err(CdpError::InvalidEndpointUrl(format!(
            "expected http, https, ws, or wss scheme, got: {}",
            base_url.scheme()
        )));
    }

    // Build the /json/version URL
    let version_url = base_url
        .join("/json/version")
        .map_err(|e| CdpError::InvalidEndpointUrl(format!("failed to build version URL: {e}")))?;

    info!(url = %version_url, "Discovering WebSocket URL from HTTP endpoint");

    // Build the HTTP client with timeout
    let timeout = options.timeout.unwrap_or(DEFAULT_DISCOVERY_TIMEOUT);
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| CdpError::HttpRequestFailed(e.to_string()))?;

    // Build the request with custom headers
    let mut request = client.get(version_url.as_str());
    for (name, value) in &options.headers {
        request = request.header(name, value);
    }

    // Send the request
    let response = request.send().await.map_err(|e| {
        if e.is_timeout() {
            CdpError::ConnectionTimeout(timeout)
        } else if e.is_connect() {
            CdpError::ConnectionFailed(format!("failed to connect to {endpoint_url}: {e}"))
        } else {
            CdpError::HttpRequestFailed(e.to_string())
        }
    })?;

    // Check response status
    if !response.status().is_success() {
        return Err(CdpError::EndpointDiscoveryFailed {
            url: endpoint_url.to_string(),
            reason: format!("HTTP status {}", response.status()),
        });
    }

    // Parse the response
    let version: BrowserVersion =
        response
            .json()
            .await
            .map_err(|e| CdpError::EndpointDiscoveryFailed {
                url: endpoint_url.to_string(),
                reason: format!("failed to parse response: {e}"),
            })?;

    // Extract the WebSocket URL
    let ws_url =
        version
            .web_socket_debugger_url
            .ok_or_else(|| CdpError::EndpointDiscoveryFailed {
                url: endpoint_url.to_string(),
                reason: "response missing webSocketDebuggerUrl field".to_string(),
            })?;

    info!(ws_url = %ws_url, browser = ?version.browser, "Discovered WebSocket URL");

    Ok(ws_url)
}

#[cfg(test)]
mod tests;
