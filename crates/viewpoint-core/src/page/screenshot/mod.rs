//! Screenshot capture functionality.
//!
//! This module provides the `ScreenshotBuilder` for capturing page screenshots.

use std::path::Path;

use tracing::{debug, info, instrument};
use viewpoint_cdp::protocol::page::{
    CaptureScreenshotParams, CaptureScreenshotResult, ScreenshotFormat as CdpScreenshotFormat,
    Viewport,
};

/// Image format for screenshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenshotFormat {
    /// PNG format (default).
    #[default]
    Png,
    /// JPEG format.
    Jpeg,
    /// WebP format.
    Webp,
}

impl From<ScreenshotFormat> for CdpScreenshotFormat {
    fn from(format: ScreenshotFormat) -> Self {
        match format {
            ScreenshotFormat::Png => CdpScreenshotFormat::Png,
            ScreenshotFormat::Jpeg => CdpScreenshotFormat::Jpeg,
            ScreenshotFormat::Webp => CdpScreenshotFormat::Webp,
        }
    }
}

use crate::error::PageError;

use super::Page;

/// Animation handling mode for screenshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Animations {
    /// Allow animations to run.
    #[default]
    Allow,
    /// Disable animations before capture.
    Disabled,
}

/// Clip region for screenshots.
#[derive(Debug, Clone, Copy)]
pub struct ClipRegion {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Width.
    pub width: f64,
    /// Height.
    pub height: f64,
}

impl ClipRegion {
    /// Create a new clip region.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Builder for capturing screenshots.
#[derive(Debug, Clone)]
pub struct ScreenshotBuilder<'a> {
    page: &'a Page,
    format: ScreenshotFormat,
    quality: Option<u8>,
    full_page: bool,
    clip: Option<ClipRegion>,
    path: Option<String>,
    omit_background: bool,
    animations: Animations,
    capture_beyond_viewport: bool,
}

impl<'a> ScreenshotBuilder<'a> {
    /// Create a new screenshot builder.
    pub(crate) fn new(page: &'a Page) -> Self {
        Self {
            page,
            format: ScreenshotFormat::Png,
            quality: None,
            full_page: false,
            clip: None,
            path: None,
            omit_background: false,
            animations: Animations::default(),
            capture_beyond_viewport: false,
        }
    }

    /// Set the image format to PNG.
    #[must_use]
    pub fn png(mut self) -> Self {
        self.format = ScreenshotFormat::Png;
        self
    }

