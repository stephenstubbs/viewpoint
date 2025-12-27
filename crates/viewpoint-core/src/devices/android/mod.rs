//! Android device descriptors (Pixel and Samsung Galaxy).

use super::{BrowserType, DeviceDescriptor};
use crate::context::ViewportSize;

// =============================================================================
// Pixel Devices
// =============================================================================

/// Pixel 7 Pro device descriptor.
pub const PIXEL_7_PRO: DeviceDescriptor = DeviceDescriptor {
    name: "Pixel 7 Pro",
    user_agent: "Mozilla/5.0 (Linux; Android 14; Pixel 7 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 412, height: 915 },
    device_scale_factor: 2.625,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Pixel 7 device descriptor.
pub const PIXEL_7: DeviceDescriptor = DeviceDescriptor {
    name: "Pixel 7",
    user_agent: "Mozilla/5.0 (Linux; Android 14; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 412, height: 915 },
    device_scale_factor: 2.625,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Pixel 6 Pro device descriptor.
pub const PIXEL_6_PRO: DeviceDescriptor = DeviceDescriptor {
    name: "Pixel 6 Pro",
    user_agent: "Mozilla/5.0 (Linux; Android 13; Pixel 6 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 412, height: 892 },
    device_scale_factor: 3.5,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Pixel 6 device descriptor.
pub const PIXEL_6: DeviceDescriptor = DeviceDescriptor {
    name: "Pixel 6",
    user_agent: "Mozilla/5.0 (Linux; Android 13; Pixel 6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 412, height: 915 },
    device_scale_factor: 2.625,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Pixel 5 device descriptor.
pub const PIXEL_5: DeviceDescriptor = DeviceDescriptor {
    name: "Pixel 5",
    user_agent: "Mozilla/5.0 (Linux; Android 12; Pixel 5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 393, height: 851 },
    device_scale_factor: 2.75,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Pixel 4 device descriptor.
pub const PIXEL_4: DeviceDescriptor = DeviceDescriptor {
    name: "Pixel 4",
    user_agent: "Mozilla/5.0 (Linux; Android 11; Pixel 4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 353, height: 745 },
    device_scale_factor: 2.75,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Pixel 7 in landscape orientation.
pub const PIXEL_7_LANDSCAPE: DeviceDescriptor = DeviceDescriptor {
    name: "Pixel 7 Landscape",
    user_agent: "Mozilla/5.0 (Linux; Android 14; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 915, height: 412 },
    device_scale_factor: 2.625,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

// =============================================================================
// Samsung Galaxy Devices
// =============================================================================

/// Samsung Galaxy S23 Ultra device descriptor.
pub const GALAXY_S23_ULTRA: DeviceDescriptor = DeviceDescriptor {
    name: "Galaxy S23 Ultra",
    user_agent: "Mozilla/5.0 (Linux; Android 14; SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 384, height: 854 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Samsung Galaxy S23 device descriptor.
pub const GALAXY_S23: DeviceDescriptor = DeviceDescriptor {
    name: "Galaxy S23",
    user_agent: "Mozilla/5.0 (Linux; Android 14; SM-S911B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 360, height: 780 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Samsung Galaxy S21 device descriptor.
pub const GALAXY_S21: DeviceDescriptor = DeviceDescriptor {
    name: "Galaxy S21",
    user_agent: "Mozilla/5.0 (Linux; Android 12; SM-G991B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    viewport: ViewportSize { width: 360, height: 800 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};

/// Samsung Galaxy Tab S8 device descriptor.
pub const GALAXY_TAB_S8: DeviceDescriptor = DeviceDescriptor {
    name: "Galaxy Tab S8",
    user_agent: "Mozilla/5.0 (Linux; Android 13; SM-X700) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    viewport: ViewportSize { width: 753, height: 1205 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Chromium,
};
