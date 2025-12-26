//! Navigation types and builder.

use std::time::Duration;

use tracing::{debug, instrument};

use crate::error::NavigationError;
use crate::wait::DocumentLoadState;

use super::{NavigationResponse, Page, DEFAULT_NAVIGATION_TIMEOUT};

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
