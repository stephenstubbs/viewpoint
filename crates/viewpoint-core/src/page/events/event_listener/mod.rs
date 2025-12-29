//! Background event listener for page events.
//!
//! This module contains the CDP event listener that processes console, error,
//! dialog, frame, and download events.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{Mutex, RwLock, oneshot, watch};
use tracing::{debug, warn};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::page::{FrameAttachedEvent, FrameDetachedEvent, FrameNavigatedEvent};
use viewpoint_cdp::protocol::runtime::{ConsoleApiCalledEvent, ExceptionThrownEvent};
use viewpoint_cdp::protocol::{
    DownloadProgressEvent, DownloadWillBeginEvent, JavascriptDialogOpeningEvent,
};

use super::super::console::ConsoleMessage;
use super::super::dialog::Dialog;
use super::super::download::{Download, DownloadState};
use super::super::frame::Frame;
use super::super::page_error::PageError as PageErrorInfo;
use super::download_handling::DownloadTracker;
use super::types::{
    ConsoleHandler, DialogHandler, DownloadHandler, FrameAttachedHandler, FrameDetachedHandler,
    FrameNavigatedHandler, PageErrorHandler,
};

/// Start the background event listener for console, pageerror, dialog, frame, and download events.
#[allow(clippy::too_many_arguments)]
pub(super) fn start_event_listener(
    connection: Arc<CdpConnection>,
    session_id: String,
    console_handler: Arc<RwLock<Option<ConsoleHandler>>>,
    pageerror_handler: Arc<RwLock<Option<PageErrorHandler>>>,
    dialog_handler: Arc<RwLock<Option<DialogHandler>>>,
    frameattached_handler: Arc<RwLock<Option<FrameAttachedHandler>>>,
    framenavigated_handler: Arc<RwLock<Option<FrameNavigatedHandler>>>,
    framedetached_handler: Arc<RwLock<Option<FrameDetachedHandler>>>,
    wait_for_console_tx: Arc<Mutex<Option<oneshot::Sender<ConsoleMessage>>>>,
    wait_for_pageerror_tx: Arc<Mutex<Option<oneshot::Sender<PageErrorInfo>>>>,
    wait_for_dialog_tx: Arc<Mutex<Option<oneshot::Sender<Dialog>>>>,
    // Download handling
    download_handler: Arc<RwLock<Option<DownloadHandler>>>,
    downloads: Arc<Mutex<HashMap<String, DownloadTracker>>>,
    wait_for_download_tx: Arc<Mutex<Option<oneshot::Sender<Download>>>>,
) {
    let mut events = connection.subscribe_events();

    tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            // Browser.download* events may not have session_id, so handle them separately
            match event.method.as_str() {
                "Browser.downloadWillBegin" => {
                    // Download events are sent with session_id when triggered from a page
                    if event.session_id.as_deref() == Some(&session_id) {
                        handle_download_will_begin(
                            event.params.as_ref(),
                            &download_handler,
                            &downloads,
                            &wait_for_download_tx,
                        )
                        .await;
                    }
                    continue;
                }
                "Browser.downloadProgress" => {
                    if event.session_id.as_deref() == Some(&session_id) {
                        handle_download_progress(event.params.as_ref(), &downloads).await;
                    }
                    continue;
                }
                _ => {}
            }

            // Filter other events for this session
            if event.session_id.as_deref() != Some(&session_id) {
                continue;
            }

            match event.method.as_str() {
                "Runtime.consoleAPICalled" => {
                    handle_console_event(
                        event.params.as_ref(),
                        &connection,
                        &session_id,
                        &console_handler,
                        &wait_for_console_tx,
                    )
                    .await;
                }
                "Runtime.exceptionThrown" => {
                    handle_exception_event(
                        event.params.as_ref(),
                        &pageerror_handler,
                        &wait_for_pageerror_tx,
                    )
                    .await;
                }
                "Page.frameAttached" => {
                    handle_frame_attached_event(
                        event.params.as_ref(),
                        &connection,
                        &session_id,
                        &frameattached_handler,
                    )
                    .await;
                }
                "Page.frameNavigated" => {
                    handle_frame_navigated_event(
                        event.params.as_ref(),
                        &connection,
                        &session_id,
                        &framenavigated_handler,
                    )
                    .await;
                }
                "Page.frameDetached" => {
                    handle_frame_detached_event(
                        event.params.as_ref(),
                        &connection,
                        &session_id,
                        &framedetached_handler,
                    )
                    .await;
                }
                "Page.javascriptDialogOpening" => {
                    handle_dialog_event(
                        event.params.as_ref(),
                        &connection,
                        &session_id,
                        &dialog_handler,
                        &wait_for_dialog_tx,
                    )
                    .await;
                }
                _ => {}
            }
        }
    });
}

