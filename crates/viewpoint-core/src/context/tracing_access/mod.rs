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
    /// # Example
    ///
    /// ```ignore
    /// use viewpoint_core::{Browser, context::TracingOptions};
    ///
    /// let browser = Browser::launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Start tracing with screenshots
    /// context.tracing().start(
    ///     TracingOptions::new()
    ///         .screenshots(true)
    ///         .snapshots(true)
    /// ).await?;
    ///
    /// // Perform test actions
    /// let page = context.new_page().await?;
    /// page.goto("https://example.com").goto().await?;
    ///
    /// // Stop and save trace
    /// context.tracing().stop("trace.zip").await?;
    /// ```
    pub fn tracing(&self) -> Tracing {
        // The session_ids will be populated dynamically - the tracing listener
        // will be attached to page sessions via the context's pages list
        Tracing::new(
            self.connection.clone(),
            self.context_id.clone(),
            self.pages.clone(),
        )
    }
}
