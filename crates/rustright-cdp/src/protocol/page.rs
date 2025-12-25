//! Page domain types.
//!
//! The Page domain provides actions and events related to the inspected page.

use serde::{Deserialize, Serialize};

/// Frame information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    /// Frame unique identifier.
    pub id: String,
    /// Parent frame identifier.
    pub parent_id: Option<String>,
    /// Identifier of the loader associated with this frame.
    pub loader_id: String,
    /// Frame's name as specified in the tag.
    pub name: Option<String>,
    /// Frame document's URL.
    pub url: String,
    /// Frame document's security origin.
    pub security_origin: Option<String>,
    /// Frame document's mimeType.
    pub mime_type: Option<String>,
}

/// Parameters for Page.navigate.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigateParams {
    /// URL to navigate the page to.
    pub url: String,
    /// Referrer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// Intended transition type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_type: Option<String>,
    /// Frame id to navigate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<String>,
}

/// Result of Page.navigate.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigateResult {
    /// Frame id that has navigated (or failed to navigate).
    pub frame_id: String,
    /// Loader identifier.
    pub loader_id: Option<String>,
    /// User friendly error message if navigation failed.
    pub error_text: Option<String>,
}

/// Parameters for Page.reload.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReloadParams {
    /// If true, browser cache is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
    /// Script to inject into all frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_to_evaluate_on_load: Option<String>,
}

/// Result of Page.getFrameTree.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameTreeResult {
    /// Frame tree structure.
    pub frame_tree: FrameTree,
}

/// Frame tree structure.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameTree {
    /// Frame information.
    pub frame: Frame,
    /// Child frames.
    pub child_frames: Option<Vec<FrameTree>>,
}

/// Event: Page.loadEventFired
#[derive(Debug, Clone, Deserialize)]
pub struct LoadEventFiredEvent {
    /// Monotonic time.
    pub timestamp: f64,
}

/// Event: Page.domContentEventFired
#[derive(Debug, Clone, Deserialize)]
pub struct DomContentEventFiredEvent {
    /// Monotonic time.
    pub timestamp: f64,
}

/// Event: Page.frameNavigated
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameNavigatedEvent {
    /// Frame object.
    pub frame: Frame,
    /// Navigation type.
    #[serde(rename = "type")]
    pub navigation_type: Option<String>,
}

/// Event: Page.frameStartedLoading
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameStartedLoadingEvent {
    /// Frame ID.
    pub frame_id: String,
}

/// Event: Page.frameStoppedLoading
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameStoppedLoadingEvent {
    /// Frame ID.
    pub frame_id: String,
}

/// Event: Page.lifecycleEvent
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleEvent {
    /// Frame ID.
    pub frame_id: String,
    /// Loader identifier.
    pub loader_id: String,
    /// Lifecycle event name.
    pub name: String,
    /// Timestamp.
    pub timestamp: f64,
}

/// Parameters for Page.setLifecycleEventsEnabled.
#[derive(Debug, Clone, Serialize)]
pub struct SetLifecycleEventsEnabledParams {
    /// Whether to enable lifecycle events.
    pub enabled: bool,
}
