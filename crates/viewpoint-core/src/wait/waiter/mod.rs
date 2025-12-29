//! Load state waiter implementation.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use tokio::sync::{Mutex, broadcast};
use tokio::time::{Instant, sleep, timeout};
use tracing::{debug, instrument, trace, warn};
use viewpoint_cdp::CdpEvent;
use viewpoint_cdp::protocol::network::{
    LoadingFailedEvent, LoadingFinishedEvent, RequestWillBeSentEvent, ResponseReceivedEvent,
};

use super::DocumentLoadState;
use crate::error::WaitError;

/// Captured response data during navigation.
#[derive(Debug, Clone, Default)]
pub struct NavigationResponseData {
    /// HTTP status code.
    pub status: Option<u16>,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Final URL after redirects.
    pub url: Option<String>,
}

/// Default timeout for wait operations.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Network idle threshold (no requests for this duration).
const NETWORK_IDLE_THRESHOLD: Duration = Duration::from_millis(500);

/// Waits for page load states by listening to CDP events.
#[derive(Debug)]
pub struct LoadStateWaiter {
    /// Current load state.
    current_state: Arc<Mutex<DocumentLoadState>>,
    /// Event receiver for CDP events.
    event_rx: broadcast::Receiver<CdpEvent>,
    /// Session ID to filter events for.
    session_id: String,
    /// Frame ID to wait for.
    frame_id: String,
    /// Pending network request count.
    pending_requests: Arc<AtomicUsize>,
    /// Set of pending request IDs.
    pending_request_ids: Arc<Mutex<HashSet<String>>>,
    /// Captured response data from navigation.
    response_data: Arc<Mutex<NavigationResponseData>>,
    /// The main document request ID (for tracking the navigation response).
    main_request_id: Arc<Mutex<Option<String>>>,
}

impl LoadStateWaiter {
    /// Create a new load state waiter.
    pub fn new(
        event_rx: broadcast::Receiver<CdpEvent>,
        session_id: String,
        frame_id: String,
    ) -> Self {
        debug!(session_id = %session_id, frame_id = %frame_id, "Created LoadStateWaiter");
        Self {
            current_state: Arc::new(Mutex::new(DocumentLoadState::Commit)),
            event_rx,
            session_id,
            frame_id,
            pending_requests: Arc::new(AtomicUsize::new(0)),
            pending_request_ids: Arc::new(Mutex::new(HashSet::new())),
            response_data: Arc::new(Mutex::new(NavigationResponseData::default())),
            main_request_id: Arc::new(Mutex::new(None)),
        }
    }

    /// Wait for the specified load state to be reached.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out or is cancelled.
    pub async fn wait_for_load_state(
        &mut self,
        target_state: DocumentLoadState,
    ) -> Result<(), WaitError> {
        self.wait_for_load_state_with_timeout(target_state, DEFAULT_TIMEOUT)
            .await
    }

    /// Wait for the specified load state with a custom timeout.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out or is cancelled.
    #[instrument(level = "debug", skip(self), fields(target_state = ?target_state, timeout_ms = timeout_duration.as_millis()))]
    pub async fn wait_for_load_state_with_timeout(
        &mut self,
        target_state: DocumentLoadState,
        timeout_duration: Duration,
    ) -> Result<(), WaitError> {
        // Check if already reached
        {
            let current = *self.current_state.lock().await;
            if target_state.is_reached(current) {
                debug!(current = ?current, "Target state already reached");
                return Ok(());
            }
            trace!(current = ?current, "Starting wait for target state");
        }

        let result = timeout(timeout_duration, self.wait_for_state_impl(target_state)).await;

        match result {
            Ok(Ok(())) => {
                debug!("Wait completed successfully");
                Ok(())
            }
            Ok(Err(e)) => {
                warn!(error = ?e, "Wait failed with error");
                Err(e)
            }
            Err(_) => {
                warn!(timeout_ms = timeout_duration.as_millis(), "Wait timed out");
                Err(WaitError::Timeout(timeout_duration))
            }
        }
    }

