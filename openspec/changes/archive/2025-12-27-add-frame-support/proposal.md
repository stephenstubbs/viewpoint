# Change: Add Frame Support

## Why

Modern web applications frequently use iframes for:
- Embedded content (videos, maps, widgets)
- Sandboxed third-party content
- Payment forms and secure inputs
- Advertising
- Legacy content integration

Testing these applications requires interacting with elements inside frames.

This is proposal 6 of 12 in the Playwright feature parity series.

## What Changes

### New Capabilities

1. **Frame Locator** - Locate and interact with iframe content
   - `page.frame_locator(selector)` - create frame locator
   - `frame_locator.locator(selector)` - locate within frame
   - `frame_locator.get_by_*()` - all locator methods within frame
   - Nested frame support

2. **Frame Object** - Direct frame access
   - `page.frame(name_or_url)` - get frame by name or URL
   - `page.frames()` - list all frames
   - `page.main_frame()` - get main frame
   - `frame.child_frames()` - get child frames
   - `frame.parent_frame()` - get parent frame

3. **Frame Navigation** - Navigate within frames
   - `frame.goto(url)` - navigate frame
   - `frame.url()` - get frame URL
   - `frame.name()` - get frame name
   - `frame.content()` - get frame HTML

4. **Frame Events** - Monitor frame lifecycle
   - `page.on('frameattached')` - frame added
   - `page.on('framenavigated')` - frame navigated
   - `page.on('framedetached')` - frame removed

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1-5 | High Priority | Complete |
| **6** | **Frame Support** (this) | **Current** |
| 7-12 | Medium Priority | Pending |

## Impact

- **New specs**: `frames`
- **Affected code**: 
  - `viewpoint-core/src/page/frame.rs` - new Frame type
  - `viewpoint-core/src/page/locator/` - FrameLocator type
  - `viewpoint-cdp/` - frame-related CDP events
- **Breaking changes**: None
- **Dependencies**: Proposal 1 (page operations)
