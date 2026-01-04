//! Browser launching and management.
//!
//! This module provides the [`Browser`] type for connecting to and controlling
//! Chromium-based browsers via the Chrome DevTools Protocol (CDP).
//!
//! # Connection Methods
//!
//! There are three ways to get a `Browser` instance:
//!
//! 1. **Launch a new browser** - [`Browser::launch()`] spawns a new Chromium process
//! 2. **Connect via WebSocket URL** - [`Browser::connect()`] for direct WebSocket connection  
//! 3. **Connect via HTTP endpoint** - [`Browser::connect_over_cdp()`] discovers WebSocket URL
//!    from an HTTP endpoint like `http://localhost:9222`
//!
//! # Example: Launching a Browser
//!
//! ```no_run
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! let browser = Browser::launch()
//!     .headless(true)
//!     .launch()
//!     .await?;
//!
//! let context = browser.new_context().await?;
//! let page = context.new_page().await?;
//! page.goto("https://example.com").goto().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Example: Connecting to Existing Browser (MCP-style)
//!
//! This is useful for MCP servers or tools that need to connect to an already-running
//! browser instance:
//!
//! ```no_run
//! use viewpoint_core::Browser;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! // Connect via HTTP endpoint (discovers WebSocket URL automatically)
//! let browser = Browser::connect_over_cdp("http://localhost:9222")
//!     .timeout(Duration::from_secs(10))
//!     .connect()
//!     .await?;
//!
//! // Access existing browser contexts (including the default one)
//! let contexts = browser.contexts().await?;
//! for context in &contexts {
//!     if context.is_default() {
//!         // The default context has the browser's existing tabs
//!         let pages = context.pages().await?;
//!         println!("Found {} existing pages", pages.len());
//!     }
//! }
//!
//! // You can also create new contexts in the connected browser
//! let new_context = browser.new_context().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Ownership Model
//!
//! Browsers and contexts track ownership:
//!
//! - **Launched browsers** (`Browser::launch()`) are "owned" - closing them terminates the process
//! - **Connected browsers** (`connect()`, `connect_over_cdp()`) are not owned - closing only
//!   disconnects, leaving the browser process running
//! - **Created contexts** (`new_context()`) are owned - closing disposes them
//! - **Discovered contexts** (`contexts()`) are not owned - closing only disconnects

mod connector;
mod context_builder;
mod launcher;
mod process;

use std::process::Child;
use std::sync::Arc;
use std::time::Duration;

use tempfile::TempDir;
use tokio::sync::Mutex;
use tracing::info;
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::target_domain::{
    CreateBrowserContextParams, CreateBrowserContextResult, GetBrowserContextsResult,
};

use crate::context::{BrowserContext, ContextOptions, StorageState, StorageStateSource};
use crate::error::BrowserError;

pub use connector::ConnectOverCdpBuilder;
pub use context_builder::NewContextBuilder;
pub use launcher::{BrowserBuilder, UserDataDir};

/// Default timeout for browser operations.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A browser instance connected via CDP.
///
/// The `Browser` struct represents a connection to a Chromium-based browser.
/// It can be obtained by:
///
/// - [`Browser::launch()`] - Spawn and connect to a new browser process
/// - [`Browser::connect()`] - Connect to an existing browser via WebSocket URL
/// - [`Browser::connect_over_cdp()`] - Connect via HTTP endpoint (auto-discovers WebSocket)
///
/// # Key Methods
///
/// - [`new_context()`](Self::new_context) - Create a new isolated browser context
/// - [`contexts()`](Self::contexts) - List all browser contexts (including pre-existing ones)
/// - [`close()`](Self::close) - Close the browser connection
///
/// # Ownership
///
/// Use [`is_owned()`](Self::is_owned) to check if this browser was launched by us
/// (vs connected to an existing process). Owned browsers are terminated when closed.
///
/// # User Data Directory
///
/// By default, browsers use an isolated temporary directory for user data
/// (cookies, localStorage, settings). This prevents conflicts when running
/// multiple browser instances and ensures clean sessions. The temporary
/// directory is automatically cleaned up when the browser closes or is dropped.
///
/// See [`UserDataDir`] for configuration options.
#[derive(Debug)]
pub struct Browser {
    /// CDP connection to the browser.
    connection: Arc<CdpConnection>,
    /// Browser process (only present if we launched it).
    process: Option<Mutex<Child>>,
    /// Whether the browser was launched by us (vs connected to).
    owned: bool,
    /// Temporary user data directory (if using Temp or TempFromTemplate mode).
    /// Stored here to ensure cleanup on drop.
    _temp_user_data_dir: Option<TempDir>,
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

        // Enable target discovery to receive Target.targetCreated events
        // This is required for automatic page tracking (popups, target="_blank" links)
        connection
            .send_command::<_, serde_json::Value>(
                "Target.setDiscoverTargets",
                Some(viewpoint_cdp::protocol::target_domain::SetDiscoverTargetsParams {
                    discover: true,
                }),
                None,
            )
            .await
            .map_err(|e| BrowserError::ConnectionFailed(format!("Failed to enable target discovery: {e}")))?;

