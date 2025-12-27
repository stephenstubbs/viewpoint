//! Types for tracing implementation.
//!
//! Contains all the data structures used for recording and storing trace data.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::network::har::HarTimings;

/// Options for starting a trace.
#[derive(Debug, Clone, Default)]
pub struct TracingOptions {
    /// Name for the trace (appears in Trace Viewer).
    pub name: Option<String>,
    /// Whether to capture screenshots during tracing.
    pub screenshots: bool,
    /// Whether to capture DOM snapshots.
    pub snapshots: bool,
    /// Whether to include source files.
    pub sources: bool,
    /// Title to display in Trace Viewer.
    pub title: Option<String>,
}

impl TracingOptions {
    /// Create new default tracing options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the trace name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Enable screenshot capture during tracing.
    #[must_use]
    pub fn screenshots(mut self, enabled: bool) -> Self {
        self.screenshots = enabled;
        self
    }

    /// Enable DOM snapshot capture.
    #[must_use]
    pub fn snapshots(mut self, enabled: bool) -> Self {
        self.snapshots = enabled;
        self
    }

    /// Enable source file inclusion.
    #[must_use]
    pub fn sources(mut self, enabled: bool) -> Self {
        self.sources = enabled;
        self
    }

    /// Set the trace title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// A source file entry in the trace.
#[derive(Debug, Clone)]
pub(super) struct SourceFileEntry {
    /// Path to the source file (as it should appear in the trace).
    pub path: String,
    /// Content of the source file.
    pub content: String,
}

/// Internal state for an active trace.
#[derive(Debug, Default)]
pub(super) struct TracingState {
    /// Whether tracing is currently active.
    pub is_recording: bool,
    /// Tracing options.
    pub options: TracingOptions,
    /// Recorded actions.
    pub actions: Vec<ActionEntry>,
    /// Recorded events.
    pub events: Vec<serde_json::Value>,
    /// Screenshots captured during tracing.
    pub screenshots: Vec<ScreenshotEntry>,
    /// DOM snapshots captured during tracing.
    pub snapshots: Vec<serde_json::Value>,
    /// Network requests being tracked.
    pub pending_requests: HashMap<String, PendingRequest>,
    /// Completed network entries.
    pub network_entries: Vec<NetworkEntryState>,
    /// HAR pages.
    pub har_pages: Vec<crate::network::har::HarPage>,
    /// Current page ID for HAR entries.
    pub current_page_id: Option<String>,
    /// Source files to include.
    pub source_files: Vec<SourceFileEntry>,
}

/// A pending network request.
#[derive(Debug, Clone)]
pub(super) struct PendingRequest {
    /// Request ID.
    pub request_id: String,
    /// Request URL.
    pub url: String,
    /// Request method.
    pub method: String,
    /// Request headers.
    pub headers: HashMap<String, String>,
    /// Request body.
    pub post_data: Option<String>,
    /// Resource type.
    pub resource_type: String,
    /// When the request was started.
    pub started_at: DateTime<Utc>,
    /// Wall time from CDP.
    pub wall_time: f64,
}

/// A completed network entry.
#[derive(Debug, Clone)]
pub(super) struct NetworkEntryState {
    /// Original request.
    pub request: PendingRequest,
    /// Response status code.
    pub status: i32,
    /// Response status text.
    pub status_text: String,
    /// Response headers.
    pub response_headers: HashMap<String, String>,
    /// Response MIME type.
    pub mime_type: String,
    /// Request/response timing.
    pub timing: Option<HarTimings>,
    /// Server IP address.
    pub server_ip: Option<String>,
    /// Whether the request failed.
    pub failed: bool,
    /// Error text if failed.
    pub error_text: Option<String>,
    /// Encoded data length.
    pub encoded_data_length: Option<f64>,
}

/// A screenshot entry.
#[derive(Debug, Clone)]
pub(super) struct ScreenshotEntry {
    /// Screenshot data (base64 encoded).
    pub data: String,
    /// Timestamp when captured.
    pub timestamp: f64,
    /// Optional name.
    pub name: Option<String>,
}

/// An action entry in the trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionEntry {
    /// Action type (e.g., "click", "fill", "navigate").
    pub action_type: String,
    /// Target selector or description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Page ID where action was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
    /// Start time (ms since epoch).
    pub start_time: f64,
    /// End time (ms since epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// Action result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Input value for fill actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// URL for navigation actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Associated screenshot index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<usize>,
    /// Associated snapshot index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<usize>,
}

/// Trace file structure.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TraceFile {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub actions: Vec<ActionEntry>,
    pub events: Vec<serde_json::Value>,
    pub resources: Vec<ResourceEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}

/// Resource entry in the trace.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResourceEntry {
    pub name: String,
    pub timestamp: f64,
    pub resource_type: String,
    pub path: String,
}
