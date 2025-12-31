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
    /// Auxiliary data about the context, including frame information.
    pub aux_data: Option<ExecutionContextAuxData>,
}

/// Auxiliary data for execution context.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextAuxData {
    /// Frame ID associated with this execution context.
    pub frame_id: Option<String>,
    /// Whether this is the default context for the frame.
    pub is_default: Option<bool>,
    /// Type of the context (e.g., "default", "isolated", "worker").
    #[serde(rename = "type")]
    pub context_type: Option<String>,
}

/// Event: Runtime.executionContextDestroyed
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextDestroyedEvent {
    /// ID of the destroyed context.
    pub execution_context_id: ExecutionContextId,
}

// ============================================================================
// Call Function On
// ============================================================================

/// Parameters for Runtime.callFunctionOn.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnParams {
    /// Declaration of the function to call.
    pub function_declaration: String,
    /// Identifier of the object to call function on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// Call arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<CallArgument>>,
    /// In silent mode exceptions thrown during evaluation are not reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Whether the result is expected to be a JSON object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_by_value: Option<bool>,
    /// Whether to generate preview for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_preview: Option<bool>,
    /// Whether execution should be treated as initiated by user in the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_gesture: Option<bool>,
    /// Whether execution should await for resulting value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_promise: Option<bool>,
    /// Specifies execution context which global object will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<ExecutionContextId>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_group: Option<String>,
    /// Whether to throw an exception if side effect cannot be ruled out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throw_on_side_effect: Option<bool>,
    /// An alternative way to specify the execution context to call function on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_context_id: Option<String>,
    /// Specifies the result serialization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_options: Option<serde_json::Value>,
}

/// Call argument for callFunctionOn.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallArgument {
    /// Primitive value or serializable javascript object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// Primitive value which can not be JSON-stringified does not have value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unserializable_value: Option<String>,
    /// Remote object handle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}

/// Result of Runtime.callFunctionOn.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnResult {
    /// Call result.
    pub result: RemoteObject,
    /// Exception details if the call threw.
    pub exception_details: Option<ExceptionDetails>,
}

// ============================================================================
// Release Object
// ============================================================================

/// Parameters for Runtime.releaseObject.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectParams {
    /// Identifier of the object to release.
    pub object_id: String,
}

/// Parameters for Runtime.releaseObjectGroup.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectGroupParams {
    /// Symbolic object group name.
    pub object_group: String,
}

// ============================================================================
// Get Properties
// ============================================================================

/// Parameters for Runtime.getProperties.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPropertiesParams {
    /// Identifier of the object to return properties for.
    pub object_id: String,
    /// If true, returns properties belonging only to the element itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_properties: Option<bool>,
    /// If true, returns accessor properties only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessor_properties_only: Option<bool>,
    /// Whether preview should be generated for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_preview: Option<bool>,
    /// If true, returns non-indexed properties only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_indexed_properties_only: Option<bool>,
}

/// Property descriptor.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor {
    /// Property name or symbol description.
    pub name: String,
    /// The value associated with the property.
    pub value: Option<RemoteObject>,
    /// True if the value associated with the property may be changed.
    pub writable: Option<bool>,
    /// A function which serves as a getter for the property.
    pub get: Option<RemoteObject>,
    /// A function which serves as a setter for the property.
    pub set: Option<RemoteObject>,
    /// True if the type of this property descriptor may be changed.
    pub configurable: bool,
    /// True if this property shows up during enumeration.
    pub enumerable: bool,
    /// True if the result was thrown during the evaluation.
    pub was_thrown: Option<bool>,
    /// True if the property is owned for the object.
    pub is_own: Option<bool>,
    /// Property symbol object.
    pub symbol: Option<RemoteObject>,
}

/// Result of Runtime.getProperties.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPropertiesResult {
    /// Object properties.
    pub result: Vec<PropertyDescriptor>,
    /// Internal object properties (only of the element itself).
    pub internal_properties: Option<Vec<InternalPropertyDescriptor>>,
    /// Object private properties.
    pub private_properties: Option<Vec<PrivatePropertyDescriptor>>,
    /// Exception details.
    pub exception_details: Option<ExceptionDetails>,
}

/// Internal property descriptor.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalPropertyDescriptor {
    /// Conventional property name.
    pub name: String,
    /// The value associated with the property.
    pub value: Option<RemoteObject>,
}

