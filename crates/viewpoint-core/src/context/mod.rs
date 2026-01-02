//! # Browser Context Management
//!
//! Browser contexts are isolated environments within a browser, similar to incognito windows.
//! Each context has its own cookies, cache, localStorage, and other browser storage.
//!
//! ## Features
//!
//! - **Isolation**: Each context is completely isolated from others
//! - **Cookie Management**: Add, get, and clear cookies with [`BrowserContext::add_cookies`], [`BrowserContext::cookies`], [`BrowserContext::clear_cookies`]
//! - **Storage State**: Save and restore browser state (cookies, localStorage) for authentication
//! - **Permissions**: Grant permissions like geolocation, camera, microphone
//! - **Geolocation**: Mock browser location with [`BrowserContext::set_geolocation`]
//! - **HTTP Credentials**: Configure basic/digest authentication
//! - **Extra Headers**: Add headers to all requests in the context
//! - **Offline Mode**: Simulate network offline conditions
//! - **Event Handling**: Listen for page creation and context close events
//! - **Init Scripts**: Run scripts before every page load
//! - **Custom Test ID**: Configure which attribute is used for test IDs
//! - **Network Routing**: Intercept and mock requests at the context level
//! - **HAR Recording**: Record network traffic for debugging
//! - **Tracing**: Record traces for debugging
//!
//! ## Quick Start
//!
//! ```no_run
//! use viewpoint_core::{Browser, BrowserContext, Permission};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! let browser = Browser::launch().headless(true).launch().await?;
//!
//! // Create a simple context
//! let context = browser.new_context().await?;
//!
//! // Create a context with options
//! let context = browser.new_context_builder()
//!     .viewport(1920, 1080)
//!     .geolocation(37.7749, -122.4194)
//!     .permissions(vec![Permission::Geolocation])
//!     .build()
//!     .await?;
//!
//! // Create a page in the context
//! let page = context.new_page().await?;
//! page.goto("https://example.com").goto().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Cookie Management
//!
//! ```ignore
//! use viewpoint_core::{Browser, Cookie, SameSite};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! // Add cookies
//! context.add_cookies(vec![
//!     Cookie {
//!         name: "session".to_string(),
//!         value: "abc123".to_string(),
//!         domain: Some(".example.com".to_string()),
//!         path: Some("/".to_string()),
//!         expires: None,
//!         http_only: Some(true),
//!         secure: Some(true),
//!         same_site: Some(SameSite::Lax),
//!     }
//! ]).await?;
//!
//! // Get all cookies
//! let cookies = context.cookies(None).await?;
//!
//! // Get cookies for specific URLs
//! let cookies = context.cookies(Some(&["https://example.com"])).await?;
//!
//! // Clear all cookies
//! context.clear_cookies().clear().await?;
//!
//! // Clear cookies matching a pattern
//! context.clear_cookies()
//!     .domain("example.com")
//!     .clear()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Storage State
//!
//! Save and restore browser state for authentication:
//!
//! ```ignore
//! use viewpoint_core::{Browser, StorageStateSource};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! // Save storage state after login
//! context.storage_state()
//!     .path("auth.json")
//!     .save()
//!     .await?;
//!
//! // Create a new context with saved state
//! let context = browser.new_context_builder()
//!     .storage_state_path("auth.json")
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Event Handling
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! // Listen for new pages
//! let handler_id = context.on_page(|page_info| async move {
//!     println!("New page created: {}", page_info.target_id);
//!     Ok(())
//! }).await;
//!
//! // Listen for context close
//! context.on_close(|| async {
//!     println!("Context closed");
//!     Ok(())
//! }).await;
//!
//! // Remove handler later
//! context.off_page(handler_id).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Tracing
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! // Start tracing
//! context.tracing().start().await?;
//!
//! // ... perform actions ...
//!
//! // Stop and save trace
//! context.tracing().stop("trace.zip").await?;
//! # Ok(())
//! # }
//! ```

mod api;
pub mod binding;
mod construction;
mod cookies;
mod emulation;
pub mod events;
mod har;
mod page_events;
mod page_factory;
mod page_management;
mod permissions;
pub mod routing;
mod routing_impl;
mod scripts;
pub mod storage;
mod storage_restore;
mod test_id;
pub mod trace;
mod tracing_access;
pub mod types;
mod weberror;

