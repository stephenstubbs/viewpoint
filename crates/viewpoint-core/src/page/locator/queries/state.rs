//! Basic state query methods for locators.

use viewpoint_cdp::protocol::dom::BackendNodeId;
use viewpoint_js::js;

use super::super::Locator;
use super::super::Selector;
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
    pub(super) async fn is_checked_by_backend_id(
        &self,
        backend_node_id: BackendNodeId,
    ) -> Result<bool, LocatorError> {
        let js_fn = js! {
            (function() {
                return { checked: this.checked || false };
            })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let js_fn = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result = self
            .call_function_on_backend_id(backend_node_id, js_fn)
            .await?;

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
}
