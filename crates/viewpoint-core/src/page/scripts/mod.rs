//! Init script methods for Page.
//!
//! This module provides methods for adding scripts that run before page loads.

use tracing::{info, instrument};

use super::Page;
use crate::error::PageError;

impl Page {
    /// Add a script to be evaluated before every page load.
    ///
    /// The script will run before any scripts on the page, and will persist
    /// across navigations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Modify the navigator before page loads
    /// page.add_init_script("Object.defineProperty(navigator, 'webdriver', { get: () => false })").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "info", skip(self, script), fields(script_len = script.as_ref().len()))]
    pub async fn add_init_script(&self, script: impl AsRef<str>) -> Result<String, PageError> {
        if self.is_closed() {
            return Err(PageError::Closed);
        }

        use viewpoint_cdp::protocol::page::AddScriptToEvaluateOnNewDocumentParams;

        let result: viewpoint_cdp::protocol::page::AddScriptToEvaluateOnNewDocumentResult = self
            .connection()
            .send_command(
                "Page.addScriptToEvaluateOnNewDocument",
                Some(AddScriptToEvaluateOnNewDocumentParams {
                    source: script.as_ref().to_string(),
                    world_name: None,
                    include_command_line_api: None,
                    run_immediately: None,
                }),
                Some(self.session_id()),
            )
            .await?;

        info!(identifier = %result.identifier, "Init script added");
        Ok(result.identifier)
    }

    /// Add an init script from a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the script cannot be added.
    #[instrument(level = "info", skip(self), fields(path = %path.as_ref().display()))]
    pub async fn add_init_script_path(&self, path: impl AsRef<std::path::Path>) -> Result<String, PageError> {
        let content = tokio::fs::read_to_string(path.as_ref()).await.map_err(|e| {
            PageError::EvaluationFailed(format!("Failed to read init script file: {e}"))
        })?;

        self.add_init_script(&content).await
    }
}
