//! Debug and visualization methods for locators.
//!
//! Methods for highlighting and debugging element selections.

use std::time::Duration;

use serde::Deserialize;
use tracing::{debug, instrument};
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_js::js;

use super::Locator;
use super::Selector;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Highlight the element for debugging purposes.
    ///
    /// This visually highlights the element with a magenta outline for 2 seconds,
    /// making it easy to verify which element is being targeted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Highlight for default duration (2 seconds)
    /// page.locator("button").highlight().await?;
    ///
    /// // Highlight for custom duration
    /// page.locator("button").highlight_for(Duration::from_secs(5)).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or highlighted.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn highlight(&self) -> Result<(), LocatorError> {
        self.highlight_for(Duration::from_secs(2)).await
    }

    /// Highlight the element for a specific duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - How long to show the highlight.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or highlighted.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn highlight_for(&self, duration: Duration) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(?duration, "Highlighting element");

        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self
                .highlight_by_backend_id(backend_node_id, duration)
                .await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self
                .highlight_by_backend_id(*backend_node_id, duration)
                .await;
        }

        // Add highlight style
        let selector_expr = self.selector.to_js_expression();
        let highlight_js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false };

                const el = elements[0];
                const originalOutline = el.style.outline;
                const originalOutlineOffset = el.style.outlineOffset;
                const originalTransition = el.style.transition;

                // Apply highlight with animation
                el.style.transition = "outline 0.2s ease-in-out";
                el.style.outline = "3px solid #ff00ff";
                el.style.outlineOffset = "2px";

                // Store original styles for restoration
                el.__viewpoint_original_outline = originalOutline;
                el.__viewpoint_original_outline_offset = originalOutlineOffset;
                el.__viewpoint_original_transition = originalTransition;

                return { found: true };
            })()
        };

        let result = self.evaluate_js(&highlight_js).await?;
        let found = result
            .get("found")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        // Wait for the duration
        tokio::time::sleep(duration).await;

        // Remove highlight
        let cleanup_js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return;

                const el = elements[0];
                el.style.outline = el.__viewpoint_original_outline || "";
                el.style.outlineOffset = el.__viewpoint_original_outline_offset || "";
                el.style.transition = el.__viewpoint_original_transition || "";

                delete el.__viewpoint_original_outline;
                delete el.__viewpoint_original_outline_offset;
                delete el.__viewpoint_original_transition;
            })()
        };

        // Ignore cleanup errors - element may have been removed
        let _ = self.evaluate_js(&cleanup_js).await;

        Ok(())
    }

    /// Highlight an element by backend node ID.
    async fn highlight_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
        duration: Duration,
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
                    object_group: Some("viewpoint-highlight".to_string()),
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

        // Apply highlight
        #[derive(Debug, Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
            #[serde(rename = "exceptionDetails")]
            exception_details: Option<viewpoint_cdp::protocol::runtime::ExceptionDetails>,
        }

        let js_highlight = js! {
            (function() {
                const el = this;
                const originalOutline = el.style.outline;
                const originalOutlineOffset = el.style.outlineOffset;
                const originalTransition = el.style.transition;

                // Apply highlight with animation
                el.style.transition = "outline 0.2s ease-in-out";
                el.style.outline = "3px solid #ff00ff";
                el.style.outlineOffset = "2px";

                // Store original styles for restoration
                el.__viewpoint_original_outline = originalOutline;
                el.__viewpoint_original_outline_offset = originalOutlineOffset;
                el.__viewpoint_original_transition = originalTransition;

                return { found: true };
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_highlight = js_highlight.trim_start_matches('(').trim_end_matches(')');

        let call_result: CallResult = self
            .page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": js_highlight,
                    "returnByValue": true
                })),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = call_result.exception_details {
            let _ = self
                .page
                .connection()
                .send_command::<_, serde_json::Value>(
                    "Runtime.releaseObject",
                    Some(serde_json::json!({ "objectId": object_id })),
                    Some(self.page.session_id()),
                )
                .await;
            return Err(LocatorError::EvaluationError(exception.text));
        }

        // Wait for the duration
        tokio::time::sleep(duration).await;

        // Remove highlight
        let js_remove_highlight = js! {
            (function() {
                const el = this;
                el.style.outline = el.__viewpoint_original_outline || "";
                el.style.outlineOffset = el.__viewpoint_original_outline_offset || "";
                el.style.transition = el.__viewpoint_original_transition || "";

                delete el.__viewpoint_original_outline;
                delete el.__viewpoint_original_outline_offset;
                delete el.__viewpoint_original_transition;
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_remove_highlight = js_remove_highlight
            .trim_start_matches('(')
            .trim_end_matches(')');

        let _ = self
            .page
            .connection()
            .send_command::<_, CallResult>(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": js_remove_highlight,
                    "returnByValue": true
                })),
                Some(self.page.session_id()),
            )
            .await;

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

        Ok(())
    }
}
