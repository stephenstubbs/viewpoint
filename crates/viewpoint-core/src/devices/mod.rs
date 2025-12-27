//! Device descriptors for device emulation.
//!
//! This module provides predefined device descriptors that match common devices
//! like iPhones, iPads, Pixel phones, and desktop configurations.
//!
//! # Example
//!
//! ```no_run
//! use viewpoint_core::{Browser, devices};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! let browser = Browser::launch().headless(true).launch().await?;
//!
//! let context = browser.new_context_builder()
//!     .device(devices::IPHONE_13)
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```

mod android;
mod desktop;
mod ipad;
mod iphone;

pub use android::*;
pub use desktop::*;
pub use ipad::*;
pub use iphone::*;

use crate::context::ViewportSize;

/// Browser type for device descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BrowserType {
    /// Chromium-based browser.
    #[default]
    Chromium,
    /// Firefox browser.
    Firefox,
    /// `WebKit` browser.
    Webkit,
}

/// A device descriptor containing all properties needed to emulate a device.
///
/// Device descriptors include viewport dimensions, user agent strings,
/// device scale factor, and touch/mobile capabilities.
#[derive(Debug, Clone)]
pub struct DeviceDescriptor {
    /// Human-readable device name.
    pub name: &'static str,
    /// User agent string for this device.
    pub user_agent: &'static str,
    /// Viewport dimensions.
    pub viewport: ViewportSize,
    /// Device pixel ratio (scale factor).
    pub device_scale_factor: f64,
    /// Whether the device is a mobile device.
    pub is_mobile: bool,
    /// Whether the device has touch capability.
    pub has_touch: bool,
    /// Default browser type for this device.
    pub default_browser_type: BrowserType,
}

impl DeviceDescriptor {
    /// Create a new device descriptor.
    pub const fn new(
        name: &'static str,
        user_agent: &'static str,
        viewport: ViewportSize,
        device_scale_factor: f64,
        is_mobile: bool,
        has_touch: bool,
        default_browser_type: BrowserType,
    ) -> Self {
        Self {
            name,
            user_agent,
            viewport,
            device_scale_factor,
            is_mobile,
            has_touch,
            default_browser_type,
        }
    }
}

/// Get a list of all available device descriptors.
pub fn all_devices() -> Vec<&'static DeviceDescriptor> {
    vec![
        // iPhones
        &IPHONE_14_PRO_MAX,
        &IPHONE_14_PRO,
        &IPHONE_14,
        &IPHONE_13_PRO_MAX,
        &IPHONE_13_PRO,
        &IPHONE_13,
        &IPHONE_13_MINI,
        &IPHONE_12_PRO_MAX,
        &IPHONE_12_PRO,
        &IPHONE_12,
        &IPHONE_12_MINI,
        &IPHONE_11_PRO_MAX,
        &IPHONE_11_PRO,
        &IPHONE_11,
        &IPHONE_SE_3,
        &IPHONE_SE,
        &IPHONE_13_LANDSCAPE,
        // iPads
        &IPAD_PRO_12_9,
        &IPAD_PRO_11,
        &IPAD_AIR,
        &IPAD,
        &IPAD_MINI,
        &IPAD_PRO_11_LANDSCAPE,
        // Pixels
        &PIXEL_7_PRO,
        &PIXEL_7,
        &PIXEL_6_PRO,
        &PIXEL_6,
        &PIXEL_5,
        &PIXEL_4,
        &PIXEL_7_LANDSCAPE,
        // Samsung
        &GALAXY_S23_ULTRA,
        &GALAXY_S23,
        &GALAXY_S21,
        &GALAXY_TAB_S8,
        // Desktop
        &DESKTOP_CHROME,
        &DESKTOP_CHROME_HIDPI,
        &DESKTOP_SAFARI,
        &DESKTOP_FIREFOX,
        &DESKTOP_EDGE,
    ]
}

/// Find a device descriptor by name (case-insensitive).
pub fn find_device(name: &str) -> Option<&'static DeviceDescriptor> {
    let name_lower = name.to_lowercase();
    all_devices()
        .into_iter()
        .find(|d| d.name.to_lowercase() == name_lower)
}

#[cfg(test)]
mod tests;
