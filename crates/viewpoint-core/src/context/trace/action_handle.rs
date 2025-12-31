//! Action handle for tracking action duration in traces.

use tokio::sync::RwLock;
use std::sync::Arc;

use super::types::TracingState;

/// Handle for tracking an action's duration in the trace.
pub struct ActionHandle {
    state: Arc<RwLock<TracingState>>,
    index: usize,
}

impl ActionHandle {
    /// Create a new action handle.
    pub(crate) fn new(state: Arc<RwLock<TracingState>>, index: usize) -> Self {
        Self { state, index }
    }

    /// Complete the action with success.
    pub async fn complete(self, result: Option<serde_json::Value>) {
        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            * 1000.0;

        let mut state = self.state.write().await;
        if let Some(action) = state.actions.get_mut(self.index) {
            action.end_time = Some(end_time);
            action.result = result;
        }
    }

    /// Complete the action with an error.
    pub async fn fail(self, error: &str) {
        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            * 1000.0;

        let mut state = self.state.write().await;
        if let Some(action) = state.actions.get_mut(self.index) {
            action.end_time = Some(end_time);
            action.result = Some(serde_json::json!({ "error": error }));
        }
    }
}
