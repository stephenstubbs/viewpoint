//! Builder types for locator actions.
//!
//! These builders provide a fluent API for configuring locator operations
//! like click, type, hover, and tap with various options.

use std::time::Duration;

use tracing::{debug, instrument, trace};
use viewpoint_cdp::protocol::input::{
    DispatchKeyEventParams, DispatchMouseEventParams, MouseButton,
};

use super::Locator;
use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

// =============================================================================
// ClickBuilder
// =============================================================================

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

// =============================================================================
// TypeBuilder
// =============================================================================

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

// =============================================================================
// HoverBuilder
// =============================================================================

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

// =============================================================================
// TapBuilder
// =============================================================================

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

// =============================================================================
// DblclickBuilder
// =============================================================================

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

// =============================================================================
// PressBuilder
// =============================================================================

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

// =============================================================================
// FillBuilder
// =============================================================================

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

// =============================================================================
// CheckBuilder
// =============================================================================

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

// =============================================================================
// SelectOptionBuilder
// =============================================================================

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
