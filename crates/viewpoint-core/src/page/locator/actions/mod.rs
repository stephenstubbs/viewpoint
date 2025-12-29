//! Locator actions for element interaction.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::{
    DispatchKeyEventParams, DispatchMouseEventParams, MouseButton,
};

use super::Locator;
use super::builders::{ClickBuilder, HoverBuilder, TapBuilder, TypeBuilder};
use crate::error::LocatorError;

impl<'a> Locator<'a> {
    /// Click the element.
    ///
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for a simple click.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use viewpoint_cdp::protocol::input::{MouseButton, modifiers};
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
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
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Options
    ///
    /// - [`ClickBuilder::position`] - Click at offset from element's top-left corner
    /// - [`ClickBuilder::button`] - Use a different mouse button (right, middle)
    /// - [`ClickBuilder::modifiers`] - Hold modifier keys (Shift, Ctrl, Alt)
    /// - [`ClickBuilder::force`] - Skip actionability checks
    pub fn click(&self) -> ClickBuilder<'_, 'a> {
        ClickBuilder::new(self)
    }

    /// Double-click the element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be clicked.
    ///
    /// # Panics
    ///
    /// Panics if a visible element lacks bounding box coordinates. This should
    /// never occur as `wait_for_actionable` ensures visibility before returning.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn dblclick(&self) -> Result<(), LocatorError> {
        let info = self.wait_for_actionable().await?;

        let x = info.x.expect("visible element has x")
            + info.width.expect("visible element has width") / 2.0;
        let y = info.y.expect("visible element has y")
            + info.height.expect("visible element has height") / 2.0;

        debug!(x, y, "Double-clicking element");

