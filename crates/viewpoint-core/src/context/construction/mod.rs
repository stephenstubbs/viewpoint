//! BrowserContext construction and initialization.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tracing::debug;

use viewpoint_cdp::CdpConnection;

use crate::context::trace::TracingState;
use crate::context::{binding, routing, ContextOptions, DEFAULT_TEST_ID_ATTRIBUTE};
use crate::error::ContextError;

use super::{BrowserContext, ContextEventManager};

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
            owned: true, // We created this context
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
            owned: true, // We created this context
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

    /// Create a wrapper for an existing browser context.
    ///
    /// This is used when connecting to an already-running browser to wrap
    /// contexts that existed before our connection. External contexts are
    /// not disposed when closed - only our connection to them is closed.
    pub(crate) fn from_existing(connection: Arc<CdpConnection>, context_id: String) -> Self {
        let is_default = context_id.is_empty();
        debug!(context_id = %context_id, is_default = is_default, "Wrapping existing BrowserContext");
        let route_registry = Arc::new(routing::ContextRouteRegistry::new(
            connection.clone(),
            context_id.clone(),
        ));
        let binding_registry = Arc::new(binding::ContextBindingRegistry::new());
        let ctx = Self {
            connection: connection.clone(),
            context_id: context_id.clone(),
            closed: false,
            owned: false, // We didn't create this context
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
}
