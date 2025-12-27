//! Page download types.
//!
//! Types for Page.downloadWillBegin, Page.downloadProgress, and related events.

use serde::{Deserialize, Serialize};

/// Event: Page.downloadWillBegin
///
/// Fired when page is about to start a download.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadWillBeginEvent {
    /// Id of the frame that caused download to begin.
    pub frame_id: String,
    /// Global unique identifier of the download.
    pub guid: String,
    /// URL of the resource being downloaded.
    pub url: String,
    /// Suggested file name of the resource (the actual name of the file saved on disk may differ).
    pub suggested_filename: String,
}

/// Download progress state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DownloadProgressState {
    /// Download is in progress.
    InProgress,
    /// Download completed.
    Completed,
    /// Download was canceled.
    Canceled,
}

/// Event: Page.downloadProgress
///
/// Fired when download makes progress.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgressEvent {
    /// Global unique identifier of the download.
    pub guid: String,
    /// Total expected bytes to download.
    pub total_bytes: f64,
    /// Total bytes received.
    pub received_bytes: f64,
    /// Download status.
    pub state: DownloadProgressState,
}

/// Parameters for Page.setDownloadBehavior.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDownloadBehaviorParams {
    /// Whether to allow all or deny all download requests, or use default Chrome behavior if
    /// available (otherwise deny).
    pub behavior: DownloadBehavior,
    /// The default path to save downloaded files to. This is required if behavior is set to 'allow'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_path: Option<String>,
}

/// Download behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DownloadBehavior {
    /// Deny all downloads.
    Deny,
    /// Allow all downloads.
    Allow,
    /// Use default behavior.
    Default,
}
