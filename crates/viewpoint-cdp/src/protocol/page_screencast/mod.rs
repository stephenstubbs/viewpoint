//! Page screencast types.
//!
//! Types for Page.startScreencast, Page.stopScreencast, and related events.

use serde::{Deserialize, Serialize};

/// Screencast frame format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ScreencastFormat {
    /// JPEG format.
    #[default]
    Jpeg,
    /// PNG format.
    Png,
}

/// Parameters for Page.startScreencast.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartScreencastParams {
    /// Image compression format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScreencastFormat>,
    /// Compression quality from range [0..100].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    /// Maximum screenshot width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i32>,
    /// Maximum screenshot height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<i32>,
    /// Send every n-th frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub every_nth_frame: Option<i32>,
}

impl StartScreencastParams {
    /// Create new screencast parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the image format.
    #[must_use]
    pub fn format(mut self, format: ScreencastFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Set the compression quality (0-100).
    #[must_use]
    pub fn quality(mut self, quality: i32) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Set the maximum width.
    #[must_use]
    pub fn max_width(mut self, width: i32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set the maximum height.
    #[must_use]
    pub fn max_height(mut self, height: i32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set which frames to capture (every nth frame).
    #[must_use]
    pub fn every_nth_frame(mut self, n: i32) -> Self {
        self.every_nth_frame = Some(n);
        self
    }
}

/// Parameters for Page.stopScreencast (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct StopScreencastParams {}

/// Parameters for Page.screencastFrameAck.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameAckParams {
    /// Frame number to acknowledge.
    pub session_id: i32,
}

/// Event: Page.screencastFrame
///
/// Compressed image data requested by startScreencast.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameEvent {
    /// Base64-encoded compressed image.
    pub data: String,
    /// Screencast frame metadata.
    pub metadata: ScreencastFrameMetadata,
    /// Frame number.
    pub session_id: i32,
}

/// Screencast frame metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencastFrameMetadata {
    /// Top offset in DIP.
    pub offset_top: f64,
    /// Page scale factor.
    pub page_scale_factor: f64,
    /// Device screen width in DIP.
    pub device_width: f64,
    /// Device screen height in DIP.
    pub device_height: f64,
    /// Position of horizontal scroll in CSS pixels.
    pub scroll_offset_x: f64,
    /// Position of vertical scroll in CSS pixels.
    pub scroll_offset_y: f64,
    /// Frame timestamp in seconds.
    pub timestamp: Option<f64>,
}

/// Event: Page.screencastVisibilityChanged
///
/// Fired when the page becomes visible or hidden.
#[derive(Debug, Clone, Deserialize)]
pub struct ScreencastVisibilityChangedEvent {
    /// True if the page is visible.
    pub visible: bool,
}
