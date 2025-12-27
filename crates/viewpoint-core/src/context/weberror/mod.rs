//! Web error handling for BrowserContext.
//!
//! This module provides web error event handling functionality.

use std::future::Future;
use std::pin::Pin;

use tracing::debug;
use viewpoint_cdp::protocol::runtime::ExceptionThrownEvent;

use super::BrowserContext;
use crate::page::page_error::{PageError as PageErrorInfo, WebError};

/// Type alias for web error handler function.
pub type WebErrorHandler = Box<
    dyn Fn(WebError) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

impl BrowserContext {
    /// Start listening for web errors from all pages in this context.
    pub(crate) fn start_weberror_listener(&self) {
        let mut events = self.connection().subscribe_events();
        let pages = self.pages.clone();
        let weberror_handler = self.weberror_handler.clone();
        let context_id = self.context_id().to_string();
        
        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                if event.method == "Runtime.exceptionThrown" {
                    if let Some(params) = &event.params {
                        if let Ok(exception_event) = serde_json::from_value::<ExceptionThrownEvent>(params.clone()) {
                            // Get session ID and target ID if available
                            let session_id = event.session_id.clone().unwrap_or_default();
                            
                            // Find the matching page
                            let target_id = {
                                let pages_guard = pages.read().await;
                                pages_guard.iter()
                                    .find(|p| p.session_id == session_id)
                                    .map(|p| p.target_id.clone())
                                    .unwrap_or_default()
                            };
                            
                            // Only handle errors from pages in this context
                            if target_id.is_empty() && session_id.is_empty() {
                                continue;
                            }
                            
                            let page_error = PageErrorInfo::from_event(exception_event);
                            let web_error = WebError::new(page_error, target_id, session_id);
                            
                            // Check if there's a handler
                            let handler = weberror_handler.read().await;
                            if let Some(ref h) = *handler {
                                h(web_error).await;
                            } else {
                                debug!("Web error in context {} (no handler): {}", context_id, web_error.message());
                            }
                        }
                    }
                }
            }
        });
    }

    /// Set a handler for web errors (uncaught exceptions from any page).
    ///
    /// The handler will be called whenever an uncaught JavaScript exception
    /// occurs in any page within this context.
    ///
    /// # Example
    ///
    /// ```ignore
    /// context.on_weberror(|error| async move {
    ///     eprintln!("Error in {}: {}", error.target_id(), error.message());
    ///     if let Some(stack) = error.stack() {
    ///         eprintln!("Stack:\n{}", stack);
    ///     }
    /// }).await;
    /// ```
    pub async fn on_weberror<F, Fut>(&self, handler: F)
    where
        F: Fn(WebError) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut weberror_handler = self.weberror_handler.write().await;
        *weberror_handler = Some(Box::new(move |error| {
            Box::pin(handler(error))
        }));
    }

    /// Remove the web error handler.
    pub async fn off_weberror(&self) {
        let mut weberror_handler = self.weberror_handler.write().await;
        *weberror_handler = None;
    }
}
