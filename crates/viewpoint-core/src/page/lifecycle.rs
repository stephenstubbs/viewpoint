//! Page lifecycle management.
//!
//! This module contains methods for managing the page lifecycle (close, is_closed).

use tracing::{debug, info, instrument};
use viewpoint_cdp::protocol::target_domain::CloseTargetParams;

use super::Page;
use crate::error::PageError;

impl Page {
    /// Close this page.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    #[instrument(level = "info", skip(self), fields(target_id = %self.target_id))]
    pub async fn close(&mut self) -> Result<(), PageError> {
        if self.closed {
            debug!("Page already closed");
            return Ok(());
        }

        info!("Closing page");

        // Clean up route handlers
        self.route_registry.unroute_all().await;
        debug!("Route handlers cleaned up");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Target.closeTarget",
                Some(CloseTargetParams {
                    target_id: self.target_id.clone(),
                }),
                None,
            )
            .await?;

        // Remove this page from the context's pages list to prevent stale session accumulation
        if let Some(ref pages) = self.context_pages {
            let mut pages_guard = pages.write().await;
            pages_guard.retain(|p| p.target_id != self.target_id);
            debug!("Removed page from context's pages list");
        }

        self.closed = true;
        info!("Page closed");
        Ok(())
    }

    /// Check if this page has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
