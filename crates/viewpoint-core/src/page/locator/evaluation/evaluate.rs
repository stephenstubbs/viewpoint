//! Single element evaluation methods.

use serde::Deserialize;
use tracing::{debug, instrument};
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_js::js;

use super::super::Locator;
use super::super::Selector;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Evaluate a JavaScript expression with the element as the first argument.
    ///
    /// The element is passed as `element` to the expression. The expression
    /// should be a function body or expression that uses `element`.
    ///
    /// # Arguments
    ///
    /// * `expression` - JavaScript expression. The element is available as `element`.
    ///
    /// # Returns
    ///
    /// The result of the JavaScript expression, or an error if evaluation fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Get the element's computed style
    /// let color = page.locator("button")
    ///     .evaluate::<String>("getComputedStyle(element).color")
    ///     .await?;
    ///
    /// // Get element dimensions
    /// let rect = page.locator("button")
    ///     .evaluate::<serde_json::Value>("element.getBoundingClientRect()")
    ///     .await?;
    ///
    /// // Modify element state
    /// page.locator("input")
    ///     .evaluate::<()>("element.value = 'Hello'")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The element is not found
    /// - The JavaScript expression fails
    /// - The result cannot be deserialized to type `T`
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn evaluate<T: serde::de::DeserializeOwned>(
        &self,
        expression: &str,
    ) -> Result<T, LocatorError> {
        self.wait_for_actionable().await?;

        debug!(expression, "Evaluating expression on element");

        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self
                .evaluate_by_backend_id(backend_node_id, expression)
                .await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self
                .evaluate_by_backend_id(*backend_node_id, expression)
                .await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { __viewpoint_error: "Element not found" };

                const element = elements[0];
                try {
                    const result = (function(element) { return @{expression}; })(element);
                    return { __viewpoint_result: result };
                } catch (e) {
                    return { __viewpoint_error: e.toString() };
                }
            })()
        };

        let result = self.evaluate_js(&js).await?;

        if let Some(error) = result.get("__viewpoint_error").and_then(|v| v.as_str()) {
            return Err(LocatorError::EvaluationError(error.to_string()));
        }

        let value = result
            .get("__viewpoint_result")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        serde_json::from_value(value).map_err(|e| {
            LocatorError::EvaluationError(format!("Failed to deserialize result: {e}"))
        })
    }

    /// Evaluate a JavaScript expression on an element by backend node ID.
    pub(super) async fn evaluate_by_backend_id<T: serde::de::DeserializeOwned>(
        &self,
        backend_node_id: BackendNodeId,
        expression: &str,
    ) -> Result<T, LocatorError> {
        // Resolve the backend node ID to a RemoteObject
        let result: ResolveNodeResult = self
            .page
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-evaluate".to_string()),
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

        // Build function declaration for CDP callFunctionOn
        // Wrapping in parens makes it a valid expression for js! macro parsing
        let js_fn = js! {
            (function() {
                const element = this;
                try {
                    const result = (function(element) { return @{expression}; })(element);
                    return { __viewpoint_result: result };
                } catch (e) {
                    return { __viewpoint_error: e.toString() };
                }
            })
        };
        // Strip outer parentheses for CDP (it expects function declaration syntax)
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let call_result: CallResult = self
            .page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": js_fn,
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

        let value = call_result
            .result
            .value
            .ok_or_else(|| LocatorError::EvaluationError("No result from evaluate".to_string()))?;

        if let Some(error) = value.get("__viewpoint_error").and_then(|v| v.as_str()) {
            return Err(LocatorError::EvaluationError(error.to_string()));
        }

        let result_value = value
            .get("__viewpoint_result")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        serde_json::from_value(result_value).map_err(|e| {
            LocatorError::EvaluationError(format!("Failed to deserialize result: {e}"))
        })
    }
}
