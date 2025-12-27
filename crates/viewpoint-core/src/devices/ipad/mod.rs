//! iPad device descriptors.

use super::{BrowserType, DeviceDescriptor};
use crate::context::ViewportSize;

/// iPad Pro 12.9" (6th generation) device descriptor.
pub const IPAD_PRO_12_9: DeviceDescriptor = DeviceDescriptor {
    name: "iPad Pro 12.9",
    user_agent: "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 1024, height: 1366 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPad Pro 11" device descriptor.
pub const IPAD_PRO_11: DeviceDescriptor = DeviceDescriptor {
    name: "iPad Pro 11",
    user_agent: "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 834, height: 1194 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPad Air (5th generation) device descriptor.
pub const IPAD_AIR: DeviceDescriptor = DeviceDescriptor {
    name: "iPad Air",
    user_agent: "Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 820, height: 1180 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPad (10th generation) device descriptor.
pub const IPAD: DeviceDescriptor = DeviceDescriptor {
    name: "iPad",
    user_agent: "Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 820, height: 1180 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPad Mini (6th generation) device descriptor.
pub const IPAD_MINI: DeviceDescriptor = DeviceDescriptor {
    name: "iPad Mini",
    user_agent: "Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 744, height: 1133 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPad Pro 11" in landscape orientation.
pub const IPAD_PRO_11_LANDSCAPE: DeviceDescriptor = DeviceDescriptor {
    name: "iPad Pro 11 Landscape",
    user_agent: "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 1194, height: 834 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};
