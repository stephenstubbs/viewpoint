# Change: Fix Download Event Interception

## Why

The download event handling infrastructure exists but is never invoked. The `event_listener` module listens for console, dialog, and frame events but does not subscribe to `Browser.downloadWillBegin` or `Browser.downloadProgress` CDP events. This causes all download-related tests to timeout waiting for events that never arrive.

Additionally, there's a parameter order bug in `handle_download_begin` where `suggested_filename` and `url` are swapped when constructing the `Download` object.

## What Changes

- Add `Browser.downloadWillBegin` event handling to the event listener
- Add `Browser.downloadProgress` event handling to the event listener  
- Fix parameter order bug in `handle_download_begin` (swap `suggested_filename` and `url`)
- Handle Browser-level events which may not have session_id filtering
- Enable the 7 ignored download tests

## Impact

- Affected specs: `downloads`
- Affected code:
  - `crates/viewpoint-core/src/page/events/event_listener/mod.rs` - Add download event handlers
  - `crates/viewpoint-core/src/page/events/download_handling/mod.rs` - Fix parameter order
  - `crates/viewpoint-core/tests/download_tests.rs` - Enable ignored tests
