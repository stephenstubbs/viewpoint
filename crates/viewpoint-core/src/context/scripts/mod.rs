//! Context-level init script management.

use tracing::{debug, instrument};

use crate::error::ContextError;

use super::BrowserContext;

impl BrowserContext {
    /// Add a script to be evaluated before every new page load.
    ///
    /// The script will run before any scripts on the page, and will persist
    /// for all pages created in this context (including popups).
    ///
    /// Unlike `page.add_init_script()`, this applies to all pages in the context,
    /// not just a single page.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// // Mock navigator.webdriver for all pages
    /// context.add_init_script(
    ///     "Object.defineProperty(navigator, 'webdriver', { get: () => false })"
    /// ).await?;
    ///
    /// // All new pages will have this script applied
    /// let page = context.new_page().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
    #[instrument(level = "debug", skip(self, script), fields(script_len = script.as_ref().len()))]
    pub async fn add_init_script(&self, script: impl AsRef<str>) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        let script_content = script.as_ref().to_string();
        debug!("Adding context-level init script");

        // Store the script for future pages
        {
            let mut scripts = self.init_scripts.write().await;
            scripts.push(script_content.clone());
        }

        // Apply to existing pages
        let pages = self.pages.read().await;
        for page in pages.iter() {
            if !page.session_id.is_empty() {
                use viewpoint_cdp::protocol::page::AddScriptToEvaluateOnNewDocumentParams;

                let _ = self.connection()
                    .send_command::<_, viewpoint_cdp::protocol::page::AddScriptToEvaluateOnNewDocumentResult>(
                        "Page.addScriptToEvaluateOnNewDocument",
                        Some(AddScriptToEvaluateOnNewDocumentParams {
                            source: script_content.clone(),
                            world_name: None,
                            include_command_line_api: None,
                            run_immediately: None,
                        }),
                        Some(&page.session_id),
                    )
                    .await;
            }
        }

        Ok(())
    }

    /// Add an init script from a file path.
    ///
    /// The file contents will be read and registered as an init script for all
    /// pages in this context.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// let context = browser.new_context().await?;
    ///
    /// context.add_init_script_path("./scripts/mock-auth.js").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the context is closed.
    #[instrument(level = "debug", skip(self), fields(path = %path.as_ref().display()))]
    pub async fn add_init_script_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), ContextError> {
        let content = tokio::fs::read_to_string(path.as_ref())
            .await
            .map_err(|e| ContextError::Internal(format!("Failed to read init script file: {e}")))?;

        self.add_init_script(&content).await
    }

    /// Get all context-level init scripts.
    ///
    /// This returns the scripts that will be applied to all new pages.
    pub async fn init_scripts(&self) -> Vec<String> {
        self.init_scripts.read().await.clone()
    }

    /// Apply all context-level init scripts to a page session.
    ///
    /// This is called internally when a new page is created.
    pub(crate) async fn apply_init_scripts_to_session(
        &self,
        session_id: &str,
    ) -> Result<(), ContextError> {
        let scripts = self.init_scripts.read().await;

        for script in scripts.iter() {
            use viewpoint_cdp::protocol::page::AddScriptToEvaluateOnNewDocumentParams;

            self.connection()
                .send_command::<_, viewpoint_cdp::protocol::page::AddScriptToEvaluateOnNewDocumentResult>(
                    "Page.addScriptToEvaluateOnNewDocument",
                    Some(AddScriptToEvaluateOnNewDocumentParams {
                        source: script.clone(),
                        world_name: None,
                        include_command_line_api: None,
                        run_immediately: None,
                    }),
                    Some(session_id),
                )
                .await?;
        }

        Ok(())
    }
}
