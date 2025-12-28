//! Internal helper methods for Locator.
//!
//! These methods are used internally by action methods and builders.

use std::time::Duration;

use serde::Deserialize;
use viewpoint_cdp::protocol::input::{
    DispatchKeyEventParams, DispatchMouseEventParams, InsertTextParams,
};
use viewpoint_cdp::protocol::runtime::EvaluateParams;

use super::Locator;
use crate::error::LocatorError;

/// Result of querying element information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ElementInfo {
    /// Whether the element exists.
    pub(super) found: bool,
    /// Number of matching elements.
    pub(super) count: usize,
    /// Whether the element is visible.
    pub(super) visible: Option<bool>,
    /// Whether the element is enabled.
    pub(super) enabled: Option<bool>,
    /// Bounding box of the element.
    pub(super) x: Option<f64>,
    pub(super) y: Option<f64>,
    pub(super) width: Option<f64>,
    pub(super) height: Option<f64>,
    /// Text content of the element.
    pub(super) text: Option<String>,
    /// Element tag name.
    pub(super) tag_name: Option<String>,
}

impl Locator<'_> {
    /// Wait for element to be actionable (visible, enabled, stable).
    pub(super) async fn wait_for_actionable(&self) -> Result<ElementInfo, LocatorError> {
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
    pub(super) async fn query_element_info(&self) -> Result<ElementInfo, LocatorError> {
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
    pub(super) async fn focus_element(&self) -> Result<(), LocatorError> {
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
    pub(super) async fn evaluate_js(
        &self,
        expression: &str,
    ) -> Result<serde_json::Value, LocatorError> {
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
    pub(super) async fn dispatch_mouse_event(
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
    pub(super) async fn dispatch_key_event(
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
    pub(super) async fn insert_text(&self, text: &str) -> Result<(), LocatorError> {
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
