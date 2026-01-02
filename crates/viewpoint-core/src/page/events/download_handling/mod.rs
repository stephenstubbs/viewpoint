//! Download and file chooser event handling.
//!
//! This module contains the handlers for download and file chooser events.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, RwLock, oneshot, watch};
use tracing::debug;
use viewpoint_cdp::CdpConnection;

use super::super::download::{Download, DownloadState};
use super::super::file_chooser::FileChooser;
use super::types::{DownloadHandler, FileChooserHandler};
use crate::error::PageError;

/// Download tracking state.
pub(crate) struct DownloadTracker {
    /// Sender for state updates.
    pub state_tx: watch::Sender<DownloadState>,
    /// Sender for path updates.
    pub path_tx: watch::Sender<Option<PathBuf>>,
}

/// Handle download begin event.
pub(super) async fn handle_download_begin(
    _connection: &Arc<CdpConnection>,
    _session_id: &str,
    downloads: &Mutex<HashMap<String, DownloadTracker>>,
    download_handler: &RwLock<Option<DownloadHandler>>,
    wait_for_download_tx: &Mutex<Option<oneshot::Sender<Download>>>,
    guid: String,
    suggested_filename: String,
    url: String,
) {
    debug!(guid = %guid, filename = %suggested_filename, "Download started");

    let (state_tx, state_rx) = watch::channel(DownloadState::InProgress);
    let (path_tx, path_rx) = watch::channel(None);

    // Note: Download::new expects (guid, url, suggested_filename, ...)
    let download = Download::new(guid.clone(), url, suggested_filename, state_rx, path_rx);

    // Store the tracker
    {
        let mut downloads = downloads.lock().await;
        downloads.insert(guid, DownloadTracker { state_tx, path_tx });
    }

    // Check if there's a waiter
    {
        let mut waiter = wait_for_download_tx.lock().await;
        if let Some(tx) = waiter.take() {
            let _ = tx.send(download);
            return;
        }
    }

    // Call the handler
    let handler = download_handler.read().await;
    if let Some(ref h) = *handler {
        h(download).await;
    }
}

/// Handle download progress event.
pub(super) async fn handle_download_progress(
    downloads: &Mutex<HashMap<String, DownloadTracker>>,
    guid: String,
    state: &str,
) {
    let downloads = downloads.lock().await;
    if let Some(tracker) = downloads.get(&guid) {
        let new_state = match state {
            "completed" => DownloadState::Completed,
            "canceled" => DownloadState::Canceled,
            _ => DownloadState::InProgress,
        };
        let _ = tracker.state_tx.send(new_state);

        // Set the path for completed downloads
        if state == "completed" {
            // The path is typically in the download directory with the suggested filename
            // This will be set by the actual download complete event
        }
    }
}

/// Handle file chooser event.
pub(super) async fn handle_file_chooser_event(
    connection: &Arc<CdpConnection>,
    session_id: &str,
    file_chooser_handler: &RwLock<Option<FileChooserHandler>>,
    wait_for_file_chooser_tx: &Mutex<Option<oneshot::Sender<FileChooser>>>,
    frame_id: String,
    mode: viewpoint_cdp::protocol::page::FileChooserMode,
    backend_node_id: Option<i32>,
) {
    debug!(frame_id = %frame_id, mode = ?mode, "File chooser opened");

    let is_multiple = matches!(
        mode,
        viewpoint_cdp::protocol::page::FileChooserMode::SelectMultiple
    );

    let chooser = FileChooser::new(
        connection.clone(),
        session_id.to_string(),
        frame_id,
        backend_node_id,
        is_multiple,
    );

    // Check if there's a waiter
    {
        let mut waiter = wait_for_file_chooser_tx.lock().await;
        if let Some(tx) = waiter.take() {
            let _ = tx.send(chooser);
            return;
        }
    }

    // Call the handler
    let handler = file_chooser_handler.read().await;
    if let Some(ref h) = *handler {
        h(chooser).await;
    }
}

/// Wait for a download to start.
pub(super) async fn wait_for_download(
    wait_for_download_tx: &Mutex<Option<oneshot::Sender<Download>>>,
    timeout: Duration,
) -> Result<Download, PageError> {
    let (tx, rx) = oneshot::channel();
    {
        let mut waiter = wait_for_download_tx.lock().await;
        *waiter = Some(tx);
    }

    tokio::time::timeout(timeout, rx)
        .await
        .map_err(|_| PageError::EvaluationFailed("Timeout waiting for download".to_string()))?
        .map_err(|_| PageError::EvaluationFailed("Download wait cancelled".to_string()))
}

/// Register a download waiter and return the receiver.
/// This allows synchronous registration before an action that triggers a download.
pub(super) async fn register_download_waiter(
    wait_for_download_tx: &Mutex<Option<oneshot::Sender<Download>>>,
) -> oneshot::Receiver<Download> {
    let (tx, rx) = oneshot::channel();
    {
        let mut waiter = wait_for_download_tx.lock().await;
        *waiter = Some(tx);
    }
    rx
}

/// Await a previously registered download waiter with timeout.
pub(super) async fn await_download_waiter(
    rx: oneshot::Receiver<Download>,
    timeout: Duration,
) -> Result<Download, PageError> {
    tokio::time::timeout(timeout, rx)
        .await
        .map_err(|_| PageError::EvaluationFailed("Timeout waiting for download".to_string()))?
        .map_err(|_| PageError::EvaluationFailed("Download wait cancelled".to_string()))
}

/// Wait for a file chooser to open.
pub(super) async fn wait_for_file_chooser(
    wait_for_file_chooser_tx: &Mutex<Option<oneshot::Sender<FileChooser>>>,
    timeout: Duration,
) -> Result<FileChooser, PageError> {
    let (tx, rx) = oneshot::channel();
    {
        let mut waiter = wait_for_file_chooser_tx.lock().await;
        *waiter = Some(tx);
    }

    tokio::time::timeout(timeout, rx)
        .await
        .map_err(|_| PageError::EvaluationFailed("Timeout waiting for file chooser".to_string()))?
        .map_err(|_| PageError::EvaluationFailed("File chooser wait cancelled".to_string()))
}
