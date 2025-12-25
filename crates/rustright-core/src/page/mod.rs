//! Page management and navigation.

mod navigation;

use std::sync::Arc;
use std::time::Duration;

use rustright_cdp::protocol::page::{NavigateParams, NavigateResult};
use rustright_cdp::protocol::target::CloseTargetParams;
use rustright_cdp::CdpConnection;
use tracing::{debug, info, instrument, trace, warn};

use crate::error::{NavigationError, PageError};
use crate::wait::{DocumentLoadState, LoadStateWaiter};

pub use navigation::GotoBuilder;

/// Default navigation timeout.
const DEFAULT_NAVIGATION_TIMEOUT: Duration = Duration::from_secs(30);

/// A browser page (tab).
#[derive(Debug)]
pub struct Page {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Target ID.
    target_id: String,
    /// Session ID for this page.
    session_id: String,
    /// Main frame ID.
    frame_id: String,
    /// Whether the page has been closed.
    closed: bool,
}

impl Page {
    /// Create a new page.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        target_id: String,
        session_id: String,
        frame_id: String,
    ) -> Self {
        Self {
            connection,
            target_id,
            session_id,
            frame_id,
            closed: false,
        }
    }

    /// Navigate to a URL.
    ///
    /// Returns a builder for configuring navigation options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rustright_core::Page;
    /// use rustright_core::DocumentLoadState;
    /// use std::time::Duration;
    ///
    /// # async fn example(page: Page) -> Result<(), rustright_core::CoreError> {
    /// // Simple navigation
    /// page.goto("https://example.com").goto().await?;
    ///
    /// // Navigation with options
    /// page.goto("https://example.com")
    ///     .wait_until(DocumentLoadState::DomContentLoaded)
    ///     .timeout(Duration::from_secs(10))
    ///     .goto()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn goto(&self, url: impl Into<String>) -> GotoBuilder<'_> {
        GotoBuilder::new(self, url.into())
    }

    /// Navigate to a URL and wait for the specified load state.
    ///
    /// This is a convenience method that calls `goto(url).goto().await`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - Navigation fails
    /// - The wait times out
    pub async fn goto_url(&self, url: &str) -> Result<NavigationResponse, NavigationError> {
        self.goto(url).goto().await
    }

    /// Navigate to a URL with the given options.
    #[instrument(level = "info", skip(self), fields(target_id = %self.target_id, url = %url, wait_until = ?wait_until, timeout_ms = timeout.as_millis()))]
    pub(crate) async fn navigate_internal(
        &self,
        url: &str,
        wait_until: DocumentLoadState,
        timeout: Duration,
        referer: Option<&str>,
    ) -> Result<NavigationResponse, NavigationError> {
        if self.closed {
            warn!("Attempted navigation on closed page");
            return Err(NavigationError::Cancelled);
        }

        info!("Starting navigation");

        // Create a load state waiter
        let event_rx = self.connection.subscribe_events();
        let mut waiter = LoadStateWaiter::new(
            event_rx,
            self.session_id.clone(),
            self.frame_id.clone(),
        );
        trace!("Created load state waiter");

        // Send the navigation command
        debug!("Sending Page.navigate command");
        let result: NavigateResult = self
            .connection
            .send_command(
                "Page.navigate",
                Some(NavigateParams {
                    url: url.to_string(),
                    referrer: referer.map(ToString::to_string),
                    transition_type: None,
                    frame_id: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        debug!(frame_id = %result.frame_id, loader_id = ?result.loader_id, "Page.navigate completed");

        // Check for navigation errors
        if let Some(error_text) = result.error_text {
            warn!(error = %error_text, "Navigation failed with error");
            return Err(NavigationError::NetworkError(error_text));
        }

        // Mark commit as received
        trace!("Setting commit received");
        waiter.set_commit_received().await;

        // Wait for the target load state
        debug!(wait_until = ?wait_until, "Waiting for load state");
        waiter
            .wait_for_load_state_with_timeout(wait_until, timeout)
            .await?;

        info!(frame_id = %result.frame_id, "Navigation completed successfully");

        Ok(NavigationResponse {
            url: url.to_string(),
            frame_id: result.frame_id,
        })
    }

    /// Close this page.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    #[instrument(level = "info", skip(self), fields(target_id = %self.target_id))]
    pub async fn close(&mut self) -> Result<(), PageError> {
        if self.closed {
            debug!("Page already closed");
            return Ok(());
        }

        info!("Closing page");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Target.closeTarget",
                Some(CloseTargetParams {
                    target_id: self.target_id.clone(),
                }),
                None,
            )
            .await?;

        self.closed = true;
        info!("Page closed");
        Ok(())
    }

    /// Get the target ID.
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Get the session ID.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the main frame ID.
    pub fn frame_id(&self) -> &str {
        &self.frame_id
    }

    /// Check if this page has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get a reference to the CDP connection.
    pub fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }
}

/// Response from a navigation.
#[derive(Debug, Clone)]
pub struct NavigationResponse {
    /// The URL that was navigated to.
    pub url: String,
    /// The frame ID that navigated.
    pub frame_id: String,
}
