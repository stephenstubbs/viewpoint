//! Frame management and navigation.
//!
//! Frames represent separate browsing contexts within a page, typically
//! created by `<iframe>` elements. Each frame has its own DOM, JavaScript
//! context, and URL.

// Allow dead code for frame scaffolding (spec: frames)

use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;
use tracing::{debug, info, instrument};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::page::{NavigateParams, NavigateResult};
use viewpoint_cdp::protocol::runtime::EvaluateParams;

use crate::error::{NavigationError, PageError};
use crate::wait::{DocumentLoadState, LoadStateWaiter};

/// Default navigation timeout.
const DEFAULT_NAVIGATION_TIMEOUT: Duration = Duration::from_secs(30);

/// Internal frame data that can be updated.
#[derive(Debug, Clone)]
struct FrameData {
    /// Frame's current URL.
    url: String,
    /// Frame's name attribute.
    name: String,
    /// Whether the frame is detached.
    detached: bool,
}

/// A frame within a page.
///
/// Frames are separate browsing contexts, typically created by `<iframe>` elements.
/// Each frame has its own DOM and JavaScript execution context.
#[derive(Debug)]
pub struct Frame {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID for this frame's page.
    session_id: String,
    /// Unique frame identifier.
    id: String,
    /// Parent frame ID (None for main frame).
    parent_id: Option<String>,
    /// Loader ID for this frame.
    loader_id: String,
    /// Mutable frame data.
    data: RwLock<FrameData>,
}

impl Frame {
    /// Create a new frame from CDP frame info.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        session_id: String,
        id: String,
        parent_id: Option<String>,
        loader_id: String,
        url: String,
        name: String,
    ) -> Self {
        Self {
            connection,
            session_id,
            id,
            parent_id,
            loader_id,
            data: RwLock::new(FrameData {
                url,
                name,
                detached: false,
            }),
        }
    }

    /// Get the unique frame identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the parent frame ID.
    ///
    /// Returns `None` for the main frame.
    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    /// Check if this is the main frame.
    pub fn is_main(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Get the loader ID.
    pub fn loader_id(&self) -> &str {
        &self.loader_id
    }

    /// Get the frame's current URL.
    pub fn url(&self) -> String {
        self.data.read().url.clone()
    }

    /// Get the frame's name attribute.
    pub fn name(&self) -> String {
        self.data.read().name.clone()
    }

    /// Check if the frame has been detached.
    pub fn is_detached(&self) -> bool {
        self.data.read().detached
    }

    /// Update the frame's URL (called when frame navigates).
    pub(crate) fn set_url(&self, url: String) {
        self.data.write().url = url;
    }

    /// Update the frame's name.
    pub(crate) fn set_name(&self, name: String) {
        self.data.write().name = name;
    }

    /// Mark the frame as detached.
    pub(crate) fn set_detached(&self) {
        self.data.write().detached = true;
    }

    /// Get the frame's HTML content.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame is detached or the evaluation fails.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id))]
    pub async fn content(&self) -> Result<String, PageError> {
        if self.is_detached() {
            return Err(PageError::EvaluationFailed("Frame is detached".to_string()));
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: "document.documentElement.outerHTML".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None, // TODO: Use frame's execution context
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get content".to_string()))
    }

    /// Get the frame's document title.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame is detached or the evaluation fails.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id))]
    pub async fn title(&self) -> Result<String, PageError> {
        if self.is_detached() {
            return Err(PageError::EvaluationFailed("Frame is detached".to_string()));
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: "document.title".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None, // TODO: Use frame's execution context
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get title".to_string()))
    }

    /// Navigate the frame to a URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame is detached or navigation fails.
    #[instrument(level = "info", skip(self), fields(frame_id = %self.id, url = %url))]
    pub async fn goto(&self, url: &str) -> Result<(), NavigationError> {
        self.goto_with_options(url, DocumentLoadState::Load, DEFAULT_NAVIGATION_TIMEOUT)
            .await
    }

    /// Navigate the frame to a URL with options.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame is detached or navigation fails.
    #[instrument(level = "info", skip(self), fields(frame_id = %self.id, url = %url, wait_until = ?wait_until))]
    pub async fn goto_with_options(
        &self,
        url: &str,
        wait_until: DocumentLoadState,
        timeout: Duration,
    ) -> Result<(), NavigationError> {
        if self.is_detached() {
            return Err(NavigationError::Cancelled);
        }

        info!("Navigating frame to URL");

        // Create a load state waiter
        let event_rx = self.connection.subscribe_events();
        let mut waiter = LoadStateWaiter::new(event_rx, self.session_id.clone(), self.id.clone());

        // Send navigation command with frame_id
        debug!("Sending Page.navigate command for frame");
        let result: NavigateResult = self
            .connection
            .send_command(
                "Page.navigate",
                Some(NavigateParams {
                    url: url.to_string(),
                    referrer: None,
                    transition_type: None,
                    frame_id: Some(self.id.clone()),
                }),
                Some(&self.session_id),
            )
            .await?;

        debug!(frame_id = %result.frame_id, "Page.navigate completed for frame");

        // Check for navigation errors
        if let Some(error_text) = result.error_text {
            return Err(NavigationError::NetworkError(error_text));
        }

        // Mark commit as received
        waiter.set_commit_received().await;

        // Wait for the target load state
        debug!(wait_until = ?wait_until, "Waiting for load state");
        waiter
            .wait_for_load_state_with_timeout(wait_until, timeout)
            .await?;

        // Update the frame's URL
        self.set_url(url.to_string());

        info!(frame_id = %self.id, "Frame navigation completed");
        Ok(())
    }

    /// Set the frame's HTML content.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame is detached or setting content fails.
    #[instrument(level = "info", skip(self, html), fields(frame_id = %self.id))]
    pub async fn set_content(&self, html: &str) -> Result<(), PageError> {
        if self.is_detached() {
            return Err(PageError::EvaluationFailed("Frame is detached".to_string()));
        }

        use viewpoint_cdp::protocol::page::SetDocumentContentParams;

        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.setDocumentContent",
                Some(SetDocumentContentParams {
                    frame_id: self.id.clone(),
                    html: html.to_string(),
                }),
                Some(&self.session_id),
            )
            .await?;

        info!("Frame content set");
        Ok(())
    }

    /// Wait for the frame to reach a specific load state.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out or the frame is detached.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id, state = ?state))]
    pub async fn wait_for_load_state(
        &self,
        state: DocumentLoadState,
    ) -> Result<(), NavigationError> {
        self.wait_for_load_state_with_timeout(state, DEFAULT_NAVIGATION_TIMEOUT)
            .await
    }

    /// Wait for the frame to reach a specific load state with timeout.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out or the frame is detached.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id, state = ?state, timeout_ms = timeout.as_millis()))]
    pub async fn wait_for_load_state_with_timeout(
        &self,
        state: DocumentLoadState,
        timeout: Duration,
    ) -> Result<(), NavigationError> {
        if self.is_detached() {
            return Err(NavigationError::Cancelled);
        }

        let event_rx = self.connection.subscribe_events();
        let mut waiter = LoadStateWaiter::new(event_rx, self.session_id.clone(), self.id.clone());

        // Assume commit already happened for existing frames
        waiter.set_commit_received().await;

        waiter
            .wait_for_load_state_with_timeout(state, timeout)
            .await?;

        debug!("Frame reached load state {:?}", state);
        Ok(())
    }

    /// Get the session ID.
    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the connection.
    pub(crate) fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }

    /// Get child frames of this frame.
    ///
    /// Returns a list of frames that are direct children of this frame.
    ///
    /// # Errors
    ///
    /// Returns an error if querying the frame tree fails.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id))]
    pub async fn child_frames(&self) -> Result<Vec<Frame>, PageError> {
        if self.is_detached() {
            return Err(PageError::EvaluationFailed("Frame is detached".to_string()));
        }

        // Get the frame tree from CDP
        let result: viewpoint_cdp::protocol::page::GetFrameTreeResult = self
            .connection
            .send_command("Page.getFrameTree", None::<()>, Some(&self.session_id))
            .await?;

        // Find this frame in the tree and return its children
        let children = find_child_frames(
            &result.frame_tree,
            &self.id,
            &self.connection,
            &self.session_id,
        );

        Ok(children)
    }

    /// Get the parent frame.
    ///
    /// Returns `None` if this is the main frame.
    ///
    /// # Errors
    ///
    /// Returns an error if querying the frame tree fails.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id))]
    pub async fn parent_frame(&self) -> Result<Option<Frame>, PageError> {
        if self.is_detached() {
            return Err(PageError::EvaluationFailed("Frame is detached".to_string()));
        }

        // Main frame has no parent
        if self.is_main() {
            return Ok(None);
        }

        // Get the frame tree from CDP
        let result: viewpoint_cdp::protocol::page::GetFrameTreeResult = self
            .connection
            .send_command("Page.getFrameTree", None::<()>, Some(&self.session_id))
            .await?;

        // Find the parent frame
        let parent = find_parent_frame(
            &result.frame_tree,
            &self.id,
            &self.connection,
            &self.session_id,
        );

        Ok(parent)
    }
}

