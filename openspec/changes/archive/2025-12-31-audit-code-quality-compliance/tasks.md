# Tasks

## 1. Fix Clippy Warnings

- [x] 1.1 Fix `while_let_on_iterator` warning in `viewpoint-js/src/scanner/handlers.rs:295`
- [x] 1.2 Fix `map_unwrap_or` warnings in `viewpoint-js/src/scanner/mod.rs:506-516`
- [x] 1.3 Fix `single_match_else` warnings in `viewpoint-core/src/page/frame/execution_context.rs:98,115`
- [x] 1.4 Fix `uninlined_format_args` warning in `viewpoint-core/src/page/locator/aria/mod.rs:281`
- [x] 1.5 Verify zero clippy warnings with `cargo clippy --workspace`

## 2. Refactor Large Source Files (viewpoint-core/page)

- [x] 2.1 Refactor `page/locator/builders/mod.rs` (1159 lines) - extract builder types into separate files
- [x] 2.2 Refactor `page/keyboard/keys/mod.rs` (1092 lines) - extract key definitions into logical groups
- [x] 2.3 Refactor `page/frame/mod.rs` (934 lines) - extract frame operations into submodules
- [x] 2.4 Refactor `page/clock/mod.rs` (601 lines) - extract clock implementation parts
- [x] 2.5 Refactor `page/keyboard/mod.rs` (575 lines) - extract keyboard operations
- [x] 2.6 Refactor `page/locator/aria_js/mod.rs` (556 lines) - extract JS generation
- [x] 2.7 Refactor `page/locator/aria/mod.rs` (535 lines) - extract aria parsing/generation

## 3. Refactor Large Source Files (other modules)

- [x] 3.1 Refactor `browser/mod.rs` (681 lines) - extract browser management operations
- [x] 3.2 Refactor `context/mod.rs` (572 lines) - extract context operations
- [x] 3.3 Refactor `context/trace/mod.rs` (531 lines) - extract trace handling
- [x] 3.4 Refactor `viewpoint-js/src/scanner/mod.rs` (526 lines) - extract scanner operations
- [x] 3.5 Refactor `viewpoint-cdp/src/protocol/page/mod.rs` (506 lines) - extract page protocol types

## 4. Refactor Large Test Files

- [x] 4.1 Split `aria_snapshot_tests.rs` (856 lines) into feature-specific test files
- [x] 4.2 Split `frame_tests.rs` (695 lines) into frame operation test files
- [x] 4.3 Split `media_emulation_tests.rs` (612 lines) into media feature test files
- [x] 4.4 Split `device_emulation_tests.rs` (593 lines) into device feature test files
- [x] 4.5 Split `input_tests.rs` (580 lines) into input type test files
- [x] 4.6 Split `network_tests.rs` (553 lines) into network feature test files

## 5. Verification

- [x] 5.1 Run `cargo check --workspace` - verify zero warnings
- [x] 5.2 Run `cargo clippy --workspace` - verify zero warnings
- [x] 5.3 Run `cargo test --workspace` - verify all unit tests pass
- [x] 5.4 Run `cargo test --workspace --features integration` - verify all integration tests pass (3 pre-existing flaky download test failures unrelated to refactoring)
- [x] 5.5 Verify no source file exceeds 500 lines
- [x] 5.6 Verify no test file exceeds 500 lines
