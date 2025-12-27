//! CDP message transport types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A CDP request message.
#[derive(Debug, Clone, Serialize)]
pub struct CdpRequest {
    /// Unique message ID for matching responses.
    pub id: u64,
    /// CDP method name (e.g., "Target.createTarget").
    pub method: String,
    /// Method parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    /// Session ID for target-specific commands.
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// A CDP response message.
#[derive(Debug, Clone, Deserialize)]
pub struct CdpResponse {
    /// Message ID matching the request.
    pub id: u64,
    /// Result on success.
    pub result: Option<Value>,
    /// Error on failure.
    pub error: Option<CdpResponseError>,
    /// Session ID if this was a session-specific response.
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
}

/// Error details in a CDP response.
#[derive(Debug, Clone, Deserialize)]
pub struct CdpResponseError {
    /// Error code.
    pub code: i64,
    /// Error message.
    pub message: String,
    /// Additional error data.
    pub data: Option<String>,
}

/// A CDP event message.
#[derive(Debug, Clone, Deserialize)]
pub struct CdpEvent {
    /// Event method name (e.g., "Page.loadEventFired").
    pub method: String,
    /// Event parameters.
    pub params: Option<Value>,
    /// Session ID if this event came from a specific session.
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
}

/// An incoming CDP message (either response or event).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CdpMessage {
    /// A response to a previous request.
    Response(CdpResponse),
    /// An event pushed by the browser.
    Event(CdpEvent),
}

impl CdpMessage {
    /// Check if this message is a response with the given ID.
    pub fn is_response_for(&self, id: u64) -> bool {
        matches!(self, Self::Response(resp) if resp.id == id)
    }

    /// Try to extract this as a response.
    pub fn into_response(self) -> Option<CdpResponse> {
        match self {
            Self::Response(resp) => Some(resp),
            Self::Event(_) => None,
        }
    }

    /// Try to extract this as an event.
    pub fn into_event(self) -> Option<CdpEvent> {
        match self {
            Self::Event(evt) => Some(evt),
            Self::Response(_) => None,
        }
    }
}

#[cfg(test)]
mod tests;
