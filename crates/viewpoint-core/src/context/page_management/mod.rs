//! Page creation and management within a browser context.

use tracing::{debug, info, instrument};

use viewpoint_cdp::protocol::target_domain::{GetTargetsParams, GetTargetsResult};

use crate::context::page_factory;
use crate::error::ContextError;
use crate::page::Page;

use super::{BrowserContext, PageInfo};

impl BrowserContext {
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

        // Create target and attach to it
        let (create_result, attach_result) =
            page_factory::create_and_attach_target(&self.connection, &self.context_id).await?;

        let target_id = &create_result.target_id;
        let session_id = &attach_result.session_id;

        // Enable required CDP domains on the page
        page_factory::enable_page_domains(&self.connection, session_id).await?;

        // Apply emulation settings (viewport, touch, locale, etc.)
        page_factory::apply_emulation_settings(&self.connection, session_id, &self.options).await?;

        // Get the main frame ID
        let frame_id = page_factory::get_main_frame_id(&self.connection, session_id).await?;

        // Track the page
        page_factory::track_page(
            &self.pages,
            create_result.target_id.clone(),
            attach_result.session_id.clone(),
        )
        .await;

        // Apply context-level init scripts to the new page
        if let Err(e) = self.apply_init_scripts_to_session(session_id).await {
            debug!("Failed to apply init scripts: {}", e);
        }

        info!(target_id = %target_id, session_id = %session_id, frame_id = %frame_id, "Page created successfully");

        // Get the test ID attribute from context
        let test_id_attr = self.test_id_attribute.read().await.clone();

        // Convert context HTTP credentials to network auth credentials
        let http_credentials = page_factory::convert_http_credentials(&self.options);

        // Convert context proxy credentials to network auth proxy credentials
        let proxy_credentials = page_factory::convert_proxy_credentials(&self.options);

        // Create page with or without video recording
        let page = page_factory::create_page_instance(
            self.connection.clone(),
            create_result,
            attach_result,
            frame_id,
            &self.options,
            test_id_attr,
            self.route_registry.clone(),
            http_credentials,
            proxy_credentials,
        )
        .await;

        // Enable Fetch domain if there are context-level routes
        // This ensures requests are intercepted for context routes
        if let Err(e) = page.enable_fetch_for_context_routes().await {
            debug!("Failed to enable Fetch for context routes: {}", e);
        }

        // Emit page event to registered handlers
        self.event_manager.emit_page(page.clone_internal()).await;

        Ok(page)
    }

    /// Get all pages in this context.
    ///
    /// # Errors
    ///
    /// Returns an error if querying targets fails.
    pub async fn pages(&self) -> Result<Vec<PageInfo>, ContextError> {
        if self.closed {
            return Err(ContextError::Closed);
        }

        let result: GetTargetsResult = self
            .connection
            .send_command("Target.getTargets", Some(GetTargetsParams::default()), None)
            .await?;

        let pages: Vec<PageInfo> = result
            .target_infos
            .into_iter()
            .filter(|t| {
                // For the default context (empty string ID), match targets with no context ID
                // or with an empty context ID
                let matches_context = if self.context_id.is_empty() {
                    // Default context: match targets without a context ID or with empty context ID
                    t.browser_context_id.as_deref().is_none()
                        || t.browser_context_id.as_deref() == Some("")
                } else {
                    // Named context: exact match
                    t.browser_context_id.as_deref() == Some(&self.context_id)
                };
                matches_context && t.target_type == "page"
            })
            .map(|t| PageInfo {
                target_id: t.target_id,
                session_id: String::new(), // Would need to track sessions
            })
            .collect();

        Ok(pages)
    }
}
