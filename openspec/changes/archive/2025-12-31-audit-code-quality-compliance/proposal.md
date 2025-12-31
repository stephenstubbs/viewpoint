# Change: Audit Code Quality Compliance

## Why

The codebase has accumulated technical debt that violates established code-quality specifications. There are 6 clippy warnings, 12 source files exceeding the 500-line limit (some up to 1159 lines), and 5 integration test files over 500 lines. These violations reduce maintainability and violate the code-quality spec.

## What Changes

- Fix 6 clippy warnings across viewpoint-js and viewpoint-core
- Refactor 12 source files exceeding 500 lines into smaller modules:
  - `crates/viewpoint-core/src/page/locator/builders/mod.rs` (1159 lines)
  - `crates/viewpoint-core/src/page/keyboard/keys/mod.rs` (1092 lines)
  - `crates/viewpoint-core/src/page/frame/mod.rs` (934 lines)
  - `crates/viewpoint-core/src/browser/mod.rs` (681 lines)
  - `crates/viewpoint-core/src/page/clock/mod.rs` (601 lines)
  - `crates/viewpoint-core/src/page/keyboard/mod.rs` (575 lines)
  - `crates/viewpoint-core/src/context/mod.rs` (572 lines)
  - `crates/viewpoint-core/src/page/locator/aria_js/mod.rs` (556 lines)
  - `crates/viewpoint-core/src/page/locator/aria/mod.rs` (535 lines)
  - `crates/viewpoint-core/src/context/trace/mod.rs` (531 lines)
  - `crates/viewpoint-js/src/scanner/mod.rs` (526 lines)
  - `crates/viewpoint-cdp/src/protocol/page/mod.rs` (506 lines)
- Refactor 5 integration test files exceeding 500 lines:
  - `crates/viewpoint-core/tests/aria_snapshot_tests.rs` (856 lines)
  - `crates/viewpoint-core/tests/frame_tests.rs` (695 lines)
  - `crates/viewpoint-core/tests/media_emulation_tests.rs` (612 lines)
  - `crates/viewpoint-core/tests/device_emulation_tests.rs` (593 lines)
  - `crates/viewpoint-core/tests/input_tests.rs` (580 lines)
  - `crates/viewpoint-core/tests/network_tests.rs` (553 lines)

## Impact

- Affected specs: code-quality
- Affected code: viewpoint-js, viewpoint-core, viewpoint-cdp crates
- No API changes - purely internal refactoring
