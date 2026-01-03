//! Route handler registry constructors.

use std::sync::Arc;

use viewpoint_cdp::CdpConnection;

use super::RouteHandlerRegistry;
use crate::network::auth::{AuthHandler, HttpCredentials, ProxyCredentials};

impl RouteHandlerRegistry {
    /// Create a new route handler registry.
    pub fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        let auth_handler = AuthHandler::new(connection.clone(), session_id.clone());
        Self {
            handlers: tokio::sync::RwLock::new(Vec::new()),
            connection,
            session_id,
            fetch_enabled: tokio::sync::RwLock::new(false),
            auth_handler,
            auth_enabled: tokio::sync::RwLock::new(false),
            context_routes: None,
        }
    }

    /// Create a new route handler registry with HTTP credentials.
    pub fn with_credentials(
        connection: Arc<CdpConnection>,
        session_id: String,
        credentials: HttpCredentials,
    ) -> Self {
        let auth_handler =
            AuthHandler::with_credentials(connection.clone(), session_id.clone(), credentials);
        Self {
            handlers: tokio::sync::RwLock::new(Vec::new()),
            connection,
            session_id,
            fetch_enabled: tokio::sync::RwLock::new(false),
            auth_handler,
            auth_enabled: tokio::sync::RwLock::new(true),
            context_routes: None,
        }
    }

    /// Create a new route handler registry with context-level routes.
    ///
    /// If `http_credentials` is provided, they will be set on the auth handler
    /// for handling HTTP authentication challenges.
    pub fn with_context_routes(
        connection: Arc<CdpConnection>,
        session_id: String,
        context_routes: Arc<crate::context::routing::ContextRouteRegistry>,
        http_credentials: Option<HttpCredentials>,
    ) -> Self {
        Self::with_context_routes_and_proxy(
            connection,
            session_id,
            context_routes,
            http_credentials,
            None,
        )
    }

    /// Create a new route handler registry with context-level routes and optional proxy credentials.
    ///
    /// If `http_credentials` is provided, they will be set on the auth handler
    /// for handling HTTP authentication challenges.
    /// If `proxy_credentials` is provided, they will be used for proxy authentication.
    pub fn with_context_routes_and_proxy(
        connection: Arc<CdpConnection>,
        session_id: String,
        context_routes: Arc<crate::context::routing::ContextRouteRegistry>,
        http_credentials: Option<HttpCredentials>,
        proxy_credentials: Option<ProxyCredentials>,
    ) -> Self {
        let auth_handler = AuthHandler::new(connection.clone(), session_id.clone());

        // Set HTTP credentials if provided
        if let Some(ref creds) = http_credentials {
            tracing::debug!(
                username = %creds.username,
                has_origin = creds.origin.is_some(),
                "Setting HTTP credentials on auth handler"
            );
            auth_handler.set_credentials_sync(creds.clone());
        }

        // Set proxy credentials if provided
        if let Some(ref proxy_creds) = proxy_credentials {
            tracing::debug!(
                username = %proxy_creds.username,
                "Setting proxy credentials on auth handler"
            );
            auth_handler.set_proxy_credentials_sync(proxy_creds.clone());
        }

        // Enable auth if any credentials are provided
        let auth_enabled = http_credentials.is_some() || proxy_credentials.is_some();

        Self {
            handlers: tokio::sync::RwLock::new(Vec::new()),
            connection,
            session_id,
            fetch_enabled: tokio::sync::RwLock::new(false),
            auth_handler,
            auth_enabled: tokio::sync::RwLock::new(auth_enabled),
            context_routes: Some(context_routes),
        }
    }
}
