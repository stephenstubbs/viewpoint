//! Network event listener for tracing.

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::RwLock;

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::network::{
    LoadingFailedEvent, LoadingFinishedEvent, RequestWillBeSentEvent, ResponseReceivedEvent,
};

use crate::page::Page;

use super::types::{NetworkEntryState, PendingRequest, TracingState};

/// Start listening for network events.
///
/// Spawns a background task that processes network events and updates tracing state.
pub fn start_network_listener(
    connection: Arc<CdpConnection>,
    state: Arc<RwLock<TracingState>>,
    pages: Arc<RwLock<Vec<Page>>>,
) {
    let mut events = connection.subscribe_events();

    tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            // Check if we're still recording
            let is_recording = {
                let s = state.read().await;
                s.is_recording
            };
            if !is_recording {
                break;
            }

            // Check if this event is for one of our sessions
            let is_our_session = {
                let pages = pages.read().await;
                event
                    .session_id
                    .as_ref()
                    .is_some_and(|sid| pages.iter().any(|p| p.session_id() == sid))
            };
            if !is_our_session {
                continue;
            }

            // Handle network events
            match event.method.as_str() {
                "Network.requestWillBeSent" => {
                    handle_request_will_be_sent(&state, event.params.as_ref()).await;
                }
                "Network.responseReceived" => {
                    handle_response_received(&state, event.params.as_ref()).await;
                }
                "Network.loadingFinished" => {
                    handle_loading_finished(&state, event.params.as_ref()).await;
                }
                "Network.loadingFailed" => {
                    handle_loading_failed(&state, event.params.as_ref()).await;
                }
                _ => {}
            }
        }
    });
}

/// Handle Network.requestWillBeSent event.
async fn handle_request_will_be_sent(
    state: &Arc<RwLock<TracingState>>,
    params: Option<&serde_json::Value>,
) {
    if let Some(params) = params {
        if let Ok(req_event) = serde_json::from_value::<RequestWillBeSentEvent>(params.clone()) {
            let mut s = state.write().await;
            let pending = PendingRequest {
                request_id: req_event.request_id.clone(),
                url: req_event.request.url.clone(),
                method: req_event.request.method.clone(),
                headers: req_event.request.headers.clone(),
                post_data: req_event.request.post_data.clone(),
                resource_type: req_event.resource_type.unwrap_or_default(),
                started_at: Utc::now(),
                wall_time: req_event.wall_time,
            };
            s.pending_requests.insert(req_event.request_id, pending);
        }
    }
}

/// Handle Network.responseReceived event.
async fn handle_response_received(
    state: &Arc<RwLock<TracingState>>,
    params: Option<&serde_json::Value>,
) {
    if let Some(params) = params {
        if let Ok(resp_event) = serde_json::from_value::<ResponseReceivedEvent>(params.clone()) {
            let mut s = state.write().await;
            if let Some(pending) = s.pending_requests.get(&resp_event.request_id).cloned() {
                let entry = NetworkEntryState {
                    request: pending,
                    status: resp_event.response.status as i32,
                    status_text: resp_event.response.status_text.clone(),
                    response_headers: resp_event.response.headers.clone(),
                    mime_type: resp_event.response.mime_type.clone(),
                    timing: None,
                    server_ip: resp_event.response.remote_ip_address.clone(),
                    failed: false,
                    error_text: None,
                    encoded_data_length: None,
                };
                s.network_entries.push(entry);
            }
        }
    }
}

/// Handle Network.loadingFinished event.
async fn handle_loading_finished(
    state: &Arc<RwLock<TracingState>>,
    params: Option<&serde_json::Value>,
) {
    if let Some(params) = params {
        if let Ok(finished) = serde_json::from_value::<LoadingFinishedEvent>(params.clone()) {
            let mut s = state.write().await;
            if let Some(entry) = s
                .network_entries
                .iter_mut()
                .find(|e| e.request.request_id == finished.request_id)
            {
                entry.encoded_data_length = Some(finished.encoded_data_length);
            }
            s.pending_requests.remove(&finished.request_id);
        }
    }
}

/// Handle Network.loadingFailed event.
async fn handle_loading_failed(
    state: &Arc<RwLock<TracingState>>,
    params: Option<&serde_json::Value>,
) {
    if let Some(params) = params {
        if let Ok(failed) = serde_json::from_value::<LoadingFailedEvent>(params.clone()) {
            let mut s = state.write().await;
            if let Some(pending) = s.pending_requests.remove(&failed.request_id) {
                let entry = NetworkEntryState {
                    request: pending,
                    status: 0,
                    status_text: String::new(),
                    response_headers: HashMap::new(),
                    mime_type: String::new(),
                    timing: None,
                    server_ip: None,
                    failed: true,
                    error_text: Some(failed.error_text),
                    encoded_data_length: None,
                };
                s.network_entries.push(entry);
            }
        }
    }
}
