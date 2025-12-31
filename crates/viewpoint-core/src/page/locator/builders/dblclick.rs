//! Double-click builder for locator actions.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::{DispatchMouseEventParams, MouseButton};

use super::super::Locator;
use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

/// Builder for double-click operations with configurable options.
///
/// Created via [`Locator::dblclick`].
#[derive(Debug)]
pub struct DblclickBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    /// Position offset from element's top-left corner.
    position: Option<(f64, f64)>,
    /// Modifier keys to hold during the click.
    modifiers: i32,
    /// Whether to bypass actionability checks.
    force: bool,
    /// Whether to skip waiting for navigation after the action.
    no_wait_after: bool,
}

impl<'l, 'a> DblclickBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>) -> Self {
        Self {
            locator,
            position: None,
            modifiers: 0,
            force: false,
            no_wait_after: false,
        }
    }

    /// Set the position offset from the element's top-left corner.
    #[must_use]
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }

    /// Set modifier keys to hold during the click.
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

    /// Whether to skip waiting for navigation after the double-click.
    #[must_use]
    pub fn no_wait_after(mut self, no_wait_after: bool) -> Self {
        self.no_wait_after = no_wait_after;
        self
    }

    /// Execute the double-click operation.
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

        // Perform the double-click action
        self.perform_dblclick().await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            if let Err(e) = waiter.wait_for_navigation_if_triggered().await {
                debug!(error = ?e, "Navigation wait failed after dblclick");
                return Err(LocatorError::WaitError(e));
            }
        }

        Ok(())
    }

    /// Perform the actual double-click without navigation waiting.
    async fn perform_dblclick(&self) -> Result<(), LocatorError> {
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

        debug!(x, y, modifiers = self.modifiers, "Double-clicking element");

        // First click
        let mut move_event = DispatchMouseEventParams::mouse_move(x, y);
        if self.modifiers != 0 {
            move_event.modifiers = Some(self.modifiers);
        }
        self.locator.dispatch_mouse_event(move_event).await?;

        let mut down1 = DispatchMouseEventParams::mouse_down(x, y, MouseButton::Left);
        if self.modifiers != 0 {
            down1.modifiers = Some(self.modifiers);
        }
        self.locator.dispatch_mouse_event(down1).await?;

        let mut up1 = DispatchMouseEventParams::mouse_up(x, y, MouseButton::Left);
        if self.modifiers != 0 {
            up1.modifiers = Some(self.modifiers);
        }
        self.locator.dispatch_mouse_event(up1).await?;

        // Second click
        let mut down2 = DispatchMouseEventParams::mouse_down(x, y, MouseButton::Left);
        down2.click_count = Some(2);
        if self.modifiers != 0 {
            down2.modifiers = Some(self.modifiers);
        }
        self.locator.dispatch_mouse_event(down2).await?;

        let mut up2 = DispatchMouseEventParams::mouse_up(x, y, MouseButton::Left);
        up2.click_count = Some(2);
        if self.modifiers != 0 {
            up2.modifiers = Some(self.modifiers);
        }
        self.locator.dispatch_mouse_event(up2).await?;

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for DblclickBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
