//! Event system infrastructure for browser context.
//!
//! This module provides the event handling infrastructure for `BrowserContext`,
//! including event handlers, handler IDs, and event emitters.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::RwLock;

use crate::page::Page;

/// A unique identifier for an event handler.
///
/// This ID is returned when registering an event handler and can be used
/// to remove the handler later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandlerId(u64);

impl HandlerId {
    /// Generate a new unique handler ID.
    pub(crate) fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// Type alias for the page event handler function.
pub type PageEventHandler =
    Box<dyn Fn(Page) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for the close event handler function.
pub type CloseEventHandler =
    Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Event emitter for managing typed event handlers.
///
/// This provides a thread-safe way to register, invoke, and remove
/// event handlers of a specific type.
pub struct EventEmitter<H> {
    handlers: RwLock<HashMap<HandlerId, H>>,
}

impl<H> Default for EventEmitter<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H> EventEmitter<H> {
    /// Create a new empty event emitter.
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    /// Add a handler and return its ID.
    pub async fn add(&self, handler: H) -> HandlerId {
        let id = HandlerId::new();
        let mut handlers = self.handlers.write().await;
        handlers.insert(id, handler);
        id
    }

    /// Remove a handler by its ID.
    ///
    /// Returns `true` if a handler was removed, `false` if the ID was not found.
    pub async fn remove(&self, id: HandlerId) -> bool {
        let mut handlers = self.handlers.write().await;
        handlers.remove(&id).is_some()
    }

    /// Remove all handlers.
    pub async fn clear(&self) {
        let mut handlers = self.handlers.write().await;
        handlers.clear();
    }

    /// Check if there are any handlers registered.
    pub async fn is_empty(&self) -> bool {
        let handlers = self.handlers.read().await;
        handlers.is_empty()
    }

    /// Get the number of registered handlers.
    pub async fn len(&self) -> usize {
        let handlers = self.handlers.read().await;
        handlers.len()
    }
}

impl EventEmitter<PageEventHandler> {
    /// Emit an event to all page handlers.
    ///
    /// Each handler is called with a clone of the page.
    pub async fn emit(&self, page: Page) {
        let handlers = self.handlers.read().await;
        for handler in handlers.values() {
            // Clone the page for each handler
            // Note: We need to create a new page reference for each handler
            handler(page.clone_internal()).await;
        }
    }
}

impl EventEmitter<CloseEventHandler> {
    /// Emit a close event to all handlers.
    pub async fn emit(&self) {
        let handlers = self.handlers.read().await;
        for handler in handlers.values() {
            handler().await;
        }
    }
}

/// Context event manager that handles all context-level events.
#[derive(Default)]
pub struct ContextEventManager {
    /// Handlers for 'page' events (new page created).
    page_handlers: EventEmitter<PageEventHandler>,
    /// Handlers for 'close' events (context closing).
    close_handlers: EventEmitter<CloseEventHandler>,
}

impl ContextEventManager {
    /// Create a new context event manager.
    pub fn new() -> Self {
        Self {
            page_handlers: EventEmitter::new(),
            close_handlers: EventEmitter::new(),
        }
    }

    /// Register a handler for new page events.
    ///
    /// The handler will be called whenever a new page is created in the context.
    /// Returns a handler ID that can be used to remove the handler.
    pub async fn on_page<F, Fut>(&self, handler: F) -> HandlerId
    where
        F: Fn(Page) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed_handler: PageEventHandler = Box::new(move |page| Box::pin(handler(page)));
        self.page_handlers.add(boxed_handler).await
    }

    /// Remove a page event handler by its ID.
    ///
    /// Returns `true` if a handler was removed.
    pub async fn off_page(&self, id: HandlerId) -> bool {
        self.page_handlers.remove(id).await
    }

    /// Emit a page event to all registered handlers.
    pub async fn emit_page(&self, page: Page) {
        self.page_handlers.emit(page).await;
    }

    /// Register a handler for context close events.
    ///
    /// The handler will be called when the context is about to close.
    /// Returns a handler ID that can be used to remove the handler.
    pub async fn on_close<F, Fut>(&self, handler: F) -> HandlerId
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed_handler: CloseEventHandler = Box::new(move || Box::pin(handler()));
        self.close_handlers.add(boxed_handler).await
    }

    /// Remove a close event handler by its ID.
    ///
    /// Returns `true` if a handler was removed.
    pub async fn off_close(&self, id: HandlerId) -> bool {
        self.close_handlers.remove(id).await
    }

    /// Emit a close event to all registered handlers.
    pub async fn emit_close(&self) {
        self.close_handlers.emit().await;
    }

    /// Clear all event handlers.
    pub async fn clear(&self) {
        self.page_handlers.clear().await;
        self.close_handlers.clear().await;
    }
}

/// Builder for waiting for a page during an action.
///
/// This is used with `context.wait_for_page()` to wait for a new page
/// to be created during the execution of an action.
pub struct WaitForPageBuilder<'a, F, Fut>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), crate::error::ContextError>>,
{
    event_manager: &'a Arc<ContextEventManager>,
    action: Option<F>,
}

impl<'a, F, Fut> WaitForPageBuilder<'a, F, Fut>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), crate::error::ContextError>>,
{
    /// Create a new builder.
    pub(crate) fn new(event_manager: &'a Arc<ContextEventManager>, action: F) -> Self {
        Self {
            event_manager,
            action: Some(action),
        }
    }

    /// Execute the action and wait for a new page.
    ///
    /// Returns the new page that was created during the action.
    ///
    /// # Errors
    ///
    /// Returns an error if the action fails or times out waiting for a new page.
    ///
    /// # Panics
    ///
    /// Panics if the action has already been consumed.
    pub async fn wait(mut self) -> Result<Page, crate::error::ContextError> {
        use tokio::sync::oneshot;

        // Create a channel to receive the new page
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

        // Execute the action
        let action = self.action.take().expect("action already consumed");
        let action_result = action().await;

        // Wait for the page or handle action error
        let result = match action_result {
            Ok(()) => {
                // Wait for the page with timeout
                match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
                    Ok(Ok(page)) => Ok(page),
                    Ok(Err(_)) => Err(crate::error::ContextError::Internal(
                        "Page channel closed unexpectedly".to_string(),
                    )),
                    Err(_) => Err(crate::error::ContextError::Timeout {
                        operation: "wait_for_page".to_string(),
                        duration: std::time::Duration::from_secs(30),
                    }),
                }
            }
            Err(e) => Err(e),
        };

        // Clean up the handler
        self.event_manager.off_page(handler_id).await;

        result
    }
}

#[cfg(test)]
mod tests;
