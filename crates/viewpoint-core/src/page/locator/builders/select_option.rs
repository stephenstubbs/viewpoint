//! Select option builder for locator actions.

use tracing::{debug, instrument};

use super::super::Locator;
use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

/// Builder for select option operations with configurable options.
///
/// Created via [`Locator::select_option`].
#[derive(Debug)]
pub struct SelectOptionBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    /// Single option to select.
    option: Option<String>,
    /// Multiple options to select.
    options: Option<Vec<String>>,
    /// Whether to skip waiting for navigation after the action.
    no_wait_after: bool,
}

impl<'l, 'a> SelectOptionBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>) -> Self {
        Self {
            locator,
            option: None,
            options: None,
            no_wait_after: false,
        }
    }

    /// Set a single option to select by value or label.
    #[must_use]
    pub fn value(mut self, option: impl Into<String>) -> Self {
        self.option = Some(option.into());
        self
    }

    /// Set a single option to select by label text.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.option = Some(label.into());
        self
    }

    /// Set multiple options to select (for multi-select elements).
    #[must_use]
    pub fn values(mut self, options: &[&str]) -> Self {
        self.options = Some(options.iter().map(|s| (*s).to_string()).collect());
        self
    }

    /// Whether to skip waiting for navigation after the selection.
    #[must_use]
    pub fn no_wait_after(mut self, no_wait_after: bool) -> Self {
        self.no_wait_after = no_wait_after;
        self
    }

    /// Execute the select operation.
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

        // Perform the select action
        self.perform_select().await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            if let Err(e) = waiter.wait_for_navigation_if_triggered().await {
                debug!(error = ?e, "Navigation wait failed after select_option");
                return Err(LocatorError::WaitError(e));
            }
        }

        Ok(())
    }

    /// Perform the actual select without navigation waiting.
    async fn perform_select(&self) -> Result<(), LocatorError> {
        self.locator.wait_for_actionable().await?;

        if let Some(ref options) = self.options {
            debug!(?options, "Selecting multiple options");
            let options_refs: Vec<&str> = options.iter().map(String::as_str).collect();
            self.locator.select_options_internal(&options_refs).await
        } else if let Some(ref option) = self.option {
            debug!(option, "Selecting option");
            self.locator.select_option_internal(option).await
        } else {
            Err(LocatorError::EvaluationError(
                "No option specified for select_option".to_string(),
            ))
        }
    }
}

impl<'l> std::future::IntoFuture for SelectOptionBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
