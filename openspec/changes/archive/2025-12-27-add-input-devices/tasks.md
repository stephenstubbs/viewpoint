# Tasks: Add Input Devices

## 1. CDP Protocol Extensions

- [x] 1.1 Add `Input.dispatchKeyEvent` command (keyDown, keyUp, char types)
- [x] 1.2 Add `Input.insertText` command
- [x] 1.3 Add `Input.dispatchMouseEvent` command (mouseMoved, mousePressed, mouseReleased, mouseWheel)
- [x] 1.4 Add `Input.dispatchTouchEvent` command (touchStart, touchEnd, touchMove, touchCancel)
- [x] 1.5 Add `Input.dispatchDragEvent` command
- [x] 1.6 Define key code mappings for all Playwright key names

## 2. Keyboard Type Implementation

- [x] 2.1 Create `Keyboard` struct owned by Page
- [x] 2.2 Implement modifier key state tracking (Shift, Control, Alt, Meta)
- [x] 2.3 Implement key name parsing (F1-F12, Digit0-9, KeyA-Z, etc.)
- [x] 2.4 Implement key combination parsing (Control+C, Shift+Tab, etc.)
- [x] 2.5 Implement `ControlOrMeta` platform detection

## 3. Keyboard Press Implementation

- [x] 3.1 Implement `Keyboard::press(key)` dispatching keyDown + keyUp
- [x] 3.2 Implement delay option between keyDown and keyUp
- [x] 3.3 Implement automatic Shift for uppercase letters
- [x] 3.4 Implement modifier key combinations

## 4. Keyboard Down/Up Implementation

- [x] 4.1 Implement `Keyboard::down(key)` holding modifier state
- [x] 4.2 Implement `Keyboard::up(key)` releasing modifier state
- [x] 4.3 Implement repeat flag for held keys
- [x] 4.4 Track held keys for proper state management

## 5. Keyboard Type/Insert Implementation

- [x] 5.1 Implement `Keyboard::type_text(text)` with key events per character
- [x] 5.2 Implement delay option between characters
- [x] 5.3 Implement `Keyboard::insert_text(text)` without key events
- [x] 5.4 Handle non-ASCII characters in type/insert

## 6. Mouse Type Implementation

- [x] 6.1 Create `Mouse` struct owned by Page
- [x] 6.2 Implement position tracking (current x, y)
- [x] 6.3 Implement `MouseButton` enum (Left, Right, Middle)
- [x] 6.4 Implement button state tracking

## 7. Mouse Move Implementation

- [x] 7.1 Implement `Mouse::move_(x, y)` single move
- [x] 7.2 Implement steps option for smooth movement
- [x] 7.3 Update internal position tracking on move

## 8. Mouse Click Implementation

- [x] 8.1 Implement `Mouse::click(x, y)` with move + down + up
- [x] 8.2 Implement button option (left, right, middle)
- [x] 8.3 Implement delay option between down and up
- [x] 8.4 Implement click_count option for multi-click

## 9. Mouse Down/Up Implementation

- [x] 9.1 Implement `Mouse::down()` at current position
- [x] 9.2 Implement `Mouse::up()` at current position
- [x] 9.3 Implement button option for down/up
- [x] 9.4 Implement click_count for event detail

## 10. Mouse Double Click Implementation

- [x] 10.1 Implement `Mouse::dblclick(x, y)`
- [x] 10.2 Dispatch proper event sequence with dblclick event

## 11. Mouse Wheel Implementation

- [x] 11.1 Implement `Mouse::wheel(deltaX, deltaY)`
- [x] 11.2 Handle scroll behavior triggered by wheel

## 12. Touchscreen Type Implementation

- [x] 12.1 Create `Touchscreen` struct owned by Page
- [x] 12.2 Implement hasTouch context check
- [x] 12.3 Generate helpful error when touch not enabled

## 13. Touchscreen Tap Implementation

- [x] 13.1 Implement `Touchscreen::tap(x, y)` with touchStart + touchEnd
- [x] 13.2 Generate proper touch identifier
- [x] 13.3 Create TouchList with single touch point
- [x] 13.4 Set correct clientX, clientY coordinates

## 14. Locator Tap Implementation

- [x] 14.1 Implement `Locator::tap()` using touchscreen
- [x] 14.2 Implement position option for offset tap
- [x] 14.3 Implement force option to bypass actionability
- [x] 14.4 Implement modifiers option

## 15. Drag and Drop Implementation

- [x] 15.1 Implement `Page::drag_and_drop(source, target)`
- [x] 15.2 Implement `Locator::drag_to(target)`
- [x] 15.3 Implement source_position option
- [x] 15.4 Implement target_position option
- [x] 15.5 Implement steps option for smooth drag

## 16. Page Input Accessors

- [x] 16.1 Implement `Page::keyboard()` returning &Keyboard
- [x] 16.2 Implement `Page::mouse()` returning &Mouse
- [x] 16.3 Implement `Page::touchscreen()` returning &Touchscreen
- [x] 16.4 Initialize input devices on Page creation

## 17. Testing

- [x] 17.1 Add tests for keyboard.press with various keys
- [x] 17.2 Add tests for keyboard.down/up modifier state
- [x] 17.3 Add tests for keyboard.type_text
- [x] 17.4 Add tests for keyboard.insert_text
- [x] 17.5 Add tests for key combinations
- [x] 17.6 Add tests for mouse.move with steps
- [x] 17.7 Add tests for mouse.click with options
- [x] 17.8 Add tests for mouse.dblclick
- [x] 17.9 Add tests for mouse.wheel scrolling
- [x] 17.10 Add tests for drag and drop
- [x] 17.11 Add tests for touchscreen.tap
- [x] 17.12 Add tests for locator.tap

## 18. Documentation

- [x] 18.1 Document keyboard key names
- [x] 18.2 Document mouse coordinate system
- [x] 18.3 Document touch context requirements
- [x] 18.4 Add examples for common input patterns

## Dependencies

- All implementation tasks depend on CDP extensions (1.x)
- Keyboard (2-5) and Mouse (6-11) are independent
- Touchscreen (12-14) is independent
- Drag and drop (15) depends on Mouse (6-11)
- Page accessors (16) can be done anytime after types exist

## Parallelizable Work

After CDP extensions:
- Keyboard (2-5), Mouse (6-11), and Touchscreen (12-14) are independent
- Testing tasks can run as implementations complete
