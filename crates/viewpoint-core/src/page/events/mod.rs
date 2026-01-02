//! Page event handling for dialogs, downloads, file choosers, console, and errors.
//!
//! This module provides the event management infrastructure for page-level events.

// Allow dead code for event scaffolding (various specs)

mod download_handling;
mod event_listener;
mod page_handlers;
mod types;

use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, RwLock, oneshot};
use tracing::{debug, warn};
use viewpoint_cdp::CdpConnection;

use super::console::ConsoleMessage;
use super::dialog::Dialog;
use super::download::Download;
use super::file_chooser::FileChooser;
use super::frame::Frame;
use super::page_error::PageError as PageErrorInfo;
use crate::error::PageError;

pub use types::{
    ConsoleHandler, DialogHandler, DownloadHandler, FileChooserHandler, FrameAttachedHandler,
    FrameDetachedHandler, FrameNavigatedHandler, PageErrorHandler,
};

use download_handling::DownloadTracker;

/// Page event manager for handling CDP events.
pub struct PageEventManager {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Dialog handler.
    dialog_handler: Arc<RwLock<Option<DialogHandler>>>,
    /// Download handler.
    download_handler: Arc<RwLock<Option<DownloadHandler>>>,
    /// File chooser handler.
    file_chooser_handler: Arc<RwLock<Option<FileChooserHandler>>>,
    /// Console message handler.
    console_handler: Arc<RwLock<Option<ConsoleHandler>>>,
    /// Page error handler.
    pageerror_handler: Arc<RwLock<Option<PageErrorHandler>>>,
    /// Frame attached handler.
    frameattached_handler: Arc<RwLock<Option<FrameAttachedHandler>>>,
    /// Frame navigated handler.
    framenavigated_handler: Arc<RwLock<Option<FrameNavigatedHandler>>>,
    /// Frame detached handler.
    framedetached_handler: Arc<RwLock<Option<FrameDetachedHandler>>>,
    /// Active downloads.
    downloads: Arc<Mutex<HashMap<String, DownloadTracker>>>,
    /// Download directory.
    download_dir: PathBuf,
    /// Whether file chooser interception is enabled.
    file_chooser_intercepted: Arc<RwLock<bool>>,
    /// One-shot sender for wait_for_dialog.
    wait_for_dialog_tx: Arc<Mutex<Option<oneshot::Sender<Dialog>>>>,
    /// One-shot sender for wait_for_download.
    wait_for_download_tx: Arc<Mutex<Option<oneshot::Sender<Download>>>>,
    /// One-shot sender for wait_for_file_chooser.
    wait_for_file_chooser_tx: Arc<Mutex<Option<oneshot::Sender<FileChooser>>>>,
    /// One-shot sender for wait_for_console.
    wait_for_console_tx: Arc<Mutex<Option<oneshot::Sender<ConsoleMessage>>>>,
    /// One-shot sender for wait_for_pageerror.
    wait_for_pageerror_tx: Arc<Mutex<Option<oneshot::Sender<PageErrorInfo>>>>,
}

// Manual Debug implementation since handlers don't implement Debug
impl std::fmt::Debug for PageEventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageEventManager")
            .field("session_id", &self.session_id)
            .field("download_dir", &self.download_dir)
            .finish_non_exhaustive()
    }
}

impl PageEventManager {
    /// Create a new page event manager.
    pub fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        let download_dir = std::env::temp_dir().join("viewpoint-downloads");
        let manager = Self {
            connection: connection.clone(),
            session_id: session_id.clone(),
            dialog_handler: Arc::new(RwLock::new(None)),
            download_handler: Arc::new(RwLock::new(None)),
            file_chooser_handler: Arc::new(RwLock::new(None)),
            console_handler: Arc::new(RwLock::new(None)),
            pageerror_handler: Arc::new(RwLock::new(None)),
            frameattached_handler: Arc::new(RwLock::new(None)),
            framenavigated_handler: Arc::new(RwLock::new(None)),
            framedetached_handler: Arc::new(RwLock::new(None)),
            downloads: Arc::new(Mutex::new(HashMap::new())),
            download_dir,
            file_chooser_intercepted: Arc::new(RwLock::new(false)),
            wait_for_dialog_tx: Arc::new(Mutex::new(None)),
            wait_for_download_tx: Arc::new(Mutex::new(None)),
            wait_for_file_chooser_tx: Arc::new(Mutex::new(None)),
            wait_for_console_tx: Arc::new(Mutex::new(None)),
            wait_for_pageerror_tx: Arc::new(Mutex::new(None)),
        };

        // Start the event listener
        manager.start_event_listener();

