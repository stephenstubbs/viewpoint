//! API context options for configuring HTTP clients.

use std::collections::HashMap;
use std::time::Duration;

/// Options for creating an API request context.
///
/// # Example
///
/// ```no_run
/// use viewpoint_core::api::APIContextOptions;
/// use std::time::Duration;
///
/// let options = APIContextOptions::new()
///     .base_url("https://api.example.com")
///     .timeout(Duration::from_secs(30))
///     .extra_http_headers([
///         ("Authorization".to_string(), "Bearer token".to_string()),
///     ]);
/// ```
#[derive(Debug, Clone, Default)]
pub struct APIContextOptions {
    /// Base URL for all requests. Relative URLs will be resolved against this.
    pub(crate) base_url: Option<String>,
    /// Extra HTTP headers to include in all requests.
    pub(crate) extra_http_headers: HashMap<String, String>,
    /// HTTP credentials for Basic/Digest authentication.
    pub(crate) http_credentials: Option<HttpCredentials>,
    /// Whether to ignore HTTPS certificate errors.
    pub(crate) ignore_https_errors: bool,
    /// Proxy configuration.
    pub(crate) proxy: Option<ProxyConfig>,
    /// Default timeout for requests.
    pub(crate) timeout: Option<Duration>,
    /// User agent string.
    pub(crate) user_agent: Option<String>,
}

/// HTTP authentication credentials.
#[derive(Debug, Clone)]
pub struct HttpCredentials {
    /// Username for authentication.
    pub username: String,
    /// Password for authentication.
    pub password: String,
    /// Optional origin to send credentials only to specific domain.
    pub origin: Option<String>,
    /// Whether to send credentials preemptively.
    pub send: Option<CredentialSend>,
}

impl HttpCredentials {
    /// Create new HTTP credentials.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            origin: None,
            send: None,
        }
    }

    /// Set the origin to send credentials only to specific domain.
    #[must_use]
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    /// Set when to send credentials.
    #[must_use]
    pub fn send(mut self, send: CredentialSend) -> Self {
        self.send = Some(send);
        self
    }
}

/// When to send credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialSend {
    /// Send credentials only when the server requests them (401 response).
    Unauthorized,
    /// Always send credentials with every request (preemptive).
    Always,
}

/// Proxy configuration.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// Proxy server URL.
    pub server: String,
    /// Optional username for proxy authentication.
    pub username: Option<String>,
    /// Optional password for proxy authentication.
    pub password: Option<String>,
    /// Bypass proxy for these domains (comma-separated).
    pub bypass: Option<String>,
}

impl ProxyConfig {
    /// Create a new proxy configuration.
    pub fn new(server: impl Into<String>) -> Self {
        Self {
            server: server.into(),
            username: None,
            password: None,
            bypass: None,
        }
    }

    /// Set proxy authentication credentials.
    #[must_use]
    pub fn credentials(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set domains to bypass the proxy.
    #[must_use]
    pub fn bypass(mut self, bypass: impl Into<String>) -> Self {
        self.bypass = Some(bypass.into());
        self
    }
}

impl APIContextOptions {
    /// Create a new options builder with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL for all requests.
    ///
    /// Relative URLs passed to request methods will be resolved against this base URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::api::APIContextOptions;
    ///
    /// let options = APIContextOptions::new()
    ///     .base_url("https://api.example.com/v1");
    /// // Now api.get("/users") will request https://api.example.com/v1/users
    /// ```
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set extra HTTP headers to include in all requests.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::api::APIContextOptions;
    ///
    /// let options = APIContextOptions::new()
    ///     .extra_http_headers([
    ///         ("Authorization".to_string(), "Bearer token".to_string()),
    ///         ("X-API-Key".to_string(), "secret".to_string()),
    ///     ]);
    /// ```
    #[must_use]
    pub fn extra_http_headers(
        mut self,
        headers: impl IntoIterator<Item = (String, String)>,
    ) -> Self {
        self.extra_http_headers = headers.into_iter().collect();
        self
    }

    /// Add a single extra HTTP header.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_http_headers.insert(name.into(), value.into());
        self
    }

    /// Set HTTP credentials for Basic authentication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::api::{APIContextOptions, HttpCredentials};
    ///
    /// let options = APIContextOptions::new()
    ///     .http_credentials(HttpCredentials::new("user", "pass"));
    /// ```
    #[must_use]
    pub fn http_credentials(mut self, credentials: HttpCredentials) -> Self {
        self.http_credentials = Some(credentials);
        self
    }

    /// Set whether to ignore HTTPS certificate errors.
    ///
    /// **Warning**: This should only be used for testing. Never use this in production
    /// as it makes the connection vulnerable to man-in-the-middle attacks.
    #[must_use]
    pub fn ignore_https_errors(mut self, ignore: bool) -> Self {
        self.ignore_https_errors = ignore;
        self
    }

    /// Set proxy configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::api::{APIContextOptions, ProxyConfig};
    ///
    /// let options = APIContextOptions::new()
    ///     .proxy(ProxyConfig::new("http://proxy.example.com:8080")
    ///         .credentials("user", "pass"));
    /// ```
    #[must_use]
    pub fn proxy(mut self, proxy: ProxyConfig) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Set the default timeout for all requests.
    ///
    /// This can be overridden on a per-request basis.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the user agent string.
    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
}

#[cfg(test)]
mod tests;
