//! Mouse input handling.
//!
//! Provides direct mouse control for simulating clicks, movement, and scrolling.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tracing::{debug, instrument};
use viewpoint_cdp::protocol::input::{
    DispatchMouseEventParams, DispatchMouseWheelParams, MouseButton, MouseEventType,
};
use viewpoint_cdp::CdpConnection;

use crate::error::LocatorError;

/// Mouse state tracking.
#[derive(Debug)]
struct MouseState {
    /// Current X position.
    x: f64,
    /// Current Y position.
    y: f64,
    /// Currently pressed button.
    button: Option<MouseButton>,
}

impl MouseState {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            button: None,
        }
    }
}

/// Mouse controller for direct mouse input.
///
/// Provides methods for moving the mouse, clicking, and scrolling.
/// All coordinates are in CSS pixels relative to the viewport.
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
/// // Move mouse to coordinates
/// page.mouse().move_(100.0, 200.0).send().await.unwrap();
///
/// // Click at coordinates
/// page.mouse().click(100.0, 200.0).send().await.unwrap();
///
/// // Scroll
/// page.mouse().wheel(0.0, 100.0).await.unwrap();
///
/// // Drag operation
/// page.mouse().move_(100.0, 100.0).send().await.unwrap();
/// page.mouse().down().send().await.unwrap();
/// page.mouse().move_(200.0, 200.0).steps(10).send().await.unwrap();
/// page.mouse().up().send().await.unwrap();
/// # });
/// ```
#[derive(Debug)]
pub struct Mouse {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID for the page.
    session_id: String,
    /// Mouse state.
    state: Mutex<MouseState>,
}

impl Mouse {
    /// Create a new mouse controller.
    pub(crate) fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        Self {
            connection,
            session_id,
            state: Mutex::new(MouseState::new()),
        }
    }

    /// Move the mouse to the specified coordinates.
    ///
    /// Returns a builder for additional options.
    pub fn move_(&self, x: f64, y: f64) -> MoveBuilder<'_> {
        MoveBuilder {
            mouse: self,
            x,
            y,
            steps: 1,
        }
    }

    /// Click at the specified coordinates.
    ///
    /// Returns a builder for additional options.
    pub fn click(&self, x: f64, y: f64) -> ClickBuilder<'_> {
        ClickBuilder {
            mouse: self,
            x,
            y,
            button: MouseButton::Left,
            click_count: 1,
            delay: None,
        }
    }

    /// Double-click at the specified coordinates.
    #[instrument(level = "debug", skip(self), fields(x = x, y = y))]
    pub async fn dblclick(&self, x: f64, y: f64) -> Result<(), LocatorError> {
        debug!("Double-clicking at ({}, {})", x, y);

        // First click
        self.move_(x, y).send().await?;
        self.down_internal(MouseButton::Left, 1).await?;
        self.up_internal(MouseButton::Left, 1).await?;

        // Second click
        self.down_internal(MouseButton::Left, 2).await?;
        self.up_internal(MouseButton::Left, 2).await?;

        Ok(())
    }

    /// Press the mouse button at the current position.
    ///
    /// Returns a builder for additional options.
    pub fn down(&self) -> DownBuilder<'_> {
        DownBuilder {
            mouse: self,
            button: MouseButton::Left,
            click_count: 1,
        }
    }

    /// Release the mouse button at the current position.
    ///
    /// Returns a builder for additional options.
    pub fn up(&self) -> UpBuilder<'_> {
        UpBuilder {
            mouse: self,
            button: MouseButton::Left,
            click_count: 1,
        }
    }

    /// Scroll the mouse wheel.
    #[instrument(level = "debug", skip(self), fields(delta_x = delta_x, delta_y = delta_y))]
    pub async fn wheel(&self, delta_x: f64, delta_y: f64) -> Result<(), LocatorError> {
        let state = self.state.lock().await;
        let x = state.x;
        let y = state.y;
        drop(state);

        debug!("Mouse wheel at ({}, {}): delta=({}, {})", x, y, delta_x, delta_y);

        let params = DispatchMouseWheelParams {
            event_type: MouseEventType::MouseWheel,
            x,
            y,
            delta_x,
            delta_y,
            modifiers: None,
            pointer_type: None,
        };

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchMouseEvent",
                Some(params),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Internal move implementation.
    async fn move_internal(&self, x: f64, y: f64, steps: u32) -> Result<(), LocatorError> {
        let (start_x, start_y) = {
            let state = self.state.lock().await;
            (state.x, state.y)
        };

        if steps <= 1 {
            // Single move
            self.dispatch_move(x, y).await?;
        } else {
            // Move in steps for smooth animation
            for i in 1..=steps {
                let progress = f64::from(i) / f64::from(steps);
                let current_x = start_x + (x - start_x) * progress;
                let current_y = start_y + (y - start_y) * progress;
                self.dispatch_move(current_x, current_y).await?;
            }
        }

        // Update state
        {
            let mut state = self.state.lock().await;
            state.x = x;
            state.y = y;
        }

        Ok(())
    }

    /// Dispatch a mouse move event.
    async fn dispatch_move(&self, x: f64, y: f64) -> Result<(), LocatorError> {
        let params = DispatchMouseEventParams::mouse_move(x, y);

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchMouseEvent",
                Some(params),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Internal down implementation.
    async fn down_internal(
        &self,
        button: MouseButton,
        click_count: i32,
    ) -> Result<(), LocatorError> {
        let (x, y) = {
            let state = self.state.lock().await;
            (state.x, state.y)
        };

        debug!("Mouse down at ({}, {}), button={:?}, count={}", x, y, button, click_count);

        let mut params = DispatchMouseEventParams::mouse_down(x, y, button);
        params.click_count = Some(click_count);

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchMouseEvent",
                Some(params),
                Some(&self.session_id),
            )
            .await?;

        // Update state
        {
            let mut state = self.state.lock().await;
            state.button = Some(button);
        }

        Ok(())
    }

    /// Internal up implementation.
    async fn up_internal(&self, button: MouseButton, click_count: i32) -> Result<(), LocatorError> {
        let (x, y) = {
            let state = self.state.lock().await;
            (state.x, state.y)
        };

        debug!("Mouse up at ({}, {}), button={:?}, count={}", x, y, button, click_count);

        let mut params = DispatchMouseEventParams::mouse_up(x, y, button);
        params.click_count = Some(click_count);

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchMouseEvent",
                Some(params),
                Some(&self.session_id),
            )
            .await?;

        // Update state
        {
            let mut state = self.state.lock().await;
            state.button = None;
        }

        Ok(())
    }
}

