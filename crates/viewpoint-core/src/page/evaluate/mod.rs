//! JavaScript evaluation functionality.
//!
//! This module provides methods for executing JavaScript in the page context.

use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, instrument, trace};
use viewpoint_cdp::protocol::runtime::{
    CallFunctionOnParams, EvaluateParams, EvaluateResult, ReleaseObjectParams,
};

use crate::error::PageError;

use super::Page;

mod wait;

pub use wait::{Polling, WaitForFunctionBuilder};

/// Default evaluation timeout (30 seconds, matching Playwright).
pub(super) const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A handle to a JavaScript object in the page context.
///
/// Handles are useful for referencing complex objects that cannot be serialized
/// (like DOM elements). Remember to dispose of handles when done.
#[derive(Debug)]
pub struct JsHandle {
    /// The object ID from CDP.
    object_id: String,
    /// Reference to the page for cleanup.
    page_session_id: String,
    /// CDP connection.
    connection: std::sync::Arc<viewpoint_cdp::CdpConnection>,
}

impl JsHandle {
    /// Create a new handle.
    pub(crate) fn new(
        object_id: String,
        page_session_id: String,
        connection: std::sync::Arc<viewpoint_cdp::CdpConnection>,
    ) -> Self {
        Self {
            object_id,
            page_session_id,
            connection,
        }
    }

    /// Get the object ID.
    pub fn object_id(&self) -> &str {
        &self.object_id
    }

    /// Get the JSON value of this handle.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be serialized to JSON.
    pub async fn json_value<T: DeserializeOwned>(&self) -> Result<T, PageError> {
        let params = CallFunctionOnParams {
            function_declaration: "function() { return this; }".to_string(),
            object_id: Some(self.object_id.clone()),
            arguments: None,
            silent: Some(false),
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: Some(true),
            execution_context_id: None,
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let result: viewpoint_cdp::protocol::runtime::CallFunctionOnResult = self
            .connection
            .send_command(
                "Runtime.callFunctionOn",
                Some(params),
                Some(&self.page_session_id),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(PageError::EvaluationFailed(exception.text));
        }

        // Handle undefined return values - use null if no value present
        let value = result.result.value.unwrap_or(serde_json::Value::Null);

        serde_json::from_value(value)
            .map_err(|e| PageError::EvaluationFailed(format!("Failed to deserialize: {e}")))
    }

    /// Dispose of this handle, releasing the JavaScript object reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the CDP command fails.
    pub async fn dispose(self) -> Result<(), PageError> {
        self.connection
            .send_command::<_, serde_json::Value>(
                "Runtime.releaseObject",
                Some(ReleaseObjectParams {
                    object_id: self.object_id,
                }),
                Some(&self.page_session_id),
            )
            .await?;
        Ok(())
    }
}

impl Page {
    /// Evaluate JavaScript in the page context.
    ///
    /// The expression is evaluated and the result is deserialized to the specified type.
    /// Promises are automatically awaited.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The return type. Use `serde_json::Value` for dynamic results.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// use viewpoint_js::js;
    ///
    /// // Simple expression
    /// let sum: i32 = page.evaluate(js!{ 1 + 2 }).await?;
    /// assert_eq!(sum, 3);
    ///
    /// // Function expression
    /// let width: i32 = page.evaluate(js!{ () => window.innerWidth }).await?;
    ///
    /// // With interpolation (note: returns String so use &)
    /// let selector = ".my-class";
    /// let el: serde_json::Value = page.evaluate(&js!{ document.querySelector(#{selector}) }).await?;
    ///
    /// // Get document title
    /// let title: String = page.evaluate(js!{ document.title }).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - The JavaScript throws an error
    /// - The result cannot be deserialized
    #[instrument(level = "debug", skip(self), fields(expression = %expression))]
    pub async fn evaluate<T: DeserializeOwned>(&self, expression: &str) -> Result<T, PageError> {
        self.evaluate_internal(expression, None, DEFAULT_TIMEOUT)
            .await
    }

