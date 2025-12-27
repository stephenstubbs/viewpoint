# Design: Enhanced Unit and Integration Tests

## Overview

This document outlines the design for comprehensive test coverage across all Viewpoint crates, updated to reflect the new module structure and code-quality standards established in the `refactor-module-conventions` change.

## Guiding Principles

1. **Real browser testing** - Integration tests must use real Chromium, never mocks
2. **Spec alignment** - Every spec scenario should have a corresponding test
3. **Error path coverage** - Test failure modes, not just happy paths
4. **Edge case coverage** - Boundary conditions, empty inputs, malformed data
5. **Isolation** - Tests should be independent and not affect each other
6. **Validated JavaScript** - All JavaScript code MUST use the `js!` macro for compile-time validation
7. **External test directories** - Tests MUST be in `<module>/tests/` directories, not inline

## Test Infrastructure

### Module Structure (Updated)

Per the code-quality spec, all tests must follow this structure:

```
module_name/
├── mod.rs           # Public exports + #[cfg(test)] mod tests;
├── tests/           # Unit tests (folder module)
│   ├── mod.rs       # use super::*; and test function imports
│   └── *.rs         # Categorized test files
└── ...
```

Example:
```rust
// In module_name/mod.rs
pub struct MyType { ... }

impl MyType { ... }

#[cfg(test)]
mod tests;

// In module_name/tests/mod.rs
use super::*;

#[test]
fn test_my_type() {
    let t = MyType::new();
    assert!(t.is_valid());
}
```

### Integration Test Gating

Integration tests that require Chromium MUST be gated with the `integration` feature:

```toml
# In Cargo.toml
[features]
integration = []
```

```rust
// At the top of tests/*.rs files that need browser
#![cfg(feature = "integration")]

// Test code that requires real Chromium
```

Running tests:
- `cargo test` - Unit tests only (fast, no browser)
- `cargo test --features integration` - All tests including browser tests

### JavaScript Code Convention

All JavaScript code in tests MUST use the `js!` macro from `viewpoint-js`:

```rust
use viewpoint_js::js;

// Simple expression (returns &'static str)
let result = page.evaluate(js!{ document.title }).await?;

// With interpolation (returns String)
let selector = ".my-class";
let el = page.evaluate(js!{ document.querySelector(#{selector}) }).await?;

// Multi-line functions
let count = page.evaluate(js!{
    (() => {
        const items = document.querySelectorAll(#{selector});
        return items.length;
    })()
}).await?;
```

## Test Categories

| Type | Location | Chromium? | Command | Feature Gate? |
|------|----------|-----------|---------|---------------|
| Unit | `src/**/tests/` | No (mocked) | `cargo test` | No |
| Integration | `tests/` (crate root) | Yes | `cargo test --features integration` | Yes |
| Doc tests | Inline in source | No | `cargo test --doc` | No |

## Crate-Specific Design

### viewpoint-js

**Test Location:** `src/parser/tests/`, `src/interpolation/tests/`, `src/js_macro/tests/`

**Unit Tests:**
- Parser tests for valid/invalid JavaScript
- Interpolation parsing and code generation
- js! macro expansion

**Integration Tests:** `tests/integration_tests.rs`
- js! macro with page.evaluate

### viewpoint-cdp

**Test Location:** `src/connection/tests/`, `src/error/tests/`, `src/transport/tests/`

**Unit Tests:**
- Connection state machine, reconnection logic
- CDP message parsing edge cases
- Error type coverage for all `CdpError` variants

**Integration Tests:** `tests/integration_tests.rs` (feature-gated)
- Real WebSocket connection lifecycle
- Command timeout handling
- Event subscription and filtering

### viewpoint-core

**Test Locations:**
- `src/network/*/tests/` - Network type tests
- `src/page/*/tests/` - Page/locator tests
- `src/context/*/tests/` - Context tests
- `src/api/*/tests/` - API tests
- `src/wait/*/tests/` - Wait system tests

**Unit Tests:**
- URL pattern matching edge cases
- Selector parsing, JS expression generation
- HAR format compliance
- State transitions

**Integration Tests:** `tests/*.rs` (all feature-gated)
- navigation_tests.rs
- locator_tests.rs
- input_tests.rs
- network_tests.rs
- context_tests.rs
- frame_tests.rs
- And others...

### viewpoint-test

**Test Location:** `src/expect/tests/`

**Unit Tests:**
- Assertion conditions
- Timeout behavior
- Soft assertion accumulation

**Integration Tests:** `tests/e2e_tests.rs`, `tests/harness_tests.rs` (feature-gated)
- Full E2E scenarios
- Test fixture lifecycle

### viewpoint-test-macros

**Test Location:** `src/test_attr/tests/`

**Unit Tests:**
- Macro expansion tests

**Integration Tests:** `tests/` (if needed)
- Fixture injection verification

## Migration Strategy

### Phase A: Migrate Inline Tests

1. **Identify** - Find all files with `#[cfg(test)] mod tests { ... }` blocks
2. **Create** - Create `<module>/tests/mod.rs` for each
3. **Move** - Extract test code to external test files
4. **Update** - Change inline blocks to `#[cfg(test)] mod tests;` declarations
5. **Verify** - Run tests to ensure they still pass

### Phase B: Re-verify Completed Work

Many tasks were marked complete before module restructuring. For each:

1. **Check location** - Is the test in the correct directory?
2. **Check structure** - Does it follow folder module pattern?
3. **Check imports** - Does it use `use super::*;`?
4. **Check feature gate** - Is it properly gated if needed?

### Phase C: Add Missing Tests

Complete the remaining test scenarios identified in tasks.md.

## Error Path Testing Strategy

Each module should test:

1. **Input validation errors** - Invalid arguments, out-of-range values
2. **State errors** - Operations on closed/invalid objects
3. **Network errors** - Connection failures, timeouts
4. **Protocol errors** - Malformed messages, unexpected responses
5. **Resource errors** - Element not found, frame detached

## Success Metrics

1. **Zero inline test modules** - grep for `mod tests {` returns nothing
2. **All tests pass** - `cargo test --workspace --features integration` succeeds
3. **Folder module compliance** - All modules use directory structure
4. **Feature gate compliance** - Unit tests run without browser, integration tests are gated
5. **No raw JavaScript** - All JS uses js! macro
