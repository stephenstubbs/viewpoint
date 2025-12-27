//! Context-level API request functionality.

use tracing::debug;

use crate::api::{APIContextOptions, APIRequestContext};
use crate::error::ContextError;

use super::BrowserContext;

impl BrowserContext {
    /// Get an API request context associated with this browser context.
    ///
    /// The returned `APIRequestContext` can be used to make HTTP requests.
    /// Cookies from the browser context are automatically synced to the API context.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use viewpoint_core::{Browser, BrowserContext};
    ///
    /// let browser = Browser::launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Get API context (includes browser cookies)
    /// let api = context.request().await?;
    ///
    /// // Make API requests with browser cookies
    /// let response = api.get("https://api.example.com/data").send().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the API context cannot be created.
    pub async fn request(&self) -> Result<APIRequestContext, ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        debug!("Creating API request context for browser context");

        // Build options from context settings
        let mut options = APIContextOptions::new();

        // Copy extra HTTP headers from context
        if !self.options.extra_http_headers.is_empty() {
            options = options.extra_http_headers(
                self.options.extra_http_headers.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
            );
        }

        // Copy HTTP credentials if set
        if let Some(ref creds) = self.options.http_credentials {
            options = options.http_credentials(
                crate::api::HttpCredentials::new(&creds.username, &creds.password)
            );
        }

        // Create API context
        let api = APIRequestContext::new(options)
            .await
            .map_err(|e| ContextError::Internal(e.to_string()))?;

        // Sync cookies from browser to API context
        let browser_cookies = self.cookies().await?;
        crate::api::cookies::sync_to_jar(&browser_cookies, api.cookie_jar());
        debug!("Synced {} browser cookies to API context", browser_cookies.len());

        Ok(api)
    }

    /// Sync cookies from API responses back to the browser context.
    ///
    /// Call this after making API requests that may have set cookies
    /// (e.g., login endpoints) to ensure the browser has the same cookies.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Login via API
    /// let api = context.request().await?;
    /// let response = api.post("https://api.example.com/login")
    ///     .json(&serde_json::json!({"user": "admin", "pass": "secret"}))
    ///     .send()
    ///     .await?;
    ///
    /// // Sync cookies back to browser (e.g., session cookies from Set-Cookie)
    /// context.sync_cookies_from_api(&api, "https://api.example.com").await?;
    ///
    /// // Now browser pages will have the session cookie
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if cookie syncing fails.
    pub async fn sync_cookies_from_api(
        &self,
        api: &APIRequestContext,
        url: &str,
    ) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        let cookies = crate::api::cookies::extract_from_jar(api.cookie_jar(), url);
        if !cookies.is_empty() {
            debug!("Syncing {} cookies from API to browser", cookies.len());
            self.add_cookies(cookies).await?;
        }

        Ok(())
    }
}
