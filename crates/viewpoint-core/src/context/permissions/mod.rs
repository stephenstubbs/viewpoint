//! Permission management for BrowserContext.
//!
//! This module provides methods for granting and clearing browser permissions.

use tracing::{debug, instrument};

use viewpoint_cdp::protocol::browser::{GrantPermissionsParams, ResetPermissionsParams};

use super::BrowserContext;
use super::types::Permission;
use crate::error::ContextError;

impl BrowserContext {
    /// Grant permissions to the browser context.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{BrowserContext, context::Permission};
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), viewpoint_core::CoreError> {
    /// context.grant_permissions(vec![
    ///     Permission::Geolocation,
    ///     Permission::Notifications,
    /// ]).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if granting permissions fails.
    #[instrument(level = "debug", skip(self, permissions))]
    pub async fn grant_permissions(
        &self,
        permissions: Vec<Permission>,
    ) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        debug!(count = permissions.len(), "Granting permissions");

        let cdp_permissions: Vec<_> = permissions
            .into_iter()
            .map(|p| p.to_cdp_permission())
            .collect();

        self.connection()
            .send_command::<_, serde_json::Value>(
                "Browser.grantPermissions",
                Some(
                    GrantPermissionsParams::new(cdp_permissions)
                        .browser_context_id(self.context_id()),
                ),
                None,
            )
            .await?;

        Ok(())
    }

    /// Grant permissions for a specific origin.
    ///
    /// # Errors
    ///
    /// Returns an error if granting permissions fails.
    #[instrument(level = "debug", skip(self, permissions, origin))]
    pub async fn grant_permissions_for_origin(
        &self,
        permissions: Vec<Permission>,
        origin: impl Into<String>,
    ) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        let origin = origin.into();
        debug!(count = permissions.len(), origin = %origin, "Granting permissions for origin");

        let cdp_permissions: Vec<_> = permissions
            .into_iter()
            .map(|p| p.to_cdp_permission())
            .collect();

        self.connection()
            .send_command::<_, serde_json::Value>(
                "Browser.grantPermissions",
                Some(
                    GrantPermissionsParams::new(cdp_permissions)
                        .origin(origin)
                        .browser_context_id(self.context_id()),
                ),
                None,
            )
            .await?;

        Ok(())
    }

    /// Clear all granted permissions.
    ///
    /// # Errors
    ///
    /// Returns an error if clearing permissions fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn clear_permissions(&self) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        self.connection()
            .send_command::<_, serde_json::Value>(
                "Browser.resetPermissions",
                Some(ResetPermissionsParams::new().browser_context_id(self.context_id())),
                None,
            )
            .await?;

        Ok(())
    }
}
