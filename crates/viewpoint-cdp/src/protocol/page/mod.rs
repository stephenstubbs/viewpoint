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

// ============================================================================
// Screenshot
// ============================================================================

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

/// Parameters for Page.captureScreenshot.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshotParams {
    /// Image compression format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScreenshotFormat>,
    /// Compression quality from range [0..100] (jpeg/webp only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u8>,
    /// Capture the screenshot of a given region only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<Viewport>,
    /// Capture the screenshot from the surface, rather than the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_surface: Option<bool>,
    /// Capture the screenshot beyond the viewport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_beyond_viewport: Option<bool>,
    /// Optimize image encoding for speed, not for resulting size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_for_speed: Option<bool>,
}

/// Result of Page.captureScreenshot.
#[derive(Debug, Clone, Deserialize)]
pub struct CaptureScreenshotResult {
    /// Base64-encoded image data.
    pub data: String,
}

// ============================================================================
// PDF
// ============================================================================

/// Parameters for Page.printToPDF.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPdfParams {
    /// Paper orientation (default: false = portrait).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    /// Display header and footer (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_header_footer: Option<bool>,
    /// Print background graphics (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_background: Option<bool>,
    /// Scale of the webpage rendering (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Paper width in inches (default: 8.5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_width: Option<f64>,
    /// Paper height in inches (default: 11).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_height: Option<f64>,
    /// Top margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<f64>,
    /// Bottom margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<f64>,
    /// Left margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    /// Right margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<f64>,
    /// Paper ranges to print, e.g., '1-5, 8, 11-13'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<String>,
    /// HTML template for the print header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_template: Option<String>,
    /// HTML template for the print footer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_template: Option<String>,
    /// Whether or not to prefer page size as defined by css.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_css_page_size: Option<bool>,
    /// Return as stream (experimental).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<String>,
    /// Whether to generate tagged PDF. Default: true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_tagged_pdf: Option<bool>,
    /// Whether to generate document outline. Default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_document_outline: Option<bool>,
}

/// Result of Page.printToPDF.
#[derive(Debug, Clone, Deserialize)]
pub struct PrintToPdfResult {
    /// Base64-encoded pdf data.
    pub data: String,
    /// A handle of the stream that holds resulting PDF data.
    pub stream: Option<String>,
}

// ============================================================================
// Navigation History
// ============================================================================

/// Result of Page.goBack / Page.goForward.
#[derive(Debug, Clone, Deserialize)]
pub struct NavigationHistoryResult {}

/// Result of Page.getNavigationHistory.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNavigationHistoryResult {
    /// Index of the current navigation history entry.
    pub current_index: i32,
    /// Array of navigation history entries.
    pub entries: Vec<NavigationEntry>,
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

/// Parameters for Page.navigateToHistoryEntry.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigateToHistoryEntryParams {
    /// Unique id of the entry to navigate to.
    pub entry_id: i32,
}

// ============================================================================
// Init Scripts
// ============================================================================

/// Parameters for Page.addScriptToEvaluateOnNewDocument.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocumentParams {
    /// JavaScript source code to evaluate.
    pub source: String,
    /// If specified, creates an isolated world and evaluates given script in it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_name: Option<String>,
    /// Whether this script should be injected into all frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_command_line_api: Option<bool>,
    /// If true, this script will run in utility world.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_immediately: Option<bool>,
}

/// Result of Page.addScriptToEvaluateOnNewDocument.
#[derive(Debug, Clone, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocumentResult {
    /// Identifier of the added script.
    pub identifier: String,
}

/// Parameters for Page.removeScriptToEvaluateOnNewDocument.
#[derive(Debug, Clone, Serialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentParams {
    /// Identifier of the script to remove.
    pub identifier: String,
}

// ============================================================================
// Page State
// ============================================================================

/// Parameters for Page.bringToFront (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct BringToFrontParams {}

/// Parameters for Page.setDocumentContent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentContentParams {
    /// Frame id to set HTML for.
    pub frame_id: String,
    /// HTML content to set.
    pub html: String,
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

// ============================================================================
// Frame Events
// ============================================================================

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

/// Reason for frame being detached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FrameDetachedReason {
    /// Frame was removed from the DOM.
    Remove,
    /// Frame was swapped (e.g., for out-of-process iframe).
    Swap,
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

// ============================================================================
// File Chooser Events
// ============================================================================

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

/// File chooser mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileChooserMode {
    /// Select a single file.
    SelectSingle,
    /// Select multiple files.
    SelectMultiple,
}

/// Parameters for Page.setInterceptFileChooserDialog.
#[derive(Debug, Clone, Serialize)]
pub struct SetInterceptFileChooserDialogParams {
    /// Whether to intercept file chooser dialogs.
    pub enabled: bool,
}
