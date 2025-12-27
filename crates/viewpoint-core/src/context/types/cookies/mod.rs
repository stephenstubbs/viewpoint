//! Cookie types for browser contexts.

use serde::{Deserialize, Serialize};

/// Same site cookie attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SameSite {
    /// Strict same-site.
    Strict,
    /// Lax same-site (default).
    #[default]
    Lax,
    /// None (cross-site allowed, requires Secure).
    None,
}

/// A browser cookie.
///
/// Matches Playwright's cookie structure for interoperability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    /// Cookie name.
    pub name: String,
    /// Cookie value.
    pub value: String,
    /// Cookie domain. Either domain or URL must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Request URL to associate with cookie (alternative to domain+path).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Unix timestamp in seconds, -1 for session cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
    /// HTTP-only cookie flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    /// Secure cookie flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// Same site attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<SameSite>,
}

impl Cookie {
    /// Create a new cookie with name and value.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: None,
            path: None,
            url: None,
            expires: None,
            http_only: None,
            secure: None,
            same_site: None,
        }
    }

    /// Set the URL (alternative to domain+path).
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

    /// Set the expiration time (Unix timestamp in seconds).
    #[must_use]
    pub fn expires(mut self, expires: f64) -> Self {
        self.expires = Some(expires);
        self
    }

    /// Set whether the cookie is HTTP-only.
    #[must_use]
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = Some(http_only);
        self
    }

    /// Set whether the cookie is secure.
    #[must_use]
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = Some(secure);
        self
    }

    /// Set the `SameSite` attribute.
    #[must_use]
    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = Some(same_site);
        self
    }
}
