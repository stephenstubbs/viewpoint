//! Runtime domain types.
//!
//! The Runtime domain exposes JavaScript runtime by means of remote evaluation and mirror objects.

use serde::{Deserialize, Serialize};

/// Unique script identifier.
pub type ScriptId = String;

/// Unique execution context identifier.
pub type ExecutionContextId = i64;

/// Remote object value.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject {
    /// Object type.
    #[serde(rename = "type")]
    pub object_type: String,
    /// Object subtype hint.
    pub subtype: Option<String>,
    /// Object class name.
    pub class_name: Option<String>,
    /// Remote object value.
    pub value: Option<serde_json::Value>,
    /// String representation of the object.
    pub description: Option<String>,
    /// Unique object identifier.
    pub object_id: Option<String>,
}

/// Exception details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    /// Exception id.
    pub exception_id: i64,
    /// Exception text.
    pub text: String,
    /// Line number of the exception location.
    pub line_number: i64,
    /// Column number of the exception location.
    pub column_number: i64,
    /// Script ID of the exception location.
    pub script_id: Option<ScriptId>,
    /// URL of the exception location.
    pub url: Option<String>,
    /// Exception object if available.
    pub exception: Option<RemoteObject>,
    /// Execution context ID.
    pub execution_context_id: Option<ExecutionContextId>,
}

/// Parameters for Runtime.evaluate.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateParams {
    /// Expression to evaluate.
    pub expression: String,
    /// Object group for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_group: Option<String>,
    /// Whether to include command line API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_command_line_api: Option<bool>,
    /// Whether to disable side effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Execution context ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<ExecutionContextId>,
    /// Whether to return by value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_by_value: Option<bool>,
    /// Whether to await the promise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_promise: Option<bool>,
}

/// Result of Runtime.evaluate.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResult {
    /// Evaluation result.
    pub result: RemoteObject,
    /// Exception details if the evaluation threw.
    pub exception_details: Option<ExceptionDetails>,
}

/// Event: Runtime.executionContextCreated
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionContextCreatedEvent {
    /// Newly created execution context.
    pub context: ExecutionContextDescription,
}

/// Execution context description.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextDescription {
    /// Unique execution context id.
    pub id: ExecutionContextId,
    /// Execution context origin.
    pub origin: String,
    /// Human readable name describing given context.
    pub name: String,
}

/// Event: Runtime.executionContextDestroyed
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextDestroyedEvent {
    /// ID of the destroyed context.
    pub execution_context_id: ExecutionContextId,
}
