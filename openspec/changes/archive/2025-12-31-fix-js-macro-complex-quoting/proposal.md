# Change: Fix js! Macro for Complex JavaScript Quoting

## Why

The current `js!` macro relies on Rust's tokenizer to parse JavaScript code, which causes failures with valid JavaScript syntax that conflicts with Rust's lexical rules. This prevents users from writing common patterns like:

- **Single-quoted JavaScript strings**: `'hello'` is interpreted as a Rust character literal
- **Template literals**: `` `hello ${name}` `` - backticks are not valid Rust tokens
- **Regex literals**: `/pattern/` - parsed incorrectly by Rust tokenizer
- **XPath expressions**: Often require mixing single and double quotes

These are essential for browser automation (XPath selectors, CSS attribute selectors, template strings).

## What Changes

- **Replace Rust tokenizer with custom JavaScript-aware tokenizer**: Parse the raw input span directly instead of relying on `proc_macro::TokenStream`
- **Preserve existing syntax**: `js!{ ... }` syntax unchanged - users don't need to change anything
- **Support all JavaScript string types**: Single quotes, double quotes, template literals
- **Support regex literals**: `/pattern/flags` syntax
- **Maintain IDE syntax highlighting**: Because we keep the `js!{ }` syntax, IDEs continue to tokenize and highlight the content
- **Maintain Rust interpolation**: Both `#{expr}` and `@{expr}` continue to work (see rationale below)

## Rationale: Why Keep Custom Interpolation (`#{expr}` and `@{expr}`)?

Even with full JavaScript template literal support (`${expr}`), Rust interpolation remains necessary because they serve **fundamentally different purposes**:

| Syntax | Purpose | When Executed | Example |
|--------|---------|---------------|---------|
| JS `${name}` | Interpolate JS variable | JS runtime | `` `Hello, ${name}!` `` - `name` is a JS variable |
| Rust `#{selector}` | Inject Rust value as JS literal | Rust compile/runtime | `js!{ document.querySelector(#{selector}) }` - `selector` is a Rust `&str` |
| Rust `@{expr}` | Inject pre-built JS expression | Rust compile/runtime | `js!{ @{js_selector}.click() }` - `js_selector` contains JS code |

**Real usage from codebase (67+ usages of `#{expr}`, 42+ usages of `@{expr}`):**

```rust
// #{expr} - Rust value → JavaScript value (via ToJsValue trait)
let selector = ".my-class";  // Rust string
js!{ document.querySelector(#{selector}) }  // → document.querySelector(".my-class")

// @{expr} - Inject pre-built JavaScript expression
let js_code = "document.querySelectorAll('.item')";  // JS code as Rust string
js!{ Array.from(@{js_code}) }  // → Array.from(document.querySelectorAll('.item'))

// ${expr} - JavaScript template literal interpolation (NEW: will be supported)
js!{ `Hello, ${userName}!` }  // userName is a JavaScript variable, not Rust
```

**Key insight**: If we only had JavaScript's `${expr}`, there would be no way to inject Rust values into JavaScript. The custom Rust interpolation bridges the Rust ↔ JavaScript boundary.

## Impact

- **Affected specs**: `js-validation`
- **Affected code**: 
  - `crates/viewpoint-js/src/js_macro/mod.rs` - Replace token-based parsing with span-based parsing
  - `crates/viewpoint-js/src/parser/mod.rs` - May need minor adjustments
  - `crates/viewpoint-js/src/lib.rs` - Update documentation
- **Breaking changes**: None - syntax is identical, behavior improves
