//! Proxy configuration for browser contexts.

use serde::{Deserialize, Serialize};

/// Proxy configuration for browser context.
///
/// Allows routing browser traffic through a proxy server.
/// Supports HTTP, HTTPS, and SOCKS5 proxies with optional authentication.
///
/// # Example
///
/// ```no_run
/// use viewpoint_core::{Browser, context::ProxyConfig};
///
/// # async fn example() -> Result<(), viewpoint_core::CoreError> {
/// let browser = Browser::launch().headless(true).launch().await?;
///
/// // Simple proxy without authentication
/// let context = browser.new_context_builder()
///     .proxy(ProxyConfig::new("http://proxy.example.com:8080"))
///     .build()
///     .await?;
///
/// // SOCKS5 proxy with authentication
/// let context = browser.new_context_builder()
///     .proxy(ProxyConfig::new("socks5://proxy.example.com:1080")
///         .credentials("user", "password"))
///     .build()
///     .await?;
///
/// // Proxy with bypass list
/// let context = browser.new_context_builder()
///     .proxy(ProxyConfig::new("http://proxy.example.com:8080")
///         .bypass("localhost,127.0.0.1,.internal.example.com"))
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy server URL (e.g., "http://proxy:8080", "socks5://proxy:1080").
    pub server: String,
    /// Optional username for proxy authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional password for proxy authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Comma-separated list of domains to bypass the proxy.
    /// Example: "localhost,127.0.0.1,.example.com"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass: Option<String>,
}

impl ProxyConfig {
    /// Create a new proxy configuration.
    ///
    /// # Arguments
    ///
    /// * `server` - Proxy server URL (e.g., "http://proxy:8080" or "socks5://proxy:1080")
    ///
    /// # Example
    ///
    /// ```
    /// use viewpoint_core::context::ProxyConfig;
    ///
    /// let proxy = ProxyConfig::new("http://proxy.example.com:8080");
    /// ```
    pub fn new(server: impl Into<String>) -> Self {
        Self {
            server: server.into(),
            username: None,
            password: None,
            bypass: None,
        }
    }

    /// Set proxy authentication credentials.
    ///
    /// # Example
    ///
    /// ```
    /// use viewpoint_core::context::ProxyConfig;
    ///
    /// let proxy = ProxyConfig::new("http://proxy.example.com:8080")
    ///     .credentials("username", "password");
    /// ```
    #[must_use]
    pub fn credentials(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set domains to bypass the proxy.
    ///
    /// Provide a comma-separated list of domains that should not go through the proxy.
    ///
    /// # Example
    ///
    /// ```
    /// use viewpoint_core::context::ProxyConfig;
    ///
    /// let proxy = ProxyConfig::new("http://proxy.example.com:8080")
    ///     .bypass("localhost,127.0.0.1,.internal.example.com");
    /// ```
    #[must_use]
    pub fn bypass(mut self, bypass: impl Into<String>) -> Self {
        self.bypass = Some(bypass.into());
        self
    }
}

#[cfg(test)]
mod tests;
