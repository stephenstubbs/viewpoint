# Design: Tracing & Debugging

## Context

Tracing provides a detailed record of test execution for debugging failures. Video recording captures visual output. Console and error events capture JavaScript output.

## Goals

- Record traces compatible with Playwright Trace Viewer
- Support video recording of test execution
- Capture all console output and errors
- Minimal performance impact when not recording

## Decisions

### Decision 1: Trace Format

**Choice**: Use Playwright-compatible trace format (zip with resources).

**Rationale**:
- Can use Playwright's Trace Viewer
- Well-documented format
- Includes screenshots, DOM, network, actions

### Decision 2: Video Format

**Choice**: WebM format using CDP's screencast.

```rust
let context = browser.new_context()
    .record_video(VideoOptions::new("./videos"))
    .build()
    .await?;
```

### Decision 3: Trace Chunk Support

**Choice**: Support trace chunks for long-running tests.

```rust
context.tracing().start(TracingOptions::new()).await?;
// ... actions ...
context.tracing().start_chunk().await?;
// ... more actions ...
context.tracing().stop_chunk("chunk1.zip").await?;
```

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| Tracing | Tracing.start, Tracing.end | Tracing |
| Screenshots | Page.captureScreenshot | Page |
| DOM snapshot | DOMSnapshot.captureSnapshot | DOMSnapshot |
| Console | Runtime.consoleAPICalled | Runtime |
| Errors | Runtime.exceptionThrown | Runtime |
| Video | Page.startScreencast | Page |

## Open Questions

None.
