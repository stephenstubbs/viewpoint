//! Check builder for locator actions.

use tracing::{debug, instrument};

use super::super::Locator;
use super::click::ClickBuilder;
use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

/// Builder for check/uncheck operations with configurable options.
///
/// Created via [`Locator::check`] or [`Locator::uncheck`].
#[derive(Debug)]
pub struct CheckBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    /// Whether to check (true) or uncheck (false).
    check: bool,
    /// Whether to bypass actionability checks.
    force: bool,
    /// Whether to skip waiting for navigation after the action.
    no_wait_after: bool,
}

impl<'l, 'a> CheckBuilder<'l, 'a> {
    pub(crate) fn new_check(locator: &'l Locator<'a>) -> Self {
        Self {
            locator,
            check: true,
            force: false,
            no_wait_after: false,
        }
    }

    pub(crate) fn new_uncheck(locator: &'l Locator<'a>) -> Self {
        Self {
            locator,
            check: false,
            force: false,
            no_wait_after: false,
        }
    }

    /// Whether to bypass actionability checks.
    #[must_use]
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    /// Whether to skip waiting for navigation after the check/uncheck.
    #[must_use]
    pub fn no_wait_after(mut self, no_wait_after: bool) -> Self {
        self.no_wait_after = no_wait_after;
        self
    }

    /// Execute the check/uncheck operation.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.locator.selector, check = self.check))]
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

        // Perform the check/uncheck action
        self.perform_check().await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            if let Err(e) = waiter.wait_for_navigation_if_triggered().await {
                debug!(error = ?e, "Navigation wait failed after check");
                return Err(LocatorError::WaitError(e));
            }
        }

        Ok(())
    }

    /// Perform the actual check/uncheck without navigation waiting.
    async fn perform_check(&self) -> Result<(), LocatorError> {
        let is_checked = self.locator.is_checked().await?;

        if self.check {
            // Want to check
            if is_checked {
                debug!("Element already checked");
            } else {
                debug!("Checking element");
                // Use the click builder without auto-wait (we handle it ourselves)
                ClickBuilder::new(self.locator)
                    .force(self.force)
                    .no_wait_after(true)
                    .send()
                    .await?;
            }
        } else {
            // Want to uncheck
            if is_checked {
                debug!("Unchecking element");
                ClickBuilder::new(self.locator)
                    .force(self.force)
                    .no_wait_after(true)
                    .send()
                    .await?;
            } else {
                debug!("Element already unchecked");
            }
        }

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for CheckBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
