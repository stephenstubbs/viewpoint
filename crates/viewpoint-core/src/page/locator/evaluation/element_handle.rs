//! Element handle retrieval methods.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_cdp::protocol::runtime::EvaluateParams;
use viewpoint_js::js;

use super::super::Locator;
use super::super::Selector;
use super::super::element::ElementHandle;
use crate::error::LocatorError;

impl<'a> Locator<'a> {
    /// Get a raw element handle for the first matching element.
    ///
    /// The returned [`ElementHandle`] provides lower-level access to the DOM element
    /// and can be used for advanced operations that aren't covered by the Locator API.
    ///
    /// **Note:** Unlike locators, element handles are bound to the specific element
    /// at the time of creation. If the element is removed from the DOM, the handle
    /// becomes stale.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let handle = page.locator("button").element_handle().await?;
    /// let box_model = handle.box_model().await?;
    /// println!("Element at: {:?}", box_model);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn element_handle(&self) -> Result<ElementHandle<'a>, LocatorError> {
        self.wait_for_actionable().await?;

        debug!("Getting element handle");

        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.element_handle_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.element_handle_by_backend_id(*backend_node_id).await;
        }

        // Use Runtime.evaluate to get the element object ID
        let selector_expr = self.selector.to_js_expression();
        let js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return null;
                return elements[0];
            })()
        };

        let params = EvaluateParams {
            expression: js,
            object_group: Some("viewpoint-element-handle".to_string()),
            include_command_line_api: None,
            silent: Some(true),
            context_id: None,
            return_by_value: Some(false),
            await_promise: Some(false),
        };

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .page
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        let object_id = result
            .result
            .object_id
            .ok_or_else(|| LocatorError::NotFound(format!("{:?}", self.selector)))?;

        Ok(ElementHandle {
            object_id,
            page: self.page,
        })
    }

    /// Get an element handle by backend node ID.
    pub(super) async fn element_handle_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<ElementHandle<'a>, LocatorError> {
        // Resolve the backend node ID to a RemoteObject
        let result: ResolveNodeResult = self
            .page
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-element-handle".to_string()),
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

        Ok(ElementHandle {
            object_id,
            page: self.page,
        })
    }
}
