//! Context-level network routing.
//!
//! This module provides route handlers that apply to all pages in a browser context.
//! Context routes are applied before page routes, allowing context-wide mocking
//! and interception of network requests.

// Allow dead code for context routing scaffolding (spec: network-routing)

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Weak};

use tokio::sync::{broadcast, RwLock};
use tracing::debug;

use viewpoint_cdp::CdpConnection;

use crate::error::NetworkError;
use crate::network::{Route, RouteHandlerRegistry, UrlMatcher, UrlPattern};

/// Notification sent when context routes change.
#[derive(Debug, Clone)]
pub enum RouteChangeNotification {
    /// A new route was added.
    RouteAdded,
}

/// A registered context-level route handler.
struct ContextRouteHandler {
    /// Pattern to match URLs.
    pattern: Box<dyn UrlMatcher>,
    /// The handler function.
    handler: Arc<
        dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
            + Send
            + Sync,
    >,
}

impl std::fmt::Debug for ContextRouteHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextRouteHandler")
            .field("pattern", &"<pattern>")
            .field("handler", &"<fn>")
            .finish()
    }
}

/// Context-level route handler registry.
///
/// Routes registered here apply to all pages in the context.
/// New pages automatically inherit these routes.
#[derive(Debug)]
pub struct ContextRouteRegistry {
    /// Registered handlers (in reverse order - last registered is first tried).
    handlers: RwLock<Vec<ContextRouteHandler>>,
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Context ID.
    context_id: String,
    /// Broadcast channel to notify pages when routes change.
    route_change_tx: broadcast::Sender<RouteChangeNotification>,
    /// Weak references to page route registries.
    /// Used to synchronously enable Fetch when a new context route is added.
    page_registries: RwLock<Vec<Weak<RouteHandlerRegistry>>>,
}

impl ContextRouteRegistry {
    /// Create a new context route registry.
    pub fn new(connection: Arc<CdpConnection>, context_id: String) -> Self {
        // Create broadcast channel with capacity for a few notifications
        let (route_change_tx, _) = broadcast::channel(16);
        Self {
            handlers: RwLock::new(Vec::new()),
            connection,
            context_id,
            route_change_tx,
            page_registries: RwLock::new(Vec::new()),
        }
    }
    
    /// Register a page's route registry with this context.
    /// 
    /// When a new route is added to the context, Fetch will be enabled on all
    /// registered page registries before the `route()` call returns.
    pub async fn register_page_registry(&self, registry: &Arc<RouteHandlerRegistry>) {
        let mut registries = self.page_registries.write().await;
        // Clean up any stale weak references while we're at it
        registries.retain(|weak| weak.strong_count() > 0);
        registries.push(Arc::downgrade(registry));
    }
    
    /// Enable Fetch domain on all registered pages.
    /// 
    /// This is called when a new route is added to ensure all pages can intercept requests.
    async fn enable_fetch_on_all_pages(&self) -> Result<(), NetworkError> {
        let registries = self.page_registries.read().await;
        for weak in registries.iter() {
            if let Some(registry) = weak.upgrade() {
                registry.ensure_fetch_enabled_public().await?;
            }
        }
        Ok(())
    }
    
    /// Subscribe to route change notifications.
    /// 
    /// Pages use this to know when they need to enable Fetch domain
    /// for newly added context routes.
    pub fn subscribe_route_changes(&self) -> broadcast::Receiver<RouteChangeNotification> {
        self.route_change_tx.subscribe()
    }

