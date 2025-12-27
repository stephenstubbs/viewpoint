//! Screenshot and DOM snapshot capture for tracing.

use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::debug;
use viewpoint_cdp::CdpConnection;

use super::types::{ScreenshotEntry, TracingState};
use crate::error::ContextError;

/// Capture a screenshot and add it to the trace state.
pub(super) async fn capture_screenshot(
    connection: &Arc<CdpConnection>,
    state: &Arc<RwLock<TracingState>>,
    session_id: &str,
    name: Option<&str>,
) -> Result<(), ContextError> {
    {
        let state_read = state.read().await;
        if !state_read.options.screenshots {
            return Ok(());
        }
    }

    #[derive(serde::Deserialize)]
    struct ScreenshotResult {
        data: String,
    }

    let result: ScreenshotResult = connection
        .send_command(
            "Page.captureScreenshot",
            Some(serde_json::json!({
                "format": "png",
                "quality": 80
            })),
            Some(session_id),
        )
        .await?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000.0;

    let mut state_write = state.write().await;
    state_write.screenshots.push(ScreenshotEntry {
        data: result.data,
        timestamp,
        name: name.map(ToString::to_string),
    });

    Ok(())
}

/// Capture a DOM snapshot and add it to the trace state.
pub(super) async fn capture_dom_snapshot(
    connection: &Arc<CdpConnection>,
    state: &Arc<RwLock<TracingState>>,
    session_id: &str,
) -> Result<(), ContextError> {
    {
        let state_read = state.read().await;
        if !state_read.options.snapshots {
            return Ok(());
        }
    }

    let result: serde_json::Value = connection
        .send_command(
            "DOMSnapshot.captureSnapshot",
            Some(serde_json::json!({
                "computedStyles": ["display", "visibility", "opacity"],
                "includeEventListeners": false,
                "includePaintOrder": false,
                "includeUserAgentShadowTree": false
            })),
            Some(session_id),
        )
        .await?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000.0;

    // Wrap the result with timestamp
    let snapshot = serde_json::json!({
        "timestamp": timestamp,
        "data": result
    });

    let mut state_write = state.write().await;
    state_write.snapshots.push(snapshot);

    Ok(())
}

/// Capture action context (screenshot + snapshot) if enabled.
pub(super) async fn capture_action_context(
    connection: &Arc<CdpConnection>,
    state: &Arc<RwLock<TracingState>>,
    session_id: &str,
    action_name: Option<&str>,
) -> Result<(), ContextError> {
    // Capture screenshot if enabled
    if let Err(e) = capture_screenshot(connection, state, session_id, action_name).await {
        debug!("Failed to capture screenshot: {}", e);
    }

    // Capture DOM snapshot if enabled
    if let Err(e) = capture_dom_snapshot(connection, state, session_id).await {
        debug!("Failed to capture DOM snapshot: {}", e);
    }

    Ok(())
}
