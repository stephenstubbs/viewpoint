//! Tap builder for locator actions.

use tracing::{debug, instrument};

use super::super::Locator;
use crate::error::LocatorError;

/// Builder for tap operations with configurable options.
///
/// Created via [`Locator::tap`].
#[derive(Debug)]
pub struct TapBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    position: Option<(f64, f64)>,
    force: bool,
    modifiers: i32,
}

impl<'l, 'a> TapBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>) -> Self {
        Self {
            locator,
            position: None,
            force: false,
            modifiers: 0,
        }
    }

    /// Set the position offset from the element's top-left corner.
    #[must_use]
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }

    /// Whether to bypass actionability checks.
    #[must_use]
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    /// Set modifier keys to hold during the tap.
    #[must_use]
    pub fn modifiers(mut self, modifiers: i32) -> Self {
        self.modifiers = modifiers;
        self
    }

    /// Execute the tap operation.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.locator.selector))]
    pub async fn send(self) -> Result<(), LocatorError> {
        let (x, y) = if self.force {
            let info = self.locator.query_element_info().await?;
            if !info.found {
                return Err(LocatorError::NotFound(format!(
                    "{:?}",
                    self.locator.selector
                )));
            }

            if let Some((offset_x, offset_y)) = self.position {
                (
                    info.x.unwrap_or(0.0) + offset_x,
                    info.y.unwrap_or(0.0) + offset_y,
                )
            } else {
                (
                    info.x.unwrap_or(0.0) + info.width.unwrap_or(0.0) / 2.0,
                    info.y.unwrap_or(0.0) + info.height.unwrap_or(0.0) / 2.0,
                )
            }
        } else {
            let info = self.locator.wait_for_actionable().await?;

            if let Some((offset_x, offset_y)) = self.position {
                (
                    info.x.expect("visible element has x") + offset_x,
                    info.y.expect("visible element has y") + offset_y,
                )
            } else {
                (
                    info.x.expect("visible element has x")
                        + info.width.expect("visible element has width") / 2.0,
                    info.y.expect("visible element has y")
                        + info.height.expect("visible element has height") / 2.0,
                )
            }
        };

        debug!(x, y, modifiers = self.modifiers, "Tapping element");

        if self.modifiers != 0 {
            self.locator
                .page
                .touchscreen()
                .tap_with_modifiers(x, y, self.modifiers)
                .await
        } else {
            self.locator.page.touchscreen().tap(x, y).await
        }
    }
}
