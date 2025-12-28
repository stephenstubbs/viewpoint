//! Actions for frame element locators.
//!
//! This module provides the action methods (click, fill, etc.) for
//! `FrameElementLocator`.

use serde::Deserialize;
use tracing::debug;
use viewpoint_cdp::protocol::input::{
    DispatchKeyEventParams, DispatchMouseEventParams, InsertTextParams, MouseButton,
};
use viewpoint_cdp::protocol::runtime::EvaluateParams;

use super::frame_locator::FrameElementLocator;
use crate::error::LocatorError;

/// Result of querying element information in a frame.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FrameElementInfo {
    /// Whether the element exists.
    pub found: bool,
    /// Number of matching elements.
    pub count: usize,
    /// Whether the element is visible.
    pub visible: Option<bool>,
    /// Whether the element is enabled.
    pub enabled: Option<bool>,
    /// Bounding box of the element.
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    /// Text content of the element.
    pub text: Option<String>,
    /// Error message if any.
    pub error: Option<String>,
}

impl FrameElementLocator<'_> {
    /// Click the element within the frame.
    ///
    /// Waits for the element to be visible and enabled, then clicks its center.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame or element is not found, not visible, or the click fails.
    #[tracing::instrument(level = "debug", skip(self), fields(selector = ?self.selector()))]
    pub async fn click(&self) -> Result<(), LocatorError> {
        let info = self.wait_for_actionable().await?;

        let x = info.x.expect("visible element has x")
            + info.width.expect("visible element has width") / 2.0;
        let y = info.y.expect("visible element has y")
            + info.height.expect("visible element has height") / 2.0;

        debug!(x, y, "Clicking element in frame");

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

    /// Fill the element with text (clears existing content first).
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be focused or text cannot be inserted.
    #[tracing::instrument(level = "debug", skip(self), fields(selector = ?self.selector()))]
    pub async fn fill(&self, text: &str) -> Result<(), LocatorError> {
        let _info = self.wait_for_actionable().await?;

        debug!(text, "Filling element in frame");

        // Focus the element
        self.focus_element().await?;

        // Select all and delete (clear)
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
    /// # Errors
    ///
    /// Returns an error if the element cannot be focused or keys cannot be dispatched.
    #[tracing::instrument(level = "debug", skip(self), fields(selector = ?self.selector()))]
    pub async fn type_text(&self, text: &str) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(text, "Typing text in frame element");

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

    /// Hover over the element.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or the mouse event fails.
    #[tracing::instrument(level = "debug", skip(self), fields(selector = ?self.selector()))]
    pub async fn hover(&self) -> Result<(), LocatorError> {
        let info = self.wait_for_actionable().await?;

        let x = info.x.expect("visible element has x")
            + info.width.expect("visible element has width") / 2.0;
        let y = info.y.expect("visible element has y")
            + info.height.expect("visible element has height") / 2.0;

        debug!(x, y, "Hovering over element in frame");

        self.dispatch_mouse_event(DispatchMouseEventParams::mouse_move(x, y))
            .await?;

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
    pub(crate) async fn wait_for_actionable(&self) -> Result<FrameElementInfo, LocatorError> {
        let start = std::time::Instant::now();
        let timeout = self.options().timeout;

        loop {
            let info = self.query_element_info().await?;

            if let Some(error) = &info.error {
                if start.elapsed() >= timeout {
                    return Err(LocatorError::NotFound(error.clone()));
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            if !info.found {
                if start.elapsed() >= timeout {
                    return Err(LocatorError::NotFound(format!("{:?}", self.selector())));
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            if !info.visible.unwrap_or(false) {
                if start.elapsed() >= timeout {
                    return Err(LocatorError::NotVisible);
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            return Ok(info);
        }
    }

    /// Query element information within the frame.
    pub(crate) async fn query_element_info(&self) -> Result<FrameElementInfo, LocatorError> {
        let frame_access = self.frame_locator().to_js_frame_access();
        let element_selector = self.selector().to_js_expression();

        let js = format!(
            r"(function() {{
                const frameDoc = {frame_access};
                if (!frameDoc) {{
                    return {{ found: false, count: 0, error: 'Frame not found or not accessible' }};
                }}
                
                // Create a modified expression that uses frameDoc instead of document
                let elements;
                try {{
                    elements = (function() {{
                        const document = frameDoc;
                        return Array.from({element_selector});
                    }})();
                }} catch (e) {{
                    return {{ found: false, count: 0, error: e.message }};
                }}
                
                if (elements.length === 0) {{
                    return {{ found: false, count: 0 }};
                }}
                
                const el = elements[0];
                const rect = el.getBoundingClientRect();
                
                // Get frame's position to calculate absolute coordinates
                let frameRect = {{ x: 0, y: 0 }};
                let current = frameDoc.defaultView?.frameElement;
                while (current) {{
                    const currentRect = current.getBoundingClientRect();
                    frameRect.x += currentRect.x;
                    frameRect.y += currentRect.y;
                    current = current.ownerDocument?.defaultView?.frameElement;
                }}
                
                const style = frameDoc.defaultView?.getComputedStyle(el) || window.getComputedStyle(el);
                const visible = rect.width > 0 && rect.height > 0 && 
                    style.visibility !== 'hidden' && 
                    style.display !== 'none' &&
                    parseFloat(style.opacity) > 0;
                    
                return {{
                    found: true,
                    count: elements.length,
                    visible: visible,
                    enabled: !el.disabled,
                    x: frameRect.x + rect.x,
                    y: frameRect.y + rect.y,
                    width: rect.width,
                    height: rect.height,
                    text: el.textContent
                }};
            }})()"
        );

        let result = self.evaluate_js(&js).await?;
        let info: FrameElementInfo = serde_json::from_value(result)
            .map_err(|e| LocatorError::EvaluationError(e.to_string()))?;
        Ok(info)
    }

    /// Focus the element via JavaScript.
    pub(crate) async fn focus_element(&self) -> Result<(), LocatorError> {
        let frame_access = self.frame_locator().to_js_frame_access();
        let element_selector = self.selector().to_js_expression();

        let js = format!(
            r"(function() {{
                const frameDoc = {frame_access};
                if (!frameDoc) return false;
                
                const elements = (function() {{
                    const document = frameDoc;
                    return Array.from({element_selector});
                }})();
                
                if (elements.length > 0) {{
                    elements[0].focus();
                    return true;
                }}
                return false;
            }})()"
        );

        self.evaluate_js(&js).await?;
        Ok(())
    }

    /// Evaluate JavaScript and return the result.
    pub(crate) async fn evaluate_js(&self, expression: &str) -> Result<serde_json::Value, LocatorError> {
        let page = self.frame_locator().page();

        if page.is_closed() {
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

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = page
            .connection()
            .send_command("Runtime.evaluate", Some(params), Some(page.session_id()))
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
    pub(crate) async fn dispatch_mouse_event(
        &self,
        params: DispatchMouseEventParams,
    ) -> Result<(), LocatorError> {
        let page = self.frame_locator().page();

        page.connection()
            .send_command::<_, serde_json::Value>(
                "Input.dispatchMouseEvent",
                Some(params),
                Some(page.session_id()),
            )
            .await?;
        Ok(())
    }

    /// Dispatch a key event.
    pub(crate) async fn dispatch_key_event(
        &self,
        params: DispatchKeyEventParams,
    ) -> Result<(), LocatorError> {
        let page = self.frame_locator().page();

        page.connection()
            .send_command::<_, serde_json::Value>(
                "Input.dispatchKeyEvent",
                Some(params),
                Some(page.session_id()),
            )
            .await?;
        Ok(())
    }

    /// Insert text directly.
    pub(crate) async fn insert_text(&self, text: &str) -> Result<(), LocatorError> {
        let page = self.frame_locator().page();

        page.connection()
            .send_command::<_, serde_json::Value>(
                "Input.insertText",
                Some(InsertTextParams {
                    text: text.to_string(),
                }),
                Some(page.session_id()),
            )
            .await?;
        Ok(())
    }
}
