//! Hover builder for locator actions.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::DispatchMouseEventParams;

use super::super::Locator;
use crate::error::LocatorError;

/// Builder for hover operations with configurable options.
///
/// Created via [`Locator::hover`].
#[derive(Debug)]
pub struct HoverBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    position: Option<(f64, f64)>,
    modifiers: i32,
    force: bool,
}

impl<'l, 'a> HoverBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>) -> Self {
        Self {
            locator,
            position: None,
            modifiers: 0,
            force: false,
        }
    }

    /// Set the position offset from the element's top-left corner.
    #[must_use]
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }

    /// Set modifier keys to hold during hover.
    #[must_use]
    pub fn modifiers(mut self, modifiers: i32) -> Self {
        self.modifiers = modifiers;
        self
    }

    /// Whether to bypass actionability checks.
    #[must_use]
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    /// Execute the hover operation.
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

        debug!(x, y, modifiers = self.modifiers, "Hovering over element");

        let mut move_event = DispatchMouseEventParams::mouse_move(x, y);
        if self.modifiers != 0 {
            move_event.modifiers = Some(self.modifiers);
        }
        self.locator.dispatch_mouse_event(move_event).await?;

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for HoverBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