        manager
    }

    /// Start the background event listener for console, pageerror, dialog, frame, and download events.
    fn start_event_listener(&self) {
        event_listener::start_event_listener(
            self.connection.clone(),
            self.session_id.clone(),
            self.console_handler.clone(),
            self.pageerror_handler.clone(),
            self.dialog_handler.clone(),
            self.frameattached_handler.clone(),
            self.framenavigated_handler.clone(),
            self.framedetached_handler.clone(),
            self.wait_for_console_tx.clone(),
            self.wait_for_pageerror_tx.clone(),
            self.wait_for_dialog_tx.clone(),
            // Download handling
            self.download_handler.clone(),
            self.downloads.clone(),
            self.wait_for_download_tx.clone(),
        );
    }

    /// Set the dialog handler.
    pub async fn set_dialog_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(Dialog) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), PageError>> + Send + 'static,
    {
        let mut dialog_handler = self.dialog_handler.write().await;
        *dialog_handler = Some(Box::new(move |dialog| Box::pin(handler(dialog))));
    }

    /// Remove the dialog handler.
    pub async fn remove_dialog_handler(&self) {
        let mut dialog_handler = self.dialog_handler.write().await;
        *dialog_handler = None;
    }

    /// Set the download handler.
    pub async fn set_download_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(Download) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut download_handler = self.download_handler.write().await;
        *download_handler = Some(Box::new(move |download| Box::pin(handler(download))));
    }

    /// Set the file chooser handler.
    pub async fn set_file_chooser_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(FileChooser) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut file_chooser_handler = self.file_chooser_handler.write().await;
        *file_chooser_handler = Some(Box::new(move |chooser| Box::pin(handler(chooser))));
    }

    /// Set the console handler.
    pub async fn set_console_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(ConsoleMessage) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut console_handler = self.console_handler.write().await;
        *console_handler = Some(Box::new(move |message| Box::pin(handler(message))));
    }

    /// Remove the console handler.
    pub async fn remove_console_handler(&self) {
        let mut console_handler = self.console_handler.write().await;
        *console_handler = None;
    }

    /// Set the page error handler.
    pub async fn set_pageerror_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(PageErrorInfo) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut pageerror_handler = self.pageerror_handler.write().await;
        *pageerror_handler = Some(Box::new(move |error| Box::pin(handler(error))));
    }

    /// Remove the page error handler.
    pub async fn remove_pageerror_handler(&self) {
        let mut pageerror_handler = self.pageerror_handler.write().await;
        *pageerror_handler = None;
    }

    /// Set the frame attached handler.
    pub async fn set_frameattached_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(Frame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut frameattached_handler = self.frameattached_handler.write().await;
        *frameattached_handler = Some(Box::new(move |frame| Box::pin(handler(frame))));
    }

    /// Remove the frame attached handler.
    pub async fn remove_frameattached_handler(&self) {
        let mut frameattached_handler = self.frameattached_handler.write().await;
        *frameattached_handler = None;
    }

    /// Set the frame navigated handler.
    pub async fn set_framenavigated_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(Frame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut framenavigated_handler = self.framenavigated_handler.write().await;
        *framenavigated_handler = Some(Box::new(move |frame| Box::pin(handler(frame))));
    }

    /// Remove the frame navigated handler.
    pub async fn remove_framenavigated_handler(&self) {
        let mut framenavigated_handler = self.framenavigated_handler.write().await;
        *framenavigated_handler = None;
    }

    /// Set the frame detached handler.
    pub async fn set_framedetached_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(Frame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut framedetached_handler = self.framedetached_handler.write().await;
        *framedetached_handler = Some(Box::new(move |frame| Box::pin(handler(frame))));
    }

    /// Remove the frame detached handler.
    pub async fn remove_framedetached_handler(&self) {
        let mut framedetached_handler = self.framedetached_handler.write().await;
        *framedetached_handler = None;
    }

    /// Handle frame attached event externally.
    pub async fn handle_frame_attached(&self, frame: Frame) {
        let handler = self.frameattached_handler.read().await;
        if let Some(ref h) = *handler {
            h(frame).await;
        }
    }

    /// Handle frame navigated event externally.
    pub async fn handle_frame_navigated(&self, frame: Frame) {
        let handler = self.framenavigated_handler.read().await;
        if let Some(ref h) = *handler {
            h(frame).await;
        }
    }

    /// Handle frame detached event externally.
    pub async fn handle_frame_detached(&self, frame: Frame) {
        let handler = self.framedetached_handler.read().await;
        if let Some(ref h) = *handler {
            h(frame).await;
        }
    }

    /// Set whether to intercept file chooser dialogs.
    pub async fn set_intercept_file_chooser(&self, enabled: bool) -> Result<(), PageError> {
        *self.file_chooser_intercepted.write().await = enabled;

        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.setInterceptFileChooserDialog",
                Some(serde_json::json!({ "enabled": enabled })),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Set the download behavior.
    pub async fn set_download_behavior(&self, allow: bool) -> Result<(), PageError> {
        // Ensure download directory exists
        if allow {
            tokio::fs::create_dir_all(&self.download_dir)
                .await
                .map_err(|e| {
                    PageError::EvaluationFailed(format!("Failed to create download directory: {e}"))
                })?;
        }

        let behavior = if allow { "allow" } else { "deny" };

        self.connection
            .send_command::<_, serde_json::Value>(
                "Browser.setDownloadBehavior",
                Some(serde_json::json!({
                    "behavior": behavior,
                    "downloadPath": self.download_dir.to_string_lossy(),
                    "eventsEnabled": true,
                })),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Handle a dialog event from CDP.
    pub async fn handle_dialog_event(
        &self,
        dialog_type: viewpoint_cdp::protocol::DialogType,
        message: String,
        default_prompt: Option<String>,
    ) {
        let dialog = Dialog::new(
            self.connection.clone(),
            self.session_id.clone(),
            dialog_type,
            message,
            default_prompt,
        );

        // Check if there's a waiter
        {
            let mut waiter = self.wait_for_dialog_tx.lock().await;
            if let Some(tx) = waiter.take() {
                let _ = tx.send(dialog);
                return;
            }
        }

        // Check if there's a handler
        let handler = self.dialog_handler.read().await;
        if let Some(ref h) = *handler {
            if let Err(e) = h(dialog).await {
                warn!("Dialog handler failed: {}", e);
            }
        } else {
            // Auto-dismiss if no handler
            debug!("Auto-dismissing dialog (no handler registered)");
            if let Err(e) = dialog.dismiss().await {
                warn!("Failed to auto-dismiss dialog: {}", e);
            }
        }
    }

    /// Handle download begin event.
    pub async fn handle_download_begin(
        &self,
        guid: String,
        suggested_filename: String,
        url: String,
    ) {
        download_handling::handle_download_begin(
            &self.connection,
            &self.session_id,
            &self.downloads,
            &self.download_handler,
            &self.wait_for_download_tx,
            guid,
            suggested_filename,
            url,
        )
        .await;
    }

    /// Handle download progress event.
    pub async fn handle_download_progress(&self, guid: String, state: &str) {
        download_handling::handle_download_progress(&self.downloads, guid, state).await;
    }

    /// Handle file chooser event.
    pub async fn handle_file_chooser_event(
        &self,
        frame_id: String,
        mode: viewpoint_cdp::protocol::page::FileChooserMode,
        backend_node_id: Option<i32>,
    ) {
        download_handling::handle_file_chooser_event(
            &self.connection,
            &self.session_id,
            &self.file_chooser_handler,
            &self.wait_for_file_chooser_tx,
            frame_id,
            mode,
            backend_node_id,
        )
        .await;
    }

    /// Wait for a dialog to appear.
    pub async fn wait_for_dialog(&self, timeout: Duration) -> Result<Dialog, PageError> {
        let (tx, rx) = oneshot::channel();
        {
            let mut waiter = self.wait_for_dialog_tx.lock().await;
            *waiter = Some(tx);
        }

        tokio::time::timeout(timeout, rx)
            .await
            .map_err(|_| PageError::EvaluationFailed("Timeout waiting for dialog".to_string()))?
            .map_err(|_| PageError::EvaluationFailed("Dialog wait cancelled".to_string()))
    }

    /// Wait for a download to start.
    pub async fn wait_for_download(&self, timeout: Duration) -> Result<Download, PageError> {
        download_handling::wait_for_download(&self.wait_for_download_tx, timeout).await
    }

    /// Register a download waiter and return the receiver.
    /// Use this when you need to register before performing an action.
    pub async fn register_download_waiter(&self) -> oneshot::Receiver<Download> {
        download_handling::register_download_waiter(&self.wait_for_download_tx).await
    }

    /// Await a previously registered download waiter with timeout.
    pub async fn await_download_waiter(
        &self,
        rx: oneshot::Receiver<Download>,
        timeout: Duration,
    ) -> Result<Download, PageError> {
        download_handling::await_download_waiter(rx, timeout).await
    }

    /// Wait for a file chooser to open.
    pub async fn wait_for_file_chooser(&self, timeout: Duration) -> Result<FileChooser, PageError> {
        download_handling::wait_for_file_chooser(&self.wait_for_file_chooser_tx, timeout).await
    }

    /// Wait for a console message.
    pub async fn wait_for_console(&self, timeout: Duration) -> Result<ConsoleMessage, PageError> {
        let (tx, rx) = oneshot::channel();
        {
            let mut waiter = self.wait_for_console_tx.lock().await;
            *waiter = Some(tx);
        }

        tokio::time::timeout(timeout, rx)
            .await
            .map_err(|_| {
                PageError::EvaluationFailed("Timeout waiting for console message".to_string())
            })?
            .map_err(|_| PageError::EvaluationFailed("Console wait cancelled".to_string()))
    }

    /// Wait for a page error.
    pub async fn wait_for_pageerror(&self, timeout: Duration) -> Result<PageErrorInfo, PageError> {
        let (tx, rx) = oneshot::channel();
        {
            let mut waiter = self.wait_for_pageerror_tx.lock().await;
            *waiter = Some(tx);
        }

        tokio::time::timeout(timeout, rx)
            .await
            .map_err(|_| PageError::EvaluationFailed("Timeout waiting for page error".to_string()))?
            .map_err(|_| PageError::EvaluationFailed("Page error wait cancelled".to_string()))
    }
}
