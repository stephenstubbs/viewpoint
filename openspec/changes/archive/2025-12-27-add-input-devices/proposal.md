# Change: Add Input Devices

## Why

While Viewpoint supports element-based actions (click, fill, etc.), many testing scenarios require direct control over input devices:
- **Complex Interactions**: Drag-and-drop, drawing, gesture simulation
- **Keyboard Shortcuts**: Testing hotkeys, key combinations, modifier keys
- **Precise Mouse Control**: Hover effects, tooltip testing, coordinate-based clicks
- **Touch Emulation**: Mobile app testing, swipe gestures, multi-touch
- **Game/Canvas Testing**: Direct input for non-DOM interactions

This is proposal 3 of 11 in the Playwright feature parity series (Chromium only).

## What Changes

### New Capabilities

1. **Keyboard** - Direct keyboard input control
   - `keyboard.press(key)` - press and release a key
   - `keyboard.down(key)` - press and hold a key
   - `keyboard.up(key)` - release a held key
   - `keyboard.type(text)` - type text character by character
   - `keyboard.insertText(text)` - insert text without key events
   - Modifier key support (Shift, Control, Alt, Meta)
   - Key combinations (e.g., "Control+C")

2. **Mouse** - Direct mouse control
   - `mouse.move(x, y)` - move to coordinates
   - `mouse.click(x, y)` - click at coordinates
   - `mouse.dblclick(x, y)` - double-click
   - `mouse.down()` / `mouse.up()` - button press/release
   - `mouse.wheel(deltaX, deltaY)` - scroll wheel
   - Button selection (left, right, middle)
   - Click count for multi-click

3. **Touchscreen** - Touch input emulation
   - `touchscreen.tap(x, y)` - tap at coordinates
   - Requires `hasTouch: true` in context options

4. **Drag and Drop** - Combined mouse operations
   - `page.drag_and_drop(source, target)` - element-based drag
   - `locator.drag_to(target)` - locator-based drag
   - Custom source/target positions
   - Multi-step drag operations

## Relationship to Existing Actions

The existing locator actions (`click()`, `fill()`, etc.) use these input devices internally. This proposal exposes the low-level APIs for cases where element-based actions aren't sufficient.

| Level | API | Use Case |
|-------|-----|----------|
| High | `locator.click()` | Click on element (recommended) |
| Low | `mouse.click(x, y)` | Click at coordinates |
| High | `locator.fill(text)` | Fill input field (recommended) |
| Low | `keyboard.type(text)` | Type into focused element |

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1 | Core Page Operations | Complete |
| 2 | Network Interception | Complete |
| **3** | **Input Devices** (this) | **Current** |
| 4 | Browser Context Features | Pending |
| 5-11 | ... | Pending |

## Impact

- **New specs**: `keyboard`, `mouse`, `touchscreen`
- **Affected code**: 
  - `viewpoint-cdp/src/protocol/input.rs` - Input domain commands
  - `viewpoint-core/src/page/keyboard.rs` - new Keyboard type
  - `viewpoint-core/src/page/mouse.rs` - new Mouse type
  - `viewpoint-core/src/page/touchscreen.rs` - new Touchscreen type
  - `viewpoint-core/src/page/mod.rs` - expose input device accessors
- **Breaking changes**: None
- **Dependencies**: None (can be implemented in parallel with proposals 1-2)
