//! Route handler registry and dispatch.

mod constructors;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::RwLock;
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::fetch::{AuthRequiredEvent, RequestPausedEvent};

use super::auth::{AuthHandler, HttpCredentials};
use super::handler_fetch::{disable_fetch, enable_fetch};
use super::handler_request::{continue_request, create_route_from_event};
use super::route::Route;
use super::types::{UrlMatcher, UrlPattern};
use crate::error::NetworkError;

/// A registered route handler.
struct RegisteredHandler {
    /// Pattern to match URLs.
    pattern: Box<dyn UrlMatcher>,
    /// The handler function.
    handler: Arc<
        dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
            + Send
            + Sync,
    >,
}

impl std::fmt::Debug for RegisteredHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredHandler")
            .field("pattern", &"<pattern>")
            .field("handler", &"<fn>")
            .finish()
    }
}

/// Route handler registry for a page or context.
#[derive(Debug)]
pub struct RouteHandlerRegistry {
    /// Registered handlers (in reverse order - last registered is first tried).
    handlers: RwLock<Vec<RegisteredHandler>>,
    /// CDP connection for sending commands.
    connection: Arc<CdpConnection>,
    /// Session ID for CDP commands.
    session_id: String,
    /// Whether the Fetch domain is enabled.
    fetch_enabled: RwLock<bool>,
    /// HTTP authentication handler.
    auth_handler: AuthHandler,
    /// Whether auth handling is enabled.
    auth_enabled: RwLock<bool>,
    /// Context-level route registry (for fallback handling).
    context_routes: Option<Arc<crate::context::routing::ContextRouteRegistry>>,
}

impl RouteHandlerRegistry {
    /// Enable Fetch domain if context has routes or auth is enabled.
    ///
    /// This should be called after the registry is created to check if there are
    /// context-level routes that need interception or if HTTP credentials are configured.
    pub async fn enable_fetch_for_context_routes(&self) -> Result<(), NetworkError> {
        // Enable if auth is enabled (credentials were provided)
        let auth_enabled = *self.auth_enabled.read().await;
        if auth_enabled {
            self.ensure_fetch_enabled().await?;
            return Ok(());
        }

        // Also enable if there are context routes
        if let Some(ref context_routes) = self.context_routes {
            if context_routes.has_routes().await {
                self.ensure_fetch_enabled().await?;
            }
        }
        Ok(())
    }

    /// Set the context-level route registry.
    pub fn set_context_routes(
        &mut self,
        context_routes: Arc<crate::context::routing::ContextRouteRegistry>,
    ) {
        self.context_routes = Some(context_routes);
    }