pub use cookies::ClearCookiesBuilder;
pub use emulation::SetGeolocationBuilder;

// HashMap is used in emulation.rs
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::target_domain::DisposeBrowserContextParams;

use crate::error::ContextError;

pub use events::{ContextEventManager, HandlerId};
pub use storage::{StorageStateBuilder, StorageStateOptions};
use trace::TracingState;
pub use trace::{Tracing, TracingOptions};
pub use types::{
    ColorScheme, ContextOptions, ContextOptionsBuilder, Cookie, ForcedColors, Geolocation,
    HttpCredentials, IndexedDbDatabase, IndexedDbEntry, IndexedDbIndex, IndexedDbObjectStore,
    LocalStorageEntry, Permission, ProxyConfig, ReducedMotion, SameSite, StorageOrigin,
    StorageState, StorageStateSource, ViewportSize,
};
pub use weberror::WebErrorHandler;
// Re-export WebError for context-level usage
pub use crate::page::page_error::WebError;

/// Default test ID attribute name.
pub const DEFAULT_TEST_ID_ATTRIBUTE: &str = "data-testid";

/// An isolated browser context.
///
/// Browser contexts are similar to incognito windows - they have their own
/// cookies, cache, and storage that are isolated from other contexts.
///
/// # Features
///
/// - **Cookie Management**: Add, get, and clear cookies
/// - **Storage State**: Save and restore browser state
/// - **Permissions**: Grant permissions like geolocation, camera, etc.
/// - **Geolocation**: Mock browser location
/// - **HTTP Credentials**: Basic/Digest authentication
/// - **Extra Headers**: Add headers to all requests
/// - **Offline Mode**: Simulate network offline conditions
/// - **Event Handling**: Listen for page creation and context close events
/// - **Init Scripts**: Scripts that run before every page load
/// - **Custom Test ID**: Configure which attribute is used for test IDs
///
/// # Ownership
///
/// Contexts can be either "owned" (created by us) or "external" (discovered when
/// connecting to an existing browser). When closing an external context, the
/// underlying browser context is not disposed - only our connection to it is closed.
pub struct BrowserContext {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Browser context ID.
    context_id: String,
    /// Whether the context has been closed.
    closed: bool,
    /// Whether we own this context (created it) vs discovered it.
    /// Owned contexts are disposed when closed; external contexts are not.
    owned: bool,
    /// Created pages (weak tracking for `pages()` method).
    pages: Arc<RwLock<Vec<PageInfo>>>,
    /// Default timeout for actions.
    default_timeout: Duration,
    /// Default timeout for navigation.
    default_navigation_timeout: Duration,
    /// Context options used to create this context.
    options: ContextOptions,
    /// Web error handler.
    weberror_handler: Arc<RwLock<Option<WebErrorHandler>>>,
    /// Event manager for context-level events.
    event_manager: Arc<ContextEventManager>,
    /// Context-level route registry.
    route_registry: Arc<routing::ContextRouteRegistry>,
    /// Context-level binding registry.
    binding_registry: Arc<binding::ContextBindingRegistry>,
    /// Init scripts to run on every page load.
    init_scripts: Arc<RwLock<Vec<String>>>,
    /// Custom test ID attribute name (defaults to "data-testid").
    test_id_attribute: Arc<RwLock<String>>,
    /// HAR recorder for capturing network traffic.
    har_recorder: Arc<RwLock<Option<crate::network::HarRecorder>>>,
    /// Shared tracing state for persistent tracing across `tracing()` calls.
    tracing_state: Arc<RwLock<TracingState>>,
}

// Manual Debug implementation since WebErrorHandler doesn't implement Debug
impl std::fmt::Debug for BrowserContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserContext")
            .field("context_id", &self.context_id)
            .field("closed", &self.closed)
            .field("owned", &self.owned)
            .field("default_timeout", &self.default_timeout)
            .field(
                "default_navigation_timeout",
                &self.default_navigation_timeout,
            )
            .finish_non_exhaustive()
    }
}

/// Information about a page in the context.
#[derive(Debug, Clone)]
pub struct PageInfo {
    /// Target ID.
    pub target_id: String,
    /// Session ID (may be empty if not tracked).
    pub session_id: String,
}

impl BrowserContext {
    // Construction methods (new, with_options, from_existing, apply_options) are in construction.rs

    // Page management methods (new_page, pages) are in page_management.rs

