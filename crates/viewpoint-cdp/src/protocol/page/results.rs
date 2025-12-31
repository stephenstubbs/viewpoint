//! Page domain result types.

use serde::Deserialize;

use super::types::{FrameTree, NavigationEntry};

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

/// Result of Page.getFrameTree.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameTreeResult {
    /// Frame tree structure.
    pub frame_tree: FrameTree,
}

/// Result of Page.captureScreenshot.
#[derive(Debug, Clone, Deserialize)]
pub struct CaptureScreenshotResult {
    /// Base64-encoded image data.
    pub data: String,
}

/// Result of Page.printToPDF.
#[derive(Debug, Clone, Deserialize)]
pub struct PrintToPdfResult {
    /// Base64-encoded pdf data.
    pub data: String,
    /// A handle of the stream that holds resulting PDF data.
    pub stream: Option<String>,
}

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

/// Result of Page.addScriptToEvaluateOnNewDocument.
#[derive(Debug, Clone, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocumentResult {
    /// Identifier of the added script.
    pub identifier: String,
}

/// Result of Page.createIsolatedWorld.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldResult {
    /// Execution context of the isolated world.
    pub execution_context_id: crate::protocol::runtime::ExecutionContextId,
}
