//! Browser context builder for creating contexts with custom configuration.

use std::time::Duration;

use crate::BrowserContext;
use crate::context::{
    ColorScheme, ContextOptionsBuilder, ForcedColors, Permission, ReducedMotion, StorageState,
};
use crate::devices::DeviceDescriptor;
use crate::error::BrowserError;
use crate::page::VideoOptions;

use super::Browser;

/// Builder for creating a new browser context with options.
#[derive(Debug)]
pub struct NewContextBuilder<'a> {
    browser: &'a Browser,
    builder: ContextOptionsBuilder,
}

impl<'a> NewContextBuilder<'a> {
    pub(super) fn new(browser: &'a Browser) -> Self {
        Self {
            browser,
            builder: ContextOptionsBuilder::new(),
        }
    }

    /// Set storage state from a file path.
    #[must_use]
    pub fn storage_state_path(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.builder = self.builder.storage_state_path(path);
        self
    }

    /// Set storage state from an object.
    #[must_use]
    pub fn storage_state(mut self, state: StorageState) -> Self {
        self.builder = self.builder.storage_state(state);
        self
    }

    /// Set geolocation.
    #[must_use]
    pub fn geolocation(mut self, latitude: f64, longitude: f64) -> Self {
        self.builder = self.builder.geolocation(latitude, longitude);
        self
    }

    /// Set geolocation with accuracy.
    #[must_use]
    pub fn geolocation_with_accuracy(
        mut self,
        latitude: f64,
        longitude: f64,
        accuracy: f64,
    ) -> Self {
        self.builder = self
            .builder
            .geolocation_with_accuracy(latitude, longitude, accuracy);
        self
    }

    /// Grant permissions.
    #[must_use]
    pub fn permissions(mut self, permissions: Vec<Permission>) -> Self {
        self.builder = self.builder.permissions(permissions);
        self
    }

    /// Set HTTP credentials.
    #[must_use]
    pub fn http_credentials(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.builder = self.builder.http_credentials(username, password);
        self
    }

    /// Set extra HTTP headers.
    #[must_use]
    pub fn extra_http_headers(
        mut self,
        headers: std::collections::HashMap<String, String>,
    ) -> Self {
        self.builder = self.builder.extra_http_headers(headers);
        self
    }

    /// Add an extra HTTP header.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.builder = self.builder.header(name, value);
        self
    }

    /// Set offline mode.
    #[must_use]
    pub fn offline(mut self, offline: bool) -> Self {
        self.builder = self.builder.offline(offline);
        self
    }

    /// Set default timeout.
    #[must_use]
    pub fn default_timeout(mut self, timeout: Duration) -> Self {
        self.builder = self.builder.default_timeout(timeout);
        self
    }

    /// Set default navigation timeout.
    #[must_use]
    pub fn default_navigation_timeout(mut self, timeout: Duration) -> Self {
        self.builder = self.builder.default_navigation_timeout(timeout);
        self
    }

    /// Enable touch emulation.
    #[must_use]
    pub fn has_touch(mut self, has_touch: bool) -> Self {
        self.builder = self.builder.has_touch(has_touch);
        self
    }

    /// Set locale.
    #[must_use]
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.builder = self.builder.locale(locale);
        self
    }

    /// Set timezone.
    #[must_use]
    pub fn timezone_id(mut self, timezone_id: impl Into<String>) -> Self {
        self.builder = self.builder.timezone_id(timezone_id);
        self
    }

    /// Set user agent.
    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.builder = self.builder.user_agent(user_agent);
        self
    }

    /// Set viewport size.
    #[must_use]
    pub fn viewport(mut self, width: i32, height: i32) -> Self {
        self.builder = self.builder.viewport(width, height);
        self
    }

    /// Set color scheme.
    #[must_use]
    pub fn color_scheme(mut self, color_scheme: ColorScheme) -> Self {
        self.builder = self.builder.color_scheme(color_scheme);
        self
    }

    /// Set reduced motion preference.
    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: ReducedMotion) -> Self {
        self.builder = self.builder.reduced_motion(reduced_motion);
        self
    }

    /// Set forced colors preference.
    #[must_use]
    pub fn forced_colors(mut self, forced_colors: ForcedColors) -> Self {
        self.builder = self.builder.forced_colors(forced_colors);
        self
    }

    /// Set device scale factor (device pixel ratio).
    #[must_use]
    pub fn device_scale_factor(mut self, scale_factor: f64) -> Self {
        self.builder = self.builder.device_scale_factor(scale_factor);
        self
    }

    /// Set mobile mode.
    #[must_use]
    pub fn is_mobile(mut self, is_mobile: bool) -> Self {
        self.builder = self.builder.is_mobile(is_mobile);
        self
    }

    /// Apply a device descriptor to configure the context.
    ///
    /// This sets viewport, user agent, device scale factor, touch, and mobile mode
    /// based on the device descriptor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, devices};
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    ///
    /// let context = browser.new_context_builder()
    ///     .device(devices::IPHONE_13)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn device(mut self, device: DeviceDescriptor) -> Self {
        self.builder = self.builder.device(device);
        self
    }

    /// Enable video recording for pages in this context.
    ///
    /// Videos are recorded for each page and saved to the specified directory.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, page::VideoOptions};
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context_builder()
    ///     .record_video(VideoOptions::new("./videos"))
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn record_video(mut self, options: VideoOptions) -> Self {
        self.builder = self.builder.record_video(options);
        self
    }

    /// Build and create the browser context.
    ///
    /// # Errors
    ///
    /// Returns an error if context creation fails.
    pub async fn build(self) -> Result<BrowserContext, BrowserError> {
        self.browser
            .new_context_with_options(self.builder.build())
            .await
    }
}
