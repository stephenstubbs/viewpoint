//! Element handle types for low-level DOM operations.
//!
//! Unlike [`Locator`], an `ElementHandle` is bound to a specific element instance.
//! If the element is removed from the DOM, the handle becomes stale.

use crate::error::LocatorError;
use crate::Page;

/// A bounding box representing an element's position and size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    /// X coordinate of the top-left corner.
    pub x: f64,
    /// Y coordinate of the top-left corner.
    pub y: f64,
    /// Width of the element.
    pub width: f64,
    /// Height of the element.
    pub height: f64,
}

/// A handle to a DOM element.
///
/// Unlike [`Locator`], an `ElementHandle` is bound to a specific element instance.
/// If the element is removed from the DOM, the handle becomes stale.
///
/// Most operations should prefer using [`Locator`] for its auto-waiting and
/// re-querying capabilities. Use `ElementHandle` only when you need:
/// - To pass an element reference to JavaScript
/// - Low-level DOM operations
/// - Box model information
#[derive(Debug)]
pub struct ElementHandle<'a> {
    pub(crate) object_id: String,
    pub(crate) page: &'a Page,
}

impl ElementHandle<'_> {
    /// Get the object ID of this element handle.
    ///
    /// This is the CDP remote object ID that can be used for further CDP calls.
    pub fn object_id(&self) -> &str {
        &self.object_id
    }

    /// Get the box model of the element.
    ///
    /// Returns detailed information about the element's box model including
    /// content, padding, border, and margin boxes.
    ///
    /// # Errors
    ///
    /// Returns an error if the element is no longer attached to the DOM.
    pub async fn box_model(&self) -> Result<Option<BoxModel>, LocatorError> {
        #[derive(Debug, serde::Deserialize)]
        struct BoxModelResult {
            model: Option<BoxModel>,
        }

        let result: BoxModelResult = self.page
            .connection()
            .send_command(
                "DOM.getBoxModel",
                Some(serde_json::json!({
                    "objectId": self.object_id
                })),
                Some(self.page.session_id()),
            )
            .await?;

        Ok(result.model)
    }

    /// Check if the element is still attached to the DOM.
    ///
    /// Returns `true` if the element still exists in the document.
    pub async fn is_attached(&self) -> Result<bool, LocatorError> {
        #[derive(Debug, serde::Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
        }

        let result: CallResult = self.page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": self.object_id,
                    "functionDeclaration": "function() { return this.isConnected; }",
                    "returnByValue": true
                })),
                Some(self.page.session_id()),
            )
            .await?;

        Ok(result.result.value
            .and_then(|v| v.as_bool())
            .unwrap_or(false))
    }

    /// Evaluate a JavaScript expression with this element as `this`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use viewpoint_js::js;
    /// let handle = page.locator("button").element_handle().await?;
    /// let text: String = handle.evaluate(js!{ this.textContent }).await?;
    /// ```
    pub async fn evaluate<T: serde::de::DeserializeOwned>(
        &self,
        expression: &str,
    ) -> Result<T, LocatorError> {
        let function = format!("function() {{ return {expression}; }}");

        #[derive(Debug, serde::Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
            #[serde(rename = "exceptionDetails")]
            exception_details: Option<viewpoint_cdp::protocol::runtime::ExceptionDetails>,
        }

        let result: CallResult = self.page
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": self.object_id,
                    "functionDeclaration": function,
                    "returnByValue": true
                })),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        let value = result.result.value.unwrap_or(serde_json::Value::Null);
        serde_json::from_value(value)
            .map_err(|e| LocatorError::EvaluationError(format!("Failed to deserialize: {e}")))
    }
}

/// Box model information for an element.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct BoxModel {
    /// Content box coordinates.
    pub content: Vec<f64>,
    /// Padding box coordinates.
    pub padding: Vec<f64>,
    /// Border box coordinates.
    pub border: Vec<f64>,
    /// Margin box coordinates.
    pub margin: Vec<f64>,
    /// Width of the element.
    pub width: i32,
    /// Height of the element.
    pub height: i32,
}
