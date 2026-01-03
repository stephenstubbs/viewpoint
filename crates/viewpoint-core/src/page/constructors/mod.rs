//! Page constructor methods.
//!
//! This module contains factory methods for creating Page instances.

use std::sync::Arc;

use viewpoint_cdp::CdpConnection;

use crate::error::NetworkError;
use crate::network::{RouteHandlerRegistry, WebSocketManager};

use super::binding;
use super::events::PageEventManager;
use super::frame::ExecutionContextRegistry;
use super::keyboard::Keyboard;
use super::locator_handler::LocatorHandlerManager;
use super::mouse::Mouse;
use super::popup;
use super::touchscreen::Touchscreen;
use super::video::{Video, VideoOptions};
use super::{DEFAULT_TEST_ID_ATTRIBUTE, Page};

impl Page {
    /// Create a new page.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        target_id: String,
        session_id: String,
        frame_id: String,
    ) -> Self {
        Self::build_page(connection, target_id, session_id, frame_id, 0, 0, None, None)
    }

    /// Create a new page with context and page indices.
    pub(crate) fn new_with_indices(
        connection: Arc<CdpConnection>,
        target_id: String,
        session_id: String,
        frame_id: String,
        context_index: usize,
        page_index: usize,
    ) -> Self {
        Self::build_page(
            connection,
            target_id,
            session_id,
            frame_id,
            context_index,
            page_index,
            None,
            None,
        )
    }

    /// Create a new page with video recording enabled.
    pub(crate) fn with_video(
        connection: Arc<CdpConnection>,
        target_id: String,
        session_id: String,
        frame_id: String,
        video_options: VideoOptions,
    ) -> Self {
        Self::build_page(
            connection,
            target_id,
            session_id,
            frame_id,
            0,
            0,
            Some(video_options),
            None,
        )
    }

    /// Create a new page with video recording and indices.
    pub(crate) fn with_video_and_indices(
        connection: Arc<CdpConnection>,
        target_id: String,
        session_id: String,
        frame_id: String,
        context_index: usize,
        page_index: usize,
        video_options: VideoOptions,
    ) -> Self {
        Self::build_page(
            connection,
            target_id,
            session_id,
            frame_id,
            context_index,
            page_index,
            Some(video_options),
            None,
        )
    }

    /// Create a new page with an opener (for popups).
    pub(crate) fn with_opener(
        connection: Arc<CdpConnection>,
        target_id: String,
        session_id: String,
        frame_id: String,
        opener_target_id: String,
    ) -> Self {
        Self::build_page(
            connection,
            target_id,
            session_id,
            frame_id,
            0,
            0,
            None,
            Some(opener_target_id),
        )
    }

    /// Internal helper to build a page with optional video and opener.
    fn build_page(
        connection: Arc<CdpConnection>,
        target_id: String,
        session_id: String,
        frame_id: String,
        context_index: usize,
        page_index: usize,
        video_options: Option<VideoOptions>,
        opener_target_id: Option<String>,
    ) -> Self {
        let route_registry = Arc::new(RouteHandlerRegistry::new(
            connection.clone(),
            session_id.clone(),
        ));
        // Note: Don't start the fetch listener here - it will be started either:
        // - When with_context_routes() is called (which creates a new registry with credentials), OR
        // - When route() is called on this page directly (lazy start)
        // This avoids duplicate listeners when context routes are applied.
        let keyboard = Keyboard::new(connection.clone(), session_id.clone(), frame_id.clone());
        let mouse = Mouse::new(connection.clone(), session_id.clone());
        let touchscreen = Touchscreen::new(connection.clone(), session_id.clone());
        let event_manager = Arc::new(PageEventManager::new(
            connection.clone(),
            session_id.clone(),
        ));
        let locator_handler_manager = Arc::new(LocatorHandlerManager::new());
        let popup_manager = Arc::new(popup::PopupManager::new(
            connection.clone(),
            session_id.clone(),
            target_id.clone(),
        ));
        let websocket_manager = Arc::new(WebSocketManager::new(
            connection.clone(),
            session_id.clone(),
        ));
        let binding_manager = Arc::new(binding::BindingManager::new(
            connection.clone(),
            session_id.clone(),
        ));
        let video_controller = video_options.map(|opts| {
            Arc::new(Video::with_options(
                connection.clone(),
                session_id.clone(),
                opts,
            ))
        });

        // Create and start the execution context registry
        let context_registry = Arc::new(ExecutionContextRegistry::new(
            connection.clone(),
            session_id.clone(),
        ));
        context_registry.start_listening();

        Self {
            connection,
            target_id,
            session_id,
            frame_id,
            context_index,
            page_index,
            closed: false,
            route_registry,
            keyboard,
            mouse,
            touchscreen,
            event_manager,
            locator_handler_manager,
            video_controller,
            opener_target_id,
            popup_manager,
            websocket_manager,
            binding_manager,
            test_id_attribute: DEFAULT_TEST_ID_ATTRIBUTE.to_string(),
            context_registry,
            ref_map: std::sync::Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Create a new page with a custom test ID attribute.
    pub(crate) fn with_test_id_attribute(mut self, attribute: String) -> Self {
        self.test_id_attribute = attribute;
        self
    }

    /// Set context-level routes for this page.
    ///
    /// Context routes are checked as a fallback when no page-level route matches.
    /// If `http_credentials` is provided, they will be used for HTTP authentication.
    ///
    /// This method also registers the page's route registry with the context
    /// so that future `context.route()` calls can synchronously enable Fetch on this page.
    pub(crate) async fn with_context_routes(
        self,
        context_routes: Arc<crate::context::routing::ContextRouteRegistry>,
        http_credentials: Option<crate::network::auth::HttpCredentials>,
    ) -> Self {
        self.with_context_routes_and_proxy(context_routes, http_credentials, None)
            .await
    }

    /// Set context-level routes for this page with optional proxy credentials.
    ///
    /// Context routes are checked as a fallback when no page-level route matches.
    /// If `http_credentials` is provided, they will be used for HTTP authentication.
    /// If `proxy_credentials` is provided, they will be used for proxy authentication.
    ///
    /// This method also registers the page's route registry with the context
    /// so that future `context.route()` calls can synchronously enable Fetch on this page.
    pub(crate) async fn with_context_routes_and_proxy(
        self,
        context_routes: Arc<crate::context::routing::ContextRouteRegistry>,
        http_credentials: Option<crate::network::auth::HttpCredentials>,
        proxy_credentials: Option<crate::network::auth::ProxyCredentials>,
    ) -> Self {
        // Create a new registry with context routes and optional credentials
        let new_registry = Arc::new(RouteHandlerRegistry::with_context_routes_and_proxy(
            self.connection.clone(),
            self.session_id.clone(),
            context_routes.clone(),
            http_credentials,
            proxy_credentials,
        ));
        // Start the fetch event listener for route handling and authentication
        new_registry.start_fetch_listener();

        // Register this page's registry with the context so that future
        // context.route() calls can synchronously enable Fetch on this page
        context_routes.register_page_registry(&new_registry).await;

        Self {
            route_registry: new_registry,
            ..self
        }
    }

    /// Enable Fetch domain if there are context-level routes.
    ///
    /// This is called after page creation to ensure the Fetch domain is enabled
    /// when there are context-level routes that need to intercept requests.
    pub(crate) async fn enable_fetch_for_context_routes(&self) -> Result<(), NetworkError> {
        self.route_registry.enable_fetch_for_context_routes().await
    }

    /// Create an internal clone of the page for event handlers.
    ///
    /// This creates a new Page instance that shares the underlying connection
    /// and other Arc-wrapped resources. Used internally for passing pages
    /// to event handlers.
    pub(crate) fn clone_internal(&self) -> Self {
        Self {
            connection: self.connection.clone(),
            target_id: self.target_id.clone(),
            session_id: self.session_id.clone(),
            frame_id: self.frame_id.clone(),
            context_index: self.context_index,
            page_index: self.page_index,
            closed: self.closed,
            route_registry: self.route_registry.clone(),
            keyboard: Keyboard::new(
                self.connection.clone(),
                self.session_id.clone(),
                self.frame_id.clone(),
            ),
            mouse: Mouse::new(self.connection.clone(), self.session_id.clone()),
            touchscreen: Touchscreen::new(self.connection.clone(), self.session_id.clone()),
            event_manager: self.event_manager.clone(),
            locator_handler_manager: self.locator_handler_manager.clone(),
            video_controller: self.video_controller.clone(),
            opener_target_id: self.opener_target_id.clone(),
            popup_manager: self.popup_manager.clone(),
            websocket_manager: self.websocket_manager.clone(),
            binding_manager: self.binding_manager.clone(),
            test_id_attribute: self.test_id_attribute.clone(),
            context_registry: self.context_registry.clone(),
            ref_map: self.ref_map.clone(),
        }
    }

    /// Get the execution context registry.
    pub(crate) fn context_registry(&self) -> &Arc<ExecutionContextRegistry> {
        &self.context_registry
    }
}
