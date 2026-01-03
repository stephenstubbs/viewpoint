//! ARIA snapshot implementation for Locator.

use super::aria::{aria_snapshot_js, AriaSnapshot};
use super::Locator;
use crate::error::LocatorError;
use viewpoint_js::js;

impl Locator<'_> {
    /// Get an ARIA accessibility snapshot of this element.
    ///
    /// The snapshot captures the accessible tree structure as it would be
    /// exposed to assistive technologies. This is useful for accessibility testing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.locator("form").aria_snapshot().await?;
    /// println!("{}", snapshot); // YAML-like output
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the element is not found or snapshot capture fails.
    pub async fn aria_snapshot(&self) -> Result<AriaSnapshot, LocatorError> {
        if self.page.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        // Get the element and evaluate ARIA snapshot
        // Note: to_js_expression() returns code that evaluates to NodeList/array,
        // so we need to get the first element from it
        let js_selector = self.selector.to_js_expression();
        let snapshot_fn = aria_snapshot_js();
        let js_code = js! {
            (function() {
                const elements = @{js_selector};
                const element = elements && elements[0] ? elements[0] : elements;
                if (!element) {
                    return { error: "Element not found" };
                }
                const getSnapshot = @{snapshot_fn};
                return getSnapshot(element);
            })()
        };

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .page
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js_code,
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        let value = result.result.value.ok_or_else(|| {
            LocatorError::EvaluationError("No result from aria snapshot".to_string())
        })?;

        // Check for error
        if let Some(error) = value.get("error").and_then(|e| e.as_str()) {
            return Err(LocatorError::NotFound(error.to_string()));
        }

        // Parse the snapshot
        let snapshot: AriaSnapshot = serde_json::from_value(value).map_err(|e| {
            LocatorError::EvaluationError(format!("Failed to parse aria snapshot: {e}"))
        })?;

        Ok(snapshot)
    }
}
