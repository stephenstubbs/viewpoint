//! Context-level exposed function bindings.
//!
//! This module provides functionality for exposing Rust functions to JavaScript
//! across all pages in a browser context.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::debug;

/// Type alias for the binding callback function.
pub type ContextBindingCallback = Arc<
    dyn Fn(
            Vec<serde_json::Value>,
        ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, String>> + Send>>
        + Send
        + Sync,
>;

/// Stored binding information for context-level functions.
#[derive(Clone)]
pub struct ContextBinding {
    /// Function name.
    pub name: String,
    /// The callback function.
    pub callback: ContextBindingCallback,
}

/// Registry for context-level exposed functions.
///
/// Functions registered here will be exposed to all pages in the context,
/// including pages created after the function is exposed.
#[derive(Default)]
pub struct ContextBindingRegistry {
    /// Registered bindings indexed by function name.
    bindings: RwLock<HashMap<String, ContextBinding>>,
}

impl ContextBindingRegistry {
    /// Create a new context binding registry.
    pub fn new() -> Self {
        Self {
            bindings: RwLock::new(HashMap::new()),
        }
    }

    /// Register a function to be exposed to all pages.
    pub async fn expose_function<F, Fut>(&self, name: &str, callback: F)
    where
        F: Fn(Vec<serde_json::Value>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<serde_json::Value, String>> + Send + 'static,
    {
        debug!("Registering context-level binding: {}", name);

        let boxed_callback: ContextBindingCallback = Arc::new(move |args| Box::pin(callback(args)));

        let binding = ContextBinding {
            name: name.to_string(),
            callback: boxed_callback,
        };

        let mut bindings = self.bindings.write().await;
        bindings.insert(name.to_string(), binding);
    }

    /// Remove an exposed function.
    pub async fn remove_function(&self, name: &str) -> bool {
        debug!("Removing context-level binding: {}", name);
        let mut bindings = self.bindings.write().await;
        bindings.remove(name).is_some()
    }

    /// Get all registered bindings.
    pub async fn get_all(&self) -> Vec<ContextBinding> {
        let bindings = self.bindings.read().await;
        bindings.values().cloned().collect()
    }

    /// Check if a function is registered.
    pub async fn has(&self, name: &str) -> bool {
        let bindings = self.bindings.read().await;
        bindings.contains_key(name)
    }
}

use super::BrowserContext;

impl BrowserContext {
    /// Expose a Rust function to JavaScript in all pages of this context.
    ///
    /// The function will be available as `window.<name>()` in JavaScript.
    /// When called from JavaScript, the function arguments are serialized to JSON,
    /// the Rust callback is executed, and the result is returned to JavaScript.
    ///
    /// Note: Functions exposed at the context level need to be explicitly applied
    /// to each page. This method registers the function for future pages, but
    /// you need to call `expose_function` on existing pages separately.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Expose a function to all pages
    /// context.expose_function("add", |args| async move {
    ///     let x = args[0].as_i64().unwrap_or(0);
    ///     let y = args[1].as_i64().unwrap_or(0);
    ///     Ok(serde_json::json!(x + y))
    /// }).await;
    ///
    /// // Create a page - function is available
    /// let page = context.new_page().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn expose_function<F, Fut>(&self, name: &str, callback: F)
    where
        F: Fn(Vec<serde_json::Value>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<serde_json::Value, String>> + Send + 'static,
    {
        self.binding_registry.expose_function(name, callback).await;
    }

    /// Remove an exposed function from the context.
    ///
    /// Note: This only affects future pages. Existing pages will still have
    /// the function available until they are reloaded.
    pub async fn remove_exposed_function(&self, name: &str) -> bool {
        self.binding_registry.remove_function(name).await
    }
}

#[cfg(test)]
mod tests;
