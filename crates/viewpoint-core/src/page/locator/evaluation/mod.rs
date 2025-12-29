//! JavaScript evaluation methods for locators.
//!
//! Methods for evaluating JavaScript expressions on elements.

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::runtime::EvaluateParams;

use super::Locator;
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
