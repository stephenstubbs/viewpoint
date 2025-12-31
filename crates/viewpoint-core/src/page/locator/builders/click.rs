//! Click builder for locator actions.

use tracing::{debug, instrument, trace};
use viewpoint_cdp::protocol::input::{DispatchMouseEventParams, MouseButton};

use super::super::Locator;
use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

/// Builder for click operations with configurable options.
///
/// Created via [`Locator::click`].
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
/// # page.goto("about:blank").goto().await.unwrap();
///
/// // Simple click - await directly
/// page.locator("button").click().await.ok();
///
/// // Force click without waiting for actionability
/// page.locator("button").click().force(true).await.ok();
///
/// // Click without waiting for navigation
/// page.locator("a").click().no_wait_after(true).await.ok();
/// # });
/// ```
#[derive(Debug)]
pub struct ClickBuilder<'l, 'a> {
    locator: &'l Locator<'a>,
    /// Position offset from element's top-left corner.
    position: Option<(f64, f64)>,
    /// Mouse button to use.
    button: MouseButton,
    /// Modifier keys to hold during the click.
    modifiers: i32,
    /// Whether to bypass actionability checks.
    force: bool,
    /// Click count (1 for single click, 2 for double click).
    click_count: i32,
    /// Whether to skip waiting for navigation after the action.
    no_wait_after: bool,
}

impl<'l, 'a> ClickBuilder<'l, 'a> {
    pub(crate) fn new(locator: &'l Locator<'a>) -> Self {
        Self {
            locator,
            position: None,
            button: MouseButton::Left,
            modifiers: 0,
            force: false,
            click_count: 1,
            no_wait_after: false,
        }
    }

    /// Set the position offset from the element's top-left corner.
    ///
    /// By default, clicks the center of the element.
    #[must_use]
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }

    /// Set the mouse button to use.
    #[must_use]
    pub fn button(mut self, button: MouseButton) -> Self {
        self.button = button;
        self
    }

    /// Set modifier keys to hold during the click.
    ///
    /// Use the `modifiers` constants from `viewpoint_cdp::protocol::input::modifiers`.
    #[must_use]
    pub fn modifiers(mut self, modifiers: i32) -> Self {
        self.modifiers = modifiers;
        self
    }

    /// Whether to bypass actionability checks.
    ///
    /// When `true`, the click will be performed immediately without waiting
    /// for the element to be visible, enabled, or stable.
    #[must_use]
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    /// Set the click count (internal use for double-click support).
    #[must_use]
    pub(crate) fn click_count(mut self, count: i32) -> Self {
        self.click_count = count;
        self
    }

    /// Whether to skip waiting for navigation after the click.
    ///
    /// By default, the click will wait for any triggered navigation to complete.
    /// Set to `true` to return immediately after the click is performed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Click a link but don't wait for navigation
    /// page.locator("a").click().no_wait_after(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn no_wait_after(mut self, no_wait_after: bool) -> Self {
        self.no_wait_after = no_wait_after;
        self
    }

    /// Execute the click operation.
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

        // Perform the click action
        self.perform_click().await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            match waiter.wait_for_navigation_if_triggered().await {
                Ok(navigated) => {
                    if navigated {
                        trace!("Navigation completed after click");
                    }
                }
                Err(e) => {
                    debug!(error = ?e, "Navigation wait failed after click");
                    return Err(LocatorError::WaitError(e));
                }
            }
        }

        Ok(())
    }

    /// Perform the actual click without navigation waiting.
    async fn perform_click(&self) -> Result<(), LocatorError> {
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

        debug!(x, y, button = ?self.button, modifiers = self.modifiers, click_count = self.click_count, "Clicking element");

        // Move to element
        let mut move_event = DispatchMouseEventParams::mouse_move(x, y);
        if self.modifiers != 0 {
            move_event.modifiers = Some(self.modifiers);
        }
        self.locator.dispatch_mouse_event(move_event).await?;

        // Mouse down
        let mut down_event = DispatchMouseEventParams::mouse_down(x, y, self.button);
        if self.modifiers != 0 {
            down_event.modifiers = Some(self.modifiers);
        }
        down_event.click_count = Some(self.click_count);
        self.locator.dispatch_mouse_event(down_event).await?;

        // Mouse up
        let mut up_event = DispatchMouseEventParams::mouse_up(x, y, self.button);
        if self.modifiers != 0 {
            up_event.modifiers = Some(self.modifiers);
        }
        up_event.click_count = Some(self.click_count);
        self.locator.dispatch_mouse_event(up_event).await?;

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for ClickBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
