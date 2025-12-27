# Change: Add Emulation Features

## Why

Testing responsive designs and device-specific features requires:
- **Device Emulation**: Simulate mobile devices, tablets
- **Media Features**: Dark mode, reduced motion, print media
- **Locale/Timezone**: Test internationalization
- **Color Scheme**: Light/dark mode testing

This is proposal 9 of 12 in the Playwright feature parity series.

## What Changes

### New Capabilities

1. **Device Emulation** - Simulate specific devices
   - Predefined device descriptors (iPhone, Pixel, iPad, etc.)
   - Custom viewport, user agent, device scale factor
   - Touch emulation, mobile mode

2. **Media Emulation** - Override CSS media features
   - `page.emulate_media()` - set media type (screen, print)
   - Color scheme (light, dark, no-preference)
   - Reduced motion preference
   - Forced colors

3. **Locale & Timezone** - Geographic settings
   - Context option: `locale`
   - Context option: `timezone_id`
   - Affects date formatting, language

4. **Vision Deficiency** - Accessibility testing
   - Emulate color blindness types
   - Test for color accessibility

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1-8 | Previous | Complete |
| **9** | **Emulation Features** (this) | **Current** |
| 10-12 | Remaining | Pending |

## Impact

- **New specs**: `device-emulation`, `media-emulation`
- **Affected code**: 
  - `viewpoint-core/src/page/` - emulation methods
  - `viewpoint-core/src/devices.rs` - device descriptors
  - `viewpoint-cdp/` - Emulation domain
- **Breaking changes**: None
- **Dependencies**: Proposal 4 (context features)
