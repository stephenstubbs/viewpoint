//! Drag and drop operations for mouse.

use tracing::{debug, instrument};

use super::Page;
use crate::error::LocatorError;

/// Builder for drag and drop operations.
///
/// Created via [`Page::drag_and_drop`].
///
/// # Example
///
/// ```ignore
/// // Simple drag and drop
/// page.drag_and_drop("#source", "#target").send().await?;
///
/// // With position options
/// page.drag_and_drop("#source", "#target")
///     .source_position(10.0, 10.0)
///     .target_position(5.0, 5.0)
///     .send()
///     .await?;
/// ```
#[derive(Debug)]
pub struct DragAndDropBuilder<'a> {
    page: &'a Page,
    source: String,
    target: String,
    source_position: Option<(f64, f64)>,
    target_position: Option<(f64, f64)>,
    steps: u32,
}

impl<'a> DragAndDropBuilder<'a> {
    /// Create a new drag and drop builder.
    pub(crate) fn new(page: &'a Page, source: String, target: String) -> Self {
        Self {
            page,
            source,
            target,
            source_position: None,
            target_position: None,
            steps: 1,
        }
    }

    /// Set the position within the source element to start dragging from.
    ///
    /// Coordinates are relative to the element's top-left corner.
    #[must_use]
    pub fn source_position(mut self, x: f64, y: f64) -> Self {
        self.source_position = Some((x, y));
        self
    }

    /// Set the position within the target element to drop at.
    ///
    /// Coordinates are relative to the element's top-left corner.
    #[must_use]
    pub fn target_position(mut self, x: f64, y: f64) -> Self {
        self.target_position = Some((x, y));
        self
    }

    /// Set the number of intermediate steps for smooth dragging.
    #[must_use]
    pub fn steps(mut self, steps: u32) -> Self {
        self.steps = steps.max(1);
        self
    }

    /// Execute the drag and drop operation.
    #[instrument(level = "debug", skip(self), fields(source = %self.source, target = %self.target))]
    pub async fn send(self) -> Result<(), LocatorError> {
        // Get source element bounding box
        let source_box = self.get_element_box(&self.source).await?;

        // Get target element bounding box
        let target_box = self.get_element_box(&self.target).await?;

        // Calculate source coordinates
        let (source_x, source_y) = if let Some((ox, oy)) = self.source_position {
            (source_box.0 + ox, source_box.1 + oy)
        } else {
            // Use center
            (
                source_box.0 + source_box.2 / 2.0,
                source_box.1 + source_box.3 / 2.0,
            )
        };

        // Calculate target coordinates
        let (target_x, target_y) = if let Some((ox, oy)) = self.target_position {
            (target_box.0 + ox, target_box.1 + oy)
        } else {
            // Use center
            (
                target_box.0 + target_box.2 / 2.0,
                target_box.1 + target_box.3 / 2.0,
            )
        };

        debug!(
            "Dragging from ({}, {}) to ({}, {})",
            source_x, source_y, target_x, target_y
        );

        // Perform drag operation
        self.page.mouse().move_(source_x, source_y).send().await?;
        self.page.mouse().down().send().await?;
        self.page
            .mouse()
            .move_(target_x, target_y)
            .steps(self.steps)
            .send()
            .await?;
        self.page.mouse().up().send().await?;

        Ok(())
    }

    /// Get the bounding box of an element (x, y, width, height).
    async fn get_element_box(&self, selector: &str) -> Result<(f64, f64, f64, f64), LocatorError> {
        let js = format!(
            r"(function() {{
                const el = document.querySelector({selector});
                if (!el) return null;
                const rect = el.getBoundingClientRect();
                return {{ x: rect.x, y: rect.y, width: rect.width, height: rect.height }};
            }})()",
            selector = crate::page::locator::selector::js_string_literal(selector)
        );

        let result = self.evaluate_js(&js).await?;

        if result.is_null() {
            return Err(LocatorError::NotFound(selector.to_string()));
        }

        let x = result
            .get("x")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);
        let y = result
            .get("y")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);
        let width = result
            .get("width")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);
        let height = result
            .get("height")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);

        Ok((x, y, width, height))
    }

    /// Evaluate JavaScript and return the result.
    async fn evaluate_js(&self, expression: &str) -> Result<serde_json::Value, LocatorError> {
        if self.page.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        let params = viewpoint_cdp::protocol::runtime::EvaluateParams {
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
            .send_command(
                "Runtime.evaluate",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        result
            .result
            .value
            .ok_or_else(|| LocatorError::EvaluationError("No result value".to_string()))
    }
}
