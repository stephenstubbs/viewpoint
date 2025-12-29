//! Context options and builder types.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use crate::devices::DeviceDescriptor;

use super::storage::StorageState;
use super::{
    ColorScheme, ForcedColors, Geolocation, HttpCredentials, Permission, ReducedMotion,
    ViewportSize,
};

/// Options for creating a browser context.
#[derive(Debug, Clone, Default)]
pub struct ContextOptions {
    /// Storage state to restore.
    pub storage_state: Option<StorageStateSource>,
    /// Geolocation to use.
    pub geolocation: Option<Geolocation>,
    /// Permissions to grant.
    pub permissions: Vec<Permission>,
    /// HTTP credentials for authentication.
    pub http_credentials: Option<HttpCredentials>,
    /// Extra HTTP headers to add to all requests.
    pub extra_http_headers: HashMap<String, String>,
    /// Whether to start offline.
    pub offline: bool,
    /// Default timeout for actions.
    pub default_timeout: Option<Duration>,
    /// Default timeout for navigation.
    pub default_navigation_timeout: Option<Duration>,
    /// Whether to emulate touch.
    pub has_touch: bool,
    /// Locale to use.
    pub locale: Option<String>,
    /// Timezone to use.
    pub timezone_id: Option<String>,
    /// User agent to use.
    pub user_agent: Option<String>,
    /// Viewport size.
    pub viewport: Option<ViewportSize>,
    /// Device scale factor (device pixel ratio).
    pub device_scale_factor: Option<f64>,
    /// Whether to emulate mobile device.
    pub is_mobile: bool,
    /// Color scheme preference.
    pub color_scheme: Option<ColorScheme>,
    /// Reduced motion preference.
    pub reduced_motion: Option<ReducedMotion>,
    /// Forced colors preference.
    pub forced_colors: Option<ForcedColors>,
    /// Video recording options.
    pub record_video: Option<crate::page::VideoOptions>,
}

/// Source for storage state.
#[derive(Debug, Clone)]
pub enum StorageStateSource {
    /// Load from file path.
    Path(PathBuf),
    /// Use existing storage state.
    State(StorageState),
}

/// Builder for context options.
#[derive(Debug, Default)]
pub struct ContextOptionsBuilder {
    options: ContextOptions,
}

impl ContextOptionsBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set storage state from a file path.
    #[must_use]
    pub fn storage_state_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.options.storage_state = Some(StorageStateSource::Path(path.into()));
        self
    }

    /// Set storage state from an object.
    #[must_use]
    pub fn storage_state(mut self, state: StorageState) -> Self {
        self.options.storage_state = Some(StorageStateSource::State(state));
        self
    }

    /// Set geolocation.
    #[must_use]
    pub fn geolocation(mut self, latitude: f64, longitude: f64) -> Self {
        self.options.geolocation = Some(Geolocation::new(latitude, longitude));
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
        self.options.geolocation = Some(Geolocation::with_accuracy(latitude, longitude, accuracy));
        self
    }

    /// Grant permissions.
    #[must_use]
    pub fn permissions(mut self, permissions: Vec<Permission>) -> Self {
        self.options.permissions = permissions;
        self
    }

    /// Set HTTP credentials.
    #[must_use]
    pub fn http_credentials(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.options.http_credentials = Some(HttpCredentials::new(username, password));
        self
    }

    /// Set extra HTTP headers.
    #[must_use]
    pub fn extra_http_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.options.extra_http_headers = headers;
        self
    }

    /// Add an extra HTTP header.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.options
            .extra_http_headers
            .insert(name.into(), value.into());
        self
    }

    /// Set offline mode.
    #[must_use]
    pub fn offline(mut self, offline: bool) -> Self {
        self.options.offline = offline;
        self
    }

    /// Set default timeout.
    #[must_use]
    pub fn default_timeout(mut self, timeout: Duration) -> Self {
        self.options.default_timeout = Some(timeout);
        self
    }

    /// Set default navigation timeout.
    #[must_use]
    pub fn default_navigation_timeout(mut self, timeout: Duration) -> Self {
        self.options.default_navigation_timeout = Some(timeout);
        self
    }

    /// Enable touch emulation.
    #[must_use]
    pub fn has_touch(mut self, has_touch: bool) -> Self {
        self.options.has_touch = has_touch;
        self
    }

    /// Set locale.
    #[must_use]
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.options.locale = Some(locale.into());
        self
    }

    /// Set timezone.
    #[must_use]
    pub fn timezone_id(mut self, timezone_id: impl Into<String>) -> Self {
        self.options.timezone_id = Some(timezone_id.into());
        self
    }

    /// Set user agent.
    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.options.user_agent = Some(user_agent.into());
        self
    }

    /// Set viewport size.
    #[must_use]
    pub fn viewport(mut self, width: i32, height: i32) -> Self {
        self.options.viewport = Some(ViewportSize::new(width, height));
        self
    }

    /// Set color scheme.
    #[must_use]
    pub fn color_scheme(mut self, color_scheme: ColorScheme) -> Self {
        self.options.color_scheme = Some(color_scheme);
        self
    }

    /// Set reduced motion preference.
    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: ReducedMotion) -> Self {
        self.options.reduced_motion = Some(reduced_motion);
        self
    }

    /// Set forced colors preference.
    #[must_use]
    pub fn forced_colors(mut self, forced_colors: ForcedColors) -> Self {
        self.options.forced_colors = Some(forced_colors);
        self
    }

    /// Set device scale factor (device pixel ratio).
    #[must_use]
    pub fn device_scale_factor(mut self, scale_factor: f64) -> Self {
        self.options.device_scale_factor = Some(scale_factor);
        self
    }

    /// Set mobile mode.
    #[must_use]
    pub fn is_mobile(mut self, is_mobile: bool) -> Self {
        self.options.is_mobile = is_mobile;
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
    /// use viewpoint_core::context::ContextOptionsBuilder;
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
        self.options.viewport = Some(device.viewport);
        self.options.user_agent = Some(device.user_agent.to_string());
        self.options.device_scale_factor = Some(device.device_scale_factor);
        self.options.has_touch = device.has_touch;
        self.options.is_mobile = device.is_mobile;
        self
    }

    /// Enable video recording.
    ///
    /// Videos are recorded for each page and saved to the specified directory.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, VideoOptions};
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context_builder()
    ///     .record_video(VideoOptions::new("./videos"))
    ///     .build()
    ///     .await?;
    ///
    /// let page = context.new_page().await?;
    /// page.goto("https://example.com").goto().await?;
    ///
    /// // Video is automatically saved when page is closed
    /// if let Some(video) = page.video() {
    ///     let path = video.path().await?;
    ///     println!("Video: {}", path.display());
    /// }
    /// # Ok(())
    /// # }
    #[must_use]
    pub fn record_video(mut self, options: crate::page::VideoOptions) -> Self {
        self.options.record_video = Some(options);
        self
    }

    /// Build the options.
    pub fn build(self) -> ContextOptions {
        self.options
    }
}

#[cfg(test)]
mod tests;