    /// Set the image format to JPEG with optional quality (0-100).
    #[must_use]
    pub fn jpeg(mut self, quality: Option<u8>) -> Self {
        self.format = ScreenshotFormat::Jpeg;
        self.quality = quality;
        self
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

    /// Capture the full scrollable page instead of just the viewport.
    #[must_use]
    pub fn full_page(mut self, full_page: bool) -> Self {
        self.full_page = full_page;
        self.capture_beyond_viewport = full_page;
        self
    }

    /// Clip the screenshot to a specific region.
    #[must_use]
    pub fn clip(mut self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.clip = Some(ClipRegion::new(x, y, width, height));
        self
    }

    /// Clip the screenshot using a `ClipRegion`.
    #[must_use]
    pub fn clip_region(mut self, region: ClipRegion) -> Self {
        self.clip = Some(region);
        self
    }

    /// Save the screenshot to a file.
    #[must_use]
    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_string_lossy().to_string());
        self
    }

    /// Set whether to omit the background (transparent).
    /// Note: Only applicable to PNG format.
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

    /// Capture the screenshot.
    ///
    /// Returns the screenshot as a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - The CDP command fails
    /// - File saving fails (if a path was specified)
    #[instrument(level = "info", skip(self), fields(format = ?self.format, full_page = self.full_page, has_path = self.path.is_some()))]
    pub async fn capture(self) -> Result<Vec<u8>, PageError> {
        if self.page.is_closed() {
            return Err(PageError::Closed);
        }

        info!("Capturing screenshot");

        // Disable animations if requested
        if self.animations == Animations::Disabled {
            debug!("Disabling animations");
            self.disable_animations().await?;
        }

        // Build CDP parameters
        let clip = if self.full_page {
            // For full page, we need to get the full page dimensions first
            let dimensions = self.get_full_page_dimensions().await?;
            debug!(
                width = dimensions.0,
                height = dimensions.1,
                "Full page dimensions"
            );
            Some(Viewport {
                x: 0.0,
                y: 0.0,
                width: dimensions.0,
                height: dimensions.1,
                scale: 1.0,
            })
        } else {
            self.clip.map(|c| Viewport {
                x: c.x,
                y: c.y,
                width: c.width,
                height: c.height,
                scale: 1.0,
            })
        };

        let params = CaptureScreenshotParams {
            format: Some(self.format.into()),
            quality: self.quality,
            clip,
            from_surface: Some(true),
            capture_beyond_viewport: Some(self.capture_beyond_viewport),
            optimize_for_speed: None,
        };

        debug!("Sending Page.captureScreenshot command");
        let result: CaptureScreenshotResult = self
            .page
            .connection()
            .send_command(
                "Page.captureScreenshot",
                Some(params),
                Some(self.page.session_id()),
            )
            .await?;

        // Re-enable animations if they were disabled
        if self.animations == Animations::Disabled {
            debug!("Re-enabling animations");
            self.enable_animations().await?;
        }

        // Decode base64 data
        let data = base64_decode(&result.data)?;
        debug!(bytes = data.len(), "Screenshot captured");

        // Save to file if path specified
        if let Some(ref path) = self.path {
            debug!(path = path, "Saving screenshot to file");
            tokio::fs::write(path, &data).await.map_err(|e| {
                PageError::EvaluationFailed(format!("Failed to save screenshot: {e}"))
            })?;
            info!(path = path, "Screenshot saved");
        }

        Ok(data)
    }

    /// Get the full page dimensions.
    async fn get_full_page_dimensions(&self) -> Result<(f64, f64), PageError> {
        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .page
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: r"
                        JSON.stringify({
                            width: Math.max(
                                document.body.scrollWidth,
                                document.documentElement.scrollWidth,
                                document.body.offsetWidth,
                                document.documentElement.offsetWidth,
                                document.body.clientWidth,
                                document.documentElement.clientWidth
                            ),
                            height: Math.max(
                                document.body.scrollHeight,
                                document.documentElement.scrollHeight,
                                document.body.offsetHeight,
                                document.documentElement.offsetHeight,
                                document.body.clientHeight,
                                document.documentElement.clientHeight
                            )
                        })
                    "
                    .to_string(),
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

        let json_str = result
            .result
            .value
            .and_then(|v| v.as_str().map(String::from))
            .ok_or_else(|| {
                PageError::EvaluationFailed("Failed to get page dimensions".to_string())
            })?;

        let dimensions: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| PageError::EvaluationFailed(format!("Failed to parse dimensions: {e}")))?;

        let width = dimensions["width"].as_f64().unwrap_or(800.0);
        let height = dimensions["height"].as_f64().unwrap_or(600.0);

        Ok((width, height))
    }

    /// Disable CSS animations.
    async fn disable_animations(&self) -> Result<(), PageError> {
        let script = r"
            (function() {
                const style = document.createElement('style');
                style.id = '__viewpoint_disable_animations__';
                style.textContent = '*, *::before, *::after { animation-duration: 0s !important; animation-delay: 0s !important; transition-duration: 0s !important; transition-delay: 0s !important; }';
                document.head.appendChild(style);
            })()
        ";

        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: script.to_string(),
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

        Ok(())
    }

    /// Re-enable CSS animations.
    async fn enable_animations(&self) -> Result<(), PageError> {
        let script = r"
            (function() {
                const style = document.getElementById('__viewpoint_disable_animations__');
                if (style) style.remove();
            })()
        ";

        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: script.to_string(),
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

        Ok(())
    }
}

/// Decode base64 string to bytes.
pub(crate) fn base64_decode(input: &str) -> Result<Vec<u8>, PageError> {
    // Simple base64 decode implementation
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    fn decode_char(c: u8) -> Option<u8> {
        ALPHABET.iter().position(|&x| x == c).map(|p| p as u8)
    }

    let input = input.as_bytes();
    let mut output = Vec::with_capacity(input.len() * 3 / 4);

    let mut buffer = 0u32;
    let mut bits = 0u8;

    for &byte in input {
        if byte == b'=' {
            break;
        }
        if byte == b'\n' || byte == b'\r' || byte == b' ' {
            continue;
        }

        let val = decode_char(byte)
            .ok_or_else(|| PageError::EvaluationFailed("Invalid base64 character".to_string()))?;

        buffer = (buffer << 6) | u32::from(val);
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
        }
    }

    Ok(output)
}
