//! Desktop browser device descriptors.

use super::{BrowserType, DeviceDescriptor};
use crate::context::ViewportSize;

/// Desktop Chrome browser device descriptor.
pub const DESKTOP_CHROME: DeviceDescriptor = DeviceDescriptor {
    name: "Desktop Chrome",
    user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    viewport: ViewportSize {
        width: 1280,
        height: 720,
    },
    device_scale_factor: 1.0,
    is_mobile: false,
    has_touch: false,
    default_browser_type: BrowserType::Chromium,
};

/// Desktop Chrome `HiDPI` (Retina) device descriptor.
pub const DESKTOP_CHROME_HIDPI: DeviceDescriptor = DeviceDescriptor {
    name: "Desktop Chrome HiDPI",
    user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    viewport: ViewportSize {
        width: 1280,
        height: 720,
    },
    device_scale_factor: 2.0,
    is_mobile: false,
    has_touch: false,
    default_browser_type: BrowserType::Chromium,
};

/// Desktop Safari device descriptor.
pub const DESKTOP_SAFARI: DeviceDescriptor = DeviceDescriptor {
    name: "Desktop Safari",
    user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    viewport: ViewportSize {
        width: 1280,
        height: 720,
    },
    device_scale_factor: 2.0,
    is_mobile: false,
    has_touch: false,
    default_browser_type: BrowserType::Webkit,
};

/// Desktop Firefox device descriptor.
pub const DESKTOP_FIREFOX: DeviceDescriptor = DeviceDescriptor {
    name: "Desktop Firefox",
    user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    viewport: ViewportSize {
        width: 1280,
        height: 720,
    },
    device_scale_factor: 1.0,
    is_mobile: false,
    has_touch: false,
    default_browser_type: BrowserType::Firefox,
};

/// Desktop Edge device descriptor.
pub const DESKTOP_EDGE: DeviceDescriptor = DeviceDescriptor {
    name: "Desktop Edge",
    user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
    viewport: ViewportSize {
        width: 1280,
        height: 720,
    },
    device_scale_factor: 1.0,
    is_mobile: false,
    has_touch: false,
    default_browser_type: BrowserType::Chromium,
};
