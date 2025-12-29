# viewpoint-js

Compile-time validated JavaScript macro for Viewpoint.

This crate provides a `js!` macro that validates JavaScript syntax at compile time,
similar to how `serde_json::json!` validates JSON. This catches JavaScript syntax
errors early, before they reach the browser.

## Features

- **Compile-time validation**: JavaScript syntax errors are caught during compilation
- **Value interpolation**: Embed Rust expressions using `#{expr}` syntax (quoted/escaped)
- **Raw interpolation**: Inject pre-built JavaScript using `@{expr}` syntax (unquoted)
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

### Value Interpolation

Use `#{expr}` to embed Rust values into JavaScript (properly quoted and escaped):

```rust
use viewpoint_js::js;
use viewpoint_js_core::ToJsValue;

let selector = ".my-class";
let code = js!{ document.querySelector(#{selector}) };
// Produces: "document.querySelector(\".my-class\")"

let count = 42;
let code = js!{ Array(#{count}).fill(0) };
// Produces: "Array(42).fill(0)"
```

### Raw Interpolation

Use `@{expr}` to inject pre-built JavaScript expressions directly (without quoting):

```rust
use viewpoint_js::js;

// Inject a JavaScript expression as-is
let selector_expr = "document.querySelectorAll('.item')";
let code = js!{ Array.from(@{selector_expr}) };
// Produces: "Array.from(document.querySelectorAll('.item'))"

// Useful for building complex JS with dynamic parts
let frame_access = get_frame_access_code();
let code = js!{
    (function() {
        const doc = @{frame_access};
        return doc.title;
    })()
};
```

### Mixing Both Interpolation Types

You can use both `#{}` and `@{}` in the same macro call:

```rust
use viewpoint_js::js;

let selector_expr = "document.body";
let attr_name = "data-id";
let code = js!{ @{selector_expr}.setAttribute(#{attr_name}, "value") };
// Produces: "document.body.setAttribute(\"data-id\", \"value\")"
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
