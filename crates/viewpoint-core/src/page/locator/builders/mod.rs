//! Builder types for locator actions.
//!
//! These builders provide a fluent API for configuring locator operations
//! like click, type, hover, and tap with various options.

use std::time::Duration;

use viewpoint_cdp::protocol::input::{DispatchKeyEventParams, DispatchMouseEventParams, MouseButton};
use tracing::{debug, instrument};

use super::Locator;
use crate::error::LocatorError;

// =============================================================================
// ClickBuilder
// =============================================================================

/// Builder for click operations with configurable options.
///
/// Created via [`Locator::click`].
///
/// # Example
///
/// ```ignore
/// // Simple click - await directly
/// page.locator("button").click().await?;
///
/// // Click with options
/// page.locator("button").click()
///     .position(10.0, 5.0)
///     .button(MouseButton::Right)
///     .modifiers(modifiers::SHIFT)
///     .send().await?;
///
/// // Force click without waiting for actionability
/// page.locator("button").click().force(true).await?;
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

    /// Execute the click operation.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.locator.selector))]
    pub async fn send(self) -> Result<(), LocatorError> {
        let (x, y) = if self.force {
            let info = self.locator.query_element_info().await?;
            if !info.found {
                return Err(LocatorError::NotFound(format!("{:?}", self.locator.selector)));
            }
            
            if let Some((offset_x, offset_y)) = self.position {
                (info.x.unwrap_or(0.0) + offset_x, info.y.unwrap_or(0.0) + offset_y)
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
                    info.x.expect("visible element has x") + info.width.expect("visible element has width") / 2.0,
                    info.y.expect("visible element has y") + info.height.expect("visible element has height") / 2.0,
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
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

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
            self.locator.dispatch_key_event(DispatchKeyEventParams::char(&char_str)).await?;
            
            if let Some(delay) = self.delay {
                tokio::time::sleep(delay).await;
            }
        }

        Ok(())
    }
}

impl<'l> std::future::IntoFuture for TypeBuilder<'l, '_> {
    type Output = Result<(), LocatorError>;
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

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
                return Err(LocatorError::NotFound(format!("{:?}", self.locator.selector)));
            }
            
            if let Some((offset_x, offset_y)) = self.position {
                (info.x.unwrap_or(0.0) + offset_x, info.y.unwrap_or(0.0) + offset_y)
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
                    info.x.expect("visible element has x") + info.width.expect("visible element has width") / 2.0,
                    info.y.expect("visible element has y") + info.height.expect("visible element has height") / 2.0,
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
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'l>>;

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
                return Err(LocatorError::NotFound(format!("{:?}", self.locator.selector)));
            }
            
            if let Some((offset_x, offset_y)) = self.position {
                (info.x.unwrap_or(0.0) + offset_x, info.y.unwrap_or(0.0) + offset_y)
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
                    info.x.expect("visible element has x") + info.width.expect("visible element has width") / 2.0,
                    info.y.expect("visible element has y") + info.height.expect("visible element has height") / 2.0,
                )
            }
        };

        debug!(x, y, modifiers = self.modifiers, "Tapping element");

        if self.modifiers != 0 {
            self.locator.page.touchscreen().tap_with_modifiers(x, y, self.modifiers).await
        } else {
            self.locator.page.touchscreen().tap(x, y).await
        }
    }
}