async fn handle_console_event(
    params: Option<&serde_json::Value>,
    connection: &Arc<CdpConnection>,
    session_id: &str,
    console_handler: &Arc<RwLock<Option<ConsoleHandler>>>,
    wait_for_console_tx: &Arc<Mutex<Option<oneshot::Sender<ConsoleMessage>>>>,
) {
    if let Some(params) = params {
        if let Ok(console_event) = serde_json::from_value::<ConsoleApiCalledEvent>(params.clone()) {
            let message = ConsoleMessage::from_event(
                console_event,
                connection.clone(),
                session_id.to_string(),
            );

            // Check if there's a waiter
            {
                let mut waiter = wait_for_console_tx.lock().await;
                if let Some(tx) = waiter.take() {
                    let _ = tx.send(message);
                    return;
                }
            }

            // Check if there's a handler
            let handler = console_handler.read().await;
            if let Some(ref h) = *handler {
                h(message).await;
            } else {
                debug!("Console message (no handler): received");
            }
        }
    }
}

async fn handle_exception_event(
    params: Option<&serde_json::Value>,
    pageerror_handler: &Arc<RwLock<Option<PageErrorHandler>>>,
    wait_for_pageerror_tx: &Arc<Mutex<Option<oneshot::Sender<PageErrorInfo>>>>,
) {
    if let Some(params) = params {
        if let Ok(exception_event) = serde_json::from_value::<ExceptionThrownEvent>(params.clone())
        {
            let error = PageErrorInfo::from_event(exception_event);

            // Check if there's a waiter
            {
                let mut waiter = wait_for_pageerror_tx.lock().await;
                if let Some(tx) = waiter.take() {
                    let _ = tx.send(error);
                    return;
                }
            }

            // Check if there's a handler
            let handler = pageerror_handler.read().await;
            if let Some(ref h) = *handler {
                h(error).await;
            } else {
                warn!("Page error (no handler): {}", error.message());
            }
        }
    }
}

async fn handle_frame_attached_event(
    params: Option<&serde_json::Value>,
    connection: &Arc<CdpConnection>,
    session_id: &str,
    frameattached_handler: &Arc<RwLock<Option<FrameAttachedHandler>>>,
) {
    if let Some(params) = params {
        if let Ok(attached_event) = serde_json::from_value::<FrameAttachedEvent>(params.clone()) {
            debug!(
                frame_id = %attached_event.frame_id,
                parent_frame_id = %attached_event.parent_frame_id,
                "Frame attached"
            );

            // Create a minimal Frame object for the newly attached frame
            let frame = Frame::new(
                connection.clone(),
                session_id.to_string(),
                attached_event.frame_id.clone(),
                Some(attached_event.parent_frame_id.clone()),
                String::new(),
                String::new(),
                String::new(),
            );

            // Call the handler
            let handler = frameattached_handler.read().await;
            if let Some(ref h) = *handler {
                h(frame).await;
            }
        }
    }
}

async fn handle_frame_navigated_event(
    params: Option<&serde_json::Value>,
    connection: &Arc<CdpConnection>,
    session_id: &str,
    framenavigated_handler: &Arc<RwLock<Option<FrameNavigatedHandler>>>,
) {
    if let Some(params) = params {
        if let Ok(navigated_event) = serde_json::from_value::<FrameNavigatedEvent>(params.clone()) {
            let cdp_frame = &navigated_event.frame;
            debug!(frame_id = %cdp_frame.id, url = %cdp_frame.url, "Frame navigated");

            // Create a full Frame object with all info
            let frame = Frame::new(
                connection.clone(),
                session_id.to_string(),
                cdp_frame.id.clone(),
                cdp_frame.parent_id.clone(),
                cdp_frame.loader_id.clone(),
                cdp_frame.url.clone(),
                cdp_frame.name.clone().unwrap_or_default(),
            );

            // Call the handler
            let handler = framenavigated_handler.read().await;
            if let Some(ref h) = *handler {
                h(frame).await;
            }
        }
    }
}

