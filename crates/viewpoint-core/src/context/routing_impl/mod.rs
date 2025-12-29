//! Context-level network routing implementation.

use std::future::Future;
use std::sync::Arc;

use crate::error::NetworkError;
use crate::network::{HarReplayOptions, Route, UrlPattern};

use super::BrowserContext;

impl BrowserContext {
    /// Register a route handler that applies to all pages in this context.
    ///
    /// Routes registered at the context level are applied to all pages,
    /// including new pages created after the route is registered.
    /// Context routes are checked before page-level routes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, network::Route};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Block all analytics requests for all pages
    /// context.route("**/analytics/**", |route: Route| async move {
    ///     route.abort().await
    /// }).await?;
    ///
    /// // Mock an API for all pages
    /// context.route("**/api/users", |route: Route| async move {
    ///     route.fulfill()
    ///         .status(200)
    ///         .json(&serde_json::json!({"users": []}))
    ///         .send()
    ///         .await
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
    pub async fn route<M, H, Fut>(&self, pattern: M, handler: H) -> Result<(), NetworkError>
    where
        M: Into<UrlPattern>,
        H: Fn(Route) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), NetworkError>> + Send + 'static,
    {
        if self.is_closed() {
            return Err(NetworkError::Aborted);
        }
        self.route_registry.route(pattern, handler).await
    }

    /// Register a route handler with a predicate function.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, network::Route};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Block POST requests to any API endpoint
    /// context.route_predicate(
    ///     |url| url.contains("/api/"),
    ///     |route: Route| async move {
    ///         if route.request().method() == "POST" {
    ///             route.abort().await
    ///         } else {
    ///             route.continue_().await
    ///         }
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
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
        if self.is_closed() {
            return Err(NetworkError::Aborted);
        }
        self.route_registry
            .route_predicate(predicate, handler)
            .await
    }

    /// Unregister handlers matching the given pattern.
    ///
    /// This removes handlers registered with `route()` that match the pattern.
    pub async fn unroute(&self, pattern: &str) {
        self.route_registry.unroute(pattern).await;
    }

    /// Unregister all route handlers.
    ///
    /// This removes all handlers registered with `route()`.
    pub async fn unroute_all(&self) {
        self.route_registry.unroute_all().await;
    }

    /// Route requests from a HAR file for all pages in this context.
    ///
    /// Requests that match entries in the HAR file will be fulfilled with the
    /// recorded responses. Requests that don't match will continue normally
    /// unless strict mode is enabled.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, network::HarReplayOptions};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Simple HAR routing for all pages
    /// context.route_from_har("recordings/api.har").await?;
    ///
    /// // With options
    /// context.route_from_har_with_options(
    ///     "recordings/api.har",
    ///     HarReplayOptions::new()
    ///         .url("**/api/**")
    ///         .strict(true)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HAR file cannot be read or parsed
    /// - The context is closed
    pub async fn route_from_har(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), NetworkError> {
        self.route_from_har_with_options(path, HarReplayOptions::default())
            .await
    }

    /// Route requests from a HAR file with options for all pages in this context.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, network::HarReplayOptions};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Strict mode: fail if no match found
    /// context.route_from_har_with_options(
    ///     "api.har",
    ///     HarReplayOptions::new().strict(true)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, network::HarReplayOptions};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // URL filter: only match specific URLs
    /// context.route_from_har_with_options(
    ///     "api.har",
    ///     HarReplayOptions::new().url("**/api/**")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, network::HarReplayOptions};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Simulate original timing
    /// context.route_from_har_with_options(
    ///     "api.har",
    ///     HarReplayOptions::new().use_original_timing(true)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HAR file cannot be read or parsed
    /// - The context is closed
    pub async fn route_from_har_with_options(
        &self,
        path: impl AsRef<std::path::Path>,
        options: HarReplayOptions,
    ) -> Result<(), NetworkError> {
        if self.is_closed() {
            return Err(NetworkError::Aborted);
        }

        let handler = Arc::new(
            crate::network::HarReplayHandler::from_file(path)
                .await?
                .with_options(options),
        );

        // Route all requests through the HAR handler
        let route_handler = crate::network::har_replay::create_har_route_handler(handler);
        self.route_registry.route("**/*", route_handler).await
    }
}
