//! Element screenshot capture functionality.

use std::path::Path;

use tracing::{info, instrument};

use super::screenshot::{Animations, ScreenshotBuilder, ScreenshotFormat};
use crate::error::LocatorError;
use crate::page::Locator;

/// Builder for element screenshots.
#[derive(Debug)]
pub struct ElementScreenshotBuilder<'a, 'b> {
    locator: &'a Locator<'b>,
    format: ScreenshotFormat,
    quality: Option<u8>,
    path: Option<String>,
    omit_background: bool,
    animations: Animations,
}

impl<'a, 'b> ElementScreenshotBuilder<'a, 'b> {
    /// Create a new element screenshot builder.
    pub(crate) fn new(locator: &'a Locator<'b>) -> Self {
        Self {
            locator,
            format: ScreenshotFormat::Png,
            quality: None,
            path: None,
            omit_background: false,
            animations: Animations::default(),
        }
    }

    /// Set the image format.
    #[must_use]
    pub fn format(mut self, format: ScreenshotFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the image quality (0-100, applicable to JPEG and WebP only).
    #[must_use]
    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = Some(quality.min(100));
        self
    }

    /// Save the screenshot to a file.
    #[must_use]
    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_string_lossy().to_string());
        self
    }

    /// Set whether to omit the background (transparent).
    #[must_use]
    pub fn omit_background(mut self, omit: bool) -> Self {
        self.omit_background = omit;
        self
    }

    /// Set animation handling.
    #[must_use]
    pub fn animations(mut self, animations: Animations) -> Self {
        self.animations = animations;
        self
    }

    /// Capture the element screenshot.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The element is not found
    /// - The page is closed
    /// - The CDP command fails
    #[instrument(level = "info", skip(self), fields(format = ?self.format))]
    pub async fn capture(self) -> Result<Vec<u8>, LocatorError> {
        let page = self.locator.page();

        if page.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        // Get element bounding box
        let bbox = self.get_element_bounding_box().await?;

        info!(
            x = bbox.x,
            y = bbox.y,
            width = bbox.width,
            height = bbox.height,
            "Capturing element screenshot"
        );

        // Create screenshot builder with clip
        let mut builder = ScreenshotBuilder::new(page)
            .format(self.format)
            .clip(bbox.x, bbox.y, bbox.width, bbox.height)
            .omit_background(self.omit_background)
            .animations(self.animations);

        if let Some(quality) = self.quality {
            builder = builder.quality(quality);
        }

        if let Some(ref path) = self.path {
            builder = builder.path(path);
        }

        builder
            .capture()
            .await
            .map_err(|e| LocatorError::EvaluationError(e.to_string()))
    }

    /// Get the element's bounding box.
    async fn get_element_bounding_box(&self) -> Result<BoundingBox, LocatorError> {
        let page = self.locator.page();
        let js_selector = self.locator.selector().to_js_expression();

        let script = format!(
            r"
            (function() {{
                const element = {js_selector};
                if (!element) return null;
                const rect = element.getBoundingClientRect();
                return JSON.stringify({{
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rect.height
                }});
            }})()
            "
        );

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = page
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: script,
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(false),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(page.session_id()),
            )
            .await?;

        let json_str = result
            .result
            .value
            .and_then(|v| {
                if v.is_null() {
                    None
                } else {
                    v.as_str().map(String::from)
                }
            })
            .ok_or_else(|| LocatorError::NotFound(self.locator.selector().to_string()))?;

        let bbox: serde_json::Value = serde_json::from_str(&json_str).map_err(|e| {
            LocatorError::EvaluationError(format!("Failed to parse bounding box: {e}"))
        })?;

        Ok(BoundingBox {
            x: bbox["x"].as_f64().unwrap_or(0.0),
            y: bbox["y"].as_f64().unwrap_or(0.0),
            width: bbox["width"].as_f64().unwrap_or(0.0),
            height: bbox["height"].as_f64().unwrap_or(0.0),
        })
    }
}

/// Element bounding box.
#[derive(Debug, Clone)]
struct BoundingBox {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
