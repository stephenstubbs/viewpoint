//! File handling methods for locators.
//!
//! Methods for setting files on `<input type="file">` elements.

use tracing::{debug, instrument};

use super::Locator;
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

        // Get the element's backend node ID via JavaScript
        let js = format!(
            r"(function() {{
                const elements = {selector};
                if (elements.length === 0) return {{ found: false, error: 'Element not found' }};
                
                const el = elements[0];
                if (el.tagName.toLowerCase() !== 'input' || el.type !== 'file') {{
                    return {{ found: false, error: 'Element is not a file input' }};
                }}
                
                return {{ found: true, isMultiple: el.multiple }};
            }})()",
            selector = self.selector.to_js_expression()
        );

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
        let get_object_js = format!(
            r"(function() {{
                const elements = {selector};
                return elements[0];
            }})()",
            selector = self.selector.to_js_expression()
        );

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

    /// Set files on a file input element from memory buffers.
    ///
    /// This is useful when you want to upload files without having them on disk,
    /// such as dynamically generated content or test data.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Page, FilePayload};
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Upload a text file from memory
    /// let payload = FilePayload::new("test.txt", "text/plain", b"Hello, World!".to_vec());
    /// page.locator("input[type=file]").set_input_files_from_buffer(&[payload]).await?;
    ///
    /// // Upload multiple files
    /// let file1 = FilePayload::from_text("doc1.txt", "Content 1");
    /// let file2 = FilePayload::new("data.json", "application/json", r#"{"key": "value"}"#.as_bytes().to_vec());
    /// page.locator("input[type=file]").set_input_files_from_buffer(&[file1, file2]).await?;
    ///
    /// // Clear files
    /// page.locator("input[type=file]").set_input_files_from_buffer(&[]).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self, files), fields(selector = ?self.selector, file_count = files.len()))]
    pub async fn set_input_files_from_buffer(
        &self,
        files: &[crate::page::FilePayload],
    ) -> Result<(), LocatorError> {
        use base64::{Engine, engine::general_purpose::STANDARD};

        self.wait_for_actionable().await?;

        debug!("Setting {} files from buffer on file input", files.len());

        // Get the element's info via JavaScript
        let js = format!(
            r"(function() {{
                const elements = {selector};
                if (elements.length === 0) return {{ found: false, error: 'Element not found' }};
                
                const el = elements[0];
                if (el.tagName.toLowerCase() !== 'input' || el.type !== 'file') {{
                    return {{ found: false, error: 'Element is not a file input' }};
                }}
                
                return {{ found: true, isMultiple: el.multiple }};
            }})()",
            selector = self.selector.to_js_expression()
        );

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

        if !is_multiple && files.len() > 1 {
            return Err(LocatorError::EvaluationError(
                "Cannot set multiple files on a single file input".to_string(),
            ));
        }

        // Build the file data array for JavaScript
        let file_data: Vec<serde_json::Value> = files
            .iter()
            .map(|f| {
                serde_json::json!({
                    "name": f.name,
                    "mimeType": f.mime_type,
                    "data": STANDARD.encode(&f.buffer),
                })
            })
            .collect();

        let file_data_json = serde_json::to_string(&file_data)
            .map_err(|e| LocatorError::EvaluationError(e.to_string()))?;

        // Use JavaScript to create File objects and set them on the input
        let set_files_js = format!(
            r"(async function() {{
                const elements = {selector};
                if (elements.length === 0) return {{ success: false, error: 'Element not found' }};
                
                const input = elements[0];
                const fileData = {file_data};
                
                // Create File objects from the data
                const files = await Promise.all(fileData.map(async (fd) => {{
                    // Decode base64 to binary
                    const binaryString = atob(fd.data);
                    const bytes = new Uint8Array(binaryString.length);
                    for (let i = 0; i < binaryString.length; i++) {{
                        bytes[i] = binaryString.charCodeAt(i);
                    }}
                    
                    return new File([bytes], fd.name, {{ type: fd.mimeType }});
                }}));
                
                // Create a DataTransfer to hold the files
                const dataTransfer = new DataTransfer();
                for (const file of files) {{
                    dataTransfer.items.add(file);
                }}
                
                // Set the files on the input
                input.files = dataTransfer.files;
                
                // Dispatch change event
                input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                
                return {{ success: true }};
            }})()",
            selector = self.selector.to_js_expression(),
            file_data = file_data_json
        );

        let params = viewpoint_cdp::protocol::runtime::EvaluateParams {
            expression: set_files_js,
            object_group: Some("viewpoint-file-input".to_string()),
            include_command_line_api: None,
            silent: Some(false),
            context_id: None,
            return_by_value: Some(true),
            await_promise: Some(true),
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

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(format!(
                "Failed to set files: {}",
                exception.text
            )));
        }

        if let Some(value) = result.result.value {
            let success = value
                .get("success")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            if !success {
                let error = value
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");
                return Err(LocatorError::EvaluationError(error.to_string()));
            }
        }

        Ok(())
    }
}
