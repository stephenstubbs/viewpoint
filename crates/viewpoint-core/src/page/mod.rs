//! Page management and navigation.

pub mod locator;
mod navigation;

use std::sync::Arc;
use std::time::Duration;

use viewpoint_cdp::protocol::page::{NavigateParams, NavigateResult};
use viewpoint_cdp::protocol::target::CloseTargetParams;
use viewpoint_cdp::CdpConnection;
use tracing::{debug, info, instrument, trace, warn};

use crate::error::{NavigationError, PageError};
use crate::wait::{DocumentLoadState, LoadStateWaiter};

pub use locator::{AriaRole, Locator, LocatorOptions, Selector, TextOptions};
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
    /// use viewpoint_core::Page;
    /// use viewpoint_core::DocumentLoadState;
    /// use std::time::Duration;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
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

    /// Get the current page URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the evaluation fails.
    pub async fn url(&self) -> Result<String, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: "window.location.href".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get URL".to_string()))
    }

    /// Get the current page title.
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the evaluation fails.
    pub async fn title(&self) -> Result<String, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: "document.title".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get title".to_string()))
    }

    // =========================================================================
    // Locator Methods
    // =========================================================================

    /// Create a locator for elements matching a CSS selector.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = page.locator("button.submit");
    /// let items = page.locator(".list > .item");
    /// ```
    pub fn locator(&self, selector: impl Into<String>) -> Locator<'_> {
        Locator::new(self, Selector::Css(selector.into()))
    }

    /// Create a locator for elements containing the specified text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let heading = page.get_by_text("Welcome");
    /// let exact = page.get_by_text_exact("Welcome to our site");
    /// ```
    pub fn get_by_text(&self, text: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::Text {
                text: text.into(),
                exact: false,
            },
        )
    }

    /// Create a locator for elements with exact text content.
    pub fn get_by_text_exact(&self, text: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::Text {
                text: text.into(),
                exact: true,
            },
        )
    }

    /// Create a locator for elements with the specified ARIA role.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let buttons = page.get_by_role(AriaRole::Button);
    /// let submit = page.get_by_role(AriaRole::Button).with_name("Submit");
    /// ```
    pub fn get_by_role(&self, role: AriaRole) -> RoleLocatorBuilder<'_> {
        RoleLocatorBuilder::new(self, role)
    }

    /// Create a locator for elements with the specified test ID.
    ///
    /// By default, looks for `data-testid` attribute.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = page.get_by_test_id("submit-button");
    /// ```
    pub fn get_by_test_id(&self, test_id: impl Into<String>) -> Locator<'_> {
        Locator::new(self, Selector::TestId(test_id.into()))
    }

    /// Create a locator for form controls by their associated label text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let email = page.get_by_label("Email address");
    /// ```
    pub fn get_by_label(&self, label: impl Into<String>) -> Locator<'_> {
        Locator::new(self, Selector::Label(label.into()))
    }

    /// Create a locator for inputs by their placeholder text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let search = page.get_by_placeholder("Search...");
    /// ```
    pub fn get_by_placeholder(&self, placeholder: impl Into<String>) -> Locator<'_> {
        Locator::new(self, Selector::Placeholder(placeholder.into()))
    }
}

/// Builder for role-based locators.
#[derive(Debug)]
pub struct RoleLocatorBuilder<'a> {
    page: &'a Page,
    role: AriaRole,
    name: Option<String>,
}

impl<'a> RoleLocatorBuilder<'a> {
    fn new(page: &'a Page, role: AriaRole) -> Self {
        Self {
            page,
            role,
            name: None,
        }
    }

    /// Filter by accessible name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Build the locator.
    pub fn build(self) -> Locator<'a> {
        Locator::new(
            self.page,
            Selector::Role {
                role: self.role,
                name: self.name,
            },
        )
    }
}

impl<'a> From<RoleLocatorBuilder<'a>> for Locator<'a> {
    fn from(builder: RoleLocatorBuilder<'a>) -> Self {
        builder.build()
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
