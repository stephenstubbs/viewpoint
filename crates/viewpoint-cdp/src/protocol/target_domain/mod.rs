//! Target domain types.
//!
//! The Target domain supports inspecting, attaching to, and managing Chrome targets.

use serde::{Deserialize, Serialize};

/// Information about a target.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    /// Target identifier.
    pub target_id: String,
    /// Target type (e.g., "page", "`background_page`", "`service_worker`").
    #[serde(rename = "type")]
    pub target_type: String,
    /// Target title.
    pub title: String,
    /// Target URL.
    pub url: String,
    /// Whether the target is attached.
    pub attached: bool,
    /// Browser context ID if this target belongs to a context.
    pub browser_context_id: Option<String>,
    /// Opener target ID (the target that opened this one, for popups).
    pub opener_id: Option<String>,
}

/// Parameters for Target.createBrowserContext.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextParams {
    /// Whether to create a context without any proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispose_on_detach: Option<bool>,
    /// Proxy server, e.g., "<http://proxy.example.com:8080>".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_server: Option<String>,
    /// Bypass list for the proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_bypass_list: Option<String>,
}

/// Result of Target.createBrowserContext.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserContextResult {
    /// Browser context ID.
    pub browser_context_id: String,
}

/// Parameters for Target.disposeBrowserContext.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisposeBrowserContextParams {
    /// Browser context ID to dispose.
    pub browser_context_id: String,
}

/// Parameters for Target.createTarget.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetParams {
    /// The initial URL the page will be navigated to.
    pub url: String,
    /// Frame width in pixels. Browser-controlled if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Frame height in pixels. Browser-controlled if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Browser context to create the page in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
    /// Whether to begin with background tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// Whether to create a new window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_window: Option<bool>,
}

/// Result of Target.createTarget.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetResult {
    /// The ID of the created target.
    pub target_id: String,
}

/// Parameters for Target.attachToTarget.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetParams {
    /// Target ID to attach to.
    pub target_id: String,
    /// Enables "flat" access to the session via specifying sessionId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
}

/// Result of Target.attachToTarget.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachToTargetResult {
    /// Session ID for the attached target.
    pub session_id: String,
}

/// Parameters for Target.closeTarget.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseTargetParams {
    /// Target ID to close.
    pub target_id: String,
}

/// Result of Target.closeTarget.
#[derive(Debug, Clone, Deserialize)]
pub struct CloseTargetResult {
    /// Whether the target was closed successfully.
    pub success: bool,
}

/// Parameters for Target.detachFromTarget.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetachFromTargetParams {
    /// Session ID to detach from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Parameters for Target.getTargets.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTargetsParams {
    /// Filter targets by their types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<TargetFilter>>,
}

/// Target filter for getTargets.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetFilter {
    /// Target type to filter.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// Whether to exclude the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
}

/// Result of Target.getTargets.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTargetsResult {
    /// List of targets.
    pub target_infos: Vec<TargetInfo>,
}

/// Result of Target.getBrowserContexts.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBrowserContextsResult {
    /// List of browser context IDs.
    pub browser_context_ids: Vec<String>,
}

/// Event: Target.targetCreated
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetCreatedEvent {
    /// Target info.
    pub target_info: TargetInfo,
}

/// Event: Target.targetDestroyed
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetDestroyedEvent {
    /// Target ID.
    pub target_id: String,
}

/// Event: Target.attachedToTarget
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedToTargetEvent {
    /// Session ID.
    pub session_id: String,
    /// Target info.
    pub target_info: TargetInfo,
    /// Whether waiting for debugger.
    pub waiting_for_debugger: bool,
}

/// Parameters for Target.setDiscoverTargets.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDiscoverTargetsParams {
    /// Whether to discover targets.
    pub discover: bool,
}
