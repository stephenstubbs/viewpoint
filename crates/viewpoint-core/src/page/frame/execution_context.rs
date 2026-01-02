//! Execution context tracking for frames.
//!
//! This module provides a registry for tracking JavaScript execution contexts
//! across frames. Each frame can have multiple execution contexts (main world
//! and isolated worlds), and this registry maps frame IDs to their contexts.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use tracing::{debug, trace};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::runtime::{
    ExecutionContextCreatedEvent, ExecutionContextDestroyedEvent, ExecutionContextId,
};

/// Key for the main world execution context.
pub const MAIN_WORLD_KEY: &str = "";

/// Registry for tracking execution contexts across frames.
///
/// This registry is shared across all frames in a page session and tracks
/// the mapping from frame IDs to their execution context IDs.
#[derive(Debug)]
pub struct ExecutionContextRegistry {
    /// Session ID for this registry.
    session_id: String,
    /// CDP connection (for subscribing to events).
    connection: Arc<CdpConnection>,
    /// Map of frame_id -> (world_name -> context_id).
    contexts: RwLock<HashMap<String, HashMap<String, ExecutionContextId>>>,
    /// Reverse map of context_id -> frame_id for efficient removal.
    context_to_frame: RwLock<HashMap<ExecutionContextId, String>>,
}

impl ExecutionContextRegistry {
    /// Create a new execution context registry.
    pub fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        Self {
            session_id,
            connection,
            contexts: RwLock::new(HashMap::new()),
            context_to_frame: RwLock::new(HashMap::new()),
        }
    }

    /// Start listening for execution context events.
    ///
    /// This spawns a background task that handles `Runtime.executionContextCreated`
    /// and `Runtime.executionContextDestroyed` events.
    pub fn start_listening(self: &Arc<Self>) {
        let registry = Arc::clone(self);
        let mut events = registry.connection.subscribe_events();
        let session_id = registry.session_id.clone();

        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                // Filter for this session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                match event.method.as_str() {
                    "Runtime.executionContextCreated" => {
                        if let Some(params) = event.params.as_ref() {
                            if let Ok(created_event) = serde_json::from_value::<
                                ExecutionContextCreatedEvent,
                            >(params.clone())
                            {
                                registry.handle_context_created(created_event);
                            }
                        }
                    }
                    "Runtime.executionContextDestroyed" => {
                        if let Some(params) = event.params.as_ref() {
                            if let Ok(destroyed_event) =
                                serde_json::from_value::<ExecutionContextDestroyedEvent>(
                                    params.clone(),
                                )
                            {
                                registry.handle_context_destroyed(destroyed_event);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    /// Handle a `Runtime.executionContextCreated` event.
    fn handle_context_created(&self, event: ExecutionContextCreatedEvent) {
        let context = &event.context;
        let context_id = context.id;

        // Extract frame ID and world info from auxData
        let (frame_id, is_default) = if let Some(aux_data) = &context.aux_data {
            let frame_id = aux_data.frame_id.clone();
            let is_default = aux_data.is_default.unwrap_or(false);
            (frame_id, is_default)
        } else {
            // No auxData - can't associate with a frame
            trace!(
                context_id = context_id,
                name = %context.name,
                "Execution context created without auxData"
            );
            return;
        };

        let Some(frame_id) = frame_id else {
            trace!(
                context_id = context_id,
                name = %context.name,
                "Execution context created without frame_id"
            );
            return;
        };

        // Determine world name: empty string for main world, context.name for isolated worlds
        let world_name = if is_default {
            MAIN_WORLD_KEY.to_string()
        } else {
            context.name.clone()
        };

        debug!(
            frame_id = %frame_id,
            context_id = context_id,
            world_name = %world_name,
            is_default = is_default,
            "Execution context created"
        );

        // Store the mapping
        {
            let mut contexts = self.contexts.write();
            let frame_contexts = contexts.entry(frame_id.clone()).or_default();
            frame_contexts.insert(world_name, context_id);
        }

        // Store reverse mapping for efficient removal
        {
            let mut context_to_frame = self.context_to_frame.write();
            context_to_frame.insert(context_id, frame_id);
        }
    }

    /// Handle a `Runtime.executionContextDestroyed` event.
    fn handle_context_destroyed(&self, event: ExecutionContextDestroyedEvent) {
        let context_id = event.execution_context_id;

        // Find and remove the context using the reverse mapping
        let frame_id = {
            let mut context_to_frame = self.context_to_frame.write();
            context_to_frame.remove(&context_id)
        };

        if let Some(frame_id) = frame_id {
            let mut contexts = self.contexts.write();
            if let Some(frame_contexts) = contexts.get_mut(&frame_id) {
                // Find and remove the context by value
                frame_contexts.retain(|_, &mut id| id != context_id);

                // Clean up empty frame entries
                if frame_contexts.is_empty() {
                    contexts.remove(&frame_id);
                }

                debug!(
                    frame_id = %frame_id,
                    context_id = context_id,
                    "Execution context destroyed"
                );
            }
        } else {
            trace!(
                context_id = context_id,
                "Execution context destroyed (not tracked)"
            );
        }
    }

    /// Get the main world execution context ID for a frame.
    pub fn main_world_context(&self, frame_id: &str) -> Option<ExecutionContextId> {
        let contexts = self.contexts.read();
        contexts
            .get(frame_id)
            .and_then(|frame_contexts| frame_contexts.get(MAIN_WORLD_KEY).copied())
    }

    /// Get an execution context ID for a frame by world name.
    pub fn get_context(&self, frame_id: &str, world_name: &str) -> Option<ExecutionContextId> {
        let contexts = self.contexts.read();
        contexts
            .get(frame_id)
            .and_then(|frame_contexts| frame_contexts.get(world_name).copied())
    }

    /// Set an execution context for a frame.
    ///
    /// This is used when creating isolated worlds via `Page.createIsolatedWorld`.
    pub fn set_context(&self, frame_id: &str, world_name: &str, context_id: ExecutionContextId) {
        {
            let mut contexts = self.contexts.write();
            let frame_contexts = contexts.entry(frame_id.to_string()).or_default();
            frame_contexts.insert(world_name.to_string(), context_id);
        }

        {
            let mut context_to_frame = self.context_to_frame.write();
            context_to_frame.insert(context_id, frame_id.to_string());
        }
    }

    /// Clear all contexts for a frame.
    ///
    /// Called when a frame navigates to a new document.
    pub fn clear_frame_contexts(&self, frame_id: &str) {
        let contexts_to_remove: Vec<ExecutionContextId> = {
            let contexts = self.contexts.read();
            contexts
                .get(frame_id)
                .map(|frame_contexts| frame_contexts.values().copied().collect())
                .unwrap_or_default()
        };

        // Remove from reverse mapping
        {
            let mut context_to_frame = self.context_to_frame.write();
            for context_id in contexts_to_remove {
                context_to_frame.remove(&context_id);
            }
        }

        // Remove frame entry
        {
            let mut contexts = self.contexts.write();
            contexts.remove(frame_id);
        }

        debug!(frame_id = %frame_id, "Cleared all execution contexts for frame");
    }
}
