//! Tracing access for BrowserContext.
//!
//! This module provides the tracing controller access method.

use super::trace::Tracing;
use super::BrowserContext;

impl BrowserContext {
    /// Get a tracing controller for recording test execution traces.
    ///
    /// Traces capture screenshots, DOM snapshots, and network activity
    /// for debugging test failures. The resulting trace files are compatible
    /// with Playwright's Trace Viewer.
    ///
    /// **Note:** At least one page must exist in the context before starting
    /// tracing. The tracing state is shared across all `tracing()` calls within
    /// the same context.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "integration")]
    /// # tokio_test::block_on(async {
    /// # use viewpoint_core::Browser;
    /// use viewpoint_core::context::TracingOptions;
    /// # let browser = Browser::launch().headless(true).launch().await.unwrap();
    /// # let context = browser.new_context().await.unwrap();
    ///
    /// // Create a page first (required before starting tracing)
    /// let page = context.new_page().await.unwrap();
    ///
    /// // Start tracing with screenshots
    /// context.tracing().start(
    ///     TracingOptions::new()
    ///         .screenshots(true)
    ///         .snapshots(true)
    /// ).await.unwrap();
    ///
    /// // Perform test actions
    /// page.goto("https://example.com").goto().await.unwrap();
    ///
    /// // Stop and save trace (state persists across tracing() calls)
    /// context.tracing().stop("/tmp/trace.zip").await.unwrap();
    /// # });
    /// ```
    pub fn tracing(&self) -> Tracing {
        // The session_ids will be populated dynamically - the tracing listener
        // will be attached to page sessions via the context's pages list.
        // State is shared across all Tracing instances from the same context.
        Tracing::new(
            self.connection.clone(),
            self.context_id.clone(),
            self.pages.clone(),
            self.tracing_state.clone(),
        )
    }
}