async fn handle_frame_detached_event(
    params: Option<&serde_json::Value>,
    connection: &Arc<CdpConnection>,
    session_id: &str,
    framedetached_handler: &Arc<RwLock<Option<FrameDetachedHandler>>>,
) {
    if let Some(params) = params {
        if let Ok(detached_event) = serde_json::from_value::<FrameDetachedEvent>(params.clone()) {
            debug!(
                frame_id = %detached_event.frame_id,
                reason = ?detached_event.reason,
                "Frame detached"
            );

            // Create a minimal Frame object for the detached frame
            let frame = Frame::new(
                connection.clone(),
                session_id.to_string(),
                detached_event.frame_id.clone(),
                None,
                String::new(),
                String::new(),
                String::new(),
            );
            // Mark as detached
            frame.set_detached();

            // Call the handler
            let handler = framedetached_handler.read().await;
            if let Some(ref h) = *handler {
                h(frame).await;
            }
        }
    }
}

async fn handle_dialog_event(
    params: Option<&serde_json::Value>,
    connection: &Arc<CdpConnection>,
    session_id: &str,
    dialog_handler: &Arc<RwLock<Option<DialogHandler>>>,
    wait_for_dialog_tx: &Arc<Mutex<Option<oneshot::Sender<Dialog>>>>,
) {
    if let Some(params) = params {
        if let Ok(dialog_event) =
            serde_json::from_value::<JavascriptDialogOpeningEvent>(params.clone())
        {
            debug!(
                dialog_type = ?dialog_event.dialog_type,
                message = %dialog_event.message,
                "JavaScript dialog opening"
            );

            // Create a Dialog object
            let dialog = Dialog::new(
                connection.clone(),
                session_id.to_string(),
                dialog_event.dialog_type,
                dialog_event.message,
                dialog_event.default_prompt,
            );

            // Check if there's a waiter
            {
                let mut waiter = wait_for_dialog_tx.lock().await;
                if let Some(tx) = waiter.take() {
                    let _ = tx.send(dialog);
                    return;
                }
            }

            // Check if there's a handler
            let handler = dialog_handler.read().await;
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
    }
}

async fn handle_download_will_begin(
    params: Option<&serde_json::Value>,
    download_handler: &Arc<RwLock<Option<DownloadHandler>>>,
    downloads: &Arc<Mutex<HashMap<String, DownloadTracker>>>,
    wait_for_download_tx: &Arc<Mutex<Option<oneshot::Sender<Download>>>>,
) {
    if let Some(params) = params {
        if let Ok(download_event) = serde_json::from_value::<DownloadWillBeginEvent>(params.clone())
        {
            debug!(
                guid = %download_event.guid,
                filename = %download_event.suggested_filename,
                url = %download_event.url,
                "Download will begin"
            );

            let (state_tx, state_rx) = watch::channel(DownloadState::InProgress);
            let (path_tx, path_rx) = watch::channel(None);

            // Create Download with correct parameter order: guid, url, suggested_filename
            let download = Download::new(
                download_event.guid.clone(),
                download_event.url,
                download_event.suggested_filename,
                state_rx,
                path_rx,
            );

            // Store the tracker
            {
                let mut downloads_guard = downloads.lock().await;
                downloads_guard.insert(download_event.guid, DownloadTracker { state_tx, path_tx });
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
            } else {
                debug!("Download started (no handler registered)");
            }
        }
    }
}

async fn handle_download_progress(
    params: Option<&serde_json::Value>,
    downloads: &Arc<Mutex<HashMap<String, DownloadTracker>>>,
) {
    if let Some(params) = params {
        if let Ok(progress_event) = serde_json::from_value::<DownloadProgressEvent>(params.clone())
        {
            debug!(
                guid = %progress_event.guid,
                state = ?progress_event.state,
                received = progress_event.received_bytes,
                total = progress_event.total_bytes,
                "Download progress"
            );

            let downloads_guard = downloads.lock().await;
            if let Some(tracker) = downloads_guard.get(&progress_event.guid) {
                let new_state = match progress_event.state {
                    viewpoint_cdp::protocol::DownloadProgressState::Completed => {
                        DownloadState::Completed
                    }
                    viewpoint_cdp::protocol::DownloadProgressState::Canceled => {
                        DownloadState::Canceled
                    }
                    viewpoint_cdp::protocol::DownloadProgressState::InProgress => {
                        DownloadState::InProgress
                    }
                };
                let _ = tracker.state_tx.send(new_state);
            }
        }
    }
}
