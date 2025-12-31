//! Page domain core types.

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

/// Frame tree structure.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameTree {
    /// Frame information.
    pub frame: Frame,
    /// Child frames.
    pub child_frames: Option<Vec<FrameTree>>,
}

/// Image format for screenshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ScreenshotFormat {
    /// PNG format (default).
    #[default]
    Png,
    /// JPEG format.
    Jpeg,
    /// WebP format.
    Webp,
}

/// Viewport for capturing a screenshot.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    /// X offset in device independent pixels.
    pub x: f64,
    /// Y offset in device independent pixels.
    pub y: f64,
    /// Rectangle width in device independent pixels.
    pub width: f64,
    /// Rectangle height in device independent pixels.
    pub height: f64,
    /// Page scale factor.
    pub scale: f64,
}

/// Navigation history entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEntry {
    /// Unique id of the navigation history entry.
    pub id: i32,
    /// URL of the navigation history entry.
    pub url: String,
    /// URL that the user typed in the URL bar.
    #[serde(default)]
    pub user_typed_url: String,
    /// Title of the navigation history entry.
    pub title: String,
    /// Transition type.
    #[serde(default)]
    pub transition_type: String,
}

/// Reason for frame being detached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FrameDetachedReason {
    /// Frame was removed from the DOM.
    Remove,
    /// Frame was swapped (e.g., for out-of-process iframe).
    Swap,
}

/// File chooser mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileChooserMode {
    /// Select a single file.
    SelectSingle,
    /// Select multiple files.
    SelectMultiple,
}
