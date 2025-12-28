//! Touchscreen input handling.
//!
//! Provides touch input simulation for mobile testing scenarios.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::emulation::SetTouchEmulationEnabledParams;
use viewpoint_cdp::protocol::input::{DispatchTouchEventParams, TouchPoint};
use viewpoint_cdp::CdpConnection;

use crate::error::LocatorError;

/// Global touch identifier counter for unique touch point IDs.
static TOUCH_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

/// Touchscreen controller for touch input simulation.
///
/// Provides methods for tapping and touch gestures.
/// Requires touch to be enabled via [`enable`](Touchscreen::enable) or `hasTouch: true`
/// in browser context options.
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
///
/// // Enable touch on the page
/// page.touchscreen().enable().await.unwrap();
///
/// // Tap at coordinates
/// page.touchscreen().tap(100.0, 200.0).await.unwrap();
/// # });
/// ```
#[derive(Debug)]
pub struct Touchscreen {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID for the page.
    session_id: String,
    /// Whether touch emulation is enabled.
    enabled: AtomicBool,
}

impl Touchscreen {
    /// Create a new touchscreen controller.
    pub(crate) fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        Self {
            connection,
            session_id,
            enabled: AtomicBool::new(false),
        }
    }

    /// Enable touch emulation.
    ///
    /// This must be called before using touch methods, or touch must be enabled
    /// in browser context options.
    ///
    /// # Arguments
    ///
    /// * `max_touch_points` - Maximum number of touch points. Defaults to 1.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Enable touch with default settings
    /// page.touchscreen().enable().await?;
    ///
    /// // Enable touch with multiple touch points
    /// page.touchscreen().enable_with_max_points(5).await?;
    /// ```
    #[instrument(level = "debug", skip(self))]
    pub async fn enable(&self) -> Result<(), LocatorError> {
        self.enable_with_max_points(1).await
    }

    /// Enable touch emulation with a specific maximum number of touch points.
    #[instrument(level = "debug", skip(self), fields(max_touch_points = max_touch_points))]
    pub async fn enable_with_max_points(&self, max_touch_points: i32) -> Result<(), LocatorError> {
        debug!("Enabling touch emulation with max_touch_points={}", max_touch_points);

        self.connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setTouchEmulationEnabled",
                Some(SetTouchEmulationEnabledParams {
                    enabled: true,
                    max_touch_points: Some(max_touch_points),
                }),
                Some(&self.session_id),
            )
            .await?;

        self.enabled.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Disable touch emulation.
    #[instrument(level = "debug", skip(self))]
    pub async fn disable(&self) -> Result<(), LocatorError> {
        debug!("Disabling touch emulation");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setTouchEmulationEnabled",
                Some(SetTouchEmulationEnabledParams {
                    enabled: false,
                    max_touch_points: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        self.enabled.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Check if touch emulation is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    /// Mark touch as enabled (for internal use when context is created with hasTouch).
    pub(crate) fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    /// Check that touch is enabled, returning an error if not.
    fn check_enabled(&self) -> Result<(), LocatorError> {
        if !self.is_enabled() {
            return Err(LocatorError::TouchNotEnabled);
        }
        Ok(())
    }

    /// Tap at the specified coordinates.
    ///
    /// Dispatches touchStart and touchEnd events.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate in CSS pixels
    /// * `y` - Y coordinate in CSS pixels
    ///
    /// # Errors
    ///
    /// Returns [`LocatorError::TouchNotEnabled`] if touch emulation is not enabled.
    /// Call [`enable`](Touchscreen::enable) first or set `hasTouch: true` in context options.
    ///
    /// # Example
    ///
    /// ```ignore
    /// page.touchscreen().enable().await?;
    /// page.touchscreen().tap(100.0, 200.0).await?;
    /// ```
    #[instrument(level = "debug", skip(self), fields(x = x, y = y))]
    pub async fn tap(&self, x: f64, y: f64) -> Result<(), LocatorError> {
        self.check_enabled()?;
        debug!("Tapping at ({}, {})", x, y);

        // Generate unique touch ID
        let touch_id = TOUCH_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Touch start
        let mut touch_point = TouchPoint::new(x, y);
        touch_point.id = Some(touch_id);

        let start_params = DispatchTouchEventParams {
            event_type: viewpoint_cdp::protocol::input::TouchEventType::TouchStart,
            touch_points: vec![touch_point.clone()],
            modifiers: None,
            timestamp: None,
        };

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchTouchEvent",
                Some(start_params),
                Some(&self.session_id),
            )
            .await?;

        // Touch end
        let end_params = DispatchTouchEventParams {
            event_type: viewpoint_cdp::protocol::input::TouchEventType::TouchEnd,
            touch_points: vec![],
            modifiers: None,
            timestamp: None,
        };

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchTouchEvent",
                Some(end_params),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Tap with modifiers (Shift, Control, etc).
    ///
    /// # Errors
    ///
    /// Returns [`LocatorError::TouchNotEnabled`] if touch emulation is not enabled.
    #[instrument(level = "debug", skip(self), fields(x = x, y = y, modifiers = modifiers))]
    pub async fn tap_with_modifiers(
        &self,
        x: f64,
        y: f64,
        modifiers: i32,
    ) -> Result<(), LocatorError> {
        self.check_enabled()?;
        debug!("Tapping at ({}, {}) with modifiers {}", x, y, modifiers);

        let touch_id = TOUCH_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut touch_point = TouchPoint::new(x, y);
        touch_point.id = Some(touch_id);

        // Touch start with modifiers
        let start_params = DispatchTouchEventParams {
            event_type: viewpoint_cdp::protocol::input::TouchEventType::TouchStart,
            touch_points: vec![touch_point],
            modifiers: Some(modifiers),
            timestamp: None,
        };

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchTouchEvent",
                Some(start_params),
                Some(&self.session_id),
            )
            .await?;

        // Touch end with modifiers
        let end_params = DispatchTouchEventParams {
            event_type: viewpoint_cdp::protocol::input::TouchEventType::TouchEnd,
            touch_points: vec![],
            modifiers: Some(modifiers),
            timestamp: None,
        };

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchTouchEvent",
                Some(end_params),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }
}
