# viewpoint-js-core

[![Crates.io](https://img.shields.io/crates/v/viewpoint-js-core.svg)](https://crates.io/crates/viewpoint-js-core)
[![Documentation](https://docs.rs/viewpoint-js-core/badge.svg)](https://docs.rs/viewpoint-js-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Core types for the `viewpoint-js` macro, providing the [`ToJsValue`] trait for converting Rust values to JavaScript representations, and utilities for escaping strings for use in JavaScript.

This crate is part of the [Viewpoint](https://github.com/user/viewpoint) browser automation framework.

## Features

- **`ToJsValue` trait**: Convert Rust types to JavaScript value strings
- **String escaping**: Properly escape strings for JavaScript contexts
- **CSS attribute escaping**: Escape values for CSS attribute selectors
- **JSON support**: Optional `serde_json::Value` conversion (with `json` feature)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
viewpoint-js-core = "0.2"
```

For JSON support:

```toml
[dependencies]
viewpoint-js-core = { version = "0.2", features = ["json"] }
```

## Quick Start

```rust
use viewpoint_js_core::{ToJsValue, escape_js_string};

// Convert Rust values to JavaScript representation
assert_eq!(42.to_js_value(), "42");
assert_eq!(true.to_js_value(), "true");
assert_eq!("hello".to_js_value(), r#""hello""#);

// Escape a string for JavaScript
assert_eq!(escape_js_string("line1\nline2"), r#""line1\nline2""#);
```

## The `ToJsValue` Trait

The [`ToJsValue`] trait converts Rust values to their JavaScript representations.
It's used by the `js!` macro for value interpolation (`#{expr}`):

```rust
use viewpoint_js_core::ToJsValue;

// Integers
assert_eq!(42i32.to_js_value(), "42");
assert_eq!((-17i64).to_js_value(), "-17");

// Floats
assert_eq!(3.14f64.to_js_value(), "3.14");
assert_eq!(f64::INFINITY.to_js_value(), "Infinity");
assert_eq!(f64::NAN.to_js_value(), "NaN");

// Booleans
assert_eq!(true.to_js_value(), "true");
assert_eq!(false.to_js_value(), "false");

// Strings (properly quoted and escaped)
assert_eq!("hello".to_js_value(), r#""hello""#);
assert_eq!("say \"hi\"".to_js_value(), r#""say \"hi\"""#);
assert_eq!("line1\nline2".to_js_value(), r#""line1\nline2""#);

// Option types
assert_eq!(Some(42).to_js_value(), "42");
assert_eq!(None::<i32>.to_js_value(), "null");
```

## String Escaping Functions

### `escape_js_string` - Double-Quoted Strings

Escapes a string for use in a double-quoted JavaScript string literal:

```rust
use viewpoint_js_core::escape_js_string;

assert_eq!(escape_js_string("hello"), r#""hello""#);
assert_eq!(escape_js_string("it's fine"), r#""it's fine""#);  // Single quotes preserved
assert_eq!(escape_js_string(r#"say "hi""#), r#""say \"hi\"""#);  // Double quotes escaped
assert_eq!(escape_js_string("tab\there"), r#""tab\there""#);
```

### `escape_js_string_single` - Single-Quoted Strings

Escapes a string for use in a single-quoted JavaScript string literal:

```rust
use viewpoint_js_core::escape_js_string_single;

assert_eq!(escape_js_string_single("hello"), "'hello'");
assert_eq!(escape_js_string_single("it's"), r"'it\'s'");  // Single quotes escaped
assert_eq!(escape_js_string_single(r#"say "hi""#), r#"'say "hi"'"#);  // Double quotes preserved
```

### `escape_js_contents` - Without Quotes

Escapes string contents without adding surrounding quotes:

```rust
use viewpoint_js_core::escape_js_contents;

assert_eq!(escape_js_contents("hello"), "hello");
assert_eq!(escape_js_contents("line1\nline2"), r"line1\nline2");
assert_eq!(escape_js_contents(r#"say "hi""#), r#"say \"hi\""#);
```

### `escape_for_css_attr` - CSS Attribute Selectors

Escapes a string for use in CSS attribute selectors within JavaScript:

```rust
use viewpoint_js_core::escape_for_css_attr;

// For: document.querySelector('[data-testid="submit-button"]')
let attr_value = escape_for_css_attr("submit-button");
assert_eq!(attr_value, r#"\"submit-button\""#);

// Use in a format string:
let selector = format!(r#"document.querySelector('[data-testid={}]')"#, attr_value);
assert_eq!(selector, r#"document.querySelector('[data-testid=\"submit-button\"]')"#);
```

## Implementing `ToJsValue` for Custom Types

You can implement `ToJsValue` for your own types:

```rust
use viewpoint_js_core::{ToJsValue, escape_js_string};

struct User {
    name: String,
    age: u32,
}

impl ToJsValue for User {
    fn to_js_value(&self) -> String {
        format!(
            r#"{{ name: {}, age: {} }}"#,
            escape_js_string(&self.name),
            self.age
        )
    }
}

let user = User { name: "John".to_string(), age: 30 };
assert_eq!(user.to_js_value(), r#"{ name: "John", age: 30 }"#);
```

## Integration with Viewpoint

This crate is used internally by [`viewpoint-js`](https://crates.io/crates/viewpoint-js) for the `js!` macro. Most users should use `viewpoint-js` directly rather than this crate.

## License

MIT
