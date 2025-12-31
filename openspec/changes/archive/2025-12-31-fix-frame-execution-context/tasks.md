# Tasks

## 1. Add Execution Context Tracking Infrastructure

- [x] 1.1 Add `execution_contexts: HashMap<String, ExecutionContextId>` field to `FrameData` struct in `page/frame/mod.rs`
- [x] 1.2 Add `Frame::main_world_context_id(&self) -> Option<ExecutionContextId>` getter (returns context for "" key)
- [x] 1.3 Add `Frame::set_execution_context(&self, world_name: String, id: ExecutionContextId)` setter
- [x] 1.4 Add `Frame::remove_execution_context(&self, id: ExecutionContextId)` for cleanup on destroy
- [x] 1.5 Add `Frame::get_or_create_isolated_world(&self, name: &str) -> Result<ExecutionContextId>` using `Page.createIsolatedWorld`

## 2. Enable Runtime Domain and Track Context Events

- [x] 2.1 Call `Runtime.enable` in Page initialization (in page creation flow) - Already implemented in `page_factory::enable_page_domains()`
- [x] 2.2 Add CDP types for `ExecutionContextDescription.auxData` (frameId, isDefault, type fields)
- [x] 2.3 Subscribe to `Runtime.executionContextCreated` events in `ExecutionContextRegistry`
- [x] 2.4 Parse `auxData.frameId` and `auxData.isDefault` from context description
- [x] 2.5 Map world name: use "" for `isDefault=true`, otherwise use `context.name`
- [x] 2.6 Update corresponding Frame's `execution_contexts` map when context is created
- [x] 2.7 Handle pending contexts: `ExecutionContextRegistry` stores contexts and Frames query it when needed
- [x] 2.8 Subscribe to `Runtime.executionContextDestroyed` events
- [x] 2.9 Remove destroyed context from registry's map

## 3. Update Frame Methods to Use Context ID

- [x] 3.1 Update `Frame.content()` to pass main world `context_id` to `Runtime.evaluate`
- [x] 3.2 Update `Frame.title()` to pass main world `context_id` to `Runtime.evaluate`
- [x] 3.3 Update `Frame.capture_snapshot_with_refs()` to pass main world `context_id` to `Runtime.evaluate`
- [x] 3.4 Helper methods remain unchanged - `Runtime.callFunctionOn` uses `objectId` which is already bound to the correct context

## 4. Verification

- [x] 4.1 Run unit tests: `cargo test --workspace` - 110+ tests pass
- [x] 4.2 Run integration tests: `cargo test --workspace --features integration` - frame and aria tests pass
- [x] 4.3 Add integration test: `test_iframe_content_execution_context` - verify `frame.content()` returns iframe HTML
- [x] 4.4 Add integration test: `test_iframe_title_execution_context` - verify `frame.title()` returns iframe title
- [x] 4.5 Add integration test: `test_iframe_aria_snapshot_execution_context` - verify `frame.aria_snapshot()` returns iframe elements
- [x] 4.6 Verify existing `test_frame_aria_snapshot` test passes with correct frame content
- [x] 4.7 Verify `test_aria_snapshot_with_frames` includes refs from all frames
