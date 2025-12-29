//! Cookie management for BrowserContext.
//!
//! This module provides methods for managing cookies in an isolated browser context.

use tracing::{debug, instrument};

use viewpoint_cdp::protocol::storage::{
    ClearCookiesParams as StorageClearCookiesParams,
    DeleteCookiesParams as StorageDeleteCookiesParams, GetCookiesParams as StorageGetCookiesParams,
    GetCookiesResult as StorageGetCookiesResult, SetCookiesParams as StorageSetCookiesParams,
};
use viewpoint_cdp::protocol::{CookieParam, CookieSameSite};

use super::BrowserContext;
use super::types::{Cookie, SameSite};
use crate::error::ContextError;

impl BrowserContext {
    /// Add cookies to the browser context.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{BrowserContext, context::{Cookie, SameSite}};
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), viewpoint_core::CoreError> {
    /// context.add_cookies(vec![
    ///     Cookie::new("session", "abc123")
    ///         .domain("example.com")
    ///         .path("/")
    ///         .secure(true)
    ///         .http_only(true),
    /// ]).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if setting cookies fails.
    #[instrument(level = "debug", skip(self, cookies))]
    pub async fn add_cookies(&self, cookies: Vec<Cookie>) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        debug!(count = cookies.len(), "Adding cookies");

        let cdp_cookies: Vec<CookieParam> = cookies
            .into_iter()
            .map(|c| {
                let mut param = CookieParam::new(&c.name, &c.value);
                if let Some(url) = c.url {
                    param = param.url(url);
                }
                if let Some(domain) = c.domain {
                    param = param.domain(domain);
                }
                if let Some(path) = c.path {
                    param = param.path(path);
                }
                if let Some(secure) = c.secure {
                    param = param.secure(secure);
                }
                if let Some(http_only) = c.http_only {
                    param = param.http_only(http_only);
                }
                if let Some(expires) = c.expires {
                    param = param.expires(expires);
                }
                if let Some(same_site) = c.same_site {
                    param.same_site = Some(match same_site {
                        SameSite::Strict => CookieSameSite::Strict,
                        SameSite::Lax => CookieSameSite::Lax,
                        SameSite::None => CookieSameSite::None,
                    });
                }
                param
            })
            .collect();

        self.connection()
            .send_command::<_, serde_json::Value>(
                "Storage.setCookies",
                Some(
                    StorageSetCookiesParams::new(cdp_cookies)
                        .browser_context_id(self.context_id().to_string()),
                ),
                None,
            )
            .await?;

        Ok(())
    }

    /// Get all cookies in the browser context.
    ///
    /// # Errors
    ///
    /// Returns an error if getting cookies fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn cookies(&self) -> Result<Vec<Cookie>, ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        let result: StorageGetCookiesResult = self
            .connection()
            .send_command(
                "Storage.getCookies",
                Some(
                    StorageGetCookiesParams::new()
                        .browser_context_id(self.context_id().to_string()),
                ),
                None,
            )
            .await?;

        let cookies = result
            .cookies
            .into_iter()
            .map(|c| Cookie {
                name: c.name,
                value: c.value,
                domain: Some(c.domain),
                path: Some(c.path),
                url: None,
                expires: if c.expires > 0.0 {
                    Some(c.expires)
                } else {
                    None
                },
                http_only: Some(c.http_only),
                secure: Some(c.secure),
                same_site: c.same_site.map(|s| match s {
                    CookieSameSite::Strict => SameSite::Strict,
                    CookieSameSite::Lax => SameSite::Lax,
                    CookieSameSite::None => SameSite::None,
                }),
            })
            .collect();

        Ok(cookies)
    }

    /// Get cookies for specific URLs.
    ///
    /// Note: This method gets all cookies and filters client-side by URL domain matching.
    ///
    /// # Errors
    ///
    /// Returns an error if getting cookies fails.
    #[instrument(level = "debug", skip(self, urls))]
    pub async fn cookies_for_urls(&self, urls: Vec<String>) -> Result<Vec<Cookie>, ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        // Get all cookies and filter by URL domains
        let all_cookies = self.cookies().await?;

        // Extract domains from URLs for filtering
        let domains: Vec<String> = urls
            .iter()
            .filter_map(|url| {
                url::Url::parse(url)
                    .ok()
                    .and_then(|u| u.host_str().map(std::string::ToString::to_string))
            })
            .collect();

        // Filter cookies that match any of the URL domains
        let cookies = all_cookies
            .into_iter()
            .filter(|c| {
                if let Some(ref cookie_domain) = c.domain {
                    domains.iter().any(|d| {
                        // Cookie domain can start with '.' for subdomain matching
                        let cookie_domain = cookie_domain.trim_start_matches('.');
                        d == cookie_domain || d.ends_with(&format!(".{cookie_domain}"))
                    })
                } else {
                    false
                }
            })
            .collect();

        Ok(cookies)
    }

    /// Get cookies for a specific URL.
    ///
    /// # Errors
    ///
    /// Returns an error if getting cookies fails.
    pub async fn cookies_for_url(
        &self,
        url: impl Into<String>,
    ) -> Result<Vec<Cookie>, ContextError> {
        self.cookies_for_urls(vec![url.into()]).await
    }

    /// Clear all cookies in the browser context.
    ///
    /// # Errors
    ///
    /// Returns an error if clearing cookies fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn clear_cookies(&self) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        self.connection()
            .send_command::<_, serde_json::Value>(
                "Storage.clearCookies",
                Some(
                    StorageClearCookiesParams::new()
                        .browser_context_id(self.context_id().to_string()),
                ),
                None,
            )
            .await?;

        Ok(())
    }

    /// Create a builder for clearing cookies with filters.
    pub fn clear_cookies_builder(&self) -> ClearCookiesBuilder<'_> {
        ClearCookiesBuilder::new(self)
    }
}

