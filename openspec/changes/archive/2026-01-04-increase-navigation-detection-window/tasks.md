# Tasks

## 1. Core fix (DONE)

- [x] 1.1 Change `NAVIGATION_DETECTION_WINDOW` from 50ms to 150ms in `crates/viewpoint-core/src/wait/navigation_waiter/mod.rs`
- [x] 1.2 Update test assertion in `navigation_waiter/tests/mod.rs` to expect 150ms
- [x] 1.3 Add null element handling at start of `aria_snapshot_with_refs_js()` in `crates/viewpoint-core/src/page/locator/aria_js/snapshot_with_refs.rs`

## 2. Verification

- [x] 2.1 Run `cargo test --workspace` - all tests pass
- [x] 2.2 Run `cargo clippy --workspace` - no warnings

## 3. Remaining

- [ ] 3.1 Run integration tests: `cargo test --workspace --features integration`
- [ ] 3.2 Manual verification with viewpoint-mcp reproduction case
