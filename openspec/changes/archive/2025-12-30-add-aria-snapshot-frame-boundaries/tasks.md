# Tasks: Add Frame Boundary Support to Aria Snapshots

## 1. Extend AriaSnapshot Struct
- [x] 1.1 Add `is_frame: Option<bool>` field to AriaSnapshot
- [x] 1.2 Add `frame_url: Option<String>` field to AriaSnapshot
- [x] 1.3 Add `frame_name: Option<String>` field to AriaSnapshot
- [x] 1.4 Add `iframe_refs: Vec<String>` field to AriaSnapshot (with serde default)
- [x] 1.5 Update AriaSnapshot `to_yaml()` to render frame boundaries (e.g., `[frame-boundary]`)
- [x] 1.6 Update AriaSnapshot `from_yaml()` to parse frame boundary markers
- [x] 1.7 Add unit tests for new AriaSnapshot fields and YAML serialization

## 2. Update JavaScript Aria Snapshot Capture
- [x] 2.1 Detect `IFRAME` elements in `getAriaSnapshot` function
- [x] 2.2 Return iframe nodes with `role: 'iframe'` and metadata fields
- [x] 2.3 Do NOT attempt `contentDocument` access (security)
- [x] 2.4 Handle `<frame>` and `<frameset>` elements similarly
- [x] 2.5 Add unit tests for iframe detection in JS (via integration test with HTML fixtures)

## 3. Add Frame-Level Aria Snapshot
- [x] 3.1 Add `Frame.aria_snapshot()` method returning `Result<AriaSnapshot, PageError>`
- [x] 3.2 Implement using existing locator infrastructure with frame context
- [x] 3.3 Add integration test: capture snapshot from same-origin iframe
- [x] 3.4 Add integration test: verify cross-origin iframe shows as boundary only

## 4. Add Page-Level Multi-Frame Snapshot
- [x] 4.1 Add `Page.aria_snapshot_with_frames()` method
- [x] 4.2 Get frame tree via `Page.getFrameTree` CDP command
- [x] 4.3 Capture main frame snapshot (collects iframe_refs)
- [x] 4.4 For each child frame, capture and stitch into parent at iframe boundary
- [x] 4.5 Handle CDP session targeting for out-of-process frames
- [x] 4.6 Add integration test: nested same-origin frames
- [x] 4.7 Add integration test: mixed same-origin and cross-origin frames

## 5. Documentation and Examples
- [x] 5.1 Update AriaSnapshot rustdoc with frame boundary examples
- [x] 5.2 Add example: MCP-style snapshot capture with frames
- [x] 5.3 Document cross-origin limitations in module docs

## Dependencies
- Tasks 1.x can be done in parallel with 2.x
- Task 3.x depends on 1.x and 2.x completion
- Task 4.x depends on 3.x completion
- Task 5.x can start after 1.x, completed after 4.x
