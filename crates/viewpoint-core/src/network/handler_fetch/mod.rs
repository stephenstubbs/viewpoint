//! Fetch domain enable/disable logic for the route handler registry.

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::fetch::{EnableParams, RequestPattern as CdpRequestPattern};

use crate::error::NetworkError;

/// Enable the Fetch domain.
pub(super) async fn enable_fetch(
    connection: &CdpConnection,
    session_id: &str,
    handle_auth: bool,
) -> Result<(), NetworkError> {
    connection
        .send_command::<_, serde_json::Value>(
            "Fetch.enable",
            Some(EnableParams {
                patterns: Some(vec![CdpRequestPattern {
                    url_pattern: Some("*".to_string()),
                    resource_type: None,
                    request_stage: None,
                }]),
                handle_auth_requests: Some(handle_auth),
            }),
            Some(session_id),
        )
        .await
        .map_err(NetworkError::from)?;
    Ok(())
}

/// Disable the Fetch domain.
pub(super) async fn disable_fetch(
    connection: &CdpConnection,
    session_id: &str,
) -> Result<(), NetworkError> {
    connection
        .send_command::<_, serde_json::Value>("Fetch.disable", None::<()>, Some(session_id))
        .await
        .map_err(NetworkError::from)?;
    Ok(())
}
