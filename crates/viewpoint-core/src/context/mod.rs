//! Browser context management.

mod api;
pub mod binding;
mod cookies;
mod emulation;
pub mod events;
mod har;
mod page_events;
mod page_factory;
mod permissions;
pub mod routing;
mod routing_impl;
mod scripts;
pub mod storage;
mod storage_restore;
pub mod trace;
mod test_id;
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

use viewpoint_cdp::protocol::target_domain::{
    DisposeBrowserContextParams, GetTargetsParams, GetTargetsResult,
};
use viewpoint_cdp::CdpConnection;

use crate::error::ContextError;
use crate::page::Page;

pub use events::{ContextEventManager, HandlerId};
pub use weberror::WebErrorHandler;
pub use storage::{StorageStateBuilder, StorageStateOptions};
pub use trace::{Tracing, TracingOptions};
use trace::TracingState;
pub use types::{
    ColorScheme, ContextOptions, ContextOptionsBuilder, Cookie, ForcedColors, Geolocation,
    HttpCredentials, IndexedDbDatabase, IndexedDbEntry, IndexedDbIndex, IndexedDbObjectStore,
    LocalStorageEntry, Permission, ReducedMotion, SameSite, StorageOrigin,
    StorageState, StorageStateSource, ViewportSize,
};
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
pub struct BrowserContext {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Browser context ID.
    context_id: String,
    /// Whether the context has been closed.
    closed: bool,
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
            .field("default_timeout", &self.default_timeout)
            .field("default_navigation_timeout", &self.default_navigation_timeout)
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
    /// Create a new browser context.
    pub(crate) fn new(connection: Arc<CdpConnection>, context_id: String) -> Self {
        debug!(context_id = %context_id, "Created BrowserContext");
        let route_registry = Arc::new(routing::ContextRouteRegistry::new(
            connection.clone(),
            context_id.clone(),
        ));
        let binding_registry = Arc::new(binding::ContextBindingRegistry::new());
        let ctx = Self {
            connection: connection.clone(),
            context_id: context_id.clone(),
            closed: false,
            pages: Arc::new(RwLock::new(Vec::new())),
            default_timeout: Duration::from_secs(30),
            default_navigation_timeout: Duration::from_secs(30),
            options: ContextOptions::default(),
            weberror_handler: Arc::new(RwLock::new(None)),
            event_manager: Arc::new(ContextEventManager::new()),
            route_registry,
            binding_registry,
            init_scripts: Arc::new(RwLock::new(Vec::new())),
            test_id_attribute: Arc::new(RwLock::new(DEFAULT_TEST_ID_ATTRIBUTE.to_string())),
            har_recorder: Arc::new(RwLock::new(None)),
            tracing_state: Arc::new(RwLock::new(TracingState::default())),
        };
        ctx.start_weberror_listener();
        ctx
    }

    /// Create a new browser context with options.
    pub(crate) fn with_options(
        connection: Arc<CdpConnection>,
        context_id: String,
        options: ContextOptions,
    ) -> Self {
        debug!(context_id = %context_id, "Created BrowserContext with options");
        let route_registry = Arc::new(routing::ContextRouteRegistry::new(
            connection.clone(),
            context_id.clone(),
        ));
        let binding_registry = Arc::new(binding::ContextBindingRegistry::new());
        let ctx = Self {
            connection: connection.clone(),
            context_id: context_id.clone(),
            closed: false,
            pages: Arc::new(RwLock::new(Vec::new())),
            default_timeout: options.default_timeout.unwrap_or(Duration::from_secs(30)),
            default_navigation_timeout: options
                .default_navigation_timeout
                .unwrap_or(Duration::from_secs(30)),
            options,
            weberror_handler: Arc::new(RwLock::new(None)),
            event_manager: Arc::new(ContextEventManager::new()),
            route_registry,
            binding_registry,
            init_scripts: Arc::new(RwLock::new(Vec::new())),
            test_id_attribute: Arc::new(RwLock::new(DEFAULT_TEST_ID_ATTRIBUTE.to_string())),
            har_recorder: Arc::new(RwLock::new(None)),
            tracing_state: Arc::new(RwLock::new(TracingState::default())),
        };
        ctx.start_weberror_listener();
        ctx
    }

    // Web error listener is started from weberror.rs

