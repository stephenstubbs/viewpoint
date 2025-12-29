//! Input device access methods for Page.
//!
//! This module contains methods for accessing input device controllers
//! (keyboard, mouse, touchscreen, clock) and performing drag-and-drop operations.

use crate::page::Page;
use crate::page::clock::Clock;
use crate::page::keyboard::Keyboard;
use crate::page::mouse::Mouse;
use crate::page::mouse_drag::DragAndDropBuilder;
use crate::page::touchscreen::Touchscreen;

impl Page {
    /// Get a reference to the keyboard controller.
    ///
    /// The keyboard controller provides methods for pressing keys, typing text,
    /// and managing modifier key state.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Press a key
    /// page.keyboard().press("Enter").await?;
    ///
    /// // Type text
    /// page.keyboard().type_text("Hello, World!").await?;
    ///
    /// // Use key combinations
    /// page.keyboard().press("Control+a").await?;
    ///
    /// // Hold and release modifiers
    /// page.keyboard().down("Shift").await?;
    /// page.keyboard().press("a").await?; // Types 'A'
    /// page.keyboard().up("Shift").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    /// Get a reference to the mouse controller.
    ///
    /// The mouse controller provides methods for moving the mouse,
    /// clicking, and scrolling.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use viewpoint_core::MouseButton;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Move mouse
    /// page.mouse().move_(100.0, 200.0).send().await?;
    ///
    /// // Click at coordinates
    /// page.mouse().click(100.0, 200.0).send().await?;
    ///
    /// // Right-click
    /// page.mouse().click(100.0, 200.0).button(MouseButton::Right).send().await?;
    ///
    /// // Scroll
    /// page.mouse().wheel(0.0, 100.0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    /// Get a reference to the touchscreen controller.
    ///
    /// The touchscreen controller provides methods for touch input simulation.
    /// Requires `hasTouch: true` in browser context options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.touchscreen().tap(100.0, 200.0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn touchscreen(&self) -> &Touchscreen {
        &self.touchscreen
    }

    /// Get a clock controller for this page.
    ///
    /// The clock controller allows you to mock time-related JavaScript functions
    /// including Date, setTimeout, setInterval, requestAnimationFrame, and
    /// `performance.now()`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use std::time::Duration;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Install clock mocking
    /// let mut clock = page.clock();
    /// clock.install().await?;
    ///
    /// // Freeze time at a specific moment
    /// clock.set_fixed_time("2024-01-01T00:00:00Z").await?;
    ///
    /// // Advance time and fire scheduled timers
    /// clock.run_for(Duration::from_secs(5)).await?;
    ///
    /// // Cleanup
    /// clock.uninstall().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn clock(&self) -> Clock<'_> {
        Clock::new(&self.connection, &self.session_id)
    }

    /// Drag from source to target element.
    ///
    /// Returns a builder for configuring drag options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple drag and drop
    /// page.drag_and_drop("#source", "#target").send().await?;
    ///
    /// // With position options
    /// page.drag_and_drop("#source", "#target")
    ///     .source_position(10.0, 10.0)
    ///     .target_position(5.0, 5.0)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn drag_and_drop(
        &self,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> DragAndDropBuilder<'_> {
        DragAndDropBuilder::new(self, source.into(), target.into())
    }
}
