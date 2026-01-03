//! Page information methods.
//!
//! This module contains methods for retrieving page properties like URL and title.

use viewpoint_js::js;

use super::Page;
use crate::error::PageError;

impl Page {
    /// Get the current page URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the evaluation fails.
    pub async fn url(&self) -> Result<String, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js! { window.location.href }.to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get URL".to_string()))
    }

    /// Get the current page title.
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the evaluation fails.
    pub async fn title(&self) -> Result<String, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: "document.title".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get title".to_string()))
    }
}
