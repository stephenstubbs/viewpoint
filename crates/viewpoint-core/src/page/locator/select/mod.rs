//! Select option methods for Locator.
//!
//! This module contains methods for selecting options in `<select>` elements.

use super::Locator;
use super::builders::SelectOptionBuilder;
use super::selector::js_string_literal;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Select an option in a `<select>` element by value, label, or index.
    ///
    /// Returns a builder that can be configured with additional options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Select by value
    /// page.locator("select#size").select_option().value("medium").await?;
    ///
    /// // Select by visible text (label)
    /// page.locator("select#size").select_option().label("Medium Size").await?;
    ///
    /// // Select multiple options
    /// page.locator("select#colors").select_option().values(&["red", "blue"]).await?;
    ///
    /// // Select without waiting for navigation
    /// page.locator("select#nav").select_option().value("page2").no_wait_after(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn select_option(&self) -> SelectOptionBuilder<'_, '_> {
        SelectOptionBuilder::new(self)
    }

    /// Internal method to select a single option (used by builder).
    pub(crate) async fn select_option_internal(&self, option: &str) -> Result<(), LocatorError> {
        let js = build_select_option_js(&self.selector.to_js_expression(), option);
        let result = self.evaluate_js(&js).await?;
        check_select_result(&result)?;
        Ok(())
    }

    /// Internal method to select multiple options (used by builder).
    pub(crate) async fn select_options_internal(
        &self,
        options: &[&str],
    ) -> Result<(), LocatorError> {
        let js = build_select_options_js(&self.selector.to_js_expression(), options);
        let result = self.evaluate_js(&js).await?;
        check_select_result(&result)?;
        Ok(())
    }
}

/// Build JavaScript for selecting a single option.
fn build_select_option_js(selector_expr: &str, option: &str) -> String {
    format!(
        r"(function() {{
            const elements = {selector};
            if (elements.length === 0) return {{ success: false, error: 'Element not found' }};
            
            const select = elements[0];
            if (select.tagName.toLowerCase() !== 'select') {{
                return {{ success: false, error: 'Element is not a select' }};
            }}
            
            const optionValue = {option};
            
            // Try to find by value first
            for (let i = 0; i < select.options.length; i++) {{
                if (select.options[i].value === optionValue) {{
                    select.selectedIndex = i;
                    select.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    return {{ success: true, selectedIndex: i, selectedValue: select.options[i].value }};
                }}
            }}
            
            // Try to find by text content
            for (let i = 0; i < select.options.length; i++) {{
                if (select.options[i].text === optionValue || 
                    select.options[i].textContent.trim() === optionValue) {{
                    select.selectedIndex = i;
                    select.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    return {{ success: true, selectedIndex: i, selectedValue: select.options[i].value }};
                }}
            }}
            
            return {{ success: false, error: 'Option not found: ' + optionValue }};
        }})()",
        selector = selector_expr,
        option = js_string_literal(option)
    )
}

/// Build JavaScript for selecting multiple options.
fn build_select_options_js(selector_expr: &str, options: &[&str]) -> String {
    let options_js: Vec<String> = options.iter().map(|o| js_string_literal(o)).collect();
    let options_array = format!("[{}]", options_js.join(", "));

    format!(
        r"(function() {{
            const elements = {selector_expr};
            if (elements.length === 0) return {{ success: false, error: 'Element not found' }};
            
            const select = elements[0];
            if (select.tagName.toLowerCase() !== 'select') {{
                return {{ success: false, error: 'Element is not a select' }};
            }}
            
            const optionValues = {options_array};
            const selectedIndices = [];
            
            // Clear current selection if not multiple
            if (!select.multiple) {{
                return {{ success: false, error: 'select_options requires a <select multiple>' }};
            }}
            
            // Deselect all first
            for (let i = 0; i < select.options.length; i++) {{
                select.options[i].selected = false;
            }}
            
            // Select each requested option
            for (const optionValue of optionValues) {{
                let found = false;
                
                // Try to find by value
                for (let i = 0; i < select.options.length; i++) {{
                    if (select.options[i].value === optionValue) {{
                        select.options[i].selected = true;
                        selectedIndices.push(i);
                        found = true;
                        break;
                    }}
                }}
                
                // Try to find by text if not found by value
                if (!found) {{
                    for (let i = 0; i < select.options.length; i++) {{
                        if (select.options[i].text === optionValue || 
                            select.options[i].textContent.trim() === optionValue) {{
                            select.options[i].selected = true;
                            selectedIndices.push(i);
                            found = true;
                            break;
                        }}
                    }}
                }}
                
                if (!found) {{
                    return {{ success: false, error: 'Option not found: ' + optionValue }};
                }}
            }}
            
            select.dispatchEvent(new Event('change', {{ bubbles: true }}));
            return {{ success: true, selectedIndices: selectedIndices }};
        }})()"
    )
}

/// Check the result of a select operation.
fn check_select_result(result: &serde_json::Value) -> Result<(), LocatorError> {
    let success = result
        .get("success")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);

    if !success {
        let error = result
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error");
        return Err(LocatorError::EvaluationError(error.to_string()));
    }

    Ok(())
}
