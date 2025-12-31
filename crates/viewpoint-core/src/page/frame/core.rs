//! Core frame type and basic operations.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::runtime::ExecutionContextId;

use super::execution_context::{ExecutionContextRegistry, MAIN_WORLD_KEY};

/// Internal frame data that can be updated.
#[derive(Debug, Clone)]
pub(super) struct FrameData {
    /// Frame's current URL.
    pub url: String,
    /// Frame's name attribute.
    pub name: String,
    /// Whether the frame is detached.
    pub detached: bool,
    /// Execution context IDs for this frame.
    /// Key is the world name: "" for main world, other strings for isolated worlds.
    pub execution_contexts: HashMap<String, ExecutionContextId>,
}

/// A frame within a page.
///
/// Frames are separate browsing contexts, typically created by `<iframe>` elements.
/// Each frame has its own DOM and JavaScript execution context.
#[derive(Debug)]
pub struct Frame {
    /// CDP connection.
    pub(super) connection: Arc<CdpConnection>,
    /// Session ID for this frame's page.
    pub(super) session_id: String,
    /// Unique frame identifier.
    pub(super) id: String,
    /// Parent frame ID (None for main frame).
    pub(super) parent_id: Option<String>,
    /// Loader ID for this frame.
    pub(super) loader_id: String,
    /// Mutable frame data.
    pub(super) data: RwLock<FrameData>,
    /// Execution context registry for looking up context IDs.
    pub(super) context_registry: Option<Arc<ExecutionContextRegistry>>,
}

impl Frame {
    /// Create a new frame from CDP frame info.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        session_id: String,
        id: String,
        parent_id: Option<String>,
        loader_id: String,
        url: String,
        name: String,
    ) -> Self {
        Self {
            connection,
            session_id,
            id,
            parent_id,
            loader_id,
            data: RwLock::new(FrameData {
                url,
                name,
                detached: false,
                execution_contexts: HashMap::new(),
            }),
            context_registry: None,
        }
    }

    /// Create a new frame with a context registry.
    pub(crate) fn with_context_registry(
        connection: Arc<CdpConnection>,
        session_id: String,
        id: String,
        parent_id: Option<String>,
        loader_id: String,
        url: String,
        name: String,
        context_registry: Arc<ExecutionContextRegistry>,
    ) -> Self {
        Self {
            connection,
            session_id,
            id,
            parent_id,
            loader_id,
            data: RwLock::new(FrameData {
                url,
                name,
                detached: false,
                execution_contexts: HashMap::new(),
            }),
            context_registry: Some(context_registry),
        }
    }

    /// Get the unique frame identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the parent frame ID.
    ///
    /// Returns `None` for the main frame.
    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    /// Check if this is the main frame.
    pub fn is_main(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Get the loader ID.
    pub fn loader_id(&self) -> &str {
        &self.loader_id
    }

    /// Get the frame's current URL.
    pub fn url(&self) -> String {
        self.data.read().url.clone()
    }

    /// Get the frame's name attribute.
    pub fn name(&self) -> String {
        self.data.read().name.clone()
    }

    /// Check if the frame has been detached.
    pub fn is_detached(&self) -> bool {
        self.data.read().detached
    }

    /// Update the frame's URL (called when frame navigates).
    pub(crate) fn set_url(&self, url: String) {
        self.data.write().url = url;
    }

    /// Update the frame's name.
    pub(crate) fn set_name(&self, name: String) {
        self.data.write().name = name;
    }

    /// Mark the frame as detached.
    pub(crate) fn set_detached(&self) {
        self.data.write().detached = true;
    }

    /// Get the main world execution context ID for this frame.
    ///
    /// First checks the shared context registry (if available), then falls back
    /// to the local FrameData. Returns `None` if no context has been set.
    pub(crate) fn main_world_context_id(&self) -> Option<ExecutionContextId> {
        // First try the shared registry
        if let Some(ref registry) = self.context_registry {
            if let Some(context_id) = registry.main_world_context(&self.id) {
                return Some(context_id);
            }
        }
        // Fall back to local data (for backwards compatibility)
        self.data.read().execution_contexts.get(MAIN_WORLD_KEY).copied()
    }

    /// Set an execution context for this frame.
    ///
    /// The world name should be:
    /// - "" (empty string) for the main world
    /// - A specific name for isolated worlds (e.g., "viewpoint-isolated")
    pub(crate) fn set_execution_context(&self, world_name: String, id: ExecutionContextId) {
        self.data.write().execution_contexts.insert(world_name, id);
    }

    /// Remove an execution context by its ID.
    ///
    /// Called when `Runtime.executionContextDestroyed` event is received.
    /// Returns `true` if a context was removed.
    pub(crate) fn remove_execution_context(&self, id: ExecutionContextId) -> bool {
        let mut data = self.data.write();
        let original_len = data.execution_contexts.len();
        data.execution_contexts.retain(|_, &mut ctx_id| ctx_id != id);
        data.execution_contexts.len() < original_len
    }

    /// Clear all execution contexts.
    ///
    /// Called when frame navigates to a new document.
    pub(crate) fn clear_execution_contexts(&self) {
        self.data.write().execution_contexts.clear();
    }

    /// Get the session ID.
    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the connection.
    pub(crate) fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }
}
