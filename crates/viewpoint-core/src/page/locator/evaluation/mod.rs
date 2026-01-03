//! JavaScript evaluation methods for locators.
//!
//! Methods for evaluating JavaScript expressions on elements.

use serde::Deserialize;
use tracing::{debug, instrument};
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_cdp::protocol::runtime::EvaluateParams;

use super::Locator;
use super::Selector;
use super::element::{BoundingBox, ElementHandle};
use crate::error::LocatorError;

impl<'a> Locator<'a> {
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
            return self.evaluate_by_backend_id(backend_node_id, expression).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.evaluate_by_backend_id(*backend_node_id, expression).await;
        }

        let js = format!(
            r"(function() {{
                const elements = {selector};
                if (elements.length === 0) return {{ __viewpoint_error: 'Element not found' }};
                
                const element = elements[0];
                try {{
                    const result = (function(element) {{ return {expression}; }})(element);
                    return {{ __viewpoint_result: result }};
                }} catch (e) {{
                    return {{ __viewpoint_error: e.toString() }};
                }}
            }})()",
            selector = self.selector.to_js_expression(),
            expression = expression
        );

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
    async fn evaluate_by_backend_id<T: serde::de::DeserializeOwned>(
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

        let call_result: CallResult = self
            .page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": format!(r#"function() {{
                        const element = this;
                        try {{
                            const result = (function(element) {{ return {expression}; }})(element);
                            return {{ __viewpoint_result: result }};
                        }} catch (e) {{
                            return {{ __viewpoint_error: e.toString() }};
                        }}
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
            LocatorError::EvaluationError("No result from evaluate".to_string())
        })?;

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

    /// Evaluate a JavaScript expression on all matching elements.
    ///
    /// The elements are passed as `elements` (an array) to the expression.
    ///
    /// # Arguments
    ///
    /// * `expression` - JavaScript expression. The elements are available as `elements`.
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
    /// // Get all element IDs
    /// let ids = page.locator("button")
    ///     .evaluate_all::<Vec<String>>("elements.map(e => e.id)")
    ///     .await?;
    ///
    /// // Count visible elements
    /// let count = page.locator(".item")
    ///     .evaluate_all::<usize>("elements.filter(e => e.offsetParent !== null).length")
    ///     .await?;
    ///
    /// // Get custom data attributes
    /// let data = page.locator("[data-test]")
    ///     .evaluate_all::<Vec<String>>("elements.map(e => e.dataset.test)")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The JavaScript expression fails
    /// - The result cannot be deserialized to type `T`
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn evaluate_all<T: serde::de::DeserializeOwned>(
        &self,
        expression: &str,
    ) -> Result<T, LocatorError> {
        debug!(expression, "Evaluating expression on all elements");

        // Handle Ref selector - lookup in ref map and resolve via CDP
        // For Ref selectors, evaluate_all returns an array with a single element
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.evaluate_all_by_backend_id(backend_node_id, expression).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.evaluate_all_by_backend_id(*backend_node_id, expression).await;
        }

        let js = format!(
            r"(function() {{
                const elements = Array.from({selector});
                try {{
                    const result = (function(elements) {{ return {expression}; }})(elements);
                    return {{ __viewpoint_result: result }};
                }} catch (e) {{
                    return {{ __viewpoint_error: e.toString() }};
                }}
            }})()",
            selector = self.selector.to_js_expression(),
            expression = expression
        );

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

    /// Evaluate a JavaScript expression on all elements by backend node ID.
    /// Since a backend node ID refers to a single element, this wraps it in an array.
    async fn evaluate_all_by_backend_id<T: serde::de::DeserializeOwned>(
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
                    object_group: Some("viewpoint-evaluate-all".to_string()),
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

        // Call the function on the resolved element, wrapping it in an array
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
                        const elements = [this];
                        try {{
                            const result = (function(elements) {{ return {expression}; }})(elements);
                            return {{ __viewpoint_result: result }};
                        }} catch (e) {{
                            return {{ __viewpoint_error: e.toString() }};
                        }}
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
            LocatorError::EvaluationError("No result from evaluate_all".to_string())
        })?;

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
        let js = format!(
            r"(function() {{
                const elements = {selector};
                if (elements.length === 0) return null;
                return elements[0];
            }})()",
            selector = self.selector.to_js_expression()
        );

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
    async fn element_handle_by_backend_id(
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

        let js = format!(
            r"(function() {{
                const elements = {selector};
                if (elements.length === 0) return {{ found: false }};
                
                const el = elements[0];
                el.scrollIntoView({{ behavior: 'instant', block: 'center', inline: 'center' }});
                return {{ found: true }};
            }})()",
            selector = self.selector.to_js_expression()
        );

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
    async fn scroll_into_view_by_backend_id(
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

        let call_result: CallResult = self
            .page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": r#"function() {
                        this.scrollIntoView({ behavior: 'instant', block: 'center', inline: 'center' });
                        return { found: true };
                    }"#,
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

    /// Get the bounding box of the element.
    ///
    /// Returns the element's position and dimensions relative to the viewport.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let bbox = page.locator("button").bounding_box().await?;
    /// if let Some(box_) = bbox {
    ///     println!("Element at ({}, {}), size {}x{}",
    ///         box_.x, box_.y, box_.width, box_.height);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// - `Some(BoundingBox)` if the element exists and is visible
    /// - `None` if the element exists but has no visible bounding box
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found.
    pub async fn bounding_box(&self) -> Result<Option<BoundingBox>, LocatorError> {
        let info = self.query_element_info().await?;

        if !info.found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        match (info.x, info.y, info.width, info.height) {
            (Some(x), Some(y), Some(width), Some(height)) if width > 0.0 && height > 0.0 => {
                Ok(Some(BoundingBox {
                    x,
                    y,
                    width,
                    height,
                }))
            }
            _ => Ok(None),
        }
    }
}
