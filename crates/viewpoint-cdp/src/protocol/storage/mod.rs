//! Storage domain types.
//!
//! The Storage domain exposes storage-related operations at the browser level.

use serde::{Deserialize, Serialize};

// Re-use cookie types from network_cookies module
pub use super::network_cookies::{Cookie, CookieParam, CookieSameSite};

/// Parameters for Storage.setCookies.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesParams {
    /// Cookies to be set.
    pub cookies: Vec<CookieParam>,
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
}

impl SetCookiesParams {
    /// Create new set cookies params.
    pub fn new(cookies: Vec<CookieParam>) -> Self {
        Self {
            cookies,
            browser_context_id: None,
        }
    }

    /// Set the browser context ID.
    #[must_use]
    pub fn browser_context_id(mut self, id: impl Into<String>) -> Self {
        self.browser_context_id = Some(id.into());
        self
    }
}

/// Parameters for Storage.getCookies.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesParams {
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
}

impl GetCookiesParams {
    /// Create new get cookies params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the browser context ID.
    #[must_use]
    pub fn browser_context_id(mut self, id: impl Into<String>) -> Self {
        self.browser_context_id = Some(id.into());
        self
    }
}

/// Result for Storage.getCookies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesResult {
    /// Array of cookie objects.
    pub cookies: Vec<Cookie>,
}

/// Parameters for Storage.deleteCookies.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookiesParams {
    /// Name of the cookies to remove.
    pub name: String,
    /// URL to match cookie domain and path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Cookie domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Cookie partition key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
}

impl DeleteCookiesParams {
    /// Create new delete cookies params.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
            domain: None,
            path: None,
            partition_key: None,
            browser_context_id: None,
        }
    }

    /// Set the URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the domain.
    #[must_use]
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Set the path.
    #[must_use]
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Set the browser context ID.
    #[must_use]
    pub fn browser_context_id(mut self, id: impl Into<String>) -> Self {
        self.browser_context_id = Some(id.into());
        self
    }
}

/// Parameters for Storage.clearCookies.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCookiesParams {
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
}

impl ClearCookiesParams {
    /// Create new clear cookies params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the browser context ID.
    #[must_use]
    pub fn browser_context_id(mut self, id: impl Into<String>) -> Self {
        self.browser_context_id = Some(id.into());
        self
    }
}
