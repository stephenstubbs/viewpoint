# Change: Remove Temporary Clippy Suppressions

## Why
During development, many clippy warnings were suppressed with `#[allow(...)]` attributes to enable rapid iteration on incomplete features. Now that the core implementation is complete, these suppressions should be removed and the underlying issues fixed to maintain code quality and catch real problems early.

## What Changes

### Phase 1: Remove Crate-Level Suppressions
Remove blanket `#![allow(...)]` attributes from crate `lib.rs` files:
- `viewpoint-core/src/lib.rs` - 23 suppressions
- `viewpoint-test/src/lib.rs` - 3 suppressions

### Phase 2: Fix Dead Code Warnings  
Address `#[allow(dead_code)]` and `#![allow(dead_code)]` by either:
- Using the code (wiring up features)
- Removing truly unused code
- Adding `#[cfg(test)]` for test-only code
- Documenting with `// TODO:` if genuinely planned for future

Files with module-level dead_code suppressions:
- `context/trace/mod.rs`
- `context/routing/mod.rs`
- `network/websocket/mod.rs`
- `network/har_recorder/mod.rs`
- `network/request/mod.rs`
- `page/download/mod.rs`
- `page/popup/mod.rs`
- `page/console/mod.rs`
- `page/events/mod.rs`
- `page/file_chooser/mod.rs`
- `page/frame_locator/mod.rs`
- `page/frame/mod.rs`
- `page/video/mod.rs`
- `tests/common/mod.rs`

### Phase 3: Fix Documentation Warnings
Add `# Errors` and `# Panics` documentation sections to public functions that return `Result` or can panic.

### Phase 4: Fix Remaining Clippy Warnings
Address remaining clippy pedantic warnings:
- `too_many_arguments` - refactor to use builder/options patterns
- `too_many_lines` - split large functions
- `type_complexity` - introduce type aliases
- Numeric cast warnings - add explicit conversions with comments

## Impact
- Affected specs: code-quality
- Affected code: All crates, primarily `viewpoint-core` and `viewpoint-test`
- Risk: Medium - requires careful review to avoid breaking changes
- Benefit: Better code quality, catch real issues, cleaner codebase
