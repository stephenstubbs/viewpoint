//! Press builder for locator actions.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::DispatchKeyEventParams;

use super::super::Locator;
use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

/// Builder for press operations with configurable options.
///
/// Created via [`Locator::press`].
#[derive(Debug)]
pub struct PressBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    key: String,
    /// Whether to skip waiting for navigation after the action.
    no_wait_after: bool,
}

impl<'l, 'a> PressBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>, key: &str) -> Self {
        Self {
            locator,
            key: key.to_string(),
            no_wait_after: false,
        }
    }

    /// Whether to skip waiting for navigation after the key press.
    ///
    /// By default, the press will wait for any triggered navigation to complete.
    /// Set to `true` to return immediately after the key is pressed.
    #[must_use]
    pub fn no_wait_after(mut self, no_wait_after: bool) -> Self {
        self.no_wait_after = no_wait_after;
        self
    }

    /// Execute the press operation.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.locator.selector, key = %self.key))]
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

        // Perform the press action
        self.perform_press().await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            if let Err(e) = waiter.wait_for_navigation_if_triggered().await {
                debug!(error = ?e, "Navigation wait failed after press");
                return Err(LocatorError::WaitError(e));
            }
        }

        Ok(())
    }

    /// Perform the actual key press without navigation waiting.
    async fn perform_press(&self) -> Result<(), LocatorError> {
        self.locator.wait_for_actionable().await?;

        debug!(key = %self.key, "Pressing key");

        // Focus the element
        self.locator.focus_element().await?;

        // Parse modifiers and key
        let parts: Vec<&str> = self.key.split('+').collect();
        let key_ref = self.key.as_str();
        let actual_key = *parts.last().unwrap_or(&key_ref);

        let mut modifiers = 0;
        for part in &parts[..parts.len().saturating_sub(1)] {
            match part.to_lowercase().as_str() {
                "control" | "ctrl" => {
                    modifiers |= viewpoint_cdp::protocol::input::modifiers::CTRL;
                }
                "alt" => modifiers |= viewpoint_cdp::protocol::input::modifiers::ALT,
                "shift" => modifiers |= viewpoint_cdp::protocol::input::modifiers::SHIFT,
                "meta" | "cmd" => modifiers |= viewpoint_cdp::protocol::input::modifiers::META,
                _ => {}
            }
        }

        // Key down
        let mut key_down = DispatchKeyEventParams::key_down(actual_key);
        if modifiers != 0 {
            key_down.modifiers = Some(modifiers);
        }
        self.locator.dispatch_key_event(key_down).await?;

        // Key up
        let mut key_up = DispatchKeyEventParams::key_up(actual_key);
        if modifiers != 0 {
            key_up.modifiers = Some(modifiers);
        }
        self.locator.dispatch_key_event(key_up).await?;

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for PressBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