/// Builder for mouse move operations.
#[derive(Debug)]
pub struct MoveBuilder<'a> {
    mouse: &'a Mouse,
    x: f64,
    y: f64,
    steps: u32,
}

impl MoveBuilder<'_> {
    /// Set the number of intermediate steps for smooth movement.
    ///
    /// Default is 1 (instant move).
    #[must_use]
    pub fn steps(mut self, steps: u32) -> Self {
        self.steps = steps.max(1);
        self
    }

    /// Execute the move.
    #[instrument(level = "debug", skip(self), fields(x = self.x, y = self.y, steps = self.steps))]
    pub async fn send(self) -> Result<(), LocatorError> {
        debug!("Moving mouse to ({}, {}) in {} steps", self.x, self.y, self.steps);
        self.mouse.move_internal(self.x, self.y, self.steps).await
    }
}

/// Builder for mouse click operations.
#[derive(Debug)]
pub struct ClickBuilder<'a> {
    mouse: &'a Mouse,
    x: f64,
    y: f64,
    button: MouseButton,
    click_count: i32,
    delay: Option<Duration>,
}

impl ClickBuilder<'_> {
    /// Set the mouse button to click.
    ///
    /// Default is left button.
    #[must_use]
    pub fn button(mut self, button: MouseButton) -> Self {
        self.button = button;
        self
    }

    /// Set the click count (for multi-click).
    ///
    /// Default is 1.
    #[must_use]
    pub fn click_count(mut self, count: i32) -> Self {
        self.click_count = count;
        self
    }

    /// Set the delay between mouse down and up.
    #[must_use]
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Execute the click.
    #[instrument(level = "debug", skip(self), fields(x = self.x, y = self.y, button = ?self.button))]
    pub async fn send(self) -> Result<(), LocatorError> {
        debug!("Clicking at ({}, {}), button={:?}", self.x, self.y, self.button);

        // Move to position
        self.mouse.move_(self.x, self.y).send().await?;

        // Click
        self.mouse.down_internal(self.button, self.click_count).await?;

        if let Some(delay) = self.delay {
            tokio::time::sleep(delay).await;
        }

        self.mouse.up_internal(self.button, self.click_count).await?;

        Ok(())
    }
}

/// Builder for mouse down operations.
#[derive(Debug)]
pub struct DownBuilder<'a> {
    mouse: &'a Mouse,
    button: MouseButton,
    click_count: i32,
}

impl DownBuilder<'_> {
    /// Set the mouse button.
    #[must_use]
    pub fn button(mut self, button: MouseButton) -> Self {
        self.button = button;
        self
    }

    /// Set the click count.
    #[must_use]
    pub fn click_count(mut self, count: i32) -> Self {
        self.click_count = count;
        self
    }

    /// Execute the mouse down.
    #[instrument(level = "debug", skip(self), fields(button = ?self.button))]
    pub async fn send(self) -> Result<(), LocatorError> {
        self.mouse.down_internal(self.button, self.click_count).await
    }
}

/// Builder for mouse up operations.
#[derive(Debug)]
pub struct UpBuilder<'a> {
    mouse: &'a Mouse,
    button: MouseButton,
    click_count: i32,
}

impl UpBuilder<'_> {
    /// Set the mouse button.
    #[must_use]
    pub fn button(mut self, button: MouseButton) -> Self {
        self.button = button;
        self
    }

    /// Set the click count.
    #[must_use]
    pub fn click_count(mut self, count: i32) -> Self {
        self.click_count = count;
        self
    }

    /// Execute the mouse up.
    #[instrument(level = "debug", skip(self), fields(button = ?self.button))]
    pub async fn send(self) -> Result<(), LocatorError> {
        self.mouse.up_internal(self.button, self.click_count).await
    }
}
