//! iPhone device descriptors.

use super::{BrowserType, DeviceDescriptor};
use crate::context::ViewportSize;

/// iPhone 14 Pro Max device descriptor.
pub const IPHONE_14_PRO_MAX: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 14 Pro Max",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 430, height: 932 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 14 Pro device descriptor.
pub const IPHONE_14_PRO: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 14 Pro",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 393, height: 852 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 14 device descriptor.
pub const IPHONE_14: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 14",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 390, height: 844 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 13 Pro Max device descriptor.
pub const IPHONE_13_PRO_MAX: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 13 Pro Max",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 428, height: 926 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 13 Pro device descriptor.
pub const IPHONE_13_PRO: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 13 Pro",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 390, height: 844 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 13 device descriptor.
pub const IPHONE_13: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 13",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 390, height: 844 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 13 Mini device descriptor.
pub const IPHONE_13_MINI: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 13 Mini",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 375, height: 812 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 12 Pro Max device descriptor.
pub const IPHONE_12_PRO_MAX: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 12 Pro Max",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 428, height: 926 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 12 Pro device descriptor.
pub const IPHONE_12_PRO: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 12 Pro",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 390, height: 844 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 12 device descriptor.
pub const IPHONE_12: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 12",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 390, height: 844 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 12 Mini device descriptor.
pub const IPHONE_12_MINI: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 12 Mini",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 375, height: 812 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 11 Pro Max device descriptor.
pub const IPHONE_11_PRO_MAX: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 11 Pro Max",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 414, height: 896 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 11 Pro device descriptor.
pub const IPHONE_11_PRO: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 11 Pro",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 375, height: 812 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 11 device descriptor.
pub const IPHONE_11: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 11",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 414, height: 896 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone SE (3rd generation) device descriptor.
pub const IPHONE_SE_3: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone SE 3rd Gen",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 375, height: 667 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone SE (2nd generation) device descriptor.
pub const IPHONE_SE: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone SE",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 375, height: 667 },
    device_scale_factor: 2.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};

/// iPhone 13 in landscape orientation.
pub const IPHONE_13_LANDSCAPE: DeviceDescriptor = DeviceDescriptor {
    name: "iPhone 13 Landscape",
    user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1",
    viewport: ViewportSize { width: 844, height: 390 },
    device_scale_factor: 3.0,
    is_mobile: true,
    has_touch: true,
    default_browser_type: BrowserType::Webkit,
};
