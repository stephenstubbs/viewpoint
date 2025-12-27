# Change: Add JavaScript Validation Macro

## Why

Currently, JavaScript code used with `page.evaluate()`, `locator.evaluate()`, and other JS-executing methods is written as plain strings. This has several problems:

1. **No compile-time validation** - Syntax errors are only discovered at runtime when the browser rejects the JavaScript
2. **Poor IDE support** - No syntax highlighting, autocompletion, or error checking for JavaScript within string literals
3. **Error-prone string escaping** - Complex JavaScript requires careful escaping of quotes and special characters
4. **Inconsistent with Rust philosophy** - Rust emphasizes catching errors at compile time, but JavaScript errors slip through

Similar to how `serde_json::json!` validates JSON structure at compile time, a `js!` macro can validate JavaScript syntax at compile time.

## What Changes

### New Crate: `viewpoint-js`

A new crate that provides:

1. **`js!` macro** - A declarative macro for writing validated JavaScript
2. **Compile-time parsing** - Uses a JavaScript parser to validate syntax during compilation
3. **Expression and statement support** - Handles both JavaScript expressions and statement blocks
4. **String interpolation** - Allows embedding Rust expressions into JavaScript (with proper escaping)

### Usage Examples

```rust
use viewpoint_js::js;

// Simple expression
let result: i32 = page.evaluate(js!{ 1 + 2 }).await?;

// Arrow function
let width: i32 = page.evaluate(js!{ () => window.innerWidth }).await?;

// With Rust variable interpolation
let selector = ".my-class";
let el = page.evaluate(js!{ document.querySelector(#{selector}) }).await?;

// Multi-line function
let result = page.evaluate(js!{
    (() => {
        const items = document.querySelectorAll('li');
        return items.length;
    })()
}).await?;
```

## Impact

- **New crate**: `crates/viewpoint-js/`
- **Workspace update**: Add `viewpoint-js` to workspace members
- **Dependent crates**: `viewpoint-core` and `viewpoint-test` will optionally use `viewpoint-js`
- **Breaking changes**: None (additive only, existing string-based API remains)
- **Testing**: The `enhance-integration-tests` proposal should test this crate first

## Dependencies

This change should be implemented **before** the `enhance-integration-tests` proposal proceeds, as:
1. Tests will benefit from compile-time JavaScript validation
2. The test suite will serve as validation for the macro's correctness
3. Writing tests with validated JavaScript reduces debugging time
