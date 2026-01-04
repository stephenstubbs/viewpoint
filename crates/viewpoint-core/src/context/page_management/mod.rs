//! Page creation and management within a browser context.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::oneshot;
use tracing::{debug, info, instrument};

use viewpoint_cdp::protocol::target_domain::{CreateTargetParams, CreateTargetResult};

use crate::error::ContextError;
use crate::page::Page;

use super::BrowserContext;

impl BrowserContext {
    /// Create a new page in this context.
    ///
    /// This method creates a new page target and waits for the CDP event listener
    /// to complete page initialization. All page creation goes through the unified
    /// CDP event-driven path, ensuring consistent behavior.
    ///
    /// # Errors
    ///
    /// Returns an error if page creation fails or times out.
    #[instrument(level = "info", skip(self), fields(context_id = %self.context_id))]
    pub async fn new_page(&self) -> Result<Page, ContextError> {
        if self.closed {
            return Err(ContextError::Closed);
        }

        info!("Creating new page");

        // Set up a oneshot channel to receive the page from the event listener
        let (tx, rx) = oneshot::channel::<Page>();
        let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

        // Register a temporary handler to capture the new page
        let tx_clone = tx.clone();
        let handler_id = self
            .event_manager
            .on_page(move |page| {
                let tx = tx_clone.clone();
                async move {
                    let mut guard = tx.lock().await;
                    if let Some(sender) = guard.take() {
                        let _ = sender.send(page);
                    }
                }
            })
            .await;

        // Create the target - the CDP event listener will handle attachment,
        // domain enabling, and page creation
        let create_result: Result<CreateTargetResult, _> = self
            .connection
            .send_command(
                "Target.createTarget",
                Some(CreateTargetParams {
                    url: "about:blank".to_string(),
                    width: None,
                    height: None,
                    browser_context_id: Some(self.context_id.clone()),
                    background: None,
                    new_window: None,
                }),
                None,
            )
            .await;

        // Handle target creation error
        if let Err(e) = create_result {
            // Clean up handler before returning error
            self.event_manager.off_page(handler_id).await;
            return Err(e.into());
        }

        // Wait for the event listener to complete page setup
        let timeout_duration = Duration::from_secs(30);
        let page_result = tokio::time::timeout(timeout_duration, rx).await;

        // Clean up the handler
        self.event_manager.off_page(handler_id).await;

        // Process the result
        match page_result {
            Ok(Ok(page)) => {
                // Apply context-level init scripts to the new page
                if let Err(e) = self.apply_init_scripts_to_session(page.session_id()).await {
                    debug!("Failed to apply init scripts: {}", e);
                }

                info!(
                    target_id = %page.target_id(),
                    session_id = %page.session_id(),
                    "Page created successfully"
                );

                Ok(page)
            }
            Ok(Err(_)) => Err(ContextError::Internal(
                "Page channel closed unexpectedly".to_string(),
            )),
            Err(_) => Err(ContextError::Timeout {
                operation: "new_page".to_string(),
                duration: timeout_duration,
            }),
        }
    }

    /// Get all pages in this context.
    ///
    /// Returns fully functional `Page` objects that can be used to interact
    /// with the pages (navigate, click, type, etc.).
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::BrowserContext;
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), viewpoint_core::CoreError> {
    /// // Get all pages
    /// let pages = context.pages().await?;
    /// for page in &pages {
    ///     println!("Page URL: {}", page.url().await?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pages(&self) -> Result<Vec<Page>, ContextError> {
        if self.closed {
            return Err(ContextError::Closed);
        }

        let pages_guard = self.pages.read().await;
        Ok(pages_guard.iter().map(Page::clone_internal).collect())
    }

    /// Get the number of pages in this context.
    ///
    /// This is a convenience method that avoids cloning all pages when
    /// you only need the count.
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
    pub async fn page_count(&self) -> Result<usize, ContextError> {
        if self.closed {
            return Err(ContextError::Closed);
        }

        let pages_guard = self.pages.read().await;
        Ok(pages_guard.len())
    }
}
