//! Attribute and value query methods for locators.

use viewpoint_cdp::protocol::dom::BackendNodeId;
use viewpoint_js::js;

use super::super::Locator;
use super::super::Selector;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Get an attribute value from the first matching element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn get_attribute(&self, name: &str) -> Result<Option<String>, LocatorError> {
        // Handle Ref selector - lookup in ref map and resolve via CDP
        if let Selector::Ref(ref_str) = &self.selector {
            let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
            return self
                .get_attribute_by_backend_id(backend_node_id, name)
                .await;
        }

        // Handle BackendNodeId selector
        if let Selector::BackendNodeId(backend_node_id) = &self.selector {
            return self
                .get_attribute_by_backend_id(*backend_node_id, name)
                .await;
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
    pub(super) async fn get_attribute_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
        name: &str,
    ) -> Result<Option<String>, LocatorError> {
        // Build function declaration for CDP callFunctionOn
        // Wrapping in parens makes it a valid expression for js! macro parsing
        let js_fn = js! {
            (function() {
                const attr = this.getAttribute(#{name});
                return { value: attr };
            })
        };
        // Strip outer parentheses for CDP (it expects function declaration syntax)
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');
        let result = self
            .call_function_on_backend_id_with_fn(backend_node_id, js_fn)
            .await?;

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
    pub(super) async fn input_value_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<String, LocatorError> {
        let js_fn = js! {
            (function() {
                return { value: this.value || "" };
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result = self
            .call_function_on_backend_id(backend_node_id, js_fn)
            .await?;

        Ok(result
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string())
    }
}
