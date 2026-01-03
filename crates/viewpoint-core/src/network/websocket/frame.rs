//! WebSocket frame types.

use viewpoint_cdp::protocol::WebSocketFrame as CdpWebSocketFrame;

/// A WebSocket message frame.
#[derive(Debug, Clone)]
pub struct WebSocketFrame {
    /// The frame opcode (1 for text, 2 for binary).
    opcode: u8,
    /// The frame payload data.
    payload_data: String,
}

impl WebSocketFrame {
    /// Create a new WebSocket frame.
    pub(crate) fn new(opcode: u8, payload_data: String) -> Self {
        Self {
            opcode,
            payload_data,
        }
    }

    /// Create a WebSocket frame from CDP frame data.
    pub(crate) fn from_cdp(cdp_frame: &CdpWebSocketFrame) -> Self {
        Self {
            opcode: cdp_frame.opcode as u8,
            payload_data: cdp_frame.payload_data.clone(),
        }
    }

    /// Get the frame opcode.
    ///
    /// Common opcodes:
    /// - 1: Text frame
    /// - 2: Binary frame
    /// - 8: Close frame
    /// - 9: Ping frame
    /// - 10: Pong frame
    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    /// Get the frame payload data.
    pub fn payload(&self) -> &str {
        &self.payload_data
    }

    /// Check if this is a text frame.
    pub fn is_text(&self) -> bool {
        self.opcode == 1
    }

    /// Check if this is a binary frame.
    pub fn is_binary(&self) -> bool {
        self.opcode == 2
    }
}
