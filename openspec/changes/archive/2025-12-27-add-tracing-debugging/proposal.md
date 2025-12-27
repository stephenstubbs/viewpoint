# Change: Add Tracing & Debugging

## Why

Debugging test failures requires visibility into what happened:
- **Traces**: Step-by-step recording of browser actions
- **Video**: Visual record of test execution
- **Screenshots**: Point-in-time captures
- **Console Logs**: JavaScript console output
- **Errors**: Uncaught exceptions and errors

This is proposal 8 of 12 in the Playwright feature parity series.

## What Changes

### New Capabilities

1. **Tracing** - Record test execution traces
   - `context.tracing().start()` - start recording
   - `context.tracing().stop()` - stop and save trace
   - Trace includes screenshots, DOM snapshots, network
   - Compatible with Playwright Trace Viewer

2. **Video Recording** - Record test as video
   - Context option: `record_video: { dir: path }`
   - `page.video().path()` - get video file path
   - `page.video().save_as(path)` - save video

3. **Console Events** - Capture console output
   - `page.on('console')` - console messages
   - `console_message.type_()` - log, error, warn, etc.
   - `console_message.text()` - message text
   - `console_message.args()` - message arguments

4. **Error Events** - Capture page errors
   - `page.on('pageerror')` - uncaught exceptions
   - `context.on('weberror')` - errors from any page

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1-7 | Previous | Complete |
| **8** | **Tracing & Debugging** (this) | **Current** |
| 9-12 | Remaining | Pending |

## Impact

- **New specs**: `tracing`, `video-recording`, `console-events`
- **Affected code**: 
  - `viewpoint-core/src/context/tracing.rs` - Tracing type
  - `viewpoint-core/src/page/` - console/error events, video
  - `viewpoint-cdp/` - Tracing domain, Runtime.consoleAPICalled
- **Breaking changes**: None
- **Dependencies**: Proposals 1-4 (tracing records their operations)
