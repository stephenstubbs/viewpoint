//! Browser launching and management.

mod launcher;

use std::process::Child;
use std::sync::Arc;
use std::time::Duration;

use viewpoint_cdp::protocol::target_domain::{CreateBrowserContextParams, CreateBrowserContextResult};
use viewpoint_cdp::CdpConnection;
use tokio::sync::Mutex;

use crate::context::{BrowserContext, ContextOptions, ContextOptionsBuilder, StorageState, StorageStateSource};
use crate::devices::DeviceDescriptor;
use crate::error::BrowserError;

pub use launcher::BrowserBuilder;

/// Default timeout for browser operations.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A browser instance connected via CDP.
#[derive(Debug)]
pub struct Browser {
    /// CDP connection to the browser.
    connection: Arc<CdpConnection>,
    /// Browser process (only present if we launched it).
    process: Option<Mutex<Child>>,
    /// Whether the browser was launched by us (vs connected to).
    owned: bool,
}

impl Browser {
    /// Create a browser builder for launching a new browser.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch()
    ///     .headless(true)
    ///     .launch()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn launch() -> BrowserBuilder {
        BrowserBuilder::new()
    }

    /// Connect to an already-running browser via WebSocket URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::connect("ws://localhost:9222/devtools/browser/...").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the connection fails.
    pub async fn connect(ws_url: &str) -> Result<Self, BrowserError> {
        let connection = CdpConnection::connect(ws_url).await?;

        Ok(Self {
            connection: Arc::new(connection),
            process: None,
            owned: false,
        })
    }

    /// Create a browser from an existing connection and process.
    pub(crate) fn from_connection_and_process(
        connection: CdpConnection,
        process: Child,
    ) -> Self {
        Self {
            connection: Arc::new(connection),
            process: Some(Mutex::new(process)),
            owned: true,
        }
    }

    /// Create a new isolated browser context.
    ///
    /// Browser contexts are isolated environments within the browser,
    /// similar to incognito windows. They have their own cookies,
    /// cache, and storage.
    ///
    /// # Errors
    ///
    /// Returns an error if context creation fails.
    pub async fn new_context(&self) -> Result<BrowserContext, BrowserError> {
        let result: CreateBrowserContextResult = self
            .connection
            .send_command(
                "Target.createBrowserContext",
                Some(CreateBrowserContextParams::default()),
                None,
            )
            .await?;

        Ok(BrowserContext::new(
            self.connection.clone(),
            result.browser_context_id,
        ))
    }

    /// Create a new context options builder.
    ///
    /// Use this to create a browser context with custom configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, Permission};
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    ///
    /// let context = browser.new_context_builder()
    ///     .geolocation(37.7749, -122.4194)
    ///     .permissions(vec![Permission::Geolocation])
    ///     .offline(false)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_context_builder(&self) -> NewContextBuilder<'_> {
        NewContextBuilder::new(self)
    }

    /// Create a new isolated browser context with options.
    ///
    /// # Errors
    ///
    /// Returns an error if context creation fails.
    pub async fn new_context_with_options(
        &self,
        options: ContextOptions,
    ) -> Result<BrowserContext, BrowserError> {
        // Load storage state if specified
        let storage_state = match &options.storage_state {
            Some(StorageStateSource::Path(path)) => {
                Some(StorageState::load(path).await.map_err(|e| {
                    BrowserError::LaunchFailed(format!("Failed to load storage state: {e}"))
                })?)
            }
            Some(StorageStateSource::State(state)) => Some(state.clone()),
            None => None,
        };

        let result: CreateBrowserContextResult = self
            .connection
            .send_command(
                "Target.createBrowserContext",
                Some(CreateBrowserContextParams::default()),
                None,
            )
            .await?;

        let context = BrowserContext::with_options(
            self.connection.clone(),
            result.browser_context_id,
            options,
        );

        // Apply options
        context.apply_options().await?;

        // Restore storage state if any
        if let Some(state) = storage_state {
            // Restore cookies
            context.add_cookies(state.cookies.clone()).await?;

            // Restore localStorage via init script
            let local_storage_script = state.to_local_storage_init_script();
            if !local_storage_script.is_empty() {
                context.add_init_script(&local_storage_script).await?;
            }

            // Restore IndexedDB via init script
            let indexed_db_script = state.to_indexed_db_init_script();
            if !indexed_db_script.is_empty() {
                context.add_init_script(&indexed_db_script).await?;
            }
        }

        Ok(context)
    }

    /// Close the browser.
    ///
    /// If this browser was launched by us, the process will be terminated.
    /// If it was connected to, only the WebSocket connection is closed.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    pub async fn close(&self) -> Result<(), BrowserError> {
        // If we own the process, terminate it
        if let Some(ref process) = self.process {
            let mut child = process.lock().await;
            let _ = child.kill();
        }

        Ok(())
    }

    /// Get a reference to the CDP connection.
    pub fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }

    /// Check if this browser was launched by us.
    pub fn is_owned(&self) -> bool {
        self.owned
    }
}

/// Builder for creating a new browser context with options.
#[derive(Debug)]
pub struct NewContextBuilder<'a> {
    browser: &'a Browser,
    builder: ContextOptionsBuilder,
}

impl<'a> NewContextBuilder<'a> {
    fn new(browser: &'a Browser) -> Self {
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
    pub fn geolocation_with_accuracy(mut self, latitude: f64, longitude: f64, accuracy: f64) -> Self {
        self.builder = self.builder.geolocation_with_accuracy(latitude, longitude, accuracy);
        self
    }

    /// Grant permissions.
    #[must_use]
    pub fn permissions(mut self, permissions: Vec<crate::context::Permission>) -> Self {
        self.builder = self.builder.permissions(permissions);
        self
    }

    /// Set HTTP credentials.
    #[must_use]
    pub fn http_credentials(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.builder = self.builder.http_credentials(username, password);
        self
    }

    /// Set extra HTTP headers.
    #[must_use]
    pub fn extra_http_headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
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
    pub fn color_scheme(mut self, color_scheme: crate::context::ColorScheme) -> Self {
        self.builder = self.builder.color_scheme(color_scheme);
        self
    }

    /// Set reduced motion preference.
    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: crate::context::ReducedMotion) -> Self {
        self.builder = self.builder.reduced_motion(reduced_motion);
        self
    }

    /// Set forced colors preference.
    #[must_use]
    pub fn forced_colors(mut self, forced_colors: crate::context::ForcedColors) -> Self {
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
    pub fn record_video(mut self, options: crate::page::VideoOptions) -> Self {
        self.builder = self.builder.record_video(options);
        self
    }

    /// Build and create the browser context.
    ///
    /// # Errors
    ///
    /// Returns an error if context creation fails.
    pub async fn build(self) -> Result<BrowserContext, BrowserError> {
        self.browser.new_context_with_options(self.builder.build()).await
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        // Try to kill the process if we own it
        if self.owned {
            if let Some(ref process) = self.process {
                // We can't await in drop, so we try to kill synchronously
                if let Ok(mut guard) = process.try_lock() {
                    let _ = guard.kill();
                }
            }
        }
    }
}
