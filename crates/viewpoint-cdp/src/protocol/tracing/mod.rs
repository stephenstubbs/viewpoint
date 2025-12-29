//! Tracing domain types.
//!
//! The Tracing domain allows recording a trace of the browser activity.

use serde::{Deserialize, Serialize};

/// Transfer mode for tracing data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum TransferMode {
    /// Report trace events via `Tracing.dataCollected` events.
    #[default]
    ReportEvents,
    /// Return trace data as a stream.
    ReturnAsStream,
}

/// Stream format for trace data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum StreamFormat {
    /// JSON format.
    #[default]
    Json,
    /// Protocol buffer format.
    Proto,
}

/// Stream compression type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum StreamCompression {
    /// No compression.
    #[default]
    None,
    /// Gzip compression.
    Gzip,
}

/// Tracing backend type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum TracingBackend {
    /// Auto-select backend.
    #[default]
    Auto,
    /// Use Chrome tracing.
    Chrome,
    /// Use system tracing.
    System,
}

// ============================================================================
// Tracing.start
// ============================================================================

/// Parameters for Tracing.start.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartParams {
    /// Category/tag filter as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    /// Tracing options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    /// Buffer size in kilobytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_usage_reporting_interval: Option<f64>,
    /// Transfer mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<TransferMode>,
    /// Stream format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_format: Option<StreamFormat>,
    /// Stream compression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_compression: Option<StreamCompression>,
    /// Trace config for Chromium's tracing backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_config: Option<TraceConfig>,
    /// Base64-encoded serialized perfetto.protos.TraceConfig.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perfetto_config: Option<String>,
    /// Backend type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_backend: Option<TracingBackend>,
}

impl StartParams {
    /// Create new start parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the categories filter.
    #[must_use]
    pub fn categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    /// Set the transfer mode.
    #[must_use]
    pub fn transfer_mode(mut self, mode: TransferMode) -> Self {
        self.transfer_mode = Some(mode);
        self
    }

    /// Set the stream format.
    #[must_use]
    pub fn stream_format(mut self, format: StreamFormat) -> Self {
        self.stream_format = Some(format);
        self
    }

    /// Set the trace config.
    #[must_use]
    pub fn trace_config(mut self, config: TraceConfig) -> Self {
        self.trace_config = Some(config);
        self
    }
}

/// Trace configuration.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TraceConfig {
    /// Recording mode: recordUntilFull, recordContinuously, recordAsMuchAsPossible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_mode: Option<String>,
    /// Trace buffer size in kilobytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_buffer_size_in_kb: Option<i32>,
    /// Whether to enable sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sampling: Option<bool>,
    /// Whether to enable systrace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_systrace: Option<bool>,
    /// Whether to enable argument filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_argument_filter: Option<bool>,
    /// Included categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_categories: Option<Vec<String>>,
    /// Excluded categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_categories: Option<Vec<String>>,
    /// Synthetic delays configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthetic_delays: Option<Vec<String>>,
    /// Memory dump configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_dump_config: Option<serde_json::Value>,
}

impl TraceConfig {
    /// Create a new trace config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set included categories.
    #[must_use]
    pub fn included_categories(mut self, categories: Vec<String>) -> Self {
        self.included_categories = Some(categories);
        self
    }

    /// Add an included category.
    #[must_use]
    pub fn include_category(mut self, category: impl Into<String>) -> Self {
        let categories = self.included_categories.get_or_insert_with(Vec::new);
        categories.push(category.into());
        self
    }

    /// Set excluded categories.
    #[must_use]
    pub fn excluded_categories(mut self, categories: Vec<String>) -> Self {
        self.excluded_categories = Some(categories);
        self
    }

    /// Set record mode.
    #[must_use]
    pub fn record_mode(mut self, mode: impl Into<String>) -> Self {
        self.record_mode = Some(mode.into());
        self
    }
}

// ============================================================================
// Tracing.end
// ============================================================================

/// Parameters for Tracing.end (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct EndParams {}

/// Result of Tracing.end when using stream transfer mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndResult {
    /// Stream handle for the trace data (when using stream transfer mode).
    pub stream: Option<String>,
    /// Compression type of the trace data in the stream.
    pub stream_compression: Option<StreamCompression>,
}

// ============================================================================
// Tracing.getCategories
// ============================================================================

/// Parameters for Tracing.getCategories (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetCategoriesParams {}

/// Result of Tracing.getCategories.
#[derive(Debug, Clone, Deserialize)]
pub struct GetCategoriesResult {
    /// List of supported tracing categories.
    pub categories: Vec<String>,
}

// ============================================================================
// Tracing.recordClockSyncMarker
// ============================================================================

/// Parameters for Tracing.recordClockSyncMarker.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordClockSyncMarkerParams {
    /// Sync marker ID.
    pub sync_id: String,
}

// ============================================================================
// Tracing.requestMemoryDump
// ============================================================================

/// Parameters for Tracing.requestMemoryDump.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpParams {
    /// Whether to dump memory from all processes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deterministic: Option<bool>,
    /// Request a memory-infra dump for all processes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_detail: Option<String>,
}

/// Result of Tracing.requestMemoryDump.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMemoryDumpResult {
    /// GUID of the resulting global memory dump.
    pub dump_guid: String,
    /// True if the global memory dump succeeded.
    pub success: bool,
}

// ============================================================================
// Events
// ============================================================================

/// Event: Tracing.dataCollected
///
/// Contains a bucket of collected trace events.
#[derive(Debug, Clone, Deserialize)]
pub struct DataCollectedEvent {
    /// Collected trace events.
    pub value: Vec<serde_json::Value>,
}

/// Event: Tracing.tracingComplete
///
/// Signals that tracing is stopped and there is no more data to collect.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TracingCompleteEvent {
    /// Stream handle for trace data (when using stream transfer mode).
    pub stream: Option<String>,
    /// Compression type of the trace data in the stream.
    pub stream_compression: Option<StreamCompression>,
    /// Stream format of the trace data in the stream.
    pub stream_format: Option<StreamFormat>,
}

/// Event: Tracing.bufferUsage
///
/// Signals that tracing is approaching its buffer limit.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferUsageEvent {
    /// A number in range [0..1] that indicates the used size of event buffer.
    pub percent_full: Option<f64>,
    /// An approximate number of events in the trace log.
    pub event_count: Option<f64>,
    /// A number in range [0..1] that indicates the used size of event buffer.
    pub value: Option<f64>,
}
