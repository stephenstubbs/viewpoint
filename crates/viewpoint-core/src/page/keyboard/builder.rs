//! Builder for keyboard press operations.

use std::time::Duration;

use tracing::{debug, instrument, trace};

use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

use super::Keyboard;

/// Builder for keyboard press operations with configurable options.
///
/// Created via [`Keyboard::press`].
#[derive(Debug)]
pub struct KeyboardPressBuilder<'a> {
    keyboard: &'a Keyboard,
    key: String,
    delay: Option<Duration>,
    no_wait_after: bool,
}

impl<'a> KeyboardPressBuilder<'a> {
    pub(crate) fn new(keyboard: &'a Keyboard, key: &str) -> Self {
        Self {
            keyboard,
            key: key.to_string(),
            delay: None,
            no_wait_after: false,
        }
    }

    /// Set a delay between key down and key up.
    #[must_use]
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
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
    #[instrument(level = "debug", skip(self), fields(key = %self.key))]
    pub async fn send(self) -> Result<(), LocatorError> {
        // Set up navigation waiter before the action if needed
        let navigation_waiter = if self.no_wait_after {
            None
        } else {
            Some(NavigationWaiter::new(
                self.keyboard.connection().subscribe_events(),
                self.keyboard.session_id().to_string(),
                self.keyboard.frame_id().to_string(),
            ))
        };

        // Perform the press action
        self.keyboard.press_internal(&self.key, self.delay).await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            match waiter.wait_for_navigation_if_triggered().await {
                Ok(navigated) => {
                    if navigated {
                        trace!("Navigation completed after keyboard press");
                    }
                }
                Err(e) => {
                    debug!(error = ?e, "Navigation wait failed after keyboard press");
                    return Err(LocatorError::WaitError(e));
                }
            }
        }

        Ok(())
    }
}

impl<'a> std::future::IntoFuture for KeyboardPressBuilder<'a> {
    type Output = Result<(), LocatorError>;
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
