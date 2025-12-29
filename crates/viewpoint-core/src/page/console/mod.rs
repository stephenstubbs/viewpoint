//! Console message types and event handling.
//!
//! This module provides types for capturing JavaScript console output
//! (console.log, console.error, etc.) from the page.

// Allow dead code for console scaffolding (spec: console-events)

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::runtime::{
    ConsoleApiCalledEvent, ConsoleApiType, RemoteObject, StackTrace,
};

/// Type of console message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsoleMessageType {
    /// `console.log()`
    Log,
    /// `console.debug()`
    Debug,
    /// `console.info()`
    Info,
    /// `console.error()`
    Error,
    /// `console.warn()`
    Warning,
    /// `console.dir()`
    Dir,
    /// `console.dirxml()`
    DirXml,
    /// `console.table()`
    Table,
    /// `console.trace()`
    Trace,
    /// `console.clear()`
    Clear,
    /// `console.count()`
    Count,
    /// `console.assert()`
    Assert,
    /// `console.profile()`
    Profile,
    /// `console.profileEnd()`
    ProfileEnd,
    /// `console.group()` / `console.groupCollapsed()`
    StartGroup,
    /// `console.groupEnd()`
    EndGroup,
    /// `console.timeEnd()`
    TimeEnd,
}

impl From<ConsoleApiType> for ConsoleMessageType {
    fn from(api_type: ConsoleApiType) -> Self {
        match api_type {
            ConsoleApiType::Log => Self::Log,
            ConsoleApiType::Debug => Self::Debug,
            ConsoleApiType::Info => Self::Info,
            ConsoleApiType::Error => Self::Error,
            ConsoleApiType::Warning => Self::Warning,
            ConsoleApiType::Dir => Self::Dir,
            ConsoleApiType::Dirxml => Self::DirXml,
            ConsoleApiType::Table => Self::Table,
            ConsoleApiType::Trace => Self::Trace,
            ConsoleApiType::Clear => Self::Clear,
            ConsoleApiType::Count => Self::Count,
            ConsoleApiType::Assert => Self::Assert,
            ConsoleApiType::Profile => Self::Profile,
            ConsoleApiType::ProfileEnd => Self::ProfileEnd,
            ConsoleApiType::StartGroup | ConsoleApiType::StartGroupCollapsed => Self::StartGroup,
            ConsoleApiType::EndGroup => Self::EndGroup,
            ConsoleApiType::TimeEnd => Self::TimeEnd,
        }
    }
}

impl std::fmt::Display for ConsoleMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Log => "log",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Dir => "dir",
            Self::DirXml => "dirxml",
            Self::Table => "table",
            Self::Trace => "trace",
            Self::Clear => "clear",
            Self::Count => "count",
            Self::Assert => "assert",
            Self::Profile => "profile",
            Self::ProfileEnd => "profileEnd",
            Self::StartGroup => "startGroup",
            Self::EndGroup => "endGroup",
            Self::TimeEnd => "timeEnd",
        };
        write!(f, "{s}")
    }
}

/// Location information for a console message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleMessageLocation {
    /// URL of the script that generated the message.
    pub url: String,
    /// Line number (0-based).
    pub line_number: i32,
    /// Column number (0-based).
    pub column_number: i32,
}

/// A console message captured from the page.
///
/// Console messages are emitted when JavaScript code calls console methods
/// like `console.log()`, `console.error()`, etc.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
///
/// page.on_console(|message| async move {
///     println!("{}: {}", message.type_(), message.text());
/// }).await;
/// # });
/// ```
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message type (log, error, warn, etc.).
    message_type: ConsoleMessageType,
    /// Message arguments as remote objects.
    args: Vec<RemoteObject>,
    /// Timestamp when the message was logged.
    timestamp: f64,
    /// Stack trace if available.
    stack_trace: Option<StackTrace>,
    /// Execution context ID.
    execution_context_id: i64,
    /// CDP connection for resolving arguments.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
}

impl ConsoleMessage {
    /// Create a new console message from a CDP event.
    pub(crate) fn from_event(
        event: ConsoleApiCalledEvent,
        connection: Arc<CdpConnection>,
        session_id: String,
    ) -> Self {
        Self {
            message_type: ConsoleMessageType::from(event.call_type),
            args: event.args,
            timestamp: event.timestamp,
            stack_trace: event.stack_trace,
            execution_context_id: event.execution_context_id,
            connection,
            session_id,
        }
    }

    /// Get the message type.
    ///
    /// Returns the type of console call (log, error, warn, etc.).
    pub fn type_(&self) -> ConsoleMessageType {
        self.message_type
    }

    /// Get the formatted message text.
    ///
    /// This combines all arguments into a single string, similar to how
    /// the browser console would display them.
    pub fn text(&self) -> String {
        self.args
            .iter()
            .map(|arg| {
                if let Some(value) = &arg.value {
                    format_value(value)
                } else if let Some(description) = &arg.description {
                    description.clone()
                } else {
                    arg.object_type.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Get the raw message arguments.
    ///
    /// These are the arguments passed to the console method as JS handles.
    /// Use `text()` for a formatted string representation.
    pub fn args(&self) -> Vec<JsArg> {
        self.args
            .iter()
            .map(|arg| JsArg {
                object_type: arg.object_type.clone(),
                subtype: arg.subtype.clone(),
                class_name: arg.class_name.clone(),
                value: arg.value.clone(),
                description: arg.description.clone(),
                object_id: arg.object_id.clone(),
            })
            .collect()
    }

    /// Get the source location of the console call.
    ///
    /// Returns `None` if no stack trace is available.
    pub fn location(&self) -> Option<ConsoleMessageLocation> {
        self.stack_trace.as_ref().and_then(|st| {
            st.call_frames.first().map(|frame| ConsoleMessageLocation {
                url: frame.url.clone(),
                line_number: frame.line_number,
                column_number: frame.column_number,
            })
        })
    }

    /// Get the timestamp when the message was logged.
    ///
    /// Returns the timestamp in milliseconds since Unix epoch.
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }
}

/// A JavaScript argument from a console message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsArg {
    /// Object type (object, function, string, number, etc.).
    pub object_type: String,
    /// Object subtype (array, null, regexp, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Object class name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// Primitive value or JSON representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// String representation of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Object ID for retrieving properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}

impl JsArg {
    /// Get a JSON value representation.
    pub fn json_value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref()
    }
}

/// Format a JSON value as a string for console output.
fn format_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{k}: {}", format_value(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
}