    /// Register a route handler for the given pattern.
    ///
    /// The handler will be applied to all pages in the context.
    ///
    /// # Errors
    ///
    /// Returns an error if registering the route handler fails.
    pub async fn route<M, H, Fut>(&self, pattern: M, handler: H) -> Result<(), NetworkError>
    where
        M: Into<UrlPattern>,
        H: Fn(Route) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), NetworkError>> + Send + 'static,
    {
        let pattern = pattern.into();

        debug!(context_id = %self.context_id, "Registering context route");

        // Wrap the handler
        let handler: Arc<
            dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
                + Send
                + Sync,
        > = Arc::new(move |route| Box::pin(handler(route)));

        // Add to handlers (will be matched in reverse order)
        let mut handlers = self.handlers.write().await;
        handlers.push(ContextRouteHandler {
            pattern: Box::new(pattern),
            handler,
        });
        drop(handlers); // Release write lock before enabling Fetch
        
        // Synchronously enable Fetch on all existing pages before returning.
        // This ensures that navigation that happens immediately after route()
        // will be intercepted by this route.
        self.enable_fetch_on_all_pages().await?;
        
        // Notify subscribers that a route was added (for future pages and as backup)
        // Ignore send errors (no subscribers is fine)
        let _ = self.route_change_tx.send(RouteChangeNotification::RouteAdded);

        Ok(())
    }

    /// Register a route handler with a predicate function.
    ///
    /// # Errors
    ///
    /// Returns an error if registering the route handler fails.
    pub async fn route_predicate<P, H, Fut>(&self, predicate: P, handler: H) -> Result<(), NetworkError>
    where
        P: Fn(&str) -> bool + Send + Sync + 'static,
        H: Fn(Route) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), NetworkError>> + Send + 'static,
    {
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
        handlers.push(ContextRouteHandler {
            pattern: Box::new(PredicateMatcher(predicate)),
            handler,
        });
        drop(handlers); // Release write lock before enabling Fetch
        
        // Synchronously enable Fetch on all existing pages
        self.enable_fetch_on_all_pages().await?;
        
        // Notify subscribers that a route was added
        let _ = self.route_change_tx.send(RouteChangeNotification::RouteAdded);

        Ok(())
    }

    /// Unregister handlers matching the given pattern.
    pub async fn unroute(&self, pattern: &str) {
        let mut handlers = self.handlers.write().await;
        handlers.retain(|h| !h.pattern.matches(pattern));
    }

    /// Unregister all handlers.
    pub async fn unroute_all(&self) {
        let mut handlers = self.handlers.write().await;
        handlers.clear();
    }

    /// Check if there are any registered routes.
    pub async fn has_routes(&self) -> bool {
        let handlers = self.handlers.read().await;
        !handlers.is_empty()
    }

    /// Get the number of registered routes.
    pub async fn route_count(&self) -> usize {
        let handlers = self.handlers.read().await;
        handlers.len()
    }

    /// Apply context routes to a page's route registry.
    ///
    /// This should be called when a new page is created to copy
    /// context routes to the page.
    ///
    /// # Errors
    ///
    /// Returns an error if applying routes to the page fails.
    #[deprecated(note = "Use set_context_routes on RouteHandlerRegistry instead")]
    pub async fn apply_to_page(&self, page_registry: &RouteHandlerRegistry) -> Result<(), NetworkError> {
        let handlers = self.handlers.read().await;

        for handler in handlers.iter() {
            // Clone the handler Arc for the page
            let handler_clone = handler.handler.clone();

            // We need to create a pattern that can be cloned
            // For now, we'll use a catch-all pattern and filter in the handler
            // This is a simplification - a full implementation would need
            // to serialize/deserialize patterns
            
            // Note: This is a simplified approach. In practice, we would need
            // to properly copy the pattern logic to the page registry.
            page_registry.route("*", move |route| {
                let handler = handler_clone.clone();
                async move {
                    handler(route).await
                }
            }).await?;
        }

        Ok(())
    }

    /// Try to handle a request with context routes.
    ///
    /// Returns `Some(handler)` if a matching handler is found, `None` otherwise.
    /// This is called by page route registries as a fallback.
    pub async fn find_handler(&self, url: &str) -> Option<Arc<
        dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
            + Send
            + Sync,
    >> {
        let handlers = self.handlers.read().await;
        
        // Find matching handlers (in reverse order - LIFO)
        for handler in handlers.iter().rev() {
            if handler.pattern.matches(url) {
                return Some(handler.handler.clone());
            }
        }
        
        None
    }
    
    /// Find all matching handlers for a URL.
    ///
    /// Returns all handlers that match the URL in reverse order (LIFO).
    /// This is used for fallback chaining - handlers are tried in order
    /// until one handles the request.
    pub async fn find_all_handlers(&self, url: &str) -> Vec<Arc<
        dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
            + Send
            + Sync,
    >> {
        let handlers = self.handlers.read().await;
        
        // Collect all matching handlers (in reverse order - LIFO)
        handlers
            .iter()
            .rev()
            .filter(|h| h.pattern.matches(url))
            .map(|h| h.handler.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests;
