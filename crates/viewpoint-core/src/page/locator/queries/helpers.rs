//! Helper functions for query operations.

use serde::Deserialize;
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};

use super::super::Locator;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Helper to call a function on a backend node ID and return the result.
    pub(super) async fn call_function_on_backend_id(
        &self,
        backend_node_id: BackendNodeId,
        function_declaration: &str,
    ) -> Result<serde_json::Value, LocatorError> {
        self.call_function_on_backend_id_with_fn(backend_node_id, function_declaration)
            .await
    }

    /// Helper to call a custom function on a backend node ID.
    pub(super) async fn call_function_on_backend_id_with_fn(
        &self,
        backend_node_id: BackendNodeId,
        function_declaration: &str,
    ) -> Result<serde_json::Value, LocatorError> {
        // Resolve the backend node ID to a RemoteObject
        let result: ResolveNodeResult = self
            .page
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-query".to_string()),
                    execution_context_id: None,
                }),
                Some(self.page.session_id()),
            )
            .await
            .map_err(|_| {
                LocatorError::NotFound(format!(
                    "Could not resolve backend node ID {backend_node_id}: element may no longer exist"
                ))
            })?;

        let object_id = result.object.object_id.ok_or_else(|| {
            LocatorError::NotFound(format!(
                "No object ID for backend node ID {backend_node_id}"
            ))
        })?;

        // Call the function on the resolved element
        #[derive(Debug, Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
            #[serde(rename = "exceptionDetails")]
            exception_details: Option<viewpoint_cdp::protocol::runtime::ExceptionDetails>,
        }

        let call_result: CallResult = self
            .page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": function_declaration,
                    "returnByValue": true
                })),
                Some(self.page.session_id()),
            )
            .await?;

        // Release the object
        let _ = self
            .page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Runtime.releaseObject",
                Some(serde_json::json!({ "objectId": object_id })),
                Some(self.page.session_id()),
            )
            .await;

        if let Some(exception) = call_result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        call_result
            .result
            .value
            .ok_or_else(|| LocatorError::EvaluationError("No result from query".to_string()))
    }
}
