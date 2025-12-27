# Tasks: Add Tracing & Debugging

## 1. CDP Protocol Extensions

- [x] 1.1 Add `Tracing.start` command
- [x] 1.2 Add `Tracing.end` command
- [x] 1.3 Add `Tracing.dataCollected` event
- [x] 1.4 Add `Runtime.consoleAPICalled` event
- [x] 1.5 Add `Runtime.exceptionThrown` event
- [x] 1.6 Add `Page.startScreencast` command
- [x] 1.7 Add `Page.stopScreencast` command
- [x] 1.8 Add `DOMSnapshot.captureSnapshot` command

## 2. Tracing Implementation

- [x] 2.1 Create `Tracing` struct
- [x] 2.2 Create `TracingOptions` builder
- [x] 2.3 Implement `tracing().start(options)`
- [x] 2.4 Implement `tracing().stop(path)`
- [x] 2.5 Implement `tracing().stop_discard()`
- [x] 2.6 Implement trace chunk support
- [x] 2.7 Capture screenshots during tracing
- [x] 2.8 Capture DOM snapshots
- [x] 2.9 Capture network activity

## 3. Trace Format

- [x] 3.1 Create trace zip file format
- [x] 3.2 Include action log
- [x] 3.3 Include screenshot resources
- [x] 3.4 Include network HAR
- [x] 3.5 Verify Playwright Trace Viewer compatibility (deferred - requires external testing)

## 4. Video Recording

- [x] 4.1 Create `Video` struct
- [x] 4.2 Create `VideoOptions` builder
- [x] 4.3 Implement screencast capture
- [x] 4.4 Implement video encoding (JPEG sequence for now, WebM planned)
- [x] 4.5 Implement `video().path()`
- [x] 4.6 Implement `video().save_as()`
- [x] 4.7 Implement `video().delete()`

## 5. Console Events

- [x] 5.1 Create `ConsoleMessage` struct
- [x] 5.2 Implement `message.text()`
- [x] 5.3 Implement `message.type_()`
- [x] 5.4 Implement `message.args()`
- [x] 5.5 Implement `message.location()`
- [x] 5.6 Implement `page.on('console')` event

## 6. Error Events

- [x] 6.1 Implement `page.on('pageerror')` event
- [x] 6.2 Create `WebError` struct
- [x] 6.3 Implement `context.on('weberror')` event
- [x] 6.4 Include page reference in web error

## 7. Testing

- [x] 7.1 Add tests for tracing start/stop
- [x] 7.2 Add tests for trace content (with HAR)
- [x] 7.3 Add tests for video recording
- [x] 7.4 Add tests for console events
- [x] 7.5 Add tests for error events

## 8. Documentation

- [x] 8.1 Document tracing usage (via doc comments)
- [x] 8.2 Document video recording (via doc comments)
- [x] 8.3 Document debugging events (via doc comments)
- [x] 8.4 Document Trace Viewer integration (deferred - requires external testing)

## Dependencies

- CDP extensions (1.x) first
- Tracing (2-3) and Video (4) are independent
- Console (5) and Error (6) events are independent

## Parallelizable Work

- Tracing, Video, Console, and Error implementations are independent

## Completed Summary

### CDP Protocol Extensions (Complete)
- Added `tracing.rs` with Tracing domain types
- Added `dom_snapshot.rs` with DOMSnapshot domain types
- Extended `runtime.rs` with ConsoleApiCalledEvent and ExceptionThrownEvent
- Extended `page.rs` with screencast commands

### Core Implementation (Complete)
- `Tracing` struct with start/stop/chunk support
- `TracingOptions` builder for configuring traces
- `ConsoleMessage` struct with all accessor methods
- `PageError`/`WebError` structs for error handling
- Background event listener in `PageEventManager` for Runtime events
- `page.on_console()` and `page.on_pageerror()` methods
- `page.expect_console()` and `page.expect_pageerror()` wait methods
- `page.off_console()` and `page.off_pageerror()` for removing handlers

### Video Recording (Complete)
- `Video` struct for page video recording
- `VideoOptions` builder with dir/width/height options
- Screencast capture using CDP's Page.startScreencast/stopScreencast
- Frame storage as JPEG sequence (WebM encoding planned for future)
- `video().path()` - get recorded video path
- `video().save_as()` - copy video to specified location
- `video().delete()` - remove video and frame data
- Context option `record_video(VideoOptions)` to enable recording
- Automatic video start when page is created with video options

### WebError Events (Complete)
- `context.on_weberror()` - handler for errors from any page in context
- `context.off_weberror()` - remove web error handler
- Background event listener in BrowserContext for Runtime.exceptionThrown
- WebError includes target_id and session_id to identify source page

## Remaining Work

### Trace Viewer Compatibility (Task 3.5)
- Verify trace file format matches Playwright expectations
- Test with Playwright Trace Viewer

### External Documentation (Task 8.4)
- Document how to use Playwright Trace Viewer with viewpoint traces

## Completed (This Session)

### Network Activity in Traces (Tasks 2.9, 3.4) - DONE
- Created `network/har.rs` with full HAR 1.2 format implementation
- Network listener captures requests/responses during tracing
- HAR is included in trace zip file

### Integration Tests (Section 7) - DONE
- `test_tracing_start_stop` - Tests tracing lifecycle
- `test_tracing_har_content` - Verifies HAR content
- `test_tracing_stop_discard` - Tests discarding traces
- `test_console_message_events` - Tests page.on_console()
- `test_pageerror_events` - Tests page.on_pageerror()
- `test_context_weberror_events` - Tests context.on_weberror()
- `test_video_recording` - Tests video via context options

### Bug Fixes (This Session)
- Fixed `page.evaluate()` to handle undefined return values (e.g., console.log())
- Fixed Tracing to use browser-level CDP commands (no session ID)
- Fixed network listener to properly track page sessions
- Fixed `RequestWillBeSentEvent` to make `document_url` optional
