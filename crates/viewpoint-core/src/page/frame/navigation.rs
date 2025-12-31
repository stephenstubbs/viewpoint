//! Frame navigation and load state operations.

use std::time::Duration;

use tracing::{debug, info, instrument};
use viewpoint_cdp::protocol::page::NavigateParams;

use super::Frame;
use crate::error::NavigationError;
use crate::wait::{DocumentLoadState, LoadStateWaiter};

/// Default navigation timeout.
pub(super) const DEFAULT_NAVIGATION_TIMEOUT: Duration = Duration::from_secs(30);

impl Frame {
    /// Navigate the frame to a URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame is detached or navigation fails.
    #[instrument(level = "info", skip(self), fields(frame_id = %self.id, url = %url))]
    pub async fn goto(&self, url: &str) -> Result<(), NavigationError> {
        self.goto_with_options(url, DocumentLoadState::Load, DEFAULT_NAVIGATION_TIMEOUT)
            .await
    }

    /// Navigate the frame to a URL with options.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame is detached or navigation fails.
    #[instrument(level = "info", skip(self), fields(frame_id = %self.id, url = %url, wait_until = ?wait_until))]
    pub async fn goto_with_options(
        &self,
        url: &str,
        wait_until: DocumentLoadState,
        timeout: Duration,
    ) -> Result<(), NavigationError> {
        if self.is_detached() {
            return Err(NavigationError::Cancelled);
        }

        info!("Navigating frame to URL");

        // Create a load state waiter
        let event_rx = self.connection.subscribe_events();
        let mut waiter = LoadStateWaiter::new(event_rx, self.session_id.clone(), self.id.clone());

        // Send navigation command with frame_id
        debug!("Sending Page.navigate command for frame");
        let result: viewpoint_cdp::protocol::page::NavigateResult = self
            .connection
            .send_command(
                "Page.navigate",
                Some(NavigateParams {
                    url: url.to_string(),
                    referrer: None,
                    transition_type: None,
                    frame_id: Some(self.id.clone()),
                }),
                Some(&self.session_id),
            )
            .await?;

        debug!(frame_id = %result.frame_id, "Page.navigate completed for frame");

        // Check for navigation errors
        if let Some(error_text) = result.error_text {
            return Err(NavigationError::NetworkError(error_text));
        }

        // Mark commit as received
        waiter.set_commit_received().await;

        // Wait for the target load state
        debug!(wait_until = ?wait_until, "Waiting for load state");
        waiter
            .wait_for_load_state_with_timeout(wait_until, timeout)
            .await?;

        // Update the frame's URL
        self.set_url(url.to_string());

        info!(frame_id = %self.id, "Frame navigation completed");
        Ok(())
    }

    /// Wait for the frame to reach a specific load state.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out or the frame is detached.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id, state = ?state))]
    pub async fn wait_for_load_state(
        &self,
        state: DocumentLoadState,
    ) -> Result<(), NavigationError> {
        self.wait_for_load_state_with_timeout(state, DEFAULT_NAVIGATION_TIMEOUT)
            .await
    }

    /// Wait for the frame to reach a specific load state with timeout.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out or the frame is detached.
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id, state = ?state, timeout_ms = timeout.as_millis()))]
    pub async fn wait_for_load_state_with_timeout(
        &self,
        state: DocumentLoadState,
        timeout: Duration,
    ) -> Result<(), NavigationError> {
        if self.is_detached() {
            return Err(NavigationError::Cancelled);
        }

        let event_rx = self.connection.subscribe_events();
        let mut waiter = LoadStateWaiter::new(event_rx, self.session_id.clone(), self.id.clone());

        // Assume commit already happened for existing frames
        waiter.set_commit_received().await;

        waiter
            .wait_for_load_state_with_timeout(state, timeout)
            .await?;

        debug!("Frame reached load state {:?}", state);
        Ok(())
    }
}
