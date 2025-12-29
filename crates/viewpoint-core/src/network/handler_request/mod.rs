//! Request handling logic for the route handler registry.
//!
//! This module contains the request dispatching logic for handling paused requests.

use std::sync::Arc;

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::fetch::{ContinueRequestParams, RequestPausedEvent};

use super::request::Request;
use super::route::Route;
use crate::error::NetworkError;

/// Continue a request without modification.
pub(super) async fn continue_request(
    connection: &CdpConnection,
    session_id: &str,
    request_id: &str,
) -> Result<(), NetworkError> {
    connection
        .send_command::<_, serde_json::Value>(
            "Fetch.continueRequest",
            Some(ContinueRequestParams {
                request_id: request_id.to_string(),
                ..Default::default()
            }),
            Some(session_id),
        )
        .await
        .map_err(NetworkError::from)?;
    Ok(())
}

/// Create a Route from a RequestPausedEvent.
pub(super) fn create_route_from_event(
    event: &RequestPausedEvent,
    connection: Arc<CdpConnection>,
    session_id: String,
) -> Route {
    let request = Request::from_cdp(
        event.request.clone(),
        event.resource_type,
        event.frame_id.clone(),
        Some(connection.clone()),
        Some(session_id.clone()),
        Some(event.request_id.clone()),
    );

    Route::new(
        request,
        event.request_id.clone(),
        connection,
        session_id,
        event.response_status_code.map(|c| c as u16),
        event.response_headers.clone(),
    )
}
