//! Emulation methods for BrowserContext.
//!
//! This module provides methods for geolocation, offline mode, and HTTP headers.

use std::collections::HashMap;

use tracing::{debug, instrument};

use viewpoint_cdp::protocol::emulation::{
    ClearGeolocationOverrideParams, SetGeolocationOverrideParams,
};
use viewpoint_cdp::protocol::network::{EmulateNetworkConditionsParams, SetExtraHTTPHeadersParams};

use super::BrowserContext;
use crate::error::ContextError;

impl BrowserContext {
    /// Clear the geolocation override.
    ///
    /// # Errors
    ///
    /// Returns an error if clearing geolocation fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn clear_geolocation(&self) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        // Clear on all pages
        let pages = self.pages.read().await;
        for page in pages.iter() {
            if !page.session_id().is_empty() {
                self.connection()
                    .send_command::<_, serde_json::Value>(
                        "Emulation.clearGeolocationOverride",
                        Some(ClearGeolocationOverrideParams::default()),
                        Some(page.session_id()),
                    )
                    .await?;
            }
        }

        Ok(())
    }

    /// Set extra HTTP headers to be sent with every request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::BrowserContext;
    /// use std::collections::HashMap;
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), viewpoint_core::CoreError> {
    /// let mut headers = HashMap::new();
    /// headers.insert("Authorization".to_string(), "Bearer token123".to_string());
    /// context.set_extra_http_headers(headers).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if setting headers fails.
    #[instrument(level = "debug", skip(self, headers))]
    pub async fn set_extra_http_headers(
        &self,
        headers: HashMap<String, String>,
    ) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        debug!(count = headers.len(), "Setting extra HTTP headers");

        // Set on all pages
        let pages = self.pages.read().await;
        for page in pages.iter() {
            if !page.session_id().is_empty() {
                self.connection()
                    .send_command::<_, serde_json::Value>(
                        "Network.setExtraHTTPHeaders",
                        Some(SetExtraHTTPHeadersParams {
                            headers: headers.clone(),
                        }),
                        Some(page.session_id()),
                    )
                    .await?;
            }
        }

        Ok(())
    }

    /// Set offline mode.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::BrowserContext;
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), viewpoint_core::CoreError> {
    /// // Go offline
    /// context.set_offline(true).await?;
    ///
    /// // Go back online
    /// context.set_offline(false).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if setting offline mode fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn set_offline(&self, offline: bool) -> Result<(), ContextError> {
        if self.is_closed() {
            return Err(ContextError::Closed);
        }

        debug!(offline = offline, "Setting offline mode");

        let params = if offline {
            EmulateNetworkConditionsParams::offline()
        } else {
            EmulateNetworkConditionsParams::online()
        };

        // Set on all pages
        let pages = self.pages.read().await;
        for page in pages.iter() {
            if !page.session_id().is_empty() {
                self.connection()
                    .send_command::<_, serde_json::Value>(
                        "Network.emulateNetworkConditions",
                        Some(params.clone()),
                        Some(page.session_id()),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}

// =============================================================================
// Set Geolocation Builder
// =============================================================================

/// Builder for setting geolocation.
#[derive(Debug)]
pub struct SetGeolocationBuilder<'a> {
    context: &'a BrowserContext,
    latitude: f64,
    longitude: f64,
    accuracy: f64,
}

impl<'a> SetGeolocationBuilder<'a> {
    pub(crate) fn new(context: &'a BrowserContext, latitude: f64, longitude: f64) -> Self {
        Self {
            context,
            latitude,
            longitude,
            accuracy: 0.0,
        }
    }

    /// Set the accuracy in meters.
    #[must_use]
    pub fn accuracy(mut self, accuracy: f64) -> Self {
        self.accuracy = accuracy;
        self
    }

    /// Execute the geolocation setting.
    ///
    /// # Errors
    ///
    /// Returns an error if setting geolocation fails.
    pub async fn await_(self) -> Result<(), ContextError> {
        if self.context.is_closed() {
            return Err(ContextError::Closed);
        }

        debug!(
            latitude = self.latitude,
            longitude = self.longitude,
            accuracy = self.accuracy,
            "Setting geolocation"
        );

        let params = SetGeolocationOverrideParams::with_accuracy(
            self.latitude,
            self.longitude,
            self.accuracy,
        );

        // Set on all pages
        let pages = self.context.pages.read().await;
        for page in pages.iter() {
            if !page.session_id().is_empty() {
                self.context
                    .connection()
                    .send_command::<_, serde_json::Value>(
                        "Emulation.setGeolocationOverride",
                        Some(params.clone()),
                        Some(page.session_id()),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}

// Make the builder awaitable
impl<'a> std::future::IntoFuture for SetGeolocationBuilder<'a> {
    type Output = Result<(), ContextError>;
    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.await_())
    }
}
