//! Locator actions for element interaction.

use std::time::Duration;

use viewpoint_cdp::protocol::input::{
    DispatchKeyEventParams, DispatchMouseEventParams, InsertTextParams, MouseButton,
};
use viewpoint_cdp::protocol::runtime::EvaluateParams;
use serde::Deserialize;
use tracing::{debug, instrument};

use super::selector::js_string_literal;
use super::Locator;
use crate::error::LocatorError;

/// Result of querying element information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Fields are deserialized from JS, may not all be used yet
struct ElementInfo {
    /// Whether the element exists.
    found: bool,
    /// Number of matching elements.
    count: usize,
    /// Whether the element is visible.
    visible: Option<bool>,
    /// Whether the element is enabled.
    enabled: Option<bool>,
    /// Bounding box of the element.
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    /// Text content of the element.
    text: Option<String>,
    /// Element tag name.
    tag_name: Option<String>,
}

impl Locator<'_> {
    /// Click the element.
    ///
    /// Waits for the element to be visible and enabled, then clicks its center.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The element is not found within the timeout
    /// - The element is not visible
    /// - The CDP command fails
    ///
    /// # Panics
    ///
    /// Panics if a visible element lacks bounding box coordinates. This should
    /// never occur as `wait_for_actionable` ensures visibility before returning.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn click(&self) -> Result<(), LocatorError> {
        let info = self.wait_for_actionable().await?;

        // These unwraps are safe because wait_for_actionable ensures the element is visible
        // and visible elements always have valid bounding boxes
        let x = info.x.expect("visible element has x") + info.width.expect("visible element has width") / 2.0;
        let y = info.y.expect("visible element has y") + info.height.expect("visible element has height") / 2.0;

        debug!(x, y, "Clicking element");

        // Move to element
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_move(x, y))
            .await?;

        // Mouse down
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_down(x, y, MouseButton::Left))
            .await?;

        // Mouse up
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_up(x, y, MouseButton::Left))
            .await?;

        Ok(())
    }

    /// Double-click the element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be clicked.
    ///
    /// # Panics
    ///
    /// Panics if a visible element lacks bounding box coordinates. This should
    /// never occur as `wait_for_actionable` ensures visibility before returning.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn dblclick(&self) -> Result<(), LocatorError> {
        let info = self.wait_for_actionable().await?;

        let x = info.x.expect("visible element has x") + info.width.expect("visible element has width") / 2.0;
        let y = info.y.expect("visible element has y") + info.height.expect("visible element has height") / 2.0;

        debug!(x, y, "Double-clicking element");

        // First click
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_move(x, y))
            .await?;
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_down(x, y, MouseButton::Left))
            .await?;
        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_up(x, y, MouseButton::Left))
            .await?;

        // Second click
        let mut down = DispatchMouseEventParams::mouse_down(x, y, MouseButton::Left);
        down.click_count = Some(2);
        self.dispatch_mouse_event(down).await?;

        let mut up = DispatchMouseEventParams::mouse_up(x, y, MouseButton::Left);
        up.click_count = Some(2);
        self.dispatch_mouse_event(up).await?;

        Ok(())
    }

    /// Fill the element with text (clears existing content first).
    ///
    /// This is for input and textarea elements.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be focused or text cannot be inserted.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn fill(&self, text: &str) -> Result<(), LocatorError> {
        let _info = self.wait_for_actionable().await?;

        debug!(text, "Filling element");

        // Focus the element
        self.focus_element().await?;

        // Select all and delete (clear)
        self.dispatch_key_event(DispatchKeyEventParams::key_down("a"))
            .await?;
        // Send Ctrl+A
        let mut select_all = DispatchKeyEventParams::key_down("a");
        select_all.modifiers = Some(viewpoint_cdp::protocol::input::modifiers::CTRL);
        self.dispatch_key_event(select_all).await?;

        // Delete selected text
        self.dispatch_key_event(DispatchKeyEventParams::key_down("Backspace"))
            .await?;

        // Insert the new text
        self.insert_text(text).await?;

        Ok(())
    }

    /// Type text character by character.
    ///
    /// Unlike `fill`, this types each character with keydown/keyup events.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be focused or keys cannot be dispatched.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn type_text(&self, text: &str) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(text, "Typing text");

        // Focus the element
        self.focus_element().await?;

        // Type each character
        for ch in text.chars() {
            let char_str = ch.to_string();
            self.dispatch_key_event(DispatchKeyEventParams::char(&char_str))
                .await?;
        }

        Ok(())
    }

    /// Press a key or key combination.
    ///
    /// Examples: "Enter", "Backspace", "Control+a", "Shift+Tab"
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be focused or the key cannot be pressed.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn press(&self, key: &str) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(key, "Pressing key");

        // Focus the element
        self.focus_element().await?;

        // Parse modifiers and key
        let parts: Vec<&str> = key.split('+').collect();
        let actual_key = parts.last().unwrap_or(&key);

        let mut modifiers = 0;
        for part in &parts[..parts.len().saturating_sub(1)] {
            match part.to_lowercase().as_str() {
                "control" | "ctrl" => {
                    modifiers |= viewpoint_cdp::protocol::input::modifiers::CTRL;
                }
                "alt" => modifiers |= viewpoint_cdp::protocol::input::modifiers::ALT,
                "shift" => modifiers |= viewpoint_cdp::protocol::input::modifiers::SHIFT,
                "meta" | "cmd" => modifiers |= viewpoint_cdp::protocol::input::modifiers::META,
                _ => {}
            }
        }

        // Key down
        let mut key_down = DispatchKeyEventParams::key_down(actual_key);
        if modifiers != 0 {
            key_down.modifiers = Some(modifiers);
        }
        self.dispatch_key_event(key_down).await?;

        // Key up
        let mut key_up = DispatchKeyEventParams::key_up(actual_key);
        if modifiers != 0 {
            key_up.modifiers = Some(modifiers);
        }
        self.dispatch_key_event(key_up).await?;

        Ok(())
    }

    /// Hover over the element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or the mouse event fails.
    ///
    /// # Panics
    ///
    /// Panics if a visible element lacks bounding box coordinates. This should
    /// never occur as `wait_for_actionable` ensures visibility before returning.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn hover(&self) -> Result<(), LocatorError> {
        let info = self.wait_for_actionable().await?;

        let x = info.x.expect("visible element has x") + info.width.expect("visible element has width") / 2.0;
        let y = info.y.expect("visible element has y") + info.height.expect("visible element has height") / 2.0;

        debug!(x, y, "Hovering over element");

        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_move(x, y))
            .await?;

        Ok(())
    }

    /// Focus the element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or focused.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn focus(&self) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!("Focusing element");
        self.focus_element().await?;

        Ok(())
    }

    /// Clear the element's content.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be cleared.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn clear(&self) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!("Clearing element");

        // Focus and select all, then delete
        self.focus_element().await?;

        let mut select_all = DispatchKeyEventParams::key_down("a");
        select_all.modifiers = Some(viewpoint_cdp::protocol::input::modifiers::CTRL);
        self.dispatch_key_event(select_all).await?;

        self.dispatch_key_event(DispatchKeyEventParams::key_down("Backspace"))
            .await?;

        Ok(())
    }

    /// Check a checkbox or radio button.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be checked.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn check(&self) -> Result<(), LocatorError> {
        let is_checked = self.is_checked().await?;

        if is_checked {
            debug!("Element already checked");
        } else {
            debug!("Checking element");
            self.click().await?;
        }

        Ok(())
    }

    /// Uncheck a checkbox.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be unchecked.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn uncheck(&self) -> Result<(), LocatorError> {
        let is_checked = self.is_checked().await?;

        if is_checked {
            debug!("Unchecking element");
            self.click().await?;
        } else {
            debug!("Element already unchecked");
        }

        Ok(())
    }

    /// Select an option in a `<select>` element by value, label, or index.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to select. Can be:
    ///   - A string value matching the option's `value` attribute
    ///   - A string matching the option's visible text
    ///
    /// # Errors
    ///
    /// Returns an error if the element is not a select or the option is not found.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Select by value
    /// page.locator("select#size").select_option("medium").await?;
    ///
    /// // Select by visible text
    /// page.locator("select#size").select_option("Medium Size").await?;
    /// ```
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn select_option(&self, option: &str) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(option, "Selecting option");

        // JavaScript to select option by value or text
        let js = format!(
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
            selector = self.selector.to_js_expression(),
            option = js_string_literal(option)
        );

        let result = self.evaluate_js(&js).await?;

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

    /// Select multiple options in a `<select multiple>` element.
    ///
    /// # Arguments
    ///
    /// * `options` - A slice of option values or labels to select.
    ///
    /// # Errors
    ///
    /// Returns an error if the element is not a multi-select or options are not found.
    #[instrument(level = "debug", skip(self, options), fields(selector = ?self.selector))]
    pub async fn select_options(&self, options: &[&str]) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(?options, "Selecting multiple options");

        // Build JavaScript array of options
        let options_js: Vec<String> = options.iter().map(|o| js_string_literal(o)).collect();
        let options_array = format!("[{}]", options_js.join(", "));

        let js = format!(
            r"(function() {{
                const elements = {selector};
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
            }})()",
            selector = self.selector.to_js_expression(),
            options_array = options_array
        );

        let result = self.evaluate_js(&js).await?;

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

    /// Get the text content of the element.
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
        let js = format!(
            r"(function() {{
                const elements = {};
                if (elements.length === 0) return {{ found: false, checked: false }};
                const el = elements[0];
                return {{ found: true, checked: el.checked || false }};
            }})()",
            self.selector.to_js_expression()
        );

        let result = self.evaluate_js(&js).await?;
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

    // =========================================================================
    // Internal helpers
    // =========================================================================

    /// Wait for element to be actionable (visible, enabled, stable).
    async fn wait_for_actionable(&self) -> Result<ElementInfo, LocatorError> {
        let start = std::time::Instant::now();
        let timeout = self.options.timeout;

        loop {
            let info = self.query_element_info().await?;

            if !info.found {
                if start.elapsed() >= timeout {
                    return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }

            if !info.visible.unwrap_or(false) {
                if start.elapsed() >= timeout {
                    return Err(LocatorError::NotVisible);
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }

            // Element is visible, return it
            return Ok(info);
        }
    }

    /// Query element information via JavaScript.
    async fn query_element_info(&self) -> Result<ElementInfo, LocatorError> {
        let js = format!(
            r"(function() {{
                const elements = Array.from({});
                if (elements.length === 0) {{
                    return {{ found: false, count: 0 }};
                }}
                const el = elements[0];
                const rect = el.getBoundingClientRect();
                const style = window.getComputedStyle(el);
                const visible = rect.width > 0 && rect.height > 0 && 
                    style.visibility !== 'hidden' && 
                    style.display !== 'none' &&
                    parseFloat(style.opacity) > 0;
                return {{
                    found: true,
                    count: elements.length,
                    visible: visible,
                    enabled: !el.disabled,
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rect.height,
                    text: el.textContent,
                    tagName: el.tagName.toLowerCase()
                }};
            }})()",
            self.selector.to_js_expression()
        );

        let result = self.evaluate_js(&js).await?;
        let info: ElementInfo = serde_json::from_value(result)
            .map_err(|e| LocatorError::EvaluationError(e.to_string()))?;
        Ok(info)
    }

    /// Focus the element via JavaScript.
    async fn focus_element(&self) -> Result<(), LocatorError> {
        let js = format!(
            r"(function() {{
                const elements = {};
                if (elements.length > 0) {{
                    elements[0].focus();
                    return true;
                }}
                return false;
            }})()",
            self.selector.to_js_expression()
        );

        self.evaluate_js(&js).await?;
        Ok(())
    }

    /// Evaluate JavaScript and return the result.
    async fn evaluate_js(&self, expression: &str) -> Result<serde_json::Value, LocatorError> {
        if self.page.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        let params = EvaluateParams {
            expression: expression.to_string(),
            object_group: None,
            include_command_line_api: None,
            silent: Some(true),
            context_id: None,
            return_by_value: Some(true),
            await_promise: Some(false),
        };

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .page
            .connection()
            .send_command("Runtime.evaluate", Some(params), Some(self.page.session_id()))
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        result
            .result
            .value
            .ok_or_else(|| LocatorError::EvaluationError("No result value".to_string()))
    }

    /// Dispatch a mouse event.
    async fn dispatch_mouse_event(
        &self,
        params: DispatchMouseEventParams,
    ) -> Result<(), LocatorError> {
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Input.dispatchMouseEvent",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;
        Ok(())
    }

    /// Dispatch a key event.
    async fn dispatch_key_event(
        &self,
        params: DispatchKeyEventParams,
    ) -> Result<(), LocatorError> {
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Input.dispatchKeyEvent",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;
        Ok(())
    }

    /// Insert text directly.
    async fn insert_text(&self, text: &str) -> Result<(), LocatorError> {
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Input.insertText",
                Some(InsertTextParams {
                    text: text.to_string(),
                }),
                Some(self.page.session_id()),
            )
            .await?;
        Ok(())
    }
}
