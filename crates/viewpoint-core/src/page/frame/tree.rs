//! Frame tree traversal operations.

use std::sync::Arc;

use tracing::{debug, instrument};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::runtime::ExecutionContextId;

use super::Frame;
use crate::error::PageError;

impl Frame {
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

    /// Get or create an isolated world execution context for this frame.
    ///
    /// Isolated worlds are separate JavaScript execution contexts that do not
    /// share global scope with the main world or other isolated worlds.
    /// They are useful for injecting scripts that should not interfere with
    /// page scripts.
    ///
    /// The world name is used to identify the isolated world. If an isolated
    /// world with the same name already exists for this frame, its context ID
    /// is returned. Otherwise, a new isolated world is created.
    ///
    /// # Arguments
    ///
    /// * `world_name` - A name for the isolated world (e.g., "viewpoint-isolated")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The frame is detached
    /// - Creating the isolated world fails
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id, world_name = %world_name))]
    pub(crate) async fn get_or_create_isolated_world(
        &self,
        world_name: &str,
    ) -> Result<ExecutionContextId, PageError> {
        if self.is_detached() {
            return Err(PageError::EvaluationFailed("Frame is detached".to_string()));
        }

        // Check if we already have this isolated world cached
        {
            let data = self.data.read();
            if let Some(&context_id) = data.execution_contexts.get(world_name) {
                debug!(context_id = context_id, "Using cached isolated world context");
                return Ok(context_id);
            }
        }

        // Create a new isolated world
        debug!("Creating new isolated world");
        let result: viewpoint_cdp::protocol::page::CreateIsolatedWorldResult = self
            .connection
            .send_command(
                "Page.createIsolatedWorld",
                Some(viewpoint_cdp::protocol::page::CreateIsolatedWorldParams {
                    frame_id: self.id.clone(),
                    world_name: Some(world_name.to_string()),
                    grant_univeral_access: Some(true),
                }),
                Some(&self.session_id),
            )
            .await?;

        let context_id = result.execution_context_id;
        debug!(context_id = context_id, "Created isolated world");

        // Cache the context ID
        self.set_execution_context(world_name.to_string(), context_id);

        Ok(context_id)
    }
}

/// Recursively find child frames of a given frame ID.
pub(super) fn find_child_frames(
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
pub(super) fn find_parent_frame(
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
