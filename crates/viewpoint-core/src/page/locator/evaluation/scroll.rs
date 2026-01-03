//! Scroll into view methods.

use serde::Deserialize;
use tracing::{debug, instrument};
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_js::js;

use super::super::Locator;
use super::super::Selector;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Scroll the element into view if needed.
    ///
    /// This scrolls the element's parent container(s) to make the element visible.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.locator(".footer").scroll_into_view_if_needed().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn scroll_into_view_if_needed(&self) -> Result<(), LocatorError> {
        let _info = self.wait_for_actionable().await?;

        debug!("Scrolling element into view");

        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.scroll_into_view_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.scroll_into_view_by_backend_id(*backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false };

                const el = elements[0];
                el.scrollIntoView({ behavior: "instant", block: "center", inline: "center" });
                return { found: true };
            })()
        };

        let result = self.evaluate_js(&js).await?;
        let found = result
            .get("found")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        Ok(())
    }

    /// Scroll an element into view by backend node ID.
    pub(super) async fn scroll_into_view_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<(), LocatorError> {
        // Resolve the backend node ID to a RemoteObject
        let result: ResolveNodeResult = self
            .page
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-scroll".to_string()),
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

        // Call scrollIntoView on the resolved element
        #[derive(Debug, Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
            #[serde(rename = "exceptionDetails")]
            exception_details: Option<viewpoint_cdp::protocol::runtime::ExceptionDetails>,
        }

        let js_fn = js! {
            (function() {
                this.scrollIntoView({ behavior: "instant", block: "center", inline: "center" });
                return { found: true };
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
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

        Ok(())
    }
}
