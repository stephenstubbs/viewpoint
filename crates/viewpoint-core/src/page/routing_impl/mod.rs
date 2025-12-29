//! Network routing implementation for Page.
//!
//! This module contains the network routing, event, and WebSocket monitoring methods
//! for the Page struct.

use std::future::Future;

use crate::error::NetworkError;
use crate::network::{Route, UrlMatcher, UrlPattern, WaitForRequestBuilder, WaitForResponseBuilder, WebSocket};

use super::Page;

impl Page {
    // =========================================================================
    // Network Routing Methods
    // =========================================================================

    /// Register a route handler for requests matching the given pattern.
    ///
    /// The handler will be called for each request that matches the pattern.
    /// Use `route.fulfill()`, `route.continue_()`, or `route.abort()` to handle the request.
    ///
    /// Handlers are matched in reverse registration order (LIFO). The last registered
    /// handler that matches a URL is called first.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::network::Route;
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Block all CSS requests
    /// page.route("**/*.css", |route: Route| async move {
    ///     route.abort().await
    /// }).await?;
    ///
    /// // Mock an API response
    /// page.route("**/api/users", |route: Route| async move {
    ///     route.fulfill()
    ///         .status(200)
    ///         .json(&serde_json::json!({"users": []}))
    ///         .send()
    ///         .await
    /// }).await?;
    ///
    /// // Modify request headers
    /// page.route("**/api/**", |route: Route| async move {
    ///     route.continue_()
    ///         .header("Authorization", "Bearer token")
    ///         .await
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn route<M, H, Fut>(&self, pattern: M, handler: H) -> Result<(), NetworkError>
    where
        M: Into<UrlPattern>,
        H: Fn(Route) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), NetworkError>> + Send + 'static,
    {
        if self.closed {
            return Err(NetworkError::Aborted);
        }
        self.route_registry.route(pattern, handler).await
    }

    /// Register a route handler with a predicate function.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::network::Route;
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Handle only POST requests
    /// page.route_predicate(
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
        if self.closed {
            return Err(NetworkError::Aborted);
        }
        self.route_registry.route_predicate(predicate, handler).await
    }

    /// Unregister handlers matching the given pattern.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::network::Route;
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.route("**/*.css", |route: Route| async move {
    ///     route.abort().await
    /// }).await?;
    /// // Later...
    /// page.unroute("**/*.css").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unroute(&self, pattern: &str) {
        self.route_registry.unroute(pattern).await;
    }

    /// Unregister all route handlers.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.unroute_all().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unroute_all(&self) {
        self.route_registry.unroute_all().await;
    }

    /// Route requests from a HAR file.
    ///
    /// Requests that match entries in the HAR file will be fulfilled with the
    /// recorded responses. Requests that don't match will continue normally
    /// unless strict mode is enabled.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::network::HarReplayOptions;
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple HAR routing
    /// page.route_from_har("recordings/api.har").await?;
    ///
    /// // With options
    /// page.route_from_har_with_options(
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
    /// - The page is closed
    pub async fn route_from_har(&self, path: impl AsRef<std::path::Path>) -> Result<(), NetworkError> {
        self.route_from_har_with_options(path, crate::network::HarReplayOptions::default()).await
    }

    /// Route requests from a HAR file with options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::network::HarReplayOptions;
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Strict mode: fail if no match found
    /// page.route_from_har_with_options(
    ///     "api.har",
    ///     HarReplayOptions::new().strict(true)
    /// ).await?;
    ///
    /// // URL filter: only match specific URLs
    /// page.route_from_har_with_options(
    ///     "api.har",
    ///     HarReplayOptions::new().url("**/api/**")
    /// ).await?;
    ///
    /// // Simulate original timing
    /// page.route_from_har_with_options(
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
    /// - The page is closed
    pub async fn route_from_har_with_options(
        &self,
        path: impl AsRef<std::path::Path>,
        options: crate::network::HarReplayOptions,
    ) -> Result<(), NetworkError> {
        if self.closed {
            return Err(NetworkError::Aborted);
        }

        let handler = std::sync::Arc::new(
            crate::network::HarReplayHandler::from_file(path)
                .await?
                .with_options(options)
        );

        // Route all requests through the HAR handler
        let route_handler = crate::network::har_replay::create_har_route_handler(handler);
        self.route_registry.route("**/*", route_handler).await
    }

    // =========================================================================
    // Network Event Methods
    // =========================================================================

    /// Wait for a request matching the given pattern.
    ///
    /// Returns a builder that can be used to configure timeout and wait for the request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Wait for any API request
    /// let request = page.wait_for_request("**/api/**".to_string())
    ///     .timeout(Duration::from_secs(10))
    ///     .wait()
    ///     .await?;
    ///
    /// println!("Request URL: {}", request.url());
    ///
    /// // Wait for a specific request
    /// let request = page.wait_for_request("**/users".to_string())
    ///     .wait()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn wait_for_request<M: UrlMatcher + Clone + 'static>(
        &self,
        pattern: M,
    ) -> WaitForRequestBuilder<'_, M> {
        WaitForRequestBuilder::new(&self.connection, &self.session_id, pattern)
    }

    /// Wait for a response matching the given pattern.
    ///
    /// Returns a builder that can be used to configure timeout and wait for the response.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Wait for any API response
    /// let response = page.wait_for_response("**/api/**".to_string())
    ///     .timeout(Duration::from_secs(10))
    ///     .wait()
    ///     .await?;
    ///
    /// println!("Response status: {}", response.status());
    ///
    /// // Get the response body
    /// let body = response.text().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn wait_for_response<M: UrlMatcher + Clone + 'static>(
        &self,
        pattern: M,
    ) -> WaitForResponseBuilder<'_, M> {
        WaitForResponseBuilder::new(&self.connection, &self.session_id, pattern)
    }

    // =========================================================================
    // WebSocket Monitoring Methods
    // =========================================================================

    /// Set a handler for WebSocket connection events.
    ///
    /// The handler will be called whenever a new WebSocket connection is opened
    /// from this page. You can then register handlers on the WebSocket for
    /// frame events.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_websocket(|ws| async move {
    ///     println!("WebSocket opened: {}", ws.url());
    ///     
    ///     // Register handlers for frames
    ///     ws.on_framesent(|frame| async move {
    ///         println!("Sent: {:?}", frame.payload());
    ///     }).await;
    ///     
    ///     ws.on_framereceived(|frame| async move {
    ///         println!("Received: {:?}", frame.payload());
    ///     }).await;
    ///     
    ///     ws.on_close(|| async {
    ///         println!("WebSocket closed");
    ///     }).await;
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_websocket<F, Fut>(&self, handler: F)
    where
        F: Fn(WebSocket) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.websocket_manager.set_handler(handler).await;
    }

    /// Remove the WebSocket event handler.
    pub async fn off_websocket(&self) {
        self.websocket_manager.remove_handler().await;
    }
}
