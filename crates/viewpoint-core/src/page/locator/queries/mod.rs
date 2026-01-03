//! Query methods for locators.
//!
//! These methods query element state and properties without performing actions.

use serde::Deserialize;
use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_js::js;

use super::Locator;
use super::Selector;
use super::selector::js_string_literal;
use crate::error::LocatorError;

impl<'a> Locator<'a> {
    /// Get the text content of the first matching element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn text_content(&self) -> Result<Option<String>, LocatorError> {
        let info = self.query_element_info().await?;
        Ok(info.text)
    }

    /// Check if the element is visible.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn is_visible(&self) -> Result<bool, LocatorError> {
        let info = self.query_element_info().await?;
        Ok(info.visible.unwrap_or(false))
    }

    /// Check if the element is checked (for checkboxes/radios).
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn is_checked(&self) -> Result<bool, LocatorError> {
        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.is_checked_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.is_checked_by_backend_id(*backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false, checked: false };
                const el = elements[0];
                return { found: true, checked: el.checked || false };
            })()
        };

        let result = self.evaluate_js(&js_code).await?;
        let checked: bool = result
            .get("checked")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        Ok(checked)
    }

    /// Check if element is checked by backend node ID.
    async fn is_checked_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<bool, LocatorError> {
        let result = self.call_function_on_backend_id(
            backend_node_id,
            r#"function() {
                return { checked: this.checked || false };
            }"#,
        ).await?;

        Ok(result
            .get("checked")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false))
    }

    /// Count matching elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the elements cannot be queried.
    pub async fn count(&self) -> Result<usize, LocatorError> {
        let info = self.query_element_info().await?;
        Ok(info.count)
    }

    /// Return all matching elements as individual locators.
    ///
    /// Each returned locator points to a single element (via nth index).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let items = page.locator("li").all().await?;
    /// for item in items {
    ///     println!("{}", item.text_content().await?.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the elements cannot be queried.
    pub async fn all(&self) -> Result<Vec<Locator<'a>>, LocatorError> {
        let count = self.count().await?;
        let mut locators = Vec::with_capacity(count);
        for i in 0..count {
            locators.push(self.nth(i as i32));
        }
        Ok(locators)
    }

    /// Get the inner text of all matching elements.
    ///
    /// Returns the `innerText` property for each element, which is the rendered
    /// text content as it appears on screen (respects CSS styling like `display: none`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let texts = page.locator("li").all_inner_texts().await?;
    /// assert_eq!(texts, vec!["Item 1", "Item 2", "Item 3"]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the elements cannot be queried.
    pub async fn all_inner_texts(&self) -> Result<Vec<String>, LocatorError> {
        // Handle Ref selector - returns single element's inner text as array
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.all_inner_texts_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.all_inner_texts_by_backend_id(*backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = Array.from(@{selector_expr});
                return elements.map(el => el.innerText || "");
            })()
        };

        let result = self.evaluate_js(&js_code).await?;

        result
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .ok_or_else(|| LocatorError::EvaluationError("Expected array result".to_string()))
    }

    /// Get all inner texts by backend node ID (returns single element as array).
    async fn all_inner_texts_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<Vec<String>, LocatorError> {
        let result = self.call_function_on_backend_id(
            backend_node_id,
            r#"function() {
                return [this.innerText || ""];
            }"#,
        ).await?;

        result
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .ok_or_else(|| LocatorError::EvaluationError("Expected array result".to_string()))
    }

    /// Get the text content of all matching elements.
    ///
    /// Returns the `textContent` property for each element, which includes all
    /// text including hidden elements.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let texts = page.locator("li").all_text_contents().await?;
    /// assert_eq!(texts, vec!["Item 1", "Item 2", "Item 3"]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the elements cannot be queried.
    pub async fn all_text_contents(&self) -> Result<Vec<String>, LocatorError> {
        // Handle Ref selector - returns single element's text content as array
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.all_text_contents_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.all_text_contents_by_backend_id(*backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = Array.from(@{selector_expr});
                return elements.map(el => el.textContent || "");
            })()
        };

        let result = self.evaluate_js(&js_code).await?;

        result
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .ok_or_else(|| LocatorError::EvaluationError("Expected array result".to_string()))
    }

    /// Get all text contents by backend node ID (returns single element as array).
    async fn all_text_contents_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<Vec<String>, LocatorError> {
        let result = self.call_function_on_backend_id(
            backend_node_id,
            r#"function() {
                return [this.textContent || ""];
            }"#,
        ).await?;

        result
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .ok_or_else(|| LocatorError::EvaluationError("Expected array result".to_string()))
    }

    /// Get the inner text of the first matching element.
    ///
    /// Returns the `innerText` property, which is the rendered text content.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn inner_text(&self) -> Result<String, LocatorError> {
        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.inner_text_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.inner_text_by_backend_id(*backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false };
                return { found: true, text: elements[0].innerText || "" };
            })()
        };

        let result = self.evaluate_js(&js_code).await?;

        let found = result
            .get("found")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        Ok(result
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    /// Get inner text by backend node ID.
    async fn inner_text_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<String, LocatorError> {
        let result = self.call_function_on_backend_id(
            backend_node_id,
            r#"function() {
                return { text: this.innerText || "" };
            }"#,
        ).await?;

        Ok(result
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    /// Get an attribute value from the first matching element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn get_attribute(&self, name: &str) -> Result<Option<String>, LocatorError> {
        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.get_attribute_by_backend_id(backend_node_id, name).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.get_attribute_by_backend_id(*backend_node_id, name).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false };
                const attr = elements[0].getAttribute(#{name});
                return { found: true, value: attr };
            })()
        };

        let result = self.evaluate_js(&js_code).await?;

        let found = result
            .get("found")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        Ok(result
            .get("value")
            .and_then(|v| if v.is_null() { None } else { v.as_str() })
            .map(std::string::ToString::to_string))
    }

    /// Get attribute by backend node ID.
    async fn get_attribute_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
        name: &str,
    ) -> Result<Option<String>, LocatorError> {
        let name_escaped = js_string_literal(name);
        let result = self.call_function_on_backend_id_with_fn(
            backend_node_id,
            &format!(r#"function() {{
                const attr = this.getAttribute({name_escaped});
                return {{ value: attr }};
            }}"#),
        ).await?;

        Ok(result
            .get("value")
            .and_then(|v| if v.is_null() { None } else { v.as_str() })
            .map(std::string::ToString::to_string))
    }

    /// Get the input value of a form element.
    ///
    /// Works for `<input>`, `<textarea>`, and `<select>` elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn input_value(&self) -> Result<String, LocatorError> {
        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self.input_value_by_backend_id(backend_node_id).await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self.input_value_by_backend_id(*backend_node_id).await;
        }

        let selector_expr = self.selector.to_js_expression();
        let js_code = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false };
                const el = elements[0];
                return { found: true, value: el.value || "" };
            })()
        };

        let result = self.evaluate_js(&js_code).await?;

        let found = result
            .get("found")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        Ok(result
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    /// Get input value by backend node ID.
    async fn input_value_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<String, LocatorError> {
        let result = self.call_function_on_backend_id(
            backend_node_id,
            r#"function() {
                return { value: this.value || "" };
            }"#,
        ).await?;

        Ok(result
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }

    /// Helper to call a function on a backend node ID and return the result.
    async fn call_function_on_backend_id(
        &self,
        backend_node_id: BackendNodeId,
        function_declaration: &str,
    ) -> Result<serde_json::Value, LocatorError> {
        self.call_function_on_backend_id_with_fn(backend_node_id, function_declaration).await
    }

    /// Helper to call a custom function on a backend node ID.
    async fn call_function_on_backend_id_with_fn(
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

        call_result.result.value.ok_or_else(|| {
            LocatorError::EvaluationError("No result from query".to_string())
        })
    }
}
