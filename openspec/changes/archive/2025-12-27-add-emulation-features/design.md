# Design: Emulation Features

## Context

Emulation allows testing how applications behave on different devices, in different locales, and under different media conditions.

## Goals

- Provide device descriptors matching Playwright
- Support all CSS media feature emulation
- Enable locale and timezone testing
- Support accessibility testing with vision deficiency emulation

## Decisions

### Decision 1: Device Descriptors

**Choice**: Include predefined device descriptors matching Playwright.

```rust
use viewpoint::devices;

let context = browser.new_context()
    .device(devices::IPHONE_13)
    .build()
    .await?;
```

### Decision 2: Context vs Page Emulation

**Choice**: Device settings at context level, media at page level.

**Rationale**:
- Device properties (viewport, UA) are typically context-wide
- Media preferences can change per page
- Matches Playwright behavior

### Decision 3: Device Descriptor Fields

```rust
pub struct DeviceDescriptor {
    pub name: &'static str,
    pub user_agent: &'static str,
    pub viewport: Viewport,
    pub device_scale_factor: f64,
    pub is_mobile: bool,
    pub has_touch: bool,
    pub default_browser_type: BrowserType,
}
```

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| Viewport | Emulation.setDeviceMetricsOverride | Emulation |
| User agent | Emulation.setUserAgentOverride | Emulation |
| Touch | Emulation.setTouchEmulationEnabled | Emulation |
| Media | Emulation.setEmulatedMedia | Emulation |
| Timezone | Emulation.setTimezoneOverride | Emulation |
| Locale | Emulation.setLocaleOverride | Emulation |
| Vision | Emulation.setEmulatedVisionDeficiency | Emulation |

## Open Questions

None.