        Ok(Self {
            connection: Arc::new(connection),
            process: None,
            owned: false,
            _temp_user_data_dir: None,
        })
    }

    /// Connect to an already-running browser via HTTP endpoint or WebSocket URL.
    ///
    /// This method supports both:
    /// - HTTP endpoint URLs (e.g., `http://localhost:9222`) - auto-discovers WebSocket URL
    /// - WebSocket URLs (e.g., `ws://localhost:9222/devtools/browser/...`) - direct connection
    ///
    /// For HTTP endpoints, the method fetches `/json/version` to discover the WebSocket URL,
    /// similar to Playwright's `connectOverCDP`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// // Connect via HTTP endpoint (recommended)
    /// let browser = Browser::connect_over_cdp("http://localhost:9222")
    ///     .connect()
    ///     .await?;
    ///
    /// // With custom timeout and headers
    /// let browser = Browser::connect_over_cdp("http://localhost:9222")
    ///     .timeout(Duration::from_secs(10))
    ///     .header("Authorization", "Bearer token")
    ///     .connect()
    ///     .await?;
    ///
    /// // Access existing browser contexts and pages
    /// let contexts = browser.contexts().await?;
    /// for context in contexts {
    ///     let pages = context.pages().await?;
    ///     for page in pages {
    ///         println!("Found page: {:?}", page.target_id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect_over_cdp(endpoint_url: impl Into<String>) -> ConnectOverCdpBuilder {
        ConnectOverCdpBuilder::new(endpoint_url)
    }

    /// Get all browser contexts.
    ///
    /// Returns all existing browser contexts, including:
    /// - Contexts created via `new_context()`
    /// - The default context (for connected browsers)
    /// - Any pre-existing contexts (when connecting to an already-running browser)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::connect_over_cdp("http://localhost:9222")
    ///     .connect()
    ///     .await?;
    ///
    /// let contexts = browser.contexts().await?;
    /// println!("Found {} browser contexts", contexts.len());
    ///
    /// // The default context (empty string ID) represents the browser's main profile
    /// for context in &contexts {
    ///     if context.id().is_empty() {
    ///         println!("This is the default context");
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if querying contexts fails.
    pub async fn contexts(&self) -> Result<Vec<BrowserContext>, BrowserError> {
        info!("Getting browser contexts");

        let result: GetBrowserContextsResult = self
            .connection
            .send_command("Target.getBrowserContexts", None::<()>, None)
            .await?;

        let mut contexts = Vec::new();

        // Always include the default context (empty string ID)
        // The default context represents the browser's main profile
        contexts.push(BrowserContext::from_existing(
            self.connection.clone(),
            String::new(), // Empty string = default context
        ));

        // Add other contexts
        for context_id in result.browser_context_ids {
            if !context_id.is_empty() {
                contexts.push(BrowserContext::from_existing(
                    self.connection.clone(),
                    context_id,
                ));
            }
        }

        info!(count = contexts.len(), "Found browser contexts");

        Ok(contexts)
    }

    /// Create a browser from an existing connection and process (legacy, no temp dir).
    pub(crate) fn from_connection_and_process(connection: CdpConnection, process: Child) -> Self {
        Self {
            connection: Arc::new(connection),
            process: Some(Mutex::new(process)),
            owned: true,
            _temp_user_data_dir: None,
        }
    }

    /// Create a browser from a launch operation with optional temp directory.
    pub(crate) fn from_launch(
        connection: CdpConnection,
        process: Child,
        temp_user_data_dir: Option<TempDir>,
    ) -> Self {
        Self {
            connection: Arc::new(connection),
            process: Some(Mutex::new(process)),
            owned: true,
            _temp_user_data_dir: temp_user_data_dir,
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

        // Build CDP params with proxy configuration if specified
        let create_params = match &options.proxy {
            Some(proxy) => CreateBrowserContextParams {
                dispose_on_detach: None,
                proxy_server: Some(proxy.server.clone()),
                proxy_bypass_list: proxy.bypass.clone(),
            },
            None => CreateBrowserContextParams::default(),
        };

        let result: CreateBrowserContextResult = self
            .connection
            .send_command("Target.createBrowserContext", Some(create_params), None)
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
    /// If this browser was launched by us, the process will be terminated
    /// and properly reaped to prevent zombie processes.
    /// If it was connected to, only the WebSocket connection is closed.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    pub async fn close(&self) -> Result<(), BrowserError> {
        // If we own the process, terminate it and reap it
        if let Some(ref process_mutex) = self.process {
            let mut child = process_mutex.lock().await;
            process::kill_and_reap_async(&mut child).await;
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

impl Drop for Browser {
    fn drop(&mut self) {
        // Try to kill and reap the process if we own it
        if self.owned {
            if let Some(ref process_mutex) = self.process {
                // We can't await in drop, so we try to kill synchronously
                if let Ok(mut guard) = process_mutex.try_lock() {
                    // Use the sync helper with 10 attempts and 10ms delay between attempts (100ms total)
                    process::kill_and_reap_sync(&mut guard, 10, Duration::from_millis(10));
                }
            }
        }
    }
}
