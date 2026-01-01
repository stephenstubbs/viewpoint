//! Browser connection via CDP endpoints.
//!
//! This module provides the `ConnectOverCdpBuilder` for connecting to browsers
//! via HTTP or WebSocket endpoints.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tracing::{info, instrument};
use viewpoint_cdp::{CdpConnection, CdpConnectionOptions};

use super::Browser;
use crate::error::BrowserError;

/// Default connection timeout.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Builder for connecting to a browser via CDP.
///
/// This builder supports connecting via:
/// - HTTP endpoint URL (e.g., `http://localhost:9222`) - auto-discovers WebSocket URL
/// - WebSocket URL (e.g., `ws://localhost:9222/devtools/browser/...`) - direct connection
///
/// # Example
///
/// ```no_run
/// use viewpoint_core::Browser;
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), viewpoint_core::CoreError> {
/// // Connect via HTTP endpoint (auto-discovers WebSocket URL)
/// let browser = Browser::connect_over_cdp("http://localhost:9222")
///     .timeout(Duration::from_secs(10))
///     .connect()
///     .await?;
///
/// // Connect with custom headers
/// let browser = Browser::connect_over_cdp("http://remote-host:9222")
///     .header("Authorization", "Bearer token")
///     .connect()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ConnectOverCdpBuilder {
    /// The endpoint URL (HTTP or WebSocket).
    endpoint_url: String,
    /// Connection timeout.
    timeout: Option<Duration>,
    /// Custom headers for the connection.
    headers: HashMap<String, String>,
}

impl ConnectOverCdpBuilder {
    /// Create a new connection builder.
    pub(crate) fn new(endpoint_url: impl Into<String>) -> Self {
        Self {
            endpoint_url: endpoint_url.into(),
            timeout: None,
            headers: HashMap::new(),
        }
    }

    /// Set the connection timeout.
    ///
    /// Default is 30 seconds.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Add a custom header for the WebSocket connection.
    ///
    /// Headers are sent during the WebSocket upgrade request.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add multiple custom headers for the WebSocket connection.
    #[must_use]
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Connect to the browser.
    ///
    /// If the endpoint URL is an HTTP URL, this will first discover the WebSocket
    /// URL by fetching `/json/version`. Then it connects to the browser via WebSocket.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The endpoint URL is invalid
    /// - The HTTP endpoint doesn't expose CDP
    /// - The WebSocket connection fails
    /// - The connection times out
    #[instrument(level = "info", skip(self), fields(endpoint_url = %self.endpoint_url))]
    pub async fn connect(self) -> Result<Browser, BrowserError> {
        info!("Connecting to browser via CDP endpoint");

        // Build connection options
        let options = CdpConnectionOptions::new()
            .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .headers(self.headers);

        // Connect using the CDP layer's HTTP discovery
        let connection = CdpConnection::connect_via_http_with_options(&self.endpoint_url, options)
            .await
            .map_err(|e| match e {
                viewpoint_cdp::CdpError::ConnectionTimeout(d) => BrowserError::ConnectionTimeout(d),
                viewpoint_cdp::CdpError::InvalidEndpointUrl(s) => {
                    BrowserError::InvalidEndpointUrl(s)
                }
                viewpoint_cdp::CdpError::EndpointDiscoveryFailed { url, reason } => {
                    BrowserError::EndpointDiscoveryFailed(format!("{url}: {reason}"))
                }
                viewpoint_cdp::CdpError::ConnectionFailed(s) => BrowserError::ConnectionFailed(s),
                other => BrowserError::Cdp(other),
            })?;

        info!("Successfully connected to browser");

        Ok(Browser {
            connection: Arc::new(connection),
            process: None,
            owned: false,
            _temp_user_data_dir: None,
        })
    }
}
