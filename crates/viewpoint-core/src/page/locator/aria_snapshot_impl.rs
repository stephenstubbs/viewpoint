//! ARIA snapshot implementation for Locator.

use serde::Deserialize;
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_js::js;

use super::aria::{aria_snapshot_js, AriaSnapshot};
use super::Locator;
use super::Selector;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Get an ARIA accessibility snapshot of this element.
    ///
    /// The snapshot captures the accessible tree structure as it would be
    /// exposed to assistive technologies. This is useful for accessibility testing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.locator("form").aria_snapshot().await?;
    /// println!("{}", snapshot); // YAML-like output
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the element is not found or snapshot capture fails.
    pub async fn aria_snapshot(&self) -> Result<AriaSnapshot, LocatorError> {
        if self.page.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.aria_snapshot_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.aria_snapshot_by_backend_id(*backend_node_id).await;
        }

        // Get the element and evaluate ARIA snapshot
        // Note: to_js_expression() returns code that evaluates to NodeList/array,
        // so we need to get the first element from it
        let js_selector = self.selector.to_js_expression();
        let snapshot_fn = aria_snapshot_js();
        let js_code = js! {
            (function() {
                const elements = @{js_selector};
                const element = elements && elements[0] ? elements[0] : elements;
                if (!element) {
                    return { error: "Element not found" };
                }
                const getSnapshot = @{snapshot_fn};
                return getSnapshot(element);
            })()
        };

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .page
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js_code,
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        let value = result.result.value.ok_or_else(|| {
            LocatorError::EvaluationError("No result from aria snapshot".to_string())
        })?;

        // Check for error
        if let Some(error) = value.get("error").and_then(|e| e.as_str()) {
            return Err(LocatorError::NotFound(error.to_string()));
        }

        // Parse the snapshot
        let snapshot: AriaSnapshot = serde_json::from_value(value).map_err(|e| {
            LocatorError::EvaluationError(format!("Failed to parse aria snapshot: {e}"))
        })?;

        Ok(snapshot)
    }

    /// Get ARIA snapshot for an element by backend node ID.
    async fn aria_snapshot_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<AriaSnapshot, LocatorError> {
        // Resolve the backend node ID to a RemoteObject
        let result: ResolveNodeResult = self
            .page
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-aria-snapshot".to_string()),
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

        // Call aria snapshot function on the resolved element
        let snapshot_fn = aria_snapshot_js();

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
                    "functionDeclaration": format!(r#"function() {{
                        const element = this;
                        const getSnapshot = {snapshot_fn};
                        return getSnapshot(element);
                    }}"#),
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

        let value = call_result.result.value.ok_or_else(|| {
            LocatorError::EvaluationError("No result from aria snapshot".to_string())
        })?;

        // Check for error
        if let Some(error) = value.get("error").and_then(|e| e.as_str()) {
            return Err(LocatorError::NotFound(error.to_string()));
        }

        // Parse the snapshot
        let snapshot: AriaSnapshot = serde_json::from_value(value).map_err(|e| {
            LocatorError::EvaluationError(format!("Failed to parse aria snapshot: {e}"))
        })?;

        Ok(snapshot)
    }
}
