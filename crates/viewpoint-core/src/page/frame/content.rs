//! Frame content operations (get/set content, title).

use tracing::{info, instrument, trace};
use viewpoint_cdp::protocol::runtime::EvaluateParams;

use super::Frame;
use crate::error::PageError;

impl Frame {
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

        // Get the execution context ID for this frame's main world
        let context_id = self.main_world_context_id();
        trace!(context_id = ?context_id, "Using execution context for content()");

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: "document.documentElement.outerHTML".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id,
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

        // Get the execution context ID for this frame's main world
        let context_id = self.main_world_context_id();
        trace!(context_id = ?context_id, "Using execution context for title()");

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: "document.title".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id,
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
}
