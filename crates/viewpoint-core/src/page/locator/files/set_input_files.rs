//! Set input files from file paths.

use serde::Deserialize;
use tracing::{debug, instrument};
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_js::js;

use super::super::Locator;
use super::super::Selector;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Set files on an `<input type="file">` element.
    ///
    /// This is the recommended way to upload files. Use an empty slice to clear
    /// the file selection.
    ///
    /// # Arguments
    ///
    /// * `files` - Paths to the files to upload. Pass an empty slice to clear.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Set a single file
    /// page.locator("input[type=file]").set_input_files(&["./upload.txt"]).await?;
    ///
    /// // Set multiple files
    /// page.locator("input[type=file]").set_input_files(&["file1.txt", "file2.txt"]).await?;
    ///
    /// // Clear the file selection
    /// page.locator("input[type=file]").set_input_files::<&str>(&[]).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self, files), fields(selector = ?self.selector, file_count = files.len()))]
    pub async fn set_input_files<P: AsRef<std::path::Path>>(
        &self,
        files: &[P],
    ) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        let file_paths: Vec<String> = files
            .iter()
            .map(|p| p.as_ref().to_string_lossy().into_owned())
            .collect();

        debug!("Setting {} files on file input", file_paths.len());

        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self
                .set_input_files_by_backend_id(backend_node_id, file_paths)
                .await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self
                .set_input_files_by_backend_id(*backend_node_id, file_paths)
                .await;
        }

        // Get the element's backend node ID via JavaScript
        let selector_expr = self.selector.to_js_expression();
        let js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false, error: "Element not found" };

                const el = elements[0];
                if (el.tagName.toLowerCase() !== "input" || el.type !== "file") {
                    return { found: false, error: "Element is not a file input" };
                }

                return { found: true, isMultiple: el.multiple };
            })()
        };

        let result = self.evaluate_js(&js).await?;

        let found = result
            .get("found")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !found {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return Err(LocatorError::EvaluationError(error.to_string()));
        }

        let is_multiple = result
            .get("isMultiple")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        if !is_multiple && file_paths.len() > 1 {
            return Err(LocatorError::EvaluationError(
                "Cannot set multiple files on a single file input".to_string(),
            ));
        }

        // Use Runtime.evaluate to get the element object ID
        let get_object_js = js! {
            (function() {
                const elements = @{selector_expr};
                return elements[0];
            })()
        };

        let params = viewpoint_cdp::protocol::runtime::EvaluateParams {
            expression: get_object_js,
            object_group: Some("viewpoint-file-input".to_string()),
            include_command_line_api: None,
            silent: Some(true),
            context_id: None,
            return_by_value: Some(false),
            await_promise: Some(false),
        };

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .page
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;

        let object_id = result.result.object_id.ok_or_else(|| {
            LocatorError::EvaluationError("Failed to get element object ID".to_string())
        })?;

        // Set the files using DOM.setFileInputFiles
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "DOM.setFileInputFiles",
                Some(viewpoint_cdp::protocol::dom::SetFileInputFilesParams {
                    files: file_paths,
                    node_id: None,
                    backend_node_id: None,
                    object_id: Some(object_id),
                }),
                Some(self.page.session_id()),
            )
            .await?;

        Ok(())
    }

    /// Set files on a file input element by backend node ID.
    pub(super) async fn set_input_files_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
        file_paths: Vec<String>,
    ) -> Result<(), LocatorError> {
        // First verify it's a file input and check if multiple is allowed
        let result: ResolveNodeResult = self
            .page
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-file-input".to_string()),
                    execution_context_id: None,
                }),
                Some(self.page.session_id()),
            )
            .await
            .map_err(|_| {
                LocatorError::NotFound(format!(
                    "Could not resolve backend node ID {backend_node_id}: element may no longer exist"
                ))
            })?;

        let object_id = result.object.object_id.ok_or_else(|| {
            LocatorError::NotFound(format!(
                "No object ID for backend node ID {backend_node_id}"
            ))
        })?;

        // Verify it's a file input
        #[derive(Debug, Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
            #[serde(rename = "exceptionDetails")]
            exception_details: Option<viewpoint_cdp::protocol::runtime::ExceptionDetails>,
        }

        let js_fn = js! {
            (function() {
                const el = this;
                if (el.tagName.toLowerCase() !== "input" || el.type !== "file") {
                    return { valid: false, error: "Element is not a file input" };
                }
                return { valid: true, isMultiple: el.multiple };
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let call_result: CallResult = self
            .page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": js_fn,
                    "returnByValue": true
                })),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = call_result.exception_details {
            let _ = self
                .page
                .connection()
                .send_command::<_, serde_json::Value>(
                    "Runtime.releaseObject",
                    Some(serde_json::json!({ "objectId": object_id })),
                    Some(self.page.session_id()),
                )
                .await;
            return Err(LocatorError::EvaluationError(exception.text));
        }

        let value = call_result.result.value.ok_or_else(|| {
            LocatorError::EvaluationError("No result from file input check".to_string())
        })?;

        let valid = value
            .get("valid")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !valid {
            let error = value
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            let _ = self
                .page
                .connection()
                .send_command::<_, serde_json::Value>(
                    "Runtime.releaseObject",
                    Some(serde_json::json!({ "objectId": object_id })),
                    Some(self.page.session_id()),
                )
                .await;
            return Err(LocatorError::EvaluationError(error.to_string()));
        }

        let is_multiple = value
            .get("isMultiple")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !is_multiple && file_paths.len() > 1 {
            let _ = self
                .page
                .connection()
                .send_command::<_, serde_json::Value>(
                    "Runtime.releaseObject",
                    Some(serde_json::json!({ "objectId": object_id })),
                    Some(self.page.session_id()),
                )
                .await;
            return Err(LocatorError::EvaluationError(
                "Cannot set multiple files on a single file input".to_string(),
            ));
        }

        // Set the files using DOM.setFileInputFiles with backendNodeId
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "DOM.setFileInputFiles",
                Some(viewpoint_cdp::protocol::dom::SetFileInputFilesParams {
                    files: file_paths,
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_id: None,
                }),
                Some(self.page.session_id()),
            )
            .await?;

        // Release the object
        let _ = self
            .page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Runtime.releaseObject",
                Some(serde_json::json!({ "objectId": object_id })),
                Some(self.page.session_id()),
            )
            .await;

        Ok(())
    }
}
