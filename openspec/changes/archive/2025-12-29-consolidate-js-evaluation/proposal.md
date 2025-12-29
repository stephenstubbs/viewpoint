# Change: Consolidate JavaScript Evaluation into viewpoint-js

## Why

The codebase has duplicated `evaluate_js` helper functions across multiple modules and extensive inline JavaScript constructed via `format!()` strings. This duplication:
- Increases maintenance burden
- Bypasses compile-time JS validation that the `js!` macro provides
- Makes JavaScript code harder to read and maintain embedded in Rust format strings

## What Changes

- **Consolidate `evaluate_js` helpers** into `viewpoint-js-core` as a reusable low-level evaluation utility
- **Migrate inline JavaScript** from `format!()` strings to the `js!{}` macro across `viewpoint-core`
- **Extend `js!` macro capabilities** to support the complex interpolation patterns currently used (e.g., selector expressions, multi-statement IIFEs)
- Remove duplicate `evaluate_js` implementations from:
  - `page/locator/helpers/mod.rs`
  - `page/frame_locator_actions/mod.rs`
  - `page/mouse_drag/mod.rs`

## Impact

- Affected specs: `js-validation`
- Affected code:
  - `crates/viewpoint-js-core/src/lib.rs` - add evaluation utilities
  - `crates/viewpoint-js/src/` - extend macro if needed
  - `crates/viewpoint-core/src/page/locator/helpers/mod.rs`
  - `crates/viewpoint-core/src/page/locator/queries/mod.rs`
  - `crates/viewpoint-core/src/page/locator/debug/mod.rs`
  - `crates/viewpoint-core/src/page/locator/select/mod.rs`
  - `crates/viewpoint-core/src/page/locator/files/mod.rs`
  - `crates/viewpoint-core/src/page/locator/evaluation/mod.rs`
  - `crates/viewpoint-core/src/page/frame_locator_actions/mod.rs`
  - `crates/viewpoint-core/src/page/mouse_drag/mod.rs`
- **BREAKING**: Internal API changes; tests will be updated accordingly
