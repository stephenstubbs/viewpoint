//! Text query methods for locators.

use viewpoint_cdp::protocol::dom::BackendNodeId;
use viewpoint_js::js;

use super::super::Locator;
use super::super::Selector;
use crate::error::LocatorError;

impl Locator<'_> {
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
    pub(super) async fn all_inner_texts_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<Vec<String>, LocatorError> {
        let js_fn = js! {
            (function() {
                return [this.innerText || ""];
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result = self
            .call_function_on_backend_id(backend_node_id, js_fn)
            .await?;

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
    pub(super) async fn all_text_contents_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<Vec<String>, LocatorError> {
        let js_fn = js! {
            (function() {
                return [this.textContent || ""];
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result = self
            .call_function_on_backend_id(backend_node_id, js_fn)
            .await?;

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
    pub(super) async fn inner_text_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<String, LocatorError> {
        let js_fn = js! {
            (function() {
                return { text: this.innerText || "" };
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result = self
            .call_function_on_backend_id(backend_node_id, js_fn)
            .await?;

        Ok(result
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }
}
