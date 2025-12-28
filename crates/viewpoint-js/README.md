# viewpoint-js

Compile-time validated JavaScript macro for Viewpoint.

This crate provides a `js!` macro that validates JavaScript syntax at compile time,
similar to how `serde_json::json!` validates JSON. This catches JavaScript syntax
errors early, before they reach the browser.

## Features

- **Compile-time validation**: JavaScript syntax errors are caught during compilation
- **Rust variable interpolation**: Embed Rust expressions using `#{expr}` syntax
- **Zero runtime overhead**: Static strings when no interpolation is used
- **Clear error messages**: Points to the exact location of syntax errors

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
viewpoint-js = "0.2"
viewpoint-js-core = "0.2"  # Required for interpolation
```

### Simple Expressions

```rust
use viewpoint_js::js;

// Returns &'static str
let code = js!{ 1 + 2 };
let code = js!{ () => window.innerWidth };
let code = js!{
    (() => {
        const items = document.querySelectorAll('li');
        return items.length;
    })()
};
```

### Interpolation

Use `#{expr}` to embed Rust expressions into JavaScript:

```rust
use viewpoint_js::js;
use viewpoint_js_core::ToJsValue;

let selector = ".my-class";
let code = js!{ document.querySelector(#{selector}) };
// Produces: format!(...) that results in "document.querySelector(\".my-class\")"

let count = 42;
let code = js!{ Array(#{count}).fill(0) };
// Produces: "Array(42).fill(0)"
```

### Compile-Time Errors

Invalid JavaScript produces compile-time errors:

```rust,compile_fail
use viewpoint_js::js;

// This won't compile!
let code = js!{ function( };
```

## Supported Types for Interpolation

The following types implement `ToJsValue`:

- Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Floats: `f32`, `f64` (handles `NaN`, `Infinity`, `-Infinity`)
- Boolean: `bool`
- Strings: `String`, `&str` (properly escaped)
- Option: `Option<T>` where `T: ToJsValue` (produces value or `null`)
- JSON: `serde_json::Value` (with `json` feature)

## Features

- `json` - Enable `serde_json::Value` support for interpolation

## License

MIT
