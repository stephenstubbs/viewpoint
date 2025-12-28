//! File chooser handling for file upload dialogs.
//!
//! This module provides functionality for handling file chooser dialogs.

// Allow dead code for file chooser scaffolding (spec: file-uploads)

use std::path::Path;
use std::sync::Arc;

use viewpoint_cdp::protocol::dom::SetFileInputFilesParams;
use viewpoint_cdp::CdpConnection;
use tracing::{debug, instrument};

use crate::error::LocatorError;

/// A file chooser dialog.
///
/// File choosers are emitted via the `page.on_filechooser()` callback or can be
/// obtained using `page.wait_for_file_chooser()`.
///
/// Note: You must call `page.set_intercept_file_chooser(true)` before the
/// file chooser dialog is opened.
///
/// # Example
///
/// ```ignore
/// // Enable file chooser interception
/// page.set_intercept_file_chooser(true).await?;
///
/// let file_chooser = page.wait_for_file_chooser(async {
///     page.locator("input[type=file]").click().await?;
///     Ok(())
/// }).await?;
///
/// // Set the files
/// file_chooser.set_files(&["./upload.txt"]).await?;
/// ```
#[derive(Debug)]
pub struct FileChooser {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Frame ID containing the file input.
    frame_id: String,
    /// Backend node ID of the file input element.
    backend_node_id: Option<i32>,
    /// Whether the file input accepts multiple files.
    is_multiple: bool,
}

impl FileChooser {
    /// Create a new `FileChooser`.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        session_id: String,
        frame_id: String,
        backend_node_id: Option<i32>,
        is_multiple: bool,
    ) -> Self {
        Self {
            connection,
            session_id,
            frame_id,
            backend_node_id,
            is_multiple,
        }
    }

    /// Check if this file chooser accepts multiple files.
    pub fn is_multiple(&self) -> bool {
        self.is_multiple
    }

    /// Set the files to upload.
    ///
    /// # Arguments
    ///
    /// * `files` - Paths to the files to upload
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Multiple files are provided but `is_multiple()` is false
    /// - The CDP command fails
    #[instrument(level = "debug", skip(self, files), fields(file_count = files.len()))]
    pub async fn set_files<P: AsRef<Path>>(&self, files: &[P]) -> Result<(), LocatorError> {
        if !self.is_multiple && files.len() > 1 {
            return Err(LocatorError::EvaluationError(
                "Cannot set multiple files on a single file input".to_string(),
            ));
        }

        let file_paths: Vec<String> = files
            .iter()
            .map(|p| p.as_ref().to_string_lossy().into_owned())
            .collect();

        debug!("Setting {} files on file input", file_paths.len());

        self.connection
            .send_command::<_, serde_json::Value>(
                "DOM.setFileInputFiles",
                Some(SetFileInputFilesParams {
                    files: file_paths,
                    node_id: None,
                    backend_node_id: self.backend_node_id,
                    object_id: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }
}

/// Represents a file to upload with its content.
#[derive(Debug, Clone)]
pub struct FilePayload {
    /// File name.
    pub name: String,
    /// MIME type.
    pub mime_type: String,
    /// File content as bytes.
    pub buffer: Vec<u8>,
}

impl FilePayload {
    /// Create a new file payload.
    pub fn new(name: impl Into<String>, mime_type: impl Into<String>, buffer: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            mime_type: mime_type.into(),
            buffer,
        }
    }

    /// Create a file payload from a string content.
    pub fn from_text(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mime_type: "text/plain".to_string(),
            buffer: content.into().into_bytes(),
        }
    }

    /// Create a file payload from JSON content.
    pub fn from_json(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mime_type: "application/json".to_string(),
            buffer: content.into().into_bytes(),
        }
    }

    /// Create a file payload from HTML content.
    pub fn from_html(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mime_type: "text/html".to_string(),
            buffer: content.into().into_bytes(),
        }
    }
}
