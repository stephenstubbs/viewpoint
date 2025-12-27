## Phase 0: Crate Setup

- [x] 0.1 Create `crates/viewpoint-js/` directory structure
- [x] 0.2 Add `viewpoint-js` to workspace members in root `Cargo.toml`
- [x] 0.3 Create `Cargo.toml` with proc-macro configuration and dependencies (swc_ecma_parser)
- [x] 0.4 Create initial `src/lib.rs` with crate documentation

## Phase 1: Core JavaScript Parsing

- [x] 1.1 Implement JavaScript parsing wrapper around swc_ecma_parser
- [x] 1.2 Add error conversion from swc errors to proc-macro diagnostics
- [x] 1.3 Add unit tests for valid JavaScript expressions
- [x] 1.4 Add unit tests for valid JavaScript statements
- [x] 1.5 Add unit tests for invalid JavaScript with error messages

## Phase 2: Basic js! Macro

- [x] 2.1 Implement `js!` proc-macro that parses input tokens
- [x] 2.2 Extract JavaScript source from token stream
- [x] 2.3 Validate JavaScript syntax via swc parser
- [x] 2.4 Return validated JavaScript as string literal
- [x] 2.5 Add compile-fail tests using trybuild for invalid JavaScript

## Phase 3: ToJsValue Trait

- [x] 3.1 Define `ToJsValue` trait with `to_js_value(&self) -> String` method
- [x] 3.2 Implement for primitive types (i8-i128, u8-u128, f32, f64, bool)
- [x] 3.3 Implement for String and &str with proper escaping
- [x] 3.4 Implement for Option<T> where T: ToJsValue
- [x] 3.5 Implement for serde_json::Value (optional feature)
- [x] 3.6 Add unit tests for all ToJsValue implementations

## Phase 4: Interpolation Support

- [x] 4.1 Implement `#{}` interpolation token detection
- [x] 4.2 Parse Rust expressions within interpolation markers
- [x] 4.3 Generate code that calls ToJsValue on interpolated expressions
- [x] 4.4 Produce format! based output for interpolated macros
- [x] 4.5 Add tests for single interpolation
- [x] 4.6 Add tests for multiple interpolations
- [x] 4.7 Add tests for complex expression interpolation

## Phase 5: Static vs Dynamic Output

- [x] 5.1 Detect when no interpolation is present
- [x] 5.2 Return &'static str for non-interpolated macros
- [x] 5.3 Return String for interpolated macros
- [x] 5.4 Add tests verifying output types

## Phase 6: Documentation and Examples

- [x] 6.1 Add comprehensive crate-level documentation
- [x] 6.2 Add doc examples for js! macro
- [x] 6.3 Add doc examples for ToJsValue trait
- [x] 6.4 Create `examples/basic_usage.rs` demonstrating common patterns
- [x] 6.5 Create README.md for the crate

## Phase 7: Integration Testing

- [~] 7.1 Add integration test using js! with page.evaluate (requires viewpoint-core) - DEFERRED to enhance-integration-tests
- [~] 7.2 Add integration test using js! with locator.evaluate - DEFERRED to enhance-integration-tests
- [~] 7.3 Add integration test for interpolation with real browser - DEFERRED to enhance-integration-tests
- [x] 7.4 Verify error messages are helpful and point to correct location
