//! Type builder for locator actions.

use std::time::Duration;

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::DispatchKeyEventParams;

use super::super::Locator;
use crate::error::LocatorError;

/// Builder for type operations with configurable options.
///
/// Created via [`Locator::type_text`].
#[derive(Debug)]
pub struct TypeBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    text: String,
    delay: Option<Duration>,
}

impl<'l, 'a> TypeBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>, text: &str) -> Self {
        Self {
            locator,
            text: text.to_string(),
            delay: None,
        }
    }

    /// Set the delay between characters.
    #[must_use]
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Execute the type operation.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.locator.selector))]
    pub async fn send(self) -> Result<(), LocatorError> {
        self.locator.wait_for_actionable().await?;

        debug!(text = %self.text, delay = ?self.delay, "Typing text");

        self.locator.focus_element().await?;

        for ch in self.text.chars() {
            let char_str = ch.to_string();
            self.locator
                .dispatch_key_event(DispatchKeyEventParams::char(&char_str))
                .await?;

            if let Some(delay) = self.delay {
                tokio::time::sleep(delay).await;
            }
        }

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for TypeBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