/// Private property descriptor.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePropertyDescriptor {
    /// Private property name.
    pub name: String,
    /// The value associated with the private property.
    pub value: Option<RemoteObject>,
    /// A function which serves as a getter for the private property.
    pub get: Option<RemoteObject>,
    /// A function which serves as a setter for the private property.
    pub set: Option<RemoteObject>,
}

// ============================================================================
// Binding
// ============================================================================

/// Parameters for Runtime.addBinding.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBindingParams {
    /// Binding name.
    pub name: String,
    /// If specified, the binding would only be exposed to the specified execution context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<ExecutionContextId>,
    /// If specified, the binding is exposed to the given execution context name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context_name: Option<String>,
}

/// Parameters for Runtime.removeBinding.
#[derive(Debug, Clone, Serialize)]
pub struct RemoveBindingParams {
    /// Binding name.
    pub name: String,
}

/// Event: Runtime.bindingCalled
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingCalledEvent {
    /// Binding name.
    pub name: String,
    /// Binding payload.
    pub payload: String,
    /// Identifier of the context where the call was made.
    pub execution_context_id: ExecutionContextId,
}

// ============================================================================
// Console API Called Event
// ============================================================================

/// Console message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsoleApiType {
    /// `console.log()`
    Log,
    /// `console.debug()`
    Debug,
    /// `console.info()`
    Info,
    /// `console.error()`
    Error,
    /// `console.warn()` / `console.warning()`
    Warning,
    /// `console.dir()`
    Dir,
    /// `console.dirxml()`
    Dirxml,
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
    /// `console.time()`
    StartGroup,
    /// `console.timeEnd()`
    StartGroupCollapsed,
    /// `console.groupEnd()`
    EndGroup,
    /// `console.timeLog()`
    TimeEnd,
}

impl std::fmt::Display for ConsoleApiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Log => "log",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Dir => "dir",
            Self::Dirxml => "dirxml",
            Self::Table => "table",
            Self::Trace => "trace",
            Self::Clear => "clear",
            Self::Count => "count",
            Self::Assert => "assert",
            Self::Profile => "profile",
            Self::ProfileEnd => "profileEnd",
            Self::StartGroup => "startGroup",
            Self::StartGroupCollapsed => "startGroupCollapsed",
            Self::EndGroup => "endGroup",
            Self::TimeEnd => "timeEnd",
        };
        write!(f, "{s}")
    }
}

/// Stack trace entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    /// JavaScript function name.
    pub function_name: String,
    /// JavaScript script id.
    pub script_id: ScriptId,
    /// JavaScript script name or URL.
    pub url: String,
    /// JavaScript script line number (0-based).
    pub line_number: i32,
    /// JavaScript script column number (0-based).
    pub column_number: i32,
}

/// Stack trace.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTrace {
    /// String label of this stack trace.
    pub description: Option<String>,
    /// JavaScript function call frames.
    pub call_frames: Vec<CallFrame>,
    /// Asynchronous JavaScript stack trace that preceded this stack (if available).
    pub parent: Option<Box<StackTrace>>,
    /// Asynchronous JavaScript stack trace that preceded this stack (if available).
    pub parent_id: Option<StackTraceId>,
}

/// Unique identifier of current debugger.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceId {
    /// Unique id.
    pub id: String,
    /// Debugger id (only set when created by other debugger).
    pub debugger_id: Option<String>,
}

/// Event: Runtime.consoleAPICalled
///
/// Issued when console API was called.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleApiCalledEvent {
    /// Type of the call.
    #[serde(rename = "type")]
    pub call_type: ConsoleApiType,
    /// Call arguments.
    pub args: Vec<RemoteObject>,
    /// Identifier of the context where the call was made.
    pub execution_context_id: ExecutionContextId,
    /// Call timestamp.
    pub timestamp: f64,
    /// Stack trace captured when the call was made.
    pub stack_trace: Option<StackTrace>,
    /// Console context descriptor for calls on non-default console context (not console.*):
    /// 'anonymous#unique-logger-id' for call on unnamed context.
    pub context: Option<String>,
}

// ============================================================================
// Exception Thrown Event
// ============================================================================

/// Event: Runtime.exceptionThrown
///
/// Issued when exception was thrown and unhandled.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionThrownEvent {
    /// Timestamp of the exception.
    pub timestamp: f64,
    /// Exception details.
    pub exception_details: ExceptionDetails,
}
