//! Network domain WebSocket types.
//!
//! Types for WebSocket events in the Network domain.

use serde::Deserialize;

use super::network::{MonotonicTime, RequestId, RequestInitiator};

/// Event: Network.webSocketCreated
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketCreatedEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// WebSocket request URL.
    pub url: String,
    /// Request initiator.
    pub initiator: Option<RequestInitiator>,
}

/// Event: Network.webSocketClosed
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketClosedEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Timestamp.
    pub timestamp: MonotonicTime,
}

/// WebSocket message data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrame {
    /// WebSocket message opcode.
    pub opcode: f64,
    /// WebSocket message mask.
    pub mask: bool,
    /// WebSocket message payload data.
    pub payload_data: String,
}

/// Event: Network.webSocketFrameSent
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrameSentEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// WebSocket response data.
    pub response: WebSocketFrame,
}

/// Event: Network.webSocketFrameReceived
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrameReceivedEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// WebSocket response data.
    pub response: WebSocketFrame,
}

/// Event: Network.webSocketFrameError
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrameErrorEvent {
    /// Request identifier.
    pub request_id: RequestId,
    /// Timestamp.
    pub timestamp: MonotonicTime,
    /// WebSocket error message.
    pub error_message: String,
}
