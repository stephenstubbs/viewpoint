//! Browser context management.

use std::sync::Arc;

use rustright_cdp::protocol::target::{
    AttachToTargetParams, AttachToTargetResult, CreateTargetParams, CreateTargetResult,
    DisposeBrowserContextParams,
};
use rustright_cdp::CdpConnection;
use tracing::{debug, info, instrument, trace};

use crate::error::ContextError;
use crate::page::Page;

/// An isolated browser context.
///
/// Browser contexts are similar to incognito windows - they have their own
/// cookies, cache, and storage that are isolated from other contexts.
#[derive(Debug)]
pub struct BrowserContext {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Browser context ID.
    context_id: String,
    /// Whether the context has been closed.
    closed: bool,
}

impl BrowserContext {
    /// Create a new browser context.
    pub(crate) fn new(connection: Arc<CdpConnection>, context_id: String) -> Self {
        debug!(context_id = %context_id, "Created BrowserContext");
        Self {
            connection,
            context_id,
            closed: false,
        }
    }

    /// Create a new page in this context.
    ///
    /// # Errors
    ///
    /// Returns an error if page creation fails.
    #[instrument(level = "info", skip(self), fields(context_id = %self.context_id))]
    pub async fn new_page(&self) -> Result<Page, ContextError> {
        if self.closed {
            return Err(ContextError::Closed);
        }

        info!("Creating new page");

        // Create a new target (page)
        debug!("Creating target via Target.createTarget");
        let create_result: CreateTargetResult = self
            .connection
            .send_command(
                "Target.createTarget",
                Some(CreateTargetParams {
                    url: "about:blank".to_string(),
                    width: None,
                    height: None,
                    browser_context_id: Some(self.context_id.clone()),
                    background: None,
                    new_window: None,
                }),
                None,
            )
            .await?;
        
        let target_id = &create_result.target_id;
        debug!(target_id = %target_id, "Target created");

        // Attach to the target to get a session
        debug!(target_id = %target_id, "Attaching to target");
        let attach_result: AttachToTargetResult = self
            .connection
            .send_command(
                "Target.attachToTarget",
                Some(AttachToTargetParams {
                    target_id: target_id.clone(),
                    flatten: Some(true),
                }),
                None,
            )
            .await?;

        let session_id = &attach_result.session_id;
        debug!(session_id = %session_id, "Attached to target");

        // Enable required domains on the page
        trace!("Enabling Page domain");
        self.connection
            .send_command::<(), serde_json::Value>("Page.enable", None, Some(session_id))
            .await?;

        trace!("Enabling Network domain");
        self.connection
            .send_command::<(), serde_json::Value>("Network.enable", None, Some(session_id))
            .await?;

        trace!("Enabling Runtime domain");
        self.connection
            .send_command::<(), serde_json::Value>("Runtime.enable", None, Some(session_id))
            .await?;

        trace!("Enabling lifecycle events");
        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.setLifecycleEventsEnabled",
                Some(rustright_cdp::protocol::page::SetLifecycleEventsEnabledParams {
                    enabled: true,
                }),
                Some(session_id),
            )
            .await?;

        // Get the main frame ID
        trace!("Getting frame tree");
        let frame_tree: rustright_cdp::protocol::page::GetFrameTreeResult = self
            .connection
            .send_command("Page.getFrameTree", None::<()>, Some(session_id))
            .await?;

        let frame_id = frame_tree.frame_tree.frame.id.clone();
        debug!(frame_id = %frame_id, "Got main frame ID");

        info!(target_id = %target_id, session_id = %session_id, frame_id = %frame_id, "Page created successfully");

        Ok(Page::new(
            self.connection.clone(),
            create_result.target_id,
            attach_result.session_id,
            frame_id,
        ))
    }

    /// Close this browser context and all its pages.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    #[instrument(level = "info", skip(self), fields(context_id = %self.context_id))]
    pub async fn close(&mut self) -> Result<(), ContextError> {
        if self.closed {
            debug!("Context already closed");
            return Ok(());
        }

        info!("Closing browser context");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Target.disposeBrowserContext",
                Some(DisposeBrowserContextParams {
                    browser_context_id: self.context_id.clone(),
                }),
                None,
            )
            .await?;

        self.closed = true;
        info!("Browser context closed");
        Ok(())
    }

    /// Get the context ID.
    pub fn id(&self) -> &str {
        &self.context_id
    }

    /// Check if this context has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get a reference to the CDP connection.
    pub fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }
}