    /// Start the background fetch event listener.
    ///
    /// This spawns a background task that listens for `Fetch.requestPaused` and
    /// `Fetch.authRequired` events and dispatches them to the appropriate handlers.
    ///
    /// Also listens for context route change notifications to enable Fetch when
    /// new routes are added to the context after the page was created.
    ///
    /// This should be called after creating the registry, passing an Arc reference to self.
    pub fn start_fetch_listener(self: &Arc<Self>) {
        let mut events = self.connection.subscribe_events();
        let session_id = self.session_id.clone();
        let registry = Arc::clone(self);

        // Subscribe to context route changes if we have context routes
        let mut route_change_rx = self
            .context_routes
            .as_ref()
            .map(|ctx| ctx.subscribe_route_changes());
        let registry_for_routes = Arc::clone(self);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Handle CDP events
                    event_result = events.recv() => {
                        let Ok(event) = event_result else {
                            break;
                        };

                        // Filter for this session
                        if event.session_id.as_deref() != Some(&session_id) {
                            continue;
                        }

                        match event.method.as_str() {
                            "Fetch.requestPaused" => {
                                if let Some(params) = &event.params {
                                    if let Ok(paused_event) = serde_json::from_value::<RequestPausedEvent>(params.clone()) {
                                        tracing::debug!(
                                            request_id = %paused_event.request_id,
                                            url = %paused_event.request.url,
                                            "Fetch.requestPaused received"
                                        );
                                        if let Err(e) = registry.handle_request(&paused_event).await {
                                            tracing::warn!(
                                                request_id = %paused_event.request_id,
                                                error = %e,
                                                "Failed to handle paused request"
                                            );
                                        }
                                    }
                                }
                            }
                            "Fetch.authRequired" => {
                                if let Some(params) = &event.params {
                                    if let Ok(auth_event) = serde_json::from_value::<AuthRequiredEvent>(params.clone()) {
                                        tracing::debug!(
                                            request_id = %auth_event.request_id,
                                            origin = %auth_event.auth_challenge.origin,
                                            scheme = %auth_event.auth_challenge.scheme,
                                            "Fetch.authRequired received"
                                        );
                                        if let Err(e) = registry.handle_auth_required(&auth_event).await {
                                            tracing::warn!(
                                                request_id = %auth_event.request_id,
                                                error = %e,
                                                "Failed to handle auth required"
                                            );
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    // Handle context route change notifications
                    Some(Ok(_notification)) = async {
                        match route_change_rx.as_mut() {
                            Some(rx) => Some(rx.recv().await),
                            None => std::future::pending().await,
                        }
                    } => {
                        // A new route was added to the context - enable Fetch if not already
                        tracing::debug!("Context route added, ensuring Fetch is enabled");
                        if let Err(e) = registry_for_routes.ensure_fetch_enabled().await {
                            tracing::warn!(error = %e, "Failed to enable Fetch after context route added");
                        }
                    }
                }
            }
        });
    }

    /// Set HTTP credentials for authentication.
    pub async fn set_http_credentials(&self, credentials: HttpCredentials) {
        self.auth_handler.set_credentials(credentials).await;

        // Enable auth handling if not already enabled
        let mut auth_enabled = self.auth_enabled.write().await;
        if !*auth_enabled {
            *auth_enabled = true;
            // Re-enable fetch with auth handling if fetch is already enabled
            drop(auth_enabled);
            let fetch_enabled = *self.fetch_enabled.read().await;
            if fetch_enabled {
                let _ = self.re_enable_fetch_with_auth().await;
            }
        }
    }

    /// Clear HTTP credentials.
    pub async fn clear_http_credentials(&self) {
        self.auth_handler.clear_credentials().await;
        let mut auth_enabled = self.auth_enabled.write().await;
        *auth_enabled = false;
    }

    /// Handle an authentication challenge.
    pub async fn handle_auth_required(
        &self,
        event: &AuthRequiredEvent,
    ) -> Result<(), NetworkError> {
        self.auth_handler.handle_auth_challenge(event).await?;
        Ok(())
    }

    /// Register a route handler for the given pattern.
    pub async fn route<M, H, Fut>(&self, pattern: M, handler: H) -> Result<(), NetworkError>
    where
        M: Into<UrlPattern>,
        H: Fn(Route) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), NetworkError>> + Send + 'static,
    {
        let pattern = pattern.into();

        // Enable Fetch domain if not already enabled
        self.ensure_fetch_enabled().await?;

        // Wrap the handler
        let handler: Arc<
            dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
                + Send
                + Sync,
        > = Arc::new(move |route| Box::pin(handler(route)));

        // Add to handlers (will be matched in reverse order)
        let mut handlers = self.handlers.write().await;
        handlers.push(RegisteredHandler {
            pattern: Box::new(pattern),
            handler,
        });

        Ok(())
    }

    /// Register a route handler with a predicate function.
    pub async fn route_predicate<P, H, Fut>(
        &self,
        predicate: P,
        handler: H,
    ) -> Result<(), NetworkError>
    where
        P: Fn(&str) -> bool + Send + Sync + 'static,
        H: Fn(Route) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), NetworkError>> + Send + 'static,
    {
        // Enable Fetch domain if not already enabled
        self.ensure_fetch_enabled().await?;

        // Create a matcher from the predicate
        struct PredicateMatcher<F>(F);
        impl<F: Fn(&str) -> bool + Send + Sync> UrlMatcher for PredicateMatcher<F> {
            fn matches(&self, url: &str) -> bool {
                (self.0)(url)
            }
        }

        // Wrap the handler
        let handler: Arc<
            dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
                + Send
                + Sync,
        > = Arc::new(move |route| Box::pin(handler(route)));

        // Add to handlers
        let mut handlers = self.handlers.write().await;
        handlers.push(RegisteredHandler {
            pattern: Box::new(PredicateMatcher(predicate)),
            handler,
        });

        Ok(())
    }

