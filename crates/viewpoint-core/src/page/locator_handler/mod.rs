//! Locator handlers for handling overlay elements that block actions.
//!
//! This module provides functionality for registering handlers that automatically
//! dismiss overlay elements (like cookie banners, notifications, etc.) that may
//! interfere with page interactions.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, instrument, warn};

use super::locator::{Locator, Selector};
use super::Page;
use crate::error::LocatorError;

/// Type alias for locator handler function.
pub type LocatorHandlerFn = Arc<
    dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), LocatorError>> + Send>>
        + Send
        + Sync,
>;

/// Options for locator handlers.
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct LocatorHandlerOptions {
    /// Whether to skip waiting after the handler runs.
    pub no_wait_after: bool,
    /// Maximum number of times the handler can run. None means unlimited.
    pub times: Option<u32>,
}


/// Internal representation of a registered handler.
#[derive(Clone)]
struct RegisteredHandler {
    /// A unique ID for this handler.
    id: u64,
    /// The selector to match.
    selector: Selector,
    /// The handler function.
    handler: LocatorHandlerFn,
    /// Options for the handler.
    options: LocatorHandlerOptions,
    /// Number of times the handler has run.
    run_count: u32,
}

impl std::fmt::Debug for RegisteredHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredHandler")
            .field("id", &self.id)
            .field("selector", &self.selector)
            .field("options", &self.options)
            .field("run_count", &self.run_count)
            .finish()
    }
}

/// Manager for locator handlers.
#[derive(Debug)]
pub struct LocatorHandlerManager {
    /// Registered handlers.
    handlers: RwLock<Vec<RegisteredHandler>>,
    /// Counter for generating unique handler IDs.
    next_id: std::sync::atomic::AtomicU64,
}

impl LocatorHandlerManager {
    /// Create a new locator handler manager.
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(Vec::new()),
            next_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// Add a locator handler.
    ///
    /// The handler will be called when the specified locator matches an element
    /// that is blocking an action. Returns the handler ID.
    #[instrument(level = "debug", skip(self, handler), fields(selector = ?selector))]
    pub async fn add_handler<F, Fut>(
        &self,
        selector: Selector,
        handler: F,
        options: LocatorHandlerOptions,
    ) -> u64 
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LocatorError>> + Send + 'static,
    {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let handler_fn: LocatorHandlerFn = Arc::new(move || Box::pin(handler()));
        
        let registered = RegisteredHandler {
            id,
            selector,
            handler: handler_fn,
            options,
            run_count: 0,
        };

        let mut handlers = self.handlers.write().await;
        handlers.push(registered);
        debug!(handler_id = id, "Locator handler registered");
        id
    }

    /// Remove a locator handler by ID.
    #[instrument(level = "debug", skip(self))]
    pub async fn remove_handler_by_id(&self, id: u64) {
        let mut handlers = self.handlers.write().await;
        let initial_len = handlers.len();
        handlers.retain(|h| h.id != id);
        
        if handlers.len() < initial_len {
            debug!(handler_id = id, "Locator handler removed");
        } else {
            debug!(handler_id = id, "No matching locator handler found");
        }
    }

    /// Check if any handler matches a blocking element and run it.
    ///
    /// Returns true if a handler was run.
    #[instrument(level = "debug", skip(self, page))]
    pub async fn try_handle_blocking(&self, page: &Page) -> bool {
        let handlers = self.handlers.read().await;
        
        for handler in handlers.iter() {
            // Check if the selector matches a visible element
            let locator = Locator::new(page, handler.selector.clone());
            
            if let Ok(is_visible) = locator.is_visible().await {
                if is_visible {
                    let handler_id = handler.id;
                    debug!(handler_id = handler_id, "Handler selector matched, running handler");
                    let handler_fn = handler.handler.clone();
                    drop(handlers); // Release read lock before running handler
                    
                    if let Err(e) = handler_fn().await {
                        warn!(handler_id = handler_id, "Locator handler failed: {}", e);
                    } else {
                        // Increment run count and check if we should remove
                        let mut handlers = self.handlers.write().await;
                        if let Some(handler) = handlers.iter_mut().find(|h| h.id == handler_id) {
                            handler.run_count += 1;
                            
                            if let Some(times) = handler.options.times {
                                if handler.run_count >= times {
                                    debug!(handler_id = handler_id, "Handler reached times limit, removing");
                                    handlers.retain(|h| h.id != handler_id);
                                }
                            }
                        }
                        
                        return true;
                    }
                    
                    return false;
                }
            }
        }
        
        false
    }
}

impl Default for LocatorHandlerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A handle to a registered locator handler.
///
/// Use this to remove the handler later.
#[derive(Debug, Clone, Copy)]
pub struct LocatorHandlerHandle {
    id: u64,
}

impl LocatorHandlerHandle {
    /// Create a new handle from an ID.
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }

    /// Get the handler ID.
    pub fn id(&self) -> u64 {
        self.id
    }
}

// Page impl for locator handler methods
impl super::Page {
    /// Add a handler for overlay elements that may block actions.
    ///
    /// This is useful for automatically dismissing elements like cookie banners,
    /// notification popups, or other overlays that appear during tests.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Dismiss cookie banner when it appears
    /// let handle = page.add_locator_handler(
    ///     page.get_by_role(AriaRole::Button).with_name("Accept cookies"),
    ///     || async {
    ///         page.locator(".cookie-banner .accept").click().await
    ///     }
    /// ).await;
    ///
    /// // Later, remove the handler
    /// page.remove_locator_handler(handle).await;
    /// ```
    pub async fn add_locator_handler<F, Fut>(
        &self,
        locator: impl Into<super::Locator<'_>>,
        handler: F,
    ) -> LocatorHandlerHandle
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), crate::error::LocatorError>> + Send + 'static,
    {
        let loc = locator.into();
        let id = self
            .locator_handler_manager
            .add_handler(
                loc.selector().clone(),
                handler,
                LocatorHandlerOptions::default(),
            )
            .await;
        LocatorHandlerHandle::new(id)
    }

    /// Add a locator handler with options.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Handler that only runs once
    /// page.add_locator_handler_with_options(
    ///     page.locator(".popup"),
    ///     || async { page.locator(".popup .close").click().await },
    ///     LocatorHandlerOptions { times: Some(1), ..Default::default() }
    /// ).await;
    /// ```
    pub async fn add_locator_handler_with_options<F, Fut>(
        &self,
        locator: impl Into<super::Locator<'_>>,
        handler: F,
        options: LocatorHandlerOptions,
    ) -> LocatorHandlerHandle
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), crate::error::LocatorError>> + Send + 'static,
    {
        let loc = locator.into();
        let id = self
            .locator_handler_manager
            .add_handler(loc.selector().clone(), handler, options)
            .await;
        LocatorHandlerHandle::new(id)
    }

    /// Remove a locator handler.
    pub async fn remove_locator_handler(&self, handle: LocatorHandlerHandle) {
        self.locator_handler_manager.remove_handler_by_id(handle.id()).await;
    }

    /// Get the locator handler manager.
    pub(crate) fn locator_handler_manager(&self) -> &LocatorHandlerManager {
        &self.locator_handler_manager
    }
}