/// Recursively find child frames of a given frame ID.
fn find_child_frames(
    tree: &viewpoint_cdp::protocol::page::FrameTree,
    parent_id: &str,
    connection: &Arc<CdpConnection>,
    session_id: &str,
) -> Vec<Frame> {
    let mut children = Vec::new();

    // Check if this is the parent we're looking for
    if tree.frame.id == parent_id {
        // Return all direct children
        if let Some(ref child_frames) = tree.child_frames {
            for child in child_frames {
                children.push(Frame::new(
                    connection.clone(),
                    session_id.to_string(),
                    child.frame.id.clone(),
                    Some(parent_id.to_string()),
                    child.frame.loader_id.clone(),
                    child.frame.url.clone(),
                    child.frame.name.clone().unwrap_or_default(),
                ));
            }
        }
    } else {
        // Recurse into children to find the parent
        if let Some(ref child_frames) = tree.child_frames {
            for child in child_frames {
                let found = find_child_frames(child, parent_id, connection, session_id);
                children.extend(found);
            }
        }
    }

    children
}

/// Recursively find the parent frame of a given frame ID.
fn find_parent_frame(
    tree: &viewpoint_cdp::protocol::page::FrameTree,
    frame_id: &str,
    connection: &Arc<CdpConnection>,
    session_id: &str,
) -> Option<Frame> {
    // Check if any direct child is the frame we're looking for
    if let Some(ref child_frames) = tree.child_frames {
        for child in child_frames {
            if child.frame.id == frame_id {
                // Found it - return the current frame as the parent
                return Some(Frame::new(
                    connection.clone(),
                    session_id.to_string(),
                    tree.frame.id.clone(),
                    tree.frame.parent_id.clone(),
                    tree.frame.loader_id.clone(),
                    tree.frame.url.clone(),
                    tree.frame.name.clone().unwrap_or_default(),
                ));
            }
        }

        // Recurse into children
        for child in child_frames {
            if let Some(parent) = find_parent_frame(child, frame_id, connection, session_id) {
                return Some(parent);
            }
        }
    }

    None
}