    /// Unregister handlers matching the given pattern.
    pub async fn unroute(&self, pattern: &str) {
        let mut handlers = self.handlers.write().await;

        // Remove handlers that match this pattern
        // For simplicity, we match based on glob pattern equality
        handlers.retain(|h| {
            // This is a simplification - in a real implementation,
            // we'd need to compare patterns more thoroughly
            !h.pattern.matches(pattern)
        });

        // If no handlers left, disable Fetch domain
        if handlers.is_empty() {
            drop(handlers);
            let _ = self.disable_fetch_domain().await;
        }
    }

    /// Unregister all handlers.
    pub async fn unroute_all(&self) {
        let mut handlers = self.handlers.write().await;
        handlers.clear();
        drop(handlers);
        let _ = self.disable_fetch_domain().await;
    }

    /// Handle a paused request by dispatching to matching handlers.
    ///
    /// Handlers are tried in reverse order (LIFO). If a handler calls `fallback()`,
    /// the next matching handler is tried. If no handler handles the request,
    /// it is continued to the network.
    pub async fn handle_request(&self, event: &RequestPausedEvent) -> Result<(), NetworkError> {
        let url = &event.request.url;
        let handlers = self.handlers.read().await;

        // Collect all matching handlers (in reverse order - LIFO)
        let matching_handlers: Vec<_> = handlers
            .iter()
            .rev()
            .filter(|h| h.pattern.matches(url))
            .collect();

        // Try each matching handler in order
        for handler in &matching_handlers {
            let route =
                create_route_from_event(event, self.connection.clone(), self.session_id.clone());
            let route_check = route.clone();

            // Call the handler (handler takes ownership of route)
            (handler.handler)(route).await?;

            // Check if the route was actually handled (made a CDP call)
            if route_check.is_handled().await {
                return Ok(());
            }
            tracing::debug!(
                request_id = %event.request_id,
                url = %url,
                "Handler called fallback, trying next handler"
            );
        }

        // Drop page handlers lock before checking context routes
        drop(handlers);

        // Check context routes as fallback
        if let Some(ref context_routes) = self.context_routes {
            let context_handlers = context_routes.find_all_handlers(url).await;

            for handler in context_handlers {
                let route = create_route_from_event(
                    event,
                    self.connection.clone(),
                    self.session_id.clone(),
                );
                let route_check = route.clone();

                handler(route).await?;

                if route_check.is_handled().await {
                    return Ok(());
                }
                tracing::debug!(
                    request_id = %event.request_id,
                    url = %url,
                    "Context handler called fallback, trying next handler"
                );
            }
        }

        // No handler handled the request - continue to the network
        continue_request(&self.connection, &self.session_id, &event.request_id).await
    }

    /// Enable the Fetch domain if not already enabled.
    ///
    /// This is a public version for use by `ContextRouteRegistry` when
    /// synchronously enabling Fetch on all pages after a context route is added.
    pub async fn ensure_fetch_enabled_public(&self) -> Result<(), NetworkError> {
        self.ensure_fetch_enabled().await
    }

    /// Enable the Fetch domain if not already enabled.
    async fn ensure_fetch_enabled(&self) -> Result<(), NetworkError> {
        let mut enabled = self.fetch_enabled.write().await;
        if *enabled {
            return Ok(());
        }

        let auth_enabled = *self.auth_enabled.read().await;
        enable_fetch(&self.connection, &self.session_id, auth_enabled).await?;
        *enabled = true;
        Ok(())
    }

    /// Re-enable Fetch domain with auth handling.
    async fn re_enable_fetch_with_auth(&self) -> Result<(), NetworkError> {
        // First disable, then re-enable with auth
        disable_fetch(&self.connection, &self.session_id).await?;
        enable_fetch(&self.connection, &self.session_id, true).await
    }

    /// Disable the Fetch domain.
    async fn disable_fetch_domain(&self) -> Result<(), NetworkError> {
        let mut enabled = self.fetch_enabled.write().await;
        if !*enabled {
            return Ok(());
        }

        disable_fetch(&self.connection, &self.session_id).await?;
        *enabled = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