        // First click
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_move(x, y))
            .await?;
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_down(
            x,
            y,
            MouseButton::Left,
        ))
        .await?;
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_up(x, y, MouseButton::Left))
            .await?;

        // Second click
        let mut down = DispatchMouseEventParams::mouse_down(x, y, MouseButton::Left);
        down.click_count = Some(2);
        self.dispatch_mouse_event(down).await?;

        let mut up = DispatchMouseEventParams::mouse_up(x, y, MouseButton::Left);
        up.click_count = Some(2);
        self.dispatch_mouse_event(up).await?;

        Ok(())
    }

    /// Fill the element with text (clears existing content first).
    ///
    /// This is for input and textarea elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be focused or text cannot be inserted.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn fill(&self, text: &str) -> Result<(), LocatorError> {
        let _info = self.wait_for_actionable().await?;

        debug!(text, "Filling element");

        // Focus the element
        self.focus_element().await?;

        // Select all and delete (clear)
        self.dispatch_key_event(DispatchKeyEventParams::key_down("a"))
            .await?;
        // Send Ctrl+A
        let mut select_all = DispatchKeyEventParams::key_down("a");
        select_all.modifiers = Some(viewpoint_cdp::protocol::input::modifiers::CTRL);
        self.dispatch_key_event(select_all).await?;

        // Delete selected text
        self.dispatch_key_event(DispatchKeyEventParams::key_down("Backspace"))
            .await?;

        // Insert the new text
        self.insert_text(text).await?;

        Ok(())
    }

    /// Type text character by character.
    ///
    /// Unlike `fill`, this types each character with keydown/keyup events.
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for simple typing.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple type - await directly
    /// page.locator("input").type_text("hello").await?;
    ///
    /// // Type with delay between characters
    /// page.locator("input").type_text("hello")
    ///     .delay(Duration::from_millis(100))
    ///     .send().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Options
    ///
    /// - [`TypeBuilder::delay`] - Add delay between character keystrokes
    pub fn type_text(&self, text: &str) -> TypeBuilder<'_, 'a> {
        TypeBuilder::new(self, text)
    }

    /// Press a key or key combination.
    ///
    /// Examples: "Enter", "Backspace", "Control+a", "Shift+Tab"
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be focused or the key cannot be pressed.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn press(&self, key: &str) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(key, "Pressing key");

        // Focus the element
        self.focus_element().await?;

        // Parse modifiers and key
        let parts: Vec<&str> = key.split('+').collect();
        let actual_key = parts.last().unwrap_or(&key);

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
        self.dispatch_key_event(key_down).await?;

        // Key up
        let mut key_up = DispatchKeyEventParams::key_up(actual_key);
        if modifiers != 0 {
            key_up.modifiers = Some(modifiers);
        }
        self.dispatch_key_event(key_up).await?;

        Ok(())
    }

    /// Hover over the element.
    ///
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for a simple hover.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple hover - await directly
    /// page.locator("button").hover().await?;
    ///
    /// // Hover with position offset
    /// page.locator("button").hover()
    ///     .position(10.0, 5.0)
    ///     .send().await?;
    ///
    /// // Force hover without waiting for actionability
    /// page.locator("button").hover().force(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Options
    ///
    /// - [`HoverBuilder::position`] - Hover at offset from element's top-left corner
    /// - [`HoverBuilder::modifiers`] - Hold modifier keys during hover
    /// - [`HoverBuilder::force`] - Skip actionability checks
    pub fn hover(&self) -> HoverBuilder<'_, 'a> {
        HoverBuilder::new(self)
    }

    /// Focus the element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or focused.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn focus(&self) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!("Focusing element");
        self.focus_element().await?;

        Ok(())
    }

    /// Clear the element's content.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be cleared.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn clear(&self) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!("Clearing element");

        // Focus and select all, then delete
        self.focus_element().await?;

        let mut select_all = DispatchKeyEventParams::key_down("a");
        select_all.modifiers = Some(viewpoint_cdp::protocol::input::modifiers::CTRL);
        self.dispatch_key_event(select_all).await?;

        self.dispatch_key_event(DispatchKeyEventParams::key_down("Backspace"))
            .await?;

        Ok(())
    }

    /// Check a checkbox or radio button.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be checked.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn check(&self) -> Result<(), LocatorError> {
        let is_checked = self.is_checked().await?;

        if is_checked {
            debug!("Element already checked");
        } else {
            debug!("Checking element");
            self.click().await?;
        }

        Ok(())
    }

    /// Uncheck a checkbox.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be unchecked.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn uncheck(&self) -> Result<(), LocatorError> {
        let is_checked = self.is_checked().await?;

        if is_checked {
            debug!("Unchecking element");
            self.click().await?;
        } else {
            debug!("Element already unchecked");
        }

        Ok(())
    }

    /// Tap on the element (touch event).
    ///
    /// Requires touch to be enabled via `page.touchscreen().enable()`.
    ///
    /// Returns a builder to configure tap options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use viewpoint_cdp::protocol::input::modifiers;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple tap
    /// page.locator("button").tap().send().await?;
    ///
    /// // Tap with position offset
    /// page.locator("button").tap().position(10.0, 5.0).send().await?;
    ///
    /// // Tap with modifiers
    /// page.locator("button").tap().modifiers(modifiers::SHIFT).send().await?;
    ///
    /// // Force tap without waiting for actionability
    /// page.locator("button").tap().force(true).send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tap(&self) -> TapBuilder<'_, 'a> {
        TapBuilder::new(self)
    }

    /// Drag this element to another locator.
    ///
    /// # Arguments
    ///
    /// * `target` - The target locator to drag to.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let source = page.locator("#draggable");
    /// let target = page.locator("#droppable");
    /// source.drag_to(&target).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self, target), fields(selector = ?self.selector))]
    pub async fn drag_to(&self, target: &Locator<'_>) -> Result<(), LocatorError> {
        self.drag_to_with_options(target, None, None, 1).await
    }

    /// Drag this element to another locator with options.
    ///
    /// # Arguments
    ///
    /// * `target` - The target locator to drag to.
    /// * `source_position` - Optional offset from source element's top-left corner.
    /// * `target_position` - Optional offset from target element's top-left corner.
    /// * `steps` - Number of intermediate steps for smooth dragging.
    #[instrument(level = "debug", skip(self, target))]
    pub async fn drag_to_with_options(
        &self,
        target: &Locator<'_>,
        source_position: Option<(f64, f64)>,
        target_position: Option<(f64, f64)>,
        steps: u32,
    ) -> Result<(), LocatorError> {
        // Get source element info
        let source_info = self.wait_for_actionable().await?;
        let (source_x, source_y) = if let Some((ox, oy)) = source_position {
            (
                source_info.x.expect("x") + ox,
                source_info.y.expect("y") + oy,
            )
        } else {
            (
                source_info.x.expect("x") + source_info.width.expect("width") / 2.0,
                source_info.y.expect("y") + source_info.height.expect("height") / 2.0,
            )
        };

        // Get target element info
        let target_info = target.wait_for_actionable().await?;
        let (target_x, target_y) = if let Some((ox, oy)) = target_position {
            (
                target_info.x.expect("x") + ox,
                target_info.y.expect("y") + oy,
            )
        } else {
            (
                target_info.x.expect("x") + target_info.width.expect("width") / 2.0,
                target_info.y.expect("y") + target_info.height.expect("height") / 2.0,
            )
        };

        debug!(
            "Dragging from ({}, {}) to ({}, {})",
            source_x, source_y, target_x, target_y
        );

        // Perform drag operation
        self.page.mouse().move_(source_x, source_y).send().await?;
        self.page.mouse().down().send().await?;
        self.page
            .mouse()
            .move_(target_x, target_y)
            .steps(steps)
            .send()
            .await?;
        self.page.mouse().up().send().await?;

        Ok(())
    }

    /// Take a screenshot of this element.
    ///
    /// Returns a builder to configure screenshot options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Capture element screenshot
    /// let bytes = page.locator("button").screenshot().capture().await?;
    ///
    /// // Capture and save to file
    /// page.locator("button")
    ///     .screenshot()
    ///     .path("button.png")
    ///     .capture()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn screenshot(&self) -> crate::page::screenshot_element::ElementScreenshotBuilder<'_, '_> {
        crate::page::screenshot_element::ElementScreenshotBuilder::new(self)
    }
}