    /// Evaluate JavaScript with an argument.
    ///
    /// The argument is serialized to JSON and passed to the function.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Pass a number
    /// let doubled: i32 = page.evaluate_with_arg("x => x * 2", 21).await?;
    /// assert_eq!(doubled, 42);
    ///
    /// // Pass an object
    /// let name: String = page.evaluate_with_arg("obj => obj.name", serde_json::json!({"name": "test"})).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self, arg), fields(expression = %expression))]
    pub async fn evaluate_with_arg<T: DeserializeOwned, A: Serialize>(
        &self,
        expression: &str,
        arg: A,
    ) -> Result<T, PageError> {
        let arg_json = serde_json::to_value(arg).map_err(|e| {
            PageError::EvaluationFailed(format!("Failed to serialize argument: {e}"))
        })?;

        self.evaluate_internal(expression, Some(arg_json), DEFAULT_TIMEOUT)
            .await
    }

    /// Evaluate JavaScript and return a handle to the result.
    ///
    /// Use this when you need to reference the result object later, or when the
    /// result cannot be serialized (like DOM elements).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Get a handle to the body element
    /// let body_handle = page.evaluate_handle("document.body").await?;
    ///
    /// // Use the handle in another evaluation
    /// let tag_name: String = page.evaluate_with_arg("el => el.tagName", body_handle.object_id()).await?;
    ///
    /// // Clean up
    /// body_handle.dispose().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self), fields(expression = %expression))]
    pub async fn evaluate_handle(&self, expression: &str) -> Result<JsHandle, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        debug!("Evaluating expression for handle");

        // Wrap expression in function if not already
        let wrapped = wrap_expression(expression);

        let params = EvaluateParams {
            expression: wrapped,
            object_group: Some("viewpoint".to_string()),
            include_command_line_api: None,
            silent: Some(false),
            context_id: None,
            return_by_value: Some(false), // Keep as reference
            await_promise: Some(true),
        };

        let result: EvaluateResult = self
            .connection
            .send_command("Runtime.evaluate", Some(params), Some(&self.session_id))
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(PageError::EvaluationFailed(exception.text));
        }

        let object_id = result
            .result
            .object_id
            .ok_or_else(|| PageError::EvaluationFailed("Result is not an object".to_string()))?;

        Ok(JsHandle::new(
            object_id,
            self.session_id.clone(),
            self.connection.clone(),
        ))
    }

    /// Internal evaluation helper.
    async fn evaluate_internal<T: DeserializeOwned>(
        &self,
        expression: &str,
        arg: Option<serde_json::Value>,
        _timeout: Duration,
    ) -> Result<T, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        trace!(expression = expression, "Evaluating JavaScript");

        // Wrap expression in a function call if needed and handle arguments
        let final_expression = if let Some(arg_value) = arg {
            // If we have an argument, we need to use callFunctionOn or wrap the expression
            let arg_json = serde_json::to_string(&arg_value)
                .map_err(|e| PageError::EvaluationFailed(format!("Failed to serialize: {e}")))?;

            format!("({expression})({arg_json})")
        } else {
            wrap_expression(expression)
        };

        let params = EvaluateParams {
            expression: final_expression,
            object_group: None,
            include_command_line_api: None,
            silent: Some(false),
            context_id: None,
            return_by_value: Some(true),
            await_promise: Some(true),
        };

        let result: EvaluateResult = self
            .connection
            .send_command("Runtime.evaluate", Some(params), Some(&self.session_id))
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(PageError::EvaluationFailed(exception.text));
        }

        // Handle undefined return values - use null if no value present
        // This allows expressions like console.log() which return undefined
        let value = result.result.value.unwrap_or(serde_json::Value::Null);

        serde_json::from_value(value)
            .map_err(|e| PageError::EvaluationFailed(format!("Failed to deserialize: {e}")))
    }
}

/// Wrap an expression in a function call if it looks like a function.
pub(super) fn wrap_expression(expression: &str) -> String {
    let trimmed = expression.trim();

    // If it starts with arrow function syntax or function keyword, wrap it
    if trimmed.starts_with("()")
        || trimmed.starts_with("async ()")
        || trimmed.starts_with("async()")
        || trimmed.starts_with("function")
        || trimmed.starts_with("async function")
    {
        format!("({trimmed})()")
    } else {
        trimmed.to_string()
    }
}
