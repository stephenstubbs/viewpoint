//! Page domain event types.

use serde::Deserialize;

use super::types::{FileChooserMode, Frame, FrameDetachedReason};

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

/// Event: Page.windowOpen
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOpenEvent {
    /// The URL for the new window.
    pub url: String,
    /// Window name.
    pub window_name: String,
    /// An array of enabled window features.
    pub window_features: Vec<String>,
    /// Whether or not it was triggered by user gesture.
    pub user_gesture: bool,
}

/// Event: Page.frameAttached
///
/// Fired when a frame has been attached to its parent.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameAttachedEvent {
    /// Id of the frame that has been attached.
    pub frame_id: String,
    /// Parent frame identifier.
    pub parent_frame_id: String,
    /// JavaScript stack trace of when frame was attached, only set if frame initiated from script.
    pub stack: Option<serde_json::Value>,
}

/// Event: Page.frameDetached
///
/// Fired when a frame has been detached from its parent.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameDetachedEvent {
    /// Id of the frame that has been detached.
    pub frame_id: String,
    /// Reason for the frame being detached.
    pub reason: Option<FrameDetachedReason>,
}

/// Event: Page.navigatedWithinDocument
///
/// Fired when a frame navigation happened within the same document.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigatedWithinDocumentEvent {
    /// Id of the frame.
    pub frame_id: String,
    /// Frame's new url.
    pub url: String,
}

/// Event: Page.fileChooserOpened
///
/// Emitted only when `page.setInterceptFileChooserDialog` is enabled.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChooserOpenedEvent {
    /// Id of the frame containing input node.
    pub frame_id: String,
    /// Input mode.
    pub mode: FileChooserMode,
    /// Input node id. Only present for file choosers opened via an input element
    /// with webkitdirectory attribute (directory picker).
    pub backend_node_id: Option<i32>,
}
