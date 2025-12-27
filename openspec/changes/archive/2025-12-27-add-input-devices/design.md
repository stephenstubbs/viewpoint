# Design: Input Devices

## Context

Input devices provide low-level control over keyboard, mouse, and touch input. While most tests should use high-level locator actions, direct input control is essential for complex interactions, canvas/game testing, and edge cases.

## Goals

- Provide complete keyboard, mouse, and touchscreen control matching Playwright
- Maintain state consistency (modifier keys, mouse position, button state)
- Support all key names and combinations
- Enable complex drag-and-drop operations

## Non-Goals

- Multi-touch gestures (Playwright's Touchscreen is also limited to single touch)
- Gamepad/joystick input
- Pen/stylus input (beyond touch)

## Decisions

### Decision 1: Input Device Ownership

**Choice**: Each Page owns its Keyboard, Mouse, and Touchscreen instances.

```rust
// Access via page
page.keyboard().press("Enter").await?;
page.mouse().click(100.0, 200.0).await?;
page.touchscreen().tap(100.0, 200.0).await?;
```

**Rationale**:
- Input is page-specific (each page has its own focus, coordinates)
- Matches Playwright's `page.keyboard`, `page.mouse`, `page.touchscreen`
- Allows tracking state per-page (mouse position, held keys)

### Decision 2: Key Name Handling

**Choice**: Accept both key names and single characters, matching Playwright's key specification.

**Key Names**:
- Function keys: `F1` - `F12`
- Digits: `Digit0` - `Digit9`
- Letters: `KeyA` - `KeyZ`
- Navigation: `ArrowLeft`, `ArrowRight`, `ArrowUp`, `ArrowDown`, `Home`, `End`, `PageUp`, `PageDown`
- Editing: `Backspace`, `Delete`, `Enter`, `Tab`, `Escape`, `Insert`
- Modifiers: `Shift`, `Control`, `Alt`, `Meta`, `ShiftLeft`, `ControlOrMeta`
- Special: `Backquote`, `Minus`, `Equal`, `Backslash`, `BracketLeft`, `BracketRight`

**Single Characters**:
- `a` - `z` (case-sensitive, lowercase)
- `A` - `Z` (case-sensitive, uppercase, implies Shift)
- Symbols and numbers

**Combinations**:
- `Control+C`, `Shift+Tab`, `Control+Shift+T`
- `ControlOrMeta+A` (Control on Windows/Linux, Meta on macOS)

### Decision 3: Coordinate System

**Choice**: All coordinates are CSS pixels relative to the viewport's top-left corner.

**Rationale**:
- Matches Playwright's coordinate system
- CSS pixels abstract away device pixel ratio
- Viewport-relative is intuitive and consistent

**Note**: For element-relative coordinates, use locator bounding box + offset.

### Decision 4: Modifier Key State Tracking

**Choice**: Track modifier key state internally; `keyboard.down()` sets state, `keyboard.up()` clears it.

```rust
// Hold Shift while pressing keys
page.keyboard().down("Shift").await?;
page.keyboard().press("A").await?; // Types 'A' (uppercase)
page.keyboard().press("B").await?; // Types 'B' (uppercase)
page.keyboard().up("Shift").await?;
page.keyboard().press("a").await?; // Types 'a' (lowercase)
```

**Rationale**:
- Matches real keyboard behavior
- Allows complex key sequences
- State persists until explicitly released

### Decision 5: Mouse Button Enum

**Choice**: Use enum for mouse buttons with string parsing for API convenience.

```rust
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

// API accepts both
mouse.click(100.0, 200.0).button(MouseButton::Right).await?;
mouse.click(100.0, 200.0).button("right").await?;
```

### Decision 6: Drag and Drop Architecture

**Choice**: Provide both element-based and coordinate-based drag operations.

```rust
// Element-based (high-level)
page.drag_and_drop("#source", "#target").await?;
source_locator.drag_to(target_locator).await?;

// Coordinate-based (low-level) - compose from mouse operations
page.mouse().move_(100.0, 100.0).await?;
page.mouse().down().await?;
page.mouse().move_(200.0, 200.0).steps(10).await?;
page.mouse().up().await?;
```

**Rationale**:
- High-level API for common cases
- Low-level API for custom drag behavior
- Steps option for smooth animation

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| Key down | Input.dispatchKeyEvent (keyDown) | Input |
| Key up | Input.dispatchKeyEvent (keyUp) | Input |
| Key press | Input.dispatchKeyEvent (char) | Input |
| Insert text | Input.insertText | Input |
| Mouse move | Input.dispatchMouseEvent (mouseMoved) | Input |
| Mouse down | Input.dispatchMouseEvent (mousePressed) | Input |
| Mouse up | Input.dispatchMouseEvent (mouseReleased) | Input |
| Mouse wheel | Input.dispatchMouseEvent (mouseWheel) | Input |
| Touch start | Input.dispatchTouchEvent (touchStart) | Input |
| Touch end | Input.dispatchTouchEvent (touchEnd) | Input |
| Drag and drop | Input.dispatchDragEvent | Input |

## Risks / Trade-offs

### Risk: Platform Key Differences

**Mitigation**:
- Use `ControlOrMeta` for cross-platform shortcuts
- Document platform-specific key names
- Map common names to platform-appropriate keys

### Risk: Mouse Position Drift

**Mitigation**:
- Track current position internally
- Provide absolute positioning (not just relative moves)
- Reset position on page navigation if needed

### Risk: Touch Requires Context Configuration

**Mitigation**:
- Document that `hasTouch: true` is required
- Provide helpful error message if touch used without hasTouch
- Consider auto-enabling in future

## Open Questions

None - all design decisions resolved.
