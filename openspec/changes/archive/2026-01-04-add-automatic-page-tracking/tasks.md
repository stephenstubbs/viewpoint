## 1. Research and Design
- [x] 1.1 Review `page/popup/mod.rs` - how `wait_for_popup()` handles `Target.targetCreated`
- [x] 1.2 Review `context/construction/mod.rs` - where to register CDP listeners
- [x] 1.3 Review `context/page_factory/mod.rs` - utilities for attaching to targets
- [x] 1.4 Design unified CDP event-driven approach

## 2. Rewrite Target Events Module
- [x] 2.1 Rewrite `handle_target_created` as single entry point for ALL pages:
  - Attach to target
  - Enable domains
  - Apply emulation settings
  - Create Page instance
  - Track in context
  - Emit `on_page` event (always)
- [x] 2.2 Rewrite `handle_target_destroyed` as single entry point for page removal:
  - Remove from page tracking
- [x] 2.3 Remove `opener_id` and `attached` checks (simplified - only check attached to avoid errors)
- [x] 2.4 Remove deduplication logic (no longer needed)

## 3. Simplify `new_page()`
- [x] 3.1 Remove direct attachment and domain enabling from `new_page()`
- [x] 3.2 Remove direct page tracking from `new_page()`
- [x] 3.3 Remove direct event emission from `new_page()` (handled by event listener)
- [x] 3.4 Implement new flow:
  - Set up oneshot channel listener for `on_page` event
  - Call `Target.createTarget` to create the target
  - Await the page from the listener
  - Return the Page

## 4. Clean Up
- [x] 4.1 Remove `pending_pages` module (not needed - simpler approach)
- [x] 4.2 Simplify `page_factory` - removed `create_and_attach_target`, `track_page`, `create_page_instance`
- [x] 4.3 Verify `page.close()` triggers `Target.targetDestroyed` correctly

## 5. Update Tests
- [x] 5.1 Verify `test_no_duplicate_events_for_new_page` passes (still expects 1 event)
- [x] 5.2 All target_events tests pass with new architecture
- [x] 5.3 Test: `new_page()` returns correct Page
- [x] 5.4 Test: External page (`window.open()`) emits `on_page` event
- [x] 5.5 Test: `target="_blank"` link emits `on_page` event
- [x] 5.6 Test: `page.close()` triggers cleanup
- [x] 5.7 Test: `wait_for_popup()` compatibility

## 6. Validation
- [x] 6.1 Run `cargo test --workspace` (unit tests) - PASSED
- [x] 6.2 Run `cargo test --workspace --features integration` (integration tests) - PASSED
- [ ] 6.3 Manual verification with viewpoint-mcp `browser_tabs` tool (optional)
