//! Internal helper methods for Locator.
//!
//! These methods are used internally by action methods and builders.

use std::time::Duration;

use serde::Deserialize;
use viewpoint_cdp::protocol::dom::{ResolveNodeParams, ResolveNodeResult};
use viewpoint_cdp::protocol::input::{
    DispatchKeyEventParams, DispatchMouseEventParams, InsertTextParams,
};
use viewpoint_js::js;

use super::{Locator, Selector};
use crate::error::LocatorError;

/// Result of querying element information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ElementInfo {
    /// Whether the element exists.
    pub(super) found: bool,
    /// Number of matching elements.
    pub(super) count: usize,
    /// Whether the element is visible.
    pub(super) visible: Option<bool>,
    /// Whether the element is enabled.
    pub(super) enabled: Option<bool>,
    /// Bounding box of the element.
    pub(super) x: Option<f64>,
    pub(super) y: Option<f64>,
    pub(super) width: Option<f64>,
    pub(super) height: Option<f64>,
    /// Text content of the element.
    pub(super) text: Option<String>,
    /// Element tag name.
    pub(super) tag_name: Option<String>,
}

impl Locator<'_> {
    /// Wait for element to be actionable (visible, enabled, stable).
    pub(super) async fn wait_for_actionable(&self) -> Result<ElementInfo, LocatorError> {
        let start = std::time::Instant::now();
        let timeout = self.options.timeout;

        loop {
            let info = self.query_element_info().await?;

            if !info.found {
                if start.elapsed() >= timeout {
                    return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }

            if !info.visible.unwrap_or(false) {
                if start.elapsed() >= timeout {
                    return Err(LocatorError::NotVisible);
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }

            // Element is visible, return it
            return Ok(info);
        }
    }

    /// Query element information via JavaScript.
    pub(super) async fn query_element_info(&self) -> Result<ElementInfo, LocatorError> {
        // Handle BackendNodeId selector specially - resolve via CDP
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self
                .query_element_info_by_backend_id(*backend_node_id)
                .await;
        }

        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.query_element_info_by_backend_id(backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = Array.from(@{selector_expr});
                if (elements.length === 0) {
                    return { found: false, count: 0 };
                }
                const el = elements[0];
                const rect = el.getBoundingClientRect();
                const style = window.getComputedStyle(el);
                const visible = rect.width > 0 && rect.height > 0 &&
                    style.visibility !== "hidden" &&
                    style.display !== "none" &&
                    parseFloat(style.opacity) > 0;
                return {
                    found: true,
                    count: elements.length,
                    visible: visible,
                    enabled: !el.disabled,
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rect.height,
                    text: el.textContent,
                    tagName: el.tagName.toLowerCase()
                };
            })()
        };

        let result = self.evaluate_js(&js_code).await?;
        let info: ElementInfo = serde_json::from_value(result)
            .map_err(|e| LocatorError::EvaluationError(e.to_string()))?;
        Ok(info)
    }

    /// Query element information for a BackendNodeId selector.
    async fn query_element_info_by_backend_id(
        &self,
        backend_node_id: viewpoint_cdp::protocol::dom::BackendNodeId,
    ) -> Result<ElementInfo, LocatorError> {
        // Resolve the backend node ID to a RemoteObject
        let result: ResolveNodeResult = self
            .page
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-locator".to_string()),
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

        // Call a function on the resolved element to get its info
        #[derive(Debug, Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
            #[serde(rename = "exceptionDetails")]
            exception_details: Option<viewpoint_cdp::protocol::runtime::ExceptionDetails>,
        }

        let js_fn = js! {
            (function() {
                const el = this;
                const rect = el.getBoundingClientRect();
                const style = window.getComputedStyle(el);
                const visible = rect.width > 0 && rect.height > 0 &&
                    style.visibility !== "hidden" &&
                    style.display !== "none" &&
                    parseFloat(style.opacity) > 0;
                return {
                    found: true,
                    count: 1,
                    visible: visible,
                    enabled: !el.disabled,
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rect.height,
                    text: el.textContent,
                    tagName: el.tagName.toLowerCase()
                };
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result: CallResult = self
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

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        let value = result.result.value.ok_or_else(|| {
            LocatorError::EvaluationError("No result from element info query".to_string())
        })?;

        let info: ElementInfo = serde_json::from_value(value)
            .map_err(|e| LocatorError::EvaluationError(e.to_string()))?;
        Ok(info)
    }

    /// Focus the element via JavaScript.
    pub(super) async fn focus_element(&self) -> Result<(), LocatorError> {
        // Handle BackendNodeId selector specially
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.focus_element_by_backend_id(*backend_node_id).await;
        }

        // Handle Ref selector - lookup in ref map and focus via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.focus_element_by_backend_id(backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length > 0) {
                    elements[0].focus();
                    return true;
                }
                return false;
            })()
        };

        self.evaluate_js(&js_code).await?;
        Ok(())
    }

    /// Focus an element by its backend node ID.
    async fn focus_element_by_backend_id(
        &self,
        backend_node_id: viewpoint_cdp::protocol::dom::BackendNodeId,
    ) -> Result<(), LocatorError> {
        // Use CDP DOM.focus instead of JavaScript for backend node IDs
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "DOM.focus",
                Some(serde_json::json!({
                    "backendNodeId": backend_node_id
                })),
                Some(self.page.session_id()),
            )
            .await?;

        Ok(())
    }

    /// Evaluate JavaScript and return the result.
    ///
    /// Delegates to `Page::evaluate_js_raw` for the actual evaluation.
    pub(super) async fn evaluate_js(
        &self,
        expression: &str,
    ) -> Result<serde_json::Value, LocatorError> {
        if self.page.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        self.page
            .evaluate_js_raw(expression)
            .await
            .map_err(|e| LocatorError::EvaluationError(e.to_string()))
    }

    /// Dispatch a mouse event.
    pub(super) async fn dispatch_mouse_event(
        &self,
        params: DispatchMouseEventParams,
    ) -> Result<(), LocatorError> {
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Input.dispatchMouseEvent",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;
        Ok(())
    }

    /// Dispatch a key event.
    pub(super) async fn dispatch_key_event(
        &self,
        params: DispatchKeyEventParams,
    ) -> Result<(), LocatorError> {
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Input.dispatchKeyEvent",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;
        Ok(())
    }

    /// Insert text directly.
    pub(super) async fn insert_text(&self, text: &str) -> Result<(), LocatorError> {
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Input.insertText",
                Some(InsertTextParams {
                    text: text.to_string(),
                }),
                Some(self.page.session_id()),
            )
            .await?;
        Ok(())
    }
}
