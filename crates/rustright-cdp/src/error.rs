//! CDP error types.

use thiserror::Error;

/// Errors that can occur during CDP communication.
#[derive(Error, Debug)]
pub enum CdpError {
    /// WebSocket connection failed.
    #[error("WebSocket connection failed: {0}")]
    ConnectionFailed(String),

    /// WebSocket connection was lost during operation.
    #[error("WebSocket connection lost")]
    ConnectionLost,

    /// Failed to send a CDP message.
    #[error("failed to send CDP message: {0}")]
    SendFailed(String),

    /// CDP protocol error returned by the browser.
    #[error("CDP protocol error {code}: {message}")]
    Protocol { code: i64, message: String },

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Response timeout.
    #[error("response timeout after {0:?}")]
    Timeout(std::time::Duration),

    /// Invalid message ID in response.
    #[error("invalid message ID: expected {expected}, got {got}")]
    InvalidMessageId { expected: u64, got: u64 },

    /// Failed to parse WebSocket URL.
    #[error("invalid WebSocket URL: {0}")]
    InvalidUrl(String),

    /// Session not found.
    #[error("session not found: {0}")]
    SessionNotFound(String),

    /// Browser process spawn failed.
    #[error("failed to spawn browser process: {0}")]
    SpawnFailed(String),

    /// Failed to parse the debugging URL from browser output.
    #[error("failed to get debugging URL from browser")]
    NoDebuggingUrl,

    /// Chromium executable not found.
    #[error("Chromium not found. Set CHROMIUM_PATH environment variable or ensure Chromium is installed.")]
    ChromiumNotFound,

    /// Browser launch timeout.
    #[error("browser launch timeout after {0:?}")]
    LaunchTimeout(std::time::Duration),
}

impl From<tokio_tungstenite::tungstenite::Error> for CdpError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        match err {
            tokio_tungstenite::tungstenite::Error::ConnectionClosed
            | tokio_tungstenite::tungstenite::Error::AlreadyClosed => Self::ConnectionLost,
            other => Self::ConnectionFailed(other.to_string()),
        }
    }
}
