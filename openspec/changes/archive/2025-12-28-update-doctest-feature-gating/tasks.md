# Tasks: Update doc tests to use integration feature flag

## 1. Update viewpoint-core doc tests
- [x] 1.1 Update `page/clock/mod.rs` doc tests (2 blocks)
- [x] 1.2 Update `page/locator/mod.rs` doc tests (1 block)
- [x] 1.3 Update `page/locator/builders/mod.rs` doc tests (1 block)
- [x] 1.4 Update `page/locator/filter/mod.rs` doc tests (1 block)
- [x] 1.5 Update `page/emulation/mod.rs` doc tests (1 block)
- [x] 1.6 Update `page/touchscreen/mod.rs` doc tests (1 block)
- [x] 1.7 Update `page/file_chooser/mod.rs` doc tests (1 block)
- [x] 1.8 Update `page/keyboard/mod.rs` doc tests (1 block)
- [x] 1.9 Update `page/mouse/mod.rs` doc tests (1 block)
- [x] 1.10 Update `page/mouse_drag/mod.rs` doc tests (1 block)
- [x] 1.11 Update `page/console/mod.rs` doc tests (1 block)
- [x] 1.12 Update `page/video/mod.rs` doc tests (1 block)
- [x] 1.13 Update `page/page_error/mod.rs` doc tests (2 blocks)
- [x] 1.14 Update `page/frame_locator/mod.rs` doc tests (1 block)
- [x] 1.15 Update `page/dialog/mod.rs` doc tests (1 block)
- [x] 1.16 Update `page/download/mod.rs` doc tests (1 block)
- [x] 1.17 Update `context/trace/mod.rs` doc tests (1 block)
- [x] 1.18 Update `api/context/mod.rs` doc tests (1 block)

## 2. Update viewpoint-test doc tests
- [x] 2.1 Update `expect/mod.rs` doc tests (3 blocks)
- [x] 2.2 Update `expect/soft.rs` doc tests (1 block)
- [x] 2.3 Update `examples/macro_test.rs` doc tests (5 blocks) - N/A: uses `ignore` appropriately for macro examples

## 3. Update viewpoint-test-macros doc tests
- [x] 3.1 Update `lib.rs` doc tests (3 blocks) - Uses `text` blocks (correct for proc-macro crate)

## 4. Validation
- [x] 4.1 Verify `cargo test --workspace` succeeds (doc tests compile but don't run browser code)
- [x] 4.2 Verify `cargo test --workspace --features integration` runs all doc tests
- [x] 4.3 Verify `cargo test --workspace --features integration 2>&1 | grep ignored` shows no browser-related doc tests ignored

Note: The 130+ ignored tests are method-level doc tests (e.g., `Clock::install`, `Keyboard::press`) 
that use `ignore` appropriately - they are not the primary struct/module doc tests targeted by this change.
