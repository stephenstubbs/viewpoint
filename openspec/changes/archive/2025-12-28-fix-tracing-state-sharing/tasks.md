# Tasks: Fix tracing state sharing

## 1. Core Fix - State Sharing

- [x] 1.1 Add `tracing_state: Arc<RwLock<TracingState>>` field to `BrowserContext` struct in `context/mod.rs`
- [x] 1.2 Initialize `tracing_state` in `BrowserContext::new()` 
- [x] 1.3 Initialize `tracing_state` in `BrowserContext::with_options()`
- [x] 1.4 Update `Tracing::new()` signature to accept `Arc<RwLock<TracingState>>` instead of creating new state
- [x] 1.5 Update `tracing_access/mod.rs` to pass `self.tracing_state.clone()` to `Tracing::new()`
- [x] 1.6 Make `TracingState` visibility `pub(crate)` so it can be used from context module

## 2. Validation Improvement

- [x] 2.1 Add validation in `Tracing::start()` to check if any pages exist (non-empty session IDs)
- [x] 2.2 Return descriptive error "Cannot start tracing: no pages in context. Create a page first." when validation fails
- [x] 2.3 Update doc comments to mention the page requirement

## 3. Integration Tests

- [x] 3.1 Create `crates/viewpoint-core/tests/tracing_tests.rs` with `#![cfg(feature = "integration")]`
- [x] 3.2 Add test: `test_tracing_start_stop` - basic start and stop workflow
- [x] 3.3 Add test: `test_tracing_start_stop_discard` - start and discard without saving
- [x] 3.4 Add test: `test_tracing_chunks` - start_chunk and stop_chunk workflow
- [x] 3.5 Add test: `test_tracing_is_recording` - verify `is_recording()` returns correct state
- [x] 3.6 Add test: `test_tracing_start_without_pages_fails` - verify error when no pages exist
- [x] 3.7 Add test: `test_tracing_stop_without_start_fails` - verify error when not recording
- [x] 3.8 Add test: `test_tracing_double_start_fails` - verify error when already recording
- [x] 3.9 Add test: `test_tracing_with_screenshots` - verify screenshot option works
- [x] 3.10 Add test: `test_tracing_creates_valid_zip` - verify trace.zip is created and valid

## 4. Doc Test Fixes

- [x] 4.1 Update `context/trace/mod.rs` doc example to use runnable integration test pattern
- [x] 4.2 Update `context/tracing_access/mod.rs` doc example if needed

## 5. Validation

- [x] 5.1 Run `cargo test --workspace` - ensure all unit tests pass
- [x] 5.2 Run `cargo test --workspace --features integration` - ensure all integration tests pass
- [x] 5.3 Run `cargo test --workspace --doc --features integration` - ensure doc tests pass
- [x] 5.4 Run `cargo clippy --workspace --all-targets` - ensure no warnings (pre-existing PI warning in escape_tests.rs)
