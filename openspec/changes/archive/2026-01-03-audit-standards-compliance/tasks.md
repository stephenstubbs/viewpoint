# Tasks: Standards Compliance Audit

## 1. Fix Clippy Warnings (Quick Wins)
- [x] 1.1 Fix `single_char_pattern` warnings in `aria_snapshot_ref_tests.rs`
- [x] 1.2 Fix `single_char_pattern` warnings in `aria_snapshot_performance_tests.rs`
- [x] 1.3 Fix `uninlined_format_args` warnings in `aria_snapshot_ref_tests.rs`
- [x] 1.4 Verify zero clippy warnings with `cargo clippy --workspace --all-targets --features integration`

## 2. Address Crate-Level Suppressions
- [x] 2.1 Convert `#![allow(...)]` to item-level `#[allow(...)]` with justification comments for `float_cmp` in test files
- [x] 2.2 Fix underlying issues for `uninlined_format_args` suppressions and remove them
- [x] 2.3 Review `assertions_on_constants` usages and either fix or justify with comments
- [x] 2.4 Verify no crate-level suppressions remain (except justified test utilities)

## 3. Refactor Large Source Files
- [x] 3.1 Refactor `page/mod.rs` (706 → 465 lines) - extracted lifecycle.rs, accessors.rs, page_info.rs
- [x] 3.2 Refactor `browser/mod.rs` (553 → 484 lines) - extracted process.rs
- [x] 3.3 Refactor `network/websocket/mod.rs` (531 → 479 lines) - extracted frame.rs
- [x] 3.4 Refactor `network/handler/mod.rs` (520 → 423 lines) - extracted constructors.rs
- [x] 3.5 Refactor `page/locator/mod.rs` (510 → 430 lines) - extracted aria_snapshot_impl.rs
- [x] 3.6 Refactor `context/mod.rs` (506 → 481 lines) - extracted timeout.rs
- [x] 3.7 Review `expect/locator.rs` (500 lines) - at limit, no action needed

## 4. Refactor Large Test Files
- [x] 4.1 Split `aria_snapshot_ref_tests.rs` (930 → 420 lines) - split into ref_basic_tests, ref_resolution_tests, ref_error_tests, ref_frame_tests
- [x] 4.2 Split `aria_snapshot_basic_tests.rs` (674 → 132 lines) - split into frame_boundary_tests, semantic_text_tests
- [x] 4.3 Split `locator_tests.rs` (555 → 295 lines) - split into locator_form_tests

## 5. Validation
- [x] 5.1 Run `cargo check --workspace` - verify zero warnings
- [x] 5.2 Run `cargo clippy --workspace --all-targets --features integration` - verify zero warnings
- [x] 5.3 Run `cargo test --workspace` - verify unit tests pass
- [ ] 5.4 Run `cargo test --workspace --features integration` - verify integration tests pass (requires browser)
- [x] 5.5 Verify no files exceed 500 lines

## Dependencies
- Tasks in section 1 can run in parallel
- Tasks in section 2 can run in parallel
- Tasks in sections 3 and 4 can run in parallel but may have merge conflicts
- Section 5 depends on completion of sections 1-4

## Notes
- All source files are now under 500 lines
- All test files are now under 500 lines
- Zero clippy warnings in workspace
- All unit tests pass
- Integration tests require browser (not run in this session)

## Files Created/Modified

### New Source Files
- `crates/viewpoint-core/src/browser/process.rs` - Process management utilities
- `crates/viewpoint-core/src/page/page_info.rs` - Page URL and title methods
- `crates/viewpoint-core/src/page/lifecycle.rs` - Page close and is_closed methods
- `crates/viewpoint-core/src/page/accessors.rs` - Page getter methods
- `crates/viewpoint-core/src/page/locator/aria_snapshot_impl.rs` - Locator aria_snapshot method
- `crates/viewpoint-core/src/network/websocket/frame.rs` - WebSocketFrame struct
- `crates/viewpoint-core/src/network/handler/constructors.rs` - RouteHandlerRegistry constructors
- `crates/viewpoint-core/src/context/timeout.rs` - Context timeout configuration

### New Test Files
- `crates/viewpoint-core/tests/aria_snapshot_ref_basic_tests.rs` - Basic ref tests
- `crates/viewpoint-core/tests/aria_snapshot_ref_resolution_tests.rs` - Ref resolution tests
- `crates/viewpoint-core/tests/aria_snapshot_ref_error_tests.rs` - Ref error handling tests
- `crates/viewpoint-core/tests/aria_snapshot_ref_frame_tests.rs` - Frame-related ref tests
- `crates/viewpoint-core/tests/aria_snapshot_frame_boundary_tests.rs` - Frame boundary tests
- `crates/viewpoint-core/tests/locator_form_tests.rs` - Form-related locator tests