    // Cookie methods are in cookies.rs

    // Storage state methods are in storage.rs

    // Permissions methods are in permissions.rs

    // =========================================================================
    // Geolocation
    // =========================================================================

    /// Set the geolocation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::BrowserContext;
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), viewpoint_core::CoreError> {
    /// // San Francisco
    /// context.set_geolocation(37.7749, -122.4194).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if setting geolocation fails.
    pub fn set_geolocation(&self, latitude: f64, longitude: f64) -> SetGeolocationBuilder<'_> {
        SetGeolocationBuilder::new(self, latitude, longitude)
    }

    // clear_geolocation, set_extra_http_headers, set_offline are in emulation.rs

    // =========================================================================
    // Ownership and Status
    // =========================================================================

    /// Check if this context is owned (created by us) or external.
    ///
    /// External contexts are discovered when connecting to an already-running browser.
    /// They are not disposed when closed.
    pub fn is_owned(&self) -> bool {
        self.owned
    }

    /// Check if this is the default browser context.
    ///
    /// The default context represents the browser's main profile and has an empty ID.
    pub fn is_default(&self) -> bool {
        self.context_id.is_empty()
    }

    // =========================================================================
    // Timeout Configuration
    // =========================================================================

    /// Set the default timeout for actions.
    ///
    /// This timeout is used for actions like clicking, typing, etc.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Get the default timeout for actions.
    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }

    /// Set the default navigation timeout.
    ///
    /// This timeout is used for navigation operations like goto, reload, etc.
    pub fn set_default_navigation_timeout(&mut self, timeout: Duration) {
        self.default_navigation_timeout = timeout;
    }

    /// Get the default navigation timeout.
    pub fn default_navigation_timeout(&self) -> Duration {
        self.default_navigation_timeout
    }

    // Init script methods are in scripts.rs

    // =========================================================================
    // Context Lifecycle
    // =========================================================================

    /// Close this browser context and all its pages.
    ///
    /// For contexts we created (owned), this disposes the context via CDP.
    /// For external contexts (discovered when connecting to an existing browser),
    /// this only closes our connection without disposing the context.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    #[instrument(level = "info", skip(self), fields(context_id = %self.context_id, owned = self.owned))]
    pub async fn close(&mut self) -> Result<(), ContextError> {
        if self.closed {
            debug!("Context already closed");
            return Ok(());
        }

        info!("Closing browser context");

        // Auto-save HAR if recording is active
        if let Some(recorder) = self.har_recorder.write().await.take() {
            if let Err(e) = recorder.save().await {
                debug!("Failed to auto-save HAR on close: {}", e);
            } else {
                debug!(path = %recorder.path().display(), "Auto-saved HAR on close");
            }
        }

        // Emit close event before cleanup
        self.event_manager.emit_close().await;

        // Only dispose the context if we own it
        // External contexts (from connecting to existing browser) should not be disposed
        if self.owned && !self.context_id.is_empty() {
            debug!("Disposing owned browser context");
            self.connection
                .send_command::<_, serde_json::Value>(
                    "Target.disposeBrowserContext",
                    Some(DisposeBrowserContextParams {
                        browser_context_id: self.context_id.clone(),
                    }),
                    None,
                )
                .await?;
        } else {
            debug!("Skipping dispose for external/default context");
        }

        // Clear all event handlers
        self.event_manager.clear().await;

        self.closed = true;
        info!("Browser context closed");
        Ok(())
    }

    /// Get the context ID.
    pub fn id(&self) -> &str {
        &self.context_id
    }

    /// Check if this context has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get a reference to the CDP connection.
    pub fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }

    /// Get the context ID.
    pub fn context_id(&self) -> &str {
        &self.context_id
    }

    // Web error event methods are in weberror.rs (on_weberror, off_weberror)

    // Page and close event methods are in page_events.rs (on_page, off_page, on_close, off_close, wait_for_page)

    // Context-level routing methods are in routing_impl.rs

    // HAR recording methods are in har.rs

    // Exposed function methods are in binding.rs (expose_function, remove_exposed_function)

    // API request context methods are in api.rs (request, sync_cookies_from_api)

    // Tracing method is in tracing_access.rs

    // Test ID attribute methods are in test_id.rs
}

// ClearCookiesBuilder is in cookies.rs
// SetGeolocationBuilder is in emulation.rs