    /// Internal implementation of waiting for a load state.
    async fn wait_for_state_impl(
        &mut self,
        target_state: DocumentLoadState,
    ) -> Result<(), WaitError> {
        let mut last_network_activity = Instant::now();

        loop {
            // Check current state
            {
                let current = *self.current_state.lock().await;
                if target_state.is_reached(current) {
                    // For NetworkIdle, we need additional checking
                    if target_state == DocumentLoadState::NetworkIdle {
                        let pending = self.pending_requests.load(Ordering::Relaxed);
                        if pending == 0 && last_network_activity.elapsed() >= NETWORK_IDLE_THRESHOLD
                        {
                            return Ok(());
                        }
                    } else {
                        return Ok(());
                    }
                }
            }

            // Wait for the next event
            let event = match self.event_rx.recv().await {
                Ok(event) => event,
                Err(broadcast::error::RecvError::Closed) => {
                    return Err(WaitError::PageClosed);
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    // Missed some events, continue
                    continue;
                }
            };

            // Filter for our session
            if event.session_id.as_deref() != Some(&self.session_id) {
                continue;
            }

            // Process the event
            match event.method.as_str() {
                "Page.domContentEventFired" => {
                    let mut current = self.current_state.lock().await;
                    if *current < DocumentLoadState::DomContentLoaded {
                        debug!(previous = ?*current, "State transition: DomContentLoaded");
                        *current = DocumentLoadState::DomContentLoaded;
                    }
                }
                "Page.loadEventFired" => {
                    let mut current = self.current_state.lock().await;
                    if *current < DocumentLoadState::Load {
                        debug!(previous = ?*current, "State transition: Load");
                        *current = DocumentLoadState::Load;
                    }
                }
                "Network.requestWillBeSent" => {
                    if let Some(params) = event.params {
                        if let Ok(req) = serde_json::from_value::<RequestWillBeSentEvent>(params) {
                            // Only track main frame requests
                            if req.frame_id.as_deref() == Some(&self.frame_id) {
                                let mut ids = self.pending_request_ids.lock().await;
                                if ids.insert(req.request_id.clone()) {
                                    let count =
                                        self.pending_requests.fetch_add(1, Ordering::Relaxed) + 1;
                                    trace!(request_id = %req.request_id, pending_count = count, "Network request started");
                                    last_network_activity = Instant::now();

                                    // Track the main document request (type "Document")
                                    if req.resource_type.as_deref() == Some("Document") {
                                        let mut main_req = self.main_request_id.lock().await;
                                        if main_req.is_none() {
                                            *main_req = Some(req.request_id.clone());
                                            trace!(request_id = %req.request_id, "Tracking main document request");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "Network.responseReceived" => {
                    if let Some(params) = event.params {
                        if let Ok(resp) = serde_json::from_value::<ResponseReceivedEvent>(params) {
                            // Check if this is the main document response
                            let main_req = self.main_request_id.lock().await;
                            if main_req.as_deref() == Some(&resp.request_id) {
                                let mut response_data = self.response_data.lock().await;
                                response_data.status = Some(resp.response.status as u16);
                                response_data.url = Some(resp.response.url.clone());

                                // Copy headers
                                response_data.headers = resp.response.headers.clone();

                                trace!(
                                    status = response_data.status,
                                    url = ?response_data.url,
                                    header_count = response_data.headers.len(),
                                    "Captured main document response"
                                );
                            }
                        }
                    }
                }
                "Network.loadingFinished" => {
                    if let Some(params) = event.params {
                        if let Ok(finished) = serde_json::from_value::<LoadingFinishedEvent>(params)
                        {
                            let mut ids = self.pending_request_ids.lock().await;
                            if ids.remove(&finished.request_id) {
                                let count =
                                    self.pending_requests.fetch_sub(1, Ordering::Relaxed) - 1;
                                trace!(request_id = %finished.request_id, pending_count = count, "Network request finished");
                                last_network_activity = Instant::now();
                            }
                        }
                    }
                }
                "Network.loadingFailed" => {
                    if let Some(params) = event.params {
                        if let Ok(failed) = serde_json::from_value::<LoadingFailedEvent>(params) {
                            let mut ids = self.pending_request_ids.lock().await;
                            if ids.remove(&failed.request_id) {
                                let count =
                                    self.pending_requests.fetch_sub(1, Ordering::Relaxed) - 1;
                                trace!(request_id = %failed.request_id, pending_count = count, "Network request failed");
                                last_network_activity = Instant::now();
                            }
                        }
                    }
                }
                _ => {}
            }

            // For NetworkIdle, check if we've been idle long enough
            if target_state == DocumentLoadState::NetworkIdle {
                let pending = self.pending_requests.load(Ordering::Relaxed);
                let current = *self.current_state.lock().await;
                if pending == 0 && current >= DocumentLoadState::Load {
                    // Wait for the idle threshold
                    sleep(NETWORK_IDLE_THRESHOLD).await;
                    // Check again after sleeping
                    let pending_after = self.pending_requests.load(Ordering::Relaxed);
                    if pending_after == 0 {
                        return Ok(());
                    }
                }
            }
        }
    }

    /// Set the current load state (used when commit is detected).
    pub async fn set_commit_received(&self) {
        let mut current = self.current_state.lock().await;
        if *current < DocumentLoadState::Commit {
            debug!("State transition: Commit");
            *current = DocumentLoadState::Commit;
        }
    }

    /// Get the current load state.
    pub async fn current_state(&self) -> DocumentLoadState {
        *self.current_state.lock().await
    }

    /// Get the captured response data from navigation.
    ///
    /// This returns the status code, headers, and final URL captured during navigation.
    pub async fn response_data(&self) -> NavigationResponseData {
        self.response_data.lock().await.clone()
    }
}
