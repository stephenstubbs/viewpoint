//! Wait for function functionality.
//!
//! This module provides waiting for JavaScript conditions to become truthy.

use std::time::Duration;

use serde::Serialize;
use tracing::{debug, instrument};
use viewpoint_cdp::protocol::runtime::{EvaluateParams, EvaluateResult, ReleaseObjectParams};

use crate::error::PageError;
use crate::page::Page;

use super::{wrap_expression, JsHandle, DEFAULT_TIMEOUT};

/// Polling mode for `wait_for_function`.
#[derive(Debug, Clone, Copy, Default)]
pub enum Polling {
    /// Poll on requestAnimationFrame (default).
    #[default]
    Raf,
    /// Poll at a fixed interval.
    Interval(Duration),
}

/// Builder for `wait_for_function`.
#[derive(Debug)]
pub struct WaitForFunctionBuilder<'a> {
    page: &'a Page,
    expression: String,
    arg: Option<serde_json::Value>,
    timeout: Duration,
    polling: Polling,
}

impl<'a> WaitForFunctionBuilder<'a> {
    /// Create a new builder.
    pub(crate) fn new(page: &'a Page, expression: String) -> Self {
        Self {
            page,
            expression,
            arg: None,
            timeout: DEFAULT_TIMEOUT,
            polling: Polling::default(),
        }
    }

    /// Set an argument to pass to the function.
    #[must_use]
    pub fn arg<A: Serialize>(mut self, arg: A) -> Self {
        self.arg = serde_json::to_value(arg).ok();
        self
    }

    /// Set the timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the polling mode.
    #[must_use]
    pub fn polling(mut self, polling: Polling) -> Self {
        self.polling = polling;
        self
    }

    /// Wait for the function to return a truthy value.
    ///
    /// Returns a handle to the truthy result.
    #[instrument(level = "debug", skip(self), fields(expression = %self.expression, timeout_ms = self.timeout.as_millis()))]
    pub async fn wait(self) -> Result<JsHandle, PageError> {
        if self.page.closed {
            return Err(PageError::Closed);
        }

        let start = std::time::Instant::now();

        debug!("Starting wait_for_function polling with {:?}", self.polling);

        loop {
            if start.elapsed() >= self.timeout {
                return Err(PageError::EvaluationFailed(format!(
                    "Timeout {}ms exceeded waiting for function",
                    self.timeout.as_millis()
                )));
            }

            // Try to evaluate the expression
            let result = self.try_evaluate().await?;

            if result.is_truthy {
                debug!("Function returned truthy value");
                return Ok(result.handle.expect("truthy result has handle"));
            }

            // Wait according to polling mode
            match self.polling {
                Polling::Raf => {
                    // Approximate RAF timing
                    tokio::time::sleep(Duration::from_millis(16)).await;
                }
                Polling::Interval(duration) => {
                    tokio::time::sleep(duration).await;
                }
            }
        }
    }

    /// Try to evaluate the function and check if result is truthy.
    async fn try_evaluate(&self) -> Result<TryResult, PageError> {
        let expression = if let Some(ref arg) = self.arg {
            let arg_json = serde_json::to_string(arg)
                .map_err(|e| PageError::EvaluationFailed(e.to_string()))?;
            format!("({})({})", self.expression, arg_json)
        } else {
            wrap_expression(&self.expression)
        };

        let params = EvaluateParams {
            expression,
            object_group: Some("viewpoint-wait".to_string()),
            include_command_line_api: None,
            silent: Some(false),
            context_id: None,
            return_by_value: Some(false),
            await_promise: Some(true),
        };

        let result: EvaluateResult = self
            .page
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(params),
                Some(&self.page.session_id),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(PageError::EvaluationFailed(exception.text));
        }

        // Check if the result is truthy
        let is_truthy = is_truthy_result(&result.result);

        let handle = if is_truthy {
            result.result.object_id.map(|id| {
                JsHandle::new(id, self.page.session_id.clone(), self.page.connection.clone())
            })
        } else {
            // Release non-truthy object references
            if let Some(object_id) = result.result.object_id {
                let _ = self
                    .page
                    .connection
                    .send_command::<_, serde_json::Value>(
                        "Runtime.releaseObject",
                        Some(ReleaseObjectParams { object_id }),
                        Some(&self.page.session_id),
                    )
                    .await;
            }
            None
        };

        Ok(TryResult { is_truthy, handle })
    }
}

struct TryResult {
    is_truthy: bool,
    handle: Option<JsHandle>,
}

/// Check if a `RemoteObject` represents a truthy value.
fn is_truthy_result(result: &viewpoint_cdp::protocol::runtime::RemoteObject) -> bool {
    // Check by type
    match result.object_type.as_str() {
        "undefined" => false,
        "object" => {
            // Null is falsy, other objects are truthy
            result.subtype.as_deref() != Some("null")
        }
        "boolean" => result
            .value
            .as_ref()
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        "number" => result
            .value
            .as_ref()
            .and_then(serde_json::Value::as_f64)
            .is_some_and(|n| n != 0.0 && !n.is_nan()),
        "string" => result
            .value
            .as_ref()
            .and_then(|v| v.as_str())
            .is_some_and(|s| !s.is_empty()),
        _ => {
            // Functions, symbols, bigints are truthy
            true
        }
    }
}

impl Page {
    /// Wait for a JavaScript function to return a truthy value.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use viewpoint_core::page::Polling;
    /// use viewpoint_js::js;
    ///
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Wait for an element to appear
    /// let selector = ".loaded";
    /// page.wait_for_function(js!{ () => document.querySelector(#{selector}) })
    ///     .wait()
    ///     .await?;
    ///
    /// // Wait with custom timeout
    /// page.wait_for_function(js!{ () => window.ready })
    ///     .timeout(Duration::from_secs(10))
    ///     .wait()
    ///     .await?;
    ///
    /// // Wait with interval polling
    /// page.wait_for_function(js!{ () => window.ready })
    ///     .polling(Polling::Interval(Duration::from_millis(100)))
    ///     .wait()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn wait_for_function(&self, expression: impl Into<String>) -> WaitForFunctionBuilder<'_> {
        WaitForFunctionBuilder::new(self, expression.into())
    }

    /// Wait for a JavaScript function with an argument to return a truthy value.
    pub fn wait_for_function_with_arg<A: Serialize>(
        &self,
        expression: impl Into<String>,
        arg: A,
    ) -> WaitForFunctionBuilder<'_> {
        WaitForFunctionBuilder::new(self, expression.into()).arg(arg)
    }
}
