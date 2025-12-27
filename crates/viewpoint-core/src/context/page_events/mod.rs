//! Page event handling for BrowserContext.
//!
//! This module provides page and close event handling functionality.

use std::future::Future;

use super::events::{HandlerId, WaitForPageBuilder};
use super::BrowserContext;
use crate::error::ContextError;
use crate::page::Page;

impl BrowserContext {
    /// Register a handler for new page events.
    ///
    /// The handler will be called whenever a new page is created in this context.
    /// Returns a handler ID that can be used to remove the handler with `off_page`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let handler_id = context.on_page(|page: Page| async move {
    ///     println!("New page created: {}", page.url().await.unwrap_or_default());
    /// }).await;
    ///
    /// // Later, remove the handler
    /// context.off_page(handler_id).await;
    /// ```
    pub async fn on_page<F, Fut>(&self, handler: F) -> HandlerId
    where
        F: Fn(Page) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.event_manager.on_page(handler).await
    }

    /// Remove a page event handler by its ID.
    ///
    /// Returns `true` if a handler was removed, `false` if the ID was not found.
    pub async fn off_page(&self, handler_id: HandlerId) -> bool {
        self.event_manager.off_page(handler_id).await
    }

    /// Register a handler for context close events.
    ///
    /// The handler will be called when the context is about to close,
    /// before cleanup begins.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let handler_id = context.on_close(|| async {
    ///     println!("Context is closing!");
    /// }).await;
    ///
    /// // Later, remove the handler
    /// context.off_close(handler_id).await;
    /// ```
    pub async fn on_close<F, Fut>(&self, handler: F) -> HandlerId
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.event_manager.on_close(handler).await
    }

    /// Remove a close event handler by its ID.
    ///
    /// Returns `true` if a handler was removed, `false` if the ID was not found.
    pub async fn off_close(&self, handler_id: HandlerId) -> bool {
        self.event_manager.off_close(handler_id).await
    }

    /// Wait for a new page to be created during an action.
    ///
    /// This is useful for handling popups or links that open in new tabs.
    /// The action is executed and the method waits for a new page to be
    /// created as a result.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let popup = context.wait_for_page(|| async {
    ///     page.locator("a[target=_blank]").click().await?;
    ///     Ok(())
    /// }).await?;
    ///
    /// // Now work with the popup page
    /// popup.goto("https://example.com").goto().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The action fails
    /// - No page is created within the timeout (30 seconds)
    pub fn wait_for_page<F, Fut>(
        &self,
        action: F,
    ) -> WaitForPageBuilder<'_, F, Fut>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), ContextError>>,
    {
        WaitForPageBuilder::new(&self.event_manager, action)
    }
}
