//! Network domain cookie types.
//!
//! Types for working with browser cookies via the Network domain.

use serde::{Deserialize, Serialize};

/// Same site cookie attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CookieSameSite {
    /// Strict same-site.
    Strict,
    /// Lax same-site.
    #[default]
    Lax,
    /// None (cross-site allowed).
    None,
}

/// Cookie priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CookiePriority {
    /// Low priority.
    Low,
    /// Medium priority.
    #[default]
    Medium,
    /// High priority.
    High,
}

/// Cookie source scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CookieSourceScheme {
    /// Unset.
    #[default]
    Unset,
    /// Non-secure.
    NonSecure,
    /// Secure.
    Secure,
}

/// Represents a cookie.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    /// Cookie name.
    pub name: String,
    /// Cookie value.
    pub value: String,
    /// Cookie domain.
    pub domain: String,
    /// Cookie path.
    pub path: String,
    /// Cookie expiration date as the number of seconds since the UNIX epoch.
    pub expires: f64,
    /// Cookie size.
    pub size: i32,
    /// True if cookie is http-only.
    pub http_only: bool,
    /// True if cookie is secure.
    pub secure: bool,
    /// True in case of session cookie.
    pub session: bool,
    /// Cookie `SameSite` type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<CookieSameSite>,
    /// Cookie Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CookiePriority>,
    /// True if cookie is same-party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_party: Option<bool>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_scheme: Option<CookieSourceScheme>,
    /// Cookie source port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<i32>,
    /// Cookie partition key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    /// True if cookie partition key is opaque.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key_opaque: Option<bool>,
}

/// Cookie parameter for setting cookies.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CookieParam {
    /// Cookie name.
    pub name: String,
    /// Cookie value.
    pub value: String,
    /// The request-URI to associate with the setting of the cookie.
    /// This value can affect the default domain, path, source port, and source scheme values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Cookie domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Cookie path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// True if cookie is secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// True if cookie is http-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    /// Cookie `SameSite` type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<CookieSameSite>,
    /// Cookie expiration date, session cookie if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
    /// Cookie Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CookiePriority>,
    /// True if cookie is same-party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_party: Option<bool>,
    /// Cookie source scheme type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_scheme: Option<CookieSourceScheme>,
    /// Cookie source port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<i32>,
    /// Cookie partition key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
}

impl CookieParam {
    /// Create a new cookie parameter with name and value.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            ..Default::default()
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

    /// Set whether the cookie is secure.
    #[must_use]
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = Some(secure);
        self
    }

    /// Set whether the cookie is HTTP-only.
    #[must_use]
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = Some(http_only);
        self
    }

    /// Set the `SameSite` attribute.
    #[must_use]
    pub fn same_site(mut self, same_site: CookieSameSite) -> Self {
        self.same_site = Some(same_site);
        self
    }

    /// Set the expiration time (Unix timestamp in seconds).
    #[must_use]
    pub fn expires(mut self, expires: f64) -> Self {
        self.expires = Some(expires);
        self
    }
}

/// Parameters for Network.getCookies.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesParams {
    /// URLs to get cookies for. If not specified, returns all cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// Result for Network.getCookies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesResult {
    /// Array of cookie objects.
    pub cookies: Vec<Cookie>,
}

/// Parameters for Network.setCookies.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesParams {
    /// Cookies to be set.
    pub cookies: Vec<CookieParam>,
}

/// Parameters for Network.deleteCookies.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookiesParams {
    /// Name of the cookies to remove.
    pub name: String,
    /// URL to match cooke domain and path.
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
}

/// Parameters for Network.clearBrowserCookies.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ClearBrowserCookiesParams {}
