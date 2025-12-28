# Change: Fix tracing state sharing between calls

## Why

The `BrowserContext::tracing()` method creates a **new** `Tracing` instance each call, with a fresh `TracingState`. This causes `context.tracing().stop()` to fail with "Tracing is not active" because the state from `context.tracing().start()` is lost.

Additionally, calling `start()` when no pages exist silently proceeds but CDP tracing commands are never sent (since `get_session_ids()` returns empty), leading to confusion when `stop()` fails.

## What Changes

1. **Store tracing state in BrowserContext** - Add `tracing_state: Arc<RwLock<TracingState>>` field to persist state across `tracing()` calls
2. **Pass shared state to Tracing** - Update `Tracing::new()` to accept the shared state instead of creating a new one
3. **Validate pages exist on start** - Return an error if `start()` is called when no pages exist in the context
4. **Add integration tests** - Comprehensive tests for tracing start/stop/chunks/discard workflows

## Impact

- Affected specs: `tracing`
- Affected code:
  - `crates/viewpoint-core/src/context/mod.rs` - Add `tracing_state` field
  - `crates/viewpoint-core/src/context/tracing_access/mod.rs` - Pass shared state
  - `crates/viewpoint-core/src/context/trace/mod.rs` - Accept shared state, validate pages
  - `crates/viewpoint-core/src/context/trace/types/mod.rs` - Make `TracingState` pub(crate)
  - `crates/viewpoint-core/tests/tracing_tests.rs` - New integration test file
