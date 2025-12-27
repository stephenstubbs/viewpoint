//! Navigation types and builder.

use std::collections::HashMap;
use std::time::Duration;

use tracing::{debug, info, instrument};

use crate::error::NavigationError;
use crate::wait::DocumentLoadState;

use super::{Page, DEFAULT_NAVIGATION_TIMEOUT};

/// Response from a navigation.
#[derive(Debug, Clone)]
pub struct NavigationResponse {
    /// The URL that was navigated to (final URL after redirects).
    url: String,
    /// The frame ID that navigated.
    frame_id: String,
    /// HTTP status code of the response (e.g., 200, 404, 302).
    status: Option<u16>,
    /// HTTP response headers.
    headers: Option<HashMap<String, String>>,
}

impl NavigationResponse {
    /// Create a new navigation response.
    pub(crate) fn new(url: String, frame_id: String) -> Self {
        Self {
            url,
            frame_id,
            status: None,
            headers: None,
        }
    }

    /// Create a navigation response with response data.
    pub(crate) fn with_response(
        url: String,
        frame_id: String,
        status: u16,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            url,
            frame_id,
            status: Some(status),
            headers: Some(headers),
        }
    }

    /// Get the final URL after any redirects.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the frame ID that navigated.
    pub fn frame_id(&self) -> &str {
        &self.frame_id
    }

    /// Get the HTTP status code of the final response.
    ///
    /// Returns `None` if status was not captured (e.g., for `about:blank`).
    pub fn status(&self) -> Option<u16> {
        self.status
    }

    /// Get the HTTP response headers.
    ///
    /// Returns `None` if headers were not captured.
    pub fn headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }

    /// Check if the navigation resulted in an OK response (2xx status).
    pub fn ok(&self) -> bool {
        self.status.map_or(true, |s| (200..300).contains(&s))
    }
}

/// Builder for configuring page navigation.
#[derive(Debug)]
pub struct GotoBuilder<'a> {
    page: &'a Page,
    url: String,
    wait_until: DocumentLoadState,
    timeout: Duration,
    referer: Option<String>,
}

impl<'a> GotoBuilder<'a> {
    /// Create a new navigation builder.
    pub(crate) fn new(page: &'a Page, url: String) -> Self {
        Self {
            page,
            url,
            wait_until: DocumentLoadState::default(),
            timeout: DEFAULT_NAVIGATION_TIMEOUT,
            referer: None,
        }
    }

    /// Set the load state to wait for.
    ///
    /// Default is `DocumentLoadState::Load`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::DocumentLoadState;
    ///
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Wait only for DOM content loaded (faster)
    /// page.goto("https://example.com")
    ///     .wait_until(DocumentLoadState::DomContentLoaded)
    ///     .goto()
    ///     .await?;
    ///
    /// // Wait for network idle (slower but more complete)
    /// page.goto("https://example.com")
    ///     .wait_until(DocumentLoadState::NetworkIdle)
    ///     .goto()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn wait_until(mut self, state: DocumentLoadState) -> Self {
        self.wait_until = state;
        self
    }

    /// Set the navigation timeout.
    ///
    /// Default is 30 seconds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.goto("https://slow-site.com")
    ///     .timeout(Duration::from_secs(60))
    ///     .goto()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the referer header for the navigation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.goto("https://example.com")
    ///     .referer("https://google.com")
    ///     .goto()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn referer(mut self, referer: impl Into<String>) -> Self {
        self.referer = Some(referer.into());
        self
    }

    /// Execute the navigation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - Navigation fails (network error, SSL error, etc.)
    /// - The wait times out
    #[instrument(level = "debug", skip(self), fields(url = %self.url, wait_until = ?self.wait_until, timeout_ms = self.timeout.as_millis(), has_referer = self.referer.is_some()))]
    pub async fn goto(self) -> Result<NavigationResponse, NavigationError> {
        debug!("Executing navigation via GotoBuilder");
        self.page
            .navigate_internal(&self.url, self.wait_until, self.timeout, self.referer.as_deref())
            .await
    }
}

// =============================================================================
// Navigation History Methods (impl extension for Page)
// =============================================================================

impl Page {
    /// Navigate back in history.
    ///
    /// Returns `None` if there is no previous page in history.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// if page.go_back().await?.is_some() {
    ///     println!("Navigated back");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "info", skip(self))]
    pub async fn go_back(&self) -> Result<Option<NavigationResponse>, NavigationError> {
        if self.closed {
            return Err(NavigationError::Cancelled);
        }

        // Check if we can go back
        let history: viewpoint_cdp::protocol::page::GetNavigationHistoryResult = self
            .connection
            .send_command("Page.getNavigationHistory", None::<()>, Some(&self.session_id))
            .await?;

        if history.current_index <= 0 {
            debug!("No previous page in history");
            return Ok(None);
        }

        // Navigate to the previous entry
        let previous_entry = &history.entries[history.current_index as usize - 1];
        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.navigateToHistoryEntry",
                Some(viewpoint_cdp::protocol::page::NavigateToHistoryEntryParams {
                    entry_id: previous_entry.id,
                }),
                Some(&self.session_id),
            )
            .await?;

        info!("Navigated back to {}", previous_entry.url);
        Ok(Some(NavigationResponse::new(previous_entry.url.clone(), self.frame_id.clone())))
    }

    /// Navigate forward in history.
    ///
    /// Returns `None` if there is no next page in history.
    #[instrument(level = "info", skip(self))]
    pub async fn go_forward(&self) -> Result<Option<NavigationResponse>, NavigationError> {
        if self.closed {
            return Err(NavigationError::Cancelled);
        }

        // Check if we can go forward
        let history: viewpoint_cdp::protocol::page::GetNavigationHistoryResult = self
            .connection
            .send_command("Page.getNavigationHistory", None::<()>, Some(&self.session_id))
            .await?;

        let next_index = history.current_index as usize + 1;
        if next_index >= history.entries.len() {
            debug!("No next page in history");
            return Ok(None);
        }

        // Navigate to the next entry
        let next_entry = &history.entries[next_index];
        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.navigateToHistoryEntry",
                Some(viewpoint_cdp::protocol::page::NavigateToHistoryEntryParams {
                    entry_id: next_entry.id,
                }),
                Some(&self.session_id),
            )
            .await?;

        info!("Navigated forward to {}", next_entry.url);
        Ok(Some(NavigationResponse::new(next_entry.url.clone(), self.frame_id.clone())))
    }

    /// Reload the current page.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.reload().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "info", skip(self))]
    pub async fn reload(&self) -> Result<NavigationResponse, NavigationError> {
        if self.closed {
            return Err(NavigationError::Cancelled);
        }

        info!("Reloading page");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.reload",
                Some(viewpoint_cdp::protocol::page::ReloadParams::default()),
                Some(&self.session_id),
            )
            .await?;

        // Get current URL
        let url = self.url().await.unwrap_or_else(|_| String::new());

        info!("Page reloaded");
        Ok(NavigationResponse::new(url, self.frame_id.clone()))
    }
}
