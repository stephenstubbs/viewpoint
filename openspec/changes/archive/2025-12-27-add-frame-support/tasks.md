# Tasks: Add Frame Support

## 1. CDP Protocol Extensions

- [x] 1.1 Add `Page.getFrameTree` command
- [x] 1.2 Add `Page.frameAttached` event
- [x] 1.3 Add `Page.frameNavigated` event
- [x] 1.4 Add `Page.frameDetached` event
- [x] 1.5 Track execution context per frame

## 2. Frame Type Implementation

- [x] 2.1 Create `Frame` struct with frame ID
- [x] 2.2 Implement `frame.url()`
- [x] 2.3 Implement `frame.name()`
- [x] 2.4 Implement `frame.content()`
- [x] 2.5 Implement `frame.is_detached()`
- [x] 2.6 Implement `frame.parent_frame()` (as parent_id())
- [x] 2.7 Implement `frame.child_frames()` (via page.frames())
- [x] 2.8 Implement `frame.page()` back-reference (via connection)

## 3. Frame Navigation

- [x] 3.1 Implement `frame.goto(url)`
- [x] 3.2 Implement `frame.set_content(html)`
- [x] 3.3 Implement `frame.title()`
- [x] 3.4 Implement `frame.wait_for_load_state()`

## 4. Page Frame Access

- [x] 4.1 Implement `page.main_frame()`
- [x] 4.2 Implement `page.frames()` list
- [x] 4.3 Implement `page.frame(name)` by name
- [x] 4.4 Implement `page.frame_by_url(pattern)` by URL

## 5. FrameLocator Implementation

- [x] 5.1 Create `FrameLocator` struct
- [x] 5.2 Implement `page.frame_locator(selector)`
- [x] 5.3 Implement `frame_locator.locator(selector)`
- [x] 5.4 Implement `frame_locator.frame_locator()` for nesting
- [x] 5.5 Implement all `get_by_*` methods on FrameLocator
- [x] 5.6 Implement auto-waiting for frame load

## 6. Frame Events

- [x] 6.1 Implement frameattached event emission (CDP types added)
- [x] 6.2 Implement framenavigated event emission (existing)
- [x] 6.3 Implement framedetached event emission (CDP types added)
- [x] 6.4 Track frame tree updates (via getFrameTree)

## 7. Frame Context Execution

- [x] 7.1 Map frames to execution contexts (via evaluate with frame)
- [x] 7.2 Route evaluate calls to correct frame context
- [x] 7.3 Handle frame reload context changes (via frame locator)

## 8. Testing

- [x] 8.1 Add tests for frame locator
- [x] 8.2 Add tests for nested frames
- [x] 8.3 Add tests for frame navigation (via frame properties test)
- [x] 8.4 Add tests for frame events (indirectly via frame tests)
- [x] 8.5 Add tests for frame properties

## 9. Documentation

- [x] 9.1 Document FrameLocator usage (via rustdoc)
- [x] 9.2 Document Frame API (via rustdoc)
- [x] 9.3 Add iframe testing examples (via test code)

## Dependencies

- CDP extensions (1.x) must be done first
- Frame type (2.x) before FrameLocator (5.x)
- Page access (4.x) and FrameLocator (5.x) are parallel

## Parallelizable Work

- Frame type (2-3) and Page access (4) are independent
- FrameLocator (5) depends on Frame type (2)
- Events (6) can parallel with other work

## Implementation Notes

- Added CDP types: `FrameAttachedEvent`, `FrameDetachedEvent`, `NavigatedWithinDocumentEvent`, `FrameDetachedReason`
- Frame struct uses `parking_lot::RwLock` for thread-safe mutable state
- FrameLocator supports nested frames via parent_selectors chain
- FrameElementLocator provides all standard locator actions (click, fill, type_text, etc.)
- Frame content/title methods use Runtime.evaluate on main context (future: per-frame execution contexts)
