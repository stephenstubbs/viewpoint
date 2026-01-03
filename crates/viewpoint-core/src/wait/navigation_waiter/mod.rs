//! Navigation waiter for detecting navigation triggered by actions.
//!
//! This module provides `NavigationWaiter` which listens for CDP frame navigation
//! events to detect when an action (like click or press) triggers a page navigation.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, broadcast};
use tokio::time::{Instant, timeout};
use tracing::{debug, instrument, trace};
use viewpoint_cdp::CdpEvent;

use super::{DocumentLoadState, LoadStateWaiter};
use crate::error::WaitError;

/// Duration to wait for navigation to be triggered after an action.
const NAVIGATION_DETECTION_WINDOW: Duration = Duration::from_millis(50);

/// Default navigation timeout.
const DEFAULT_NAVIGATION_TIMEOUT: Duration = Duration::from_secs(30);

/// State of navigation detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NavigationState {
    /// No navigation detected yet.
    Idle,
    /// Navigation has been triggered (frameNavigated event received).
    Navigating,
    /// Navigation completed.
    Complete,
}

/// Waiter that detects and waits for navigation triggered by actions.
///
/// # Usage
///
/// 1. Create a waiter before performing an action
/// 2. Perform the action (click, press, etc.)
/// 3. Call `wait_for_navigation_if_triggered` to wait if navigation occurred
///
/// # Example (internal use)
///
/// ```no_run
/// # use viewpoint_core::wait::NavigationWaiter;
/// # async fn example(
/// #     event_rx: tokio::sync::broadcast::Receiver<viewpoint_cdp::CdpEvent>,
/// #     session_id: String,
/// #     frame_id: String,
/// # ) -> Result<(), viewpoint_core::error::WaitError> {
/// let waiter = NavigationWaiter::new(event_rx, session_id, frame_id);
/// // ... perform action ...
/// waiter.wait_for_navigation_if_triggered().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct NavigationWaiter {
    /// Event receiver for CDP events.
    event_rx: broadcast::Receiver<CdpEvent>,
    /// Session ID to filter events for.
    session_id: String,
    /// Main frame ID to track.
    frame_id: String,
    /// Current navigation state.
    state: Arc<Mutex<NavigationState>>,
    /// When the waiter was created (to track detection window).
    created_at: Instant,
    /// Navigation timeout.
    navigation_timeout: Duration,
}

impl NavigationWaiter {
    /// Create a new navigation waiter.
    ///
    /// # Arguments
    ///
    /// * `event_rx` - CDP event receiver
    /// * `session_id` - Session ID to filter events for
    /// * `frame_id` - Main frame ID to track navigation for
    pub fn new(
        event_rx: broadcast::Receiver<CdpEvent>,
        session_id: String,
        frame_id: String,
    ) -> Self {
        debug!(
            session_id = %session_id,
            frame_id = %frame_id,
            "Created NavigationWaiter"
        );
        Self {
            event_rx,
            session_id,
            frame_id,
            state: Arc::new(Mutex::new(NavigationState::Idle)),
            created_at: Instant::now(),
            navigation_timeout: DEFAULT_NAVIGATION_TIMEOUT,
        }
    }

    /// Set the navigation timeout.
    ///
    /// This is the maximum time to wait for navigation to complete after
    /// it has been detected. Default is 30 seconds.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.navigation_timeout = timeout;
        self
    }

    /// Wait for navigation to complete if one was triggered by the action.
    ///
    /// This method:
    /// 1. Waits up to 50ms for a navigation event to be triggered
    /// 2. If navigation is detected, waits for the load state to reach `Load`
    /// 3. If no navigation is detected, returns immediately
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Navigation times out
    /// - Page is closed during navigation
    #[instrument(level = "debug", skip(self))]
    pub async fn wait_for_navigation_if_triggered(mut self) -> Result<bool, WaitError> {
        // Check if navigation was triggered within the detection window
        let navigation_detected = self.detect_navigation().await;

        if !navigation_detected {
            debug!("No navigation detected within detection window");
            return Ok(false);
        }

        debug!("Navigation detected, waiting for load state");

        // Navigation was triggered, wait for it to complete
        self.wait_for_load_complete().await?;

        debug!("Navigation completed successfully");
        Ok(true)
    }

    /// Detect if navigation was triggered within the detection window.
    async fn detect_navigation(&mut self) -> bool {
        let remaining_window = NAVIGATION_DETECTION_WINDOW
            .checked_sub(self.created_at.elapsed())
            .unwrap_or(Duration::ZERO);

        if remaining_window.is_zero() {
            // Detection window already passed, check if we already have a navigation event
            return *self.state.lock().await != NavigationState::Idle;
        }

        // Wait for navigation event within the detection window
        let result = timeout(remaining_window, self.wait_for_navigation_event()).await;

        if let Ok(true) = result {
            trace!("Navigation event received within detection window");
            true
        } else {
            trace!("No navigation event within detection window");
            false
        }
    }

    /// Wait for a navigation event (frameNavigated).
    async fn wait_for_navigation_event(&mut self) -> bool {
        loop {
            let event = match self.event_rx.recv().await {
                Ok(event) => event,
                Err(broadcast::error::RecvError::Closed) => return false,
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
            };

            // Filter for our session
            if event.session_id.as_deref() != Some(&self.session_id) {
                continue;
            }

            // Check for navigation events
            match event.method.as_str() {
                "Page.frameNavigated" => {
                    if let Some(params) = &event.params {
                        // Check if this is the main frame
                        if let Some(frame) = params.get("frame") {
                            if let Some(frame_id) = frame.get("id").and_then(|v| v.as_str()) {
                                // Check if this is the main frame or a child of the main frame
                                let parent_id = frame.get("parentId").and_then(|v| v.as_str());
                                if frame_id == self.frame_id || parent_id.is_none() {
                                    debug!(frame_id = %frame_id, "Frame navigation detected");
                                    *self.state.lock().await = NavigationState::Navigating;
                                    return true;
                                }
                            }
                        }
                    }
                }
                "Page.navigatedWithinDocument" => {
                    if let Some(params) = &event.params {
                        if let Some(frame_id) = params.get("frameId").and_then(|v| v.as_str()) {
                            if frame_id == self.frame_id {
                                debug!(
                                    frame_id = %frame_id,
                                    "Within-document navigation detected"
                                );
                                // Within-document navigation (e.g., hash change) completes immediately
                                *self.state.lock().await = NavigationState::Complete;
                                return true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Wait for navigation to complete (reach Load state).
    async fn wait_for_load_complete(&mut self) -> Result<(), WaitError> {
        // Check if it was a within-document navigation (already complete)
        if *self.state.lock().await == NavigationState::Complete {
            return Ok(());
        }

        // Create a new load state waiter
        let mut load_waiter = LoadStateWaiter::new(
            self.event_rx.resubscribe(),
            self.session_id.clone(),
            self.frame_id.clone(),
        );

        // Set commit received since navigation already started
        load_waiter.set_commit_received().await;

        // Wait for Load state
        load_waiter
            .wait_for_load_state_with_timeout(DocumentLoadState::Load, self.navigation_timeout)
            .await?;

        *self.state.lock().await = NavigationState::Complete;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