/// Builder for clearing cookies with filters.
#[derive(Debug)]
pub struct ClearCookiesBuilder<'a> {
    context: &'a BrowserContext,
    name: Option<String>,
    domain: Option<String>,
    path: Option<String>,
}

impl<'a> ClearCookiesBuilder<'a> {
    pub(crate) fn new(context: &'a BrowserContext) -> Self {
        Self {
            context,
            name: None,
            domain: None,
            path: None,
        }
    }

    /// Filter by cookie name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Filter by domain.
    #[must_use]
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Filter by path.
    #[must_use]
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Execute the clear operation.
    ///
    /// # Errors
    ///
    /// Returns an error if clearing cookies fails.
    pub async fn execute(self) -> Result<(), ContextError> {
        if self.context.is_closed() {
            return Err(ContextError::Closed);
        }

        // If no filters, clear all cookies
        if self.name.is_none() && self.domain.is_none() && self.path.is_none() {
            return self.context.clear_cookies().await;
        }

        // Get all cookies and filter
        let cookies = self.context.cookies().await?;

        for cookie in cookies {
            let matches_name = self.name.as_ref().is_none_or(|n| cookie.name == *n);
            let matches_domain = self
                .domain
                .as_ref()
                .is_none_or(|d| cookie.domain.as_deref() == Some(d.as_str()));
            let matches_path = self
                .path
                .as_ref()
                .is_none_or(|p| cookie.path.as_deref() == Some(p.as_str()));

            if matches_name && matches_domain && matches_path {
                let mut params = StorageDeleteCookiesParams::new(&cookie.name)
                    .browser_context_id(self.context.context_id().to_string());
                if let Some(domain) = &cookie.domain {
                    params = params.domain(domain);
                }
                if let Some(path) = &cookie.path {
                    params = params.path(path);
                }

                self.context
                    .connection()
                    .send_command::<_, serde_json::Value>(
                        "Storage.deleteCookies",
                        Some(params),
                        None,
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
