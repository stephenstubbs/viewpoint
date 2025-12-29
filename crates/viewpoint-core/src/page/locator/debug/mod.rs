//! Debug and visualization methods for locators.
//!
//! Methods for highlighting and debugging element selections.

use std::time::Duration;

use tracing::{debug, instrument};
use viewpoint_js::js;

use super::Locator;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Highlight the element for debugging purposes.
    ///
    /// This visually highlights the element with a magenta outline for 2 seconds,
    /// making it easy to verify which element is being targeted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Highlight for default duration (2 seconds)
    /// page.locator("button").highlight().await?;
    ///
    /// // Highlight for custom duration
    /// page.locator("button").highlight_for(Duration::from_secs(5)).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or highlighted.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn highlight(&self) -> Result<(), LocatorError> {
        self.highlight_for(Duration::from_secs(2)).await
    }

    /// Highlight the element for a specific duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - How long to show the highlight.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found or highlighted.
    #[instrument(level = "debug", skip(self), fields(selector = ?self.selector))]
    pub async fn highlight_for(&self, duration: Duration) -> Result<(), LocatorError> {
        self.wait_for_actionable().await?;

        debug!(?duration, "Highlighting element");

        // Add highlight style
        let selector_expr = self.selector.to_js_expression();
        let highlight_js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return { found: false };

                const el = elements[0];
                const originalOutline = el.style.outline;
                const originalOutlineOffset = el.style.outlineOffset;
                const originalTransition = el.style.transition;

                // Apply highlight with animation
                el.style.transition = "outline 0.2s ease-in-out";
                el.style.outline = "3px solid #ff00ff";
                el.style.outlineOffset = "2px";

                // Store original styles for restoration
                el.__viewpoint_original_outline = originalOutline;
                el.__viewpoint_original_outline_offset = originalOutlineOffset;
                el.__viewpoint_original_transition = originalTransition;

                return { found: true };
            })()
        };

        let result = self.evaluate_js(&highlight_js).await?;
        let found = result
            .get("found")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        // Wait for the duration
        tokio::time::sleep(duration).await;

        // Remove highlight
        let cleanup_js = js! {
            (function() {
                const elements = @{selector_expr};
                if (elements.length === 0) return;

                const el = elements[0];
                el.style.outline = el.__viewpoint_original_outline || "";
                el.style.outlineOffset = el.__viewpoint_original_outline_offset || "";
                el.style.transition = el.__viewpoint_original_transition || "";

                delete el.__viewpoint_original_outline;
                delete el.__viewpoint_original_outline_offset;
                delete el.__viewpoint_original_transition;
            })()
        };

        // Ignore cleanup errors - element may have been removed
        let _ = self.evaluate_js(&cleanup_js).await;

        Ok(())
    }
}
