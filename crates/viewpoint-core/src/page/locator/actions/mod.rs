//! Locator actions for element interaction.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::DispatchKeyEventParams;

use super::Locator;
use super::builders::{
    CheckBuilder, ClickBuilder, DblclickBuilder, FillBuilder, HoverBuilder, PressBuilder,
    TapBuilder, TypeBuilder,
};
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
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for a simple double-click.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple double-click - await directly
    /// page.locator("button").dblclick().await?;
    ///
    /// // Double-click with options
    /// page.locator("button").dblclick()
    ///     .position(10.0, 5.0)
    ///     .no_wait_after(true)
    ///     .send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn dblclick(&self) -> DblclickBuilder<'_, 'a> {
        DblclickBuilder::new(self)
    }

    /// Fill the element with text (clears existing content first).
    ///
    /// This is for input and textarea elements. Returns a builder that can be
    /// configured with additional options, or awaited directly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple fill - await directly
    /// page.locator("input").fill("hello").await?;
    ///
    /// // Fill without waiting for navigation
    /// page.locator("input").fill("hello").no_wait_after(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fill(&self, text: &str) -> FillBuilder<'_, 'a> {
        FillBuilder::new(self, text)
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
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for a simple key press.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple press - await directly
    /// page.locator("input").press("Enter").await?;
    ///
    /// // Press without waiting for navigation (e.g., form submission)
    /// page.locator("input").press("Enter").no_wait_after(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn press(&self, key: &str) -> PressBuilder<'_, 'a> {
        PressBuilder::new(self, key)
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
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for a simple check operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple check - await directly
    /// page.locator("input[type=checkbox]").check().await?;
    ///
    /// // Check without waiting for navigation
    /// page.locator("input[type=checkbox]").check().no_wait_after(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn check(&self) -> CheckBuilder<'_, 'a> {
        CheckBuilder::new_check(self)
    }

    /// Uncheck a checkbox.
    ///
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for a simple uncheck operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple uncheck - await directly
    /// page.locator("input[type=checkbox]").uncheck().await?;
    ///
    /// // Uncheck without waiting for navigation
    /// page.locator("input[type=checkbox]").uncheck().no_wait_after(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn uncheck(&self) -> CheckBuilder<'_, 'a> {
        CheckBuilder::new_uncheck(self)
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
