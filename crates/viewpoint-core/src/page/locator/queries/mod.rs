//! Query methods for locators.
//!
//! These methods query element state and properties without performing actions.

use viewpoint_js::js;

use super::Locator;
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

    /// Get the inner text of the first matching element.
    ///
    /// Returns the `innerText` property, which is the rendered text content.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn inner_text(&self) -> Result<String, LocatorError> {
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

    /// Get an attribute value from the first matching element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn get_attribute(&self, name: &str) -> Result<Option<String>, LocatorError> {
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

    /// Get the input value of a form element.
    ///
    /// Works for `<input>`, `<textarea>`, and `<select>` elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be queried.
    pub async fn input_value(&self) -> Result<String, LocatorError> {
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
}