    /// Apply initial options after context creation.
    ///
    /// This is called internally after the context is created to apply
    /// settings like geolocation, permissions, etc.
    pub(crate) async fn apply_options(&self) -> Result<(), ContextError> {
        // Apply geolocation if set
        if let Some(ref geo) = self.options.geolocation {
            self.set_geolocation(geo.latitude, geo.longitude)
                .accuracy(geo.accuracy)
                .await?;
        }

        // Apply permissions if set
        if !self.options.permissions.is_empty() {
            self.grant_permissions(self.options.permissions.clone())
                .await?;
        }

        // Apply extra headers if set
        if !self.options.extra_http_headers.is_empty() {
            self.set_extra_http_headers(self.options.extra_http_headers.clone())
                .await?;
        }

        // Apply offline mode if set
        if self.options.offline {
            self.set_offline(true).await?;
        }

        Ok(())
    }

    // =========================================================================
    // Page Management
    // =========================================================================

    /// Create a new page in this context.
    ///
    /// # Errors
    ///
    /// Returns an error if page creation fails.
    #[instrument(level = "info", skip(self), fields(context_id = %self.context_id))]
    pub async fn new_page(&self) -> Result<Page, ContextError> {
        if self.closed {
            return Err(ContextError::Closed);
        }

        info!("Creating new page");

        // Create target and attach to it
        let (create_result, attach_result) =
            page_factory::create_and_attach_target(&self.connection, &self.context_id).await?;

        let target_id = &create_result.target_id;
        let session_id = &attach_result.session_id;

        // Enable required CDP domains on the page
        page_factory::enable_page_domains(&self.connection, session_id).await?;

        // Apply emulation settings (viewport, touch, locale, etc.)
        page_factory::apply_emulation_settings(&self.connection, session_id, &self.options).await?;

        // Get the main frame ID
        let frame_id = page_factory::get_main_frame_id(&self.connection, session_id).await?;

        // Track the page
        page_factory::track_page(
            &self.pages,
            create_result.target_id.clone(),
            attach_result.session_id.clone(),
        )
        .await;

        // Apply context-level init scripts to the new page
        if let Err(e) = self.apply_init_scripts_to_session(session_id).await {
            debug!("Failed to apply init scripts: {}", e);
        }

        info!(target_id = %target_id, session_id = %session_id, frame_id = %frame_id, "Page created successfully");

        // Get the test ID attribute from context
        let test_id_attr = self.test_id_attribute.read().await.clone();

        // Convert context HTTP credentials to network auth credentials
        let http_credentials = page_factory::convert_http_credentials(&self.options);

        // Create page with or without video recording
        let page = page_factory::create_page_instance(
            self.connection.clone(),
            create_result,
            attach_result,
            frame_id,
            &self.options,
            test_id_attr,
            self.route_registry.clone(),
            http_credentials,
        )
        .await;

        // Enable Fetch domain if there are context-level routes
        // This ensures requests are intercepted for context routes
        if let Err(e) = page.enable_fetch_for_context_routes().await {
            debug!("Failed to enable Fetch for context routes: {}", e);
        }

        // Emit page event to registered handlers
        self.event_manager.emit_page(page.clone_internal()).await;

        Ok(page)
    }

    /// Get all pages in this context.
    ///
    /// # Errors
    ///
    /// Returns an error if querying targets fails.
    pub async fn pages(&self) -> Result<Vec<PageInfo>, ContextError> {
        if self.closed {
            return Err(ContextError::Closed);
        }

        let result: GetTargetsResult = self
            .connection
            .send_command("Target.getTargets", Some(GetTargetsParams::default()), None)
            .await?;

        let pages: Vec<PageInfo> = result
            .target_infos
            .into_iter()
            .filter(|t| {
                t.browser_context_id.as_deref() == Some(&self.context_id)
                    && t.target_type == "page"
            })
            .map(|t| PageInfo {
                target_id: t.target_id,
                session_id: String::new(), // Would need to track sessions
            })
            .collect();

        Ok(pages)
    }

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
    /// # Errors
    ///
    /// Returns an error if closing fails.
    #[instrument(level = "info", skip(self), fields(context_id = %self.context_id))]
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

        self.connection
            .send_command::<_, serde_json::Value>(
                "Target.disposeBrowserContext",
                Some(DisposeBrowserContextParams {
                    browser_context_id: self.context_id.clone(),
                }),
                None,
            )
            .await?;

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
