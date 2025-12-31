//! Fill builder for locator actions.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::DispatchKeyEventParams;

use super::super::Locator;
use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

/// Builder for fill operations with configurable options.
///
/// Created via [`Locator::fill`].
#[derive(Debug)]
pub struct FillBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    text: String,
    /// Whether to skip waiting for navigation after the action.
    no_wait_after: bool,
}

impl<'l, 'a> FillBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>, text: &str) -> Self {
        Self {
            locator,
            text: text.to_string(),
            no_wait_after: false,
        }
    }

    /// Whether to skip waiting for navigation after the fill.
    ///
    /// By default, the fill will wait for any triggered navigation to complete.
    /// Set to `true` to return immediately after the text is filled.
    #[must_use]
    pub fn no_wait_after(mut self, no_wait_after: bool) -> Self {
        self.no_wait_after = no_wait_after;
        self
    }

    /// Execute the fill operation.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.locator.selector))]
    pub async fn send(self) -> Result<(), LocatorError> {
        // Set up navigation waiter before the action if needed
        let navigation_waiter = if self.no_wait_after {
            None
        } else {
            Some(NavigationWaiter::new(
                self.locator.page.connection().subscribe_events(),
                self.locator.page.session_id().to_string(),
                self.locator.page.frame_id().to_string(),
            ))
        };

        // Perform the fill action
        self.perform_fill().await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            if let Err(e) = waiter.wait_for_navigation_if_triggered().await {
                debug!(error = ?e, "Navigation wait failed after fill");
                return Err(LocatorError::WaitError(e));
            }
        }

        Ok(())
    }

    /// Perform the actual fill without navigation waiting.
    async fn perform_fill(&self) -> Result<(), LocatorError> {
        self.locator.wait_for_actionable().await?;

        debug!(text = %self.text, "Filling element");

        // Focus the element
        self.locator.focus_element().await?;

        // Select all and delete (clear)
        self.locator
            .dispatch_key_event(DispatchKeyEventParams::key_down("a"))
            .await?;
        // Send Ctrl+A
        let mut select_all = DispatchKeyEventParams::key_down("a");
        select_all.modifiers = Some(viewpoint_cdp::protocol::input::modifiers::CTRL);
        self.locator.dispatch_key_event(select_all).await?;

        // Delete selected text
        self.locator
            .dispatch_key_event(DispatchKeyEventParams::key_down("Backspace"))
            .await?;

        // Insert the new text
        self.locator.insert_text(&self.text).await?;

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for FillBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
