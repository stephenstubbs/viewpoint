//! Exposed function bindings.
//!
//! This module provides functionality for exposing Rust functions to JavaScript
//! code running in the browser. These functions can be called from JavaScript
//! and will execute Rust code, returning the result back to JavaScript.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, trace, warn};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::runtime::{AddBindingParams, BindingCalledEvent};
use viewpoint_js::js;

use crate::error::PageError;

/// Type alias for the binding callback function.
///
/// The callback receives a vector of JSON arguments and returns a JSON result.
pub type BindingCallback = Box<
    dyn Fn(
            Vec<serde_json::Value>,
        ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, String>> + Send>>
        + Send
        + Sync,
>;

/// Manager for exposed function bindings on a page.
pub struct BindingManager {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Registered bindings indexed by function name.
    bindings: Arc<RwLock<HashMap<String, BindingCallback>>>,
    /// Whether the manager is listening for binding calls.
    is_listening: std::sync::atomic::AtomicBool,
}

impl BindingManager {
    /// Create a new binding manager for a page.
    pub fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        Self {
            connection,
            session_id,
            bindings: Arc::new(RwLock::new(HashMap::new())),
            is_listening: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Expose a function to JavaScript.
    ///
    /// The function will be available as `window.<name>()` in JavaScript.
    /// Arguments are passed as JSON values and the return value is also JSON.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::binding::BindingManager;
    ///
    /// # async fn example(manager: BindingManager) -> Result<(), viewpoint_core::CoreError> {
    /// manager.expose_function("compute", |args| async move {
    ///     let x: i64 = serde_json::from_value(args[0].clone()).map_err(|e| e.to_string())?;
    ///     let y: i64 = serde_json::from_value(args[1].clone()).map_err(|e| e.to_string())?;
    ///     serde_json::to_value(x + y).map_err(|e| e.to_string())
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn expose_function<F, Fut>(&self, name: &str, callback: F) -> Result<(), PageError>
    where
        F: Fn(Vec<serde_json::Value>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<serde_json::Value, String>> + Send + 'static,
    {
        debug!("Exposing function: {}", name);

        // Add the binding via CDP
        self.connection
            .send_command::<_, serde_json::Value>(
                "Runtime.addBinding",
                Some(AddBindingParams {
                    name: name.to_string(),
                    execution_context_id: None,
                    execution_context_name: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        // Create a wrapper script that sets up the JavaScript function
        let name_json = serde_json::to_string(name).unwrap();
        let wrapper_script = js! {
            (function() {
                const bindingName = @{name_json};
                const bindingFn = window[bindingName];
                if (!bindingFn) return;

                // Replace the binding with a proper async function
                window[bindingName] = async function(...args) {
                    const seq = (window.__viewpoint_seq = (window.__viewpoint_seq || 0) + 1);
                    const payload = JSON.stringify({ seq, args });

                    return new Promise((resolve, reject) => {
                        window.__viewpoint_callbacks = window.__viewpoint_callbacks || {};
                        window.__viewpoint_callbacks[seq] = { resolve, reject };
                        bindingFn(payload);
                    });
                };
            })()
        };

        // Inject the wrapper script
        self.connection
            .send_command::<_, serde_json::Value>(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: wrapper_script,
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        // Store the callback
        let boxed_callback: BindingCallback = Box::new(move |args| Box::pin(callback(args)));

        {
            let mut bindings = self.bindings.write().await;
            bindings.insert(name.to_string(), boxed_callback);
        }

        // Start listening for binding calls if not already
        self.start_listening().await;

        debug!("Function exposed: {}", name);
        Ok(())
    }

    /// Remove an exposed function.
    pub async fn remove_function(&self, name: &str) -> Result<(), PageError> {
        debug!("Removing exposed function: {}", name);

        // Remove the binding via CDP
        self.connection
            .send_command::<_, serde_json::Value>(
                "Runtime.removeBinding",
                Some(viewpoint_cdp::protocol::runtime::RemoveBindingParams {
                    name: name.to_string(),
                }),
                Some(&self.session_id),
            )
            .await?;

        // Remove from our registry
        {
            let mut bindings = self.bindings.write().await;
            bindings.remove(name);
        }

        Ok(())
    }

    /// Start listening for binding call events.
    async fn start_listening(&self) {
        if self
            .is_listening
            .swap(true, std::sync::atomic::Ordering::SeqCst)
        {
            // Already listening
            return;
        }

        let mut events = self.connection.subscribe_events();
        let session_id = self.session_id.clone();
        let bindings = self.bindings.clone();
        let connection = self.connection.clone();

        tokio::spawn(async move {
            debug!("Binding manager started listening for events");

            while let Ok(event) = events.recv().await {
                // Filter events for this session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                if event.method == "Runtime.bindingCalled" {
                    if let Some(params) = &event.params {
                        if let Ok(binding_event) =
                            serde_json::from_value::<BindingCalledEvent>(params.clone())
                        {
                            trace!("Binding called: {}", binding_event.name);

                            // Parse the payload
                            let payload: Result<BindingPayload, _> =
                                serde_json::from_str(&binding_event.payload);

                            if let Ok(payload) = payload {
                                let bindings_guard = bindings.read().await;
                                if let Some(callback) = bindings_guard.get(&binding_event.name) {
                                    // Execute the callback
                                    let result = callback(payload.args).await;
                                    drop(bindings_guard);

                                    // Send the result back to JavaScript
                                    let seq = payload.seq.to_string();
                                    let resolve_script = match result {
                                        Ok(value) => {
                                            let value_json = serde_json::to_string(&value)
                                                .unwrap_or_else(|_| "null".to_string());
                                            js! {
                                                (function() {
                                                    const callbacks = window.__viewpoint_callbacks;
                                                    if (callbacks && callbacks[@{seq}]) {
                                                        callbacks[@{seq}].resolve(@{value_json});
                                                        delete callbacks[@{seq}];
                                                    }
                                                })()
                                            }
                                        }
                                        Err(error) => {
                                            let error_json = serde_json::to_string(&error)
                                                .unwrap_or_else(|_| {
                                                    "\"Unknown error\"".to_string()
                                                });
                                            js! {
                                                (function() {
                                                    const callbacks = window.__viewpoint_callbacks;
                                                    if (callbacks && callbacks[@{seq}]) {
                                                        callbacks[@{seq}].reject(new Error(@{error_json}));
                                                        delete callbacks[@{seq}];
                                                    }
                                                })()
                                            }
                                        }
                                    };

                                    let _ = connection
                                        .send_command::<_, serde_json::Value>(
                                            "Runtime.evaluate",
                                            Some(
                                                viewpoint_cdp::protocol::runtime::EvaluateParams {
                                                    expression: resolve_script,
                                                    object_group: None,
                                                    include_command_line_api: None,
                                                    silent: Some(true),
                                                    context_id: None,
                                                    return_by_value: Some(true),
                                                    await_promise: Some(false),
                                                },
                                            ),
                                            Some(&session_id),
                                        )
                                        .await;
                                }
                            } else {
                                warn!("Failed to parse binding payload: {}", binding_event.payload);
                            }
                        }
                    }
                }
            }

            debug!("Binding manager stopped listening");
        });
    }

    /// Re-bind all functions after navigation.
    ///
    /// This should be called after page navigation to re-inject the wrapper scripts.
    pub async fn rebind_all(&self) -> Result<(), PageError> {
        let bindings = self.bindings.read().await;
        let names: Vec<String> = bindings.keys().cloned().collect();
        drop(bindings);

        for name in names {
            // Re-inject the wrapper script
            let name_json = serde_json::to_string(&name).unwrap();
            let wrapper_script = js! {
                (function() {
                    const bindingName = @{name_json};
                    const bindingFn = window[bindingName];
                    if (!bindingFn) return;

                    // Replace the binding with a proper async function
                    window[bindingName] = async function(...args) {
                        const seq = (window.__viewpoint_seq = (window.__viewpoint_seq || 0) + 1);
                        const payload = JSON.stringify({ seq, args });

                        return new Promise((resolve, reject) => {
                            window.__viewpoint_callbacks = window.__viewpoint_callbacks || {};
                            window.__viewpoint_callbacks[seq] = { resolve, reject };
                            bindingFn(payload);
                        });
                    };
                })()
            };

            self.connection
                .send_command::<_, serde_json::Value>(
                    "Runtime.evaluate",
                    Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                        expression: wrapper_script,
                        object_group: None,
                        include_command_line_api: None,
                        silent: Some(true),
                        context_id: None,
                        return_by_value: Some(true),
                        await_promise: Some(false),
                    }),
                    Some(&self.session_id),
                )
                .await?;
        }

        Ok(())
    }
}

/// Payload structure for binding calls.
#[derive(Debug, serde::Deserialize)]
struct BindingPayload {
    /// Sequence number for matching responses.
    seq: u64,
    /// Function arguments.
    args: Vec<serde_json::Value>,
}

// Page impl for exposed function methods
impl super::Page {
    /// Expose a Rust function to JavaScript.
    ///
    /// The function will be available as `window.<name>()` in JavaScript.
    /// When called from JavaScript, the function arguments are serialized to JSON,
    /// the Rust callback is executed, and the result is returned to JavaScript.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Expose a simple function
    /// page.expose_function("add", |args| async move {
    ///     let x = args[0].as_i64().unwrap_or(0);
    ///     let y = args[1].as_i64().unwrap_or(0);
    ///     Ok(serde_json::json!(x + y))
    /// }).await?;
    ///
    /// // Call from JavaScript:
    /// // const result = await window.add(1, 2); // returns 3
    ///
    /// // Expose a function with string processing
    /// page.expose_function("sha256", |args| async move {
    ///     let input = args[0].as_str().unwrap_or("");
    ///     // ... compute hash ...
    ///     let hash_string = "example_hash";
    ///     Ok(serde_json::json!(hash_string))
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// - The function is re-bound after each navigation
    /// - Arguments and return values must be JSON-serializable
    /// - Errors returned from the callback will reject the JavaScript promise
    pub async fn expose_function<F, Fut>(
        &self,
        name: &str,
        callback: F,
    ) -> Result<(), crate::error::PageError>
    where
        F: Fn(Vec<serde_json::Value>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<serde_json::Value, String>> + Send + 'static,
    {
        self.binding_manager.expose_function(name, callback).await
    }

    /// Remove an exposed function.
    ///
    /// The function will no longer be available in JavaScript after this call.
    pub async fn remove_exposed_function(&self, name: &str) -> Result<(), crate::error::PageError> {
        self.binding_manager.remove_function(name).await
    }
}

#[cfg(test)]
mod tests;
