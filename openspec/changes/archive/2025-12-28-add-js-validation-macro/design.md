# Design: JavaScript Validation Macro

## Context

Viewpoint extensively uses JavaScript strings for browser automation:
- `page.evaluate("expression")` - Execute JavaScript and return result
- `locator.evaluate("el => el.textContent")` - Evaluate on element
- `page.wait_for_function("() => condition")` - Wait for condition

Currently these are plain strings with no compile-time validation. The `serde_json::json!` macro demonstrates that compile-time validation of embedded DSLs is valuable and idiomatic in Rust.

## Goals

- Validate JavaScript syntax at compile time
- Provide clear error messages for invalid JavaScript
- Support Rust variable interpolation into JavaScript
- Zero runtime overhead (macro expands to a string literal)
- Optional crate - existing string API remains available

## Non-Goals

- Full JavaScript type checking (would require a complete JS type system)
- JavaScript minification or optimization
- Support for all JavaScript features (ES modules, etc.)
- Browser-specific API validation (DOM APIs, etc.)

## Decisions

### Decision: Use a declarative macro with embedded parser

**Rationale**: A proc-macro with an embedded JavaScript parser provides:
- Compile-time syntax validation
- Good error messages with span information
- Support for interpolation syntax

**Alternatives considered**:
1. **Build script validation** - Would validate at build time but couldn't provide good error spans
2. **External tool** - Would require additional tooling setup
3. **Runtime validation** - Defeats the purpose of compile-time checking

### Decision: Use `swc_ecma_parser` for JavaScript parsing

**Rationale**: swc is a mature, fast JavaScript/TypeScript parser written in Rust. It's used by major projects (Next.js, Deno) and provides:
- Complete ES2023 syntax support
- Excellent error messages
- Fast compilation
- Pure Rust (no native dependencies)

**Alternatives considered**:
1. **tree-sitter-javascript** - Good for IDE tooling but heavier dependency
2. **boa_parser** - Less mature, focused on JS engine not tooling
3. **Custom parser** - Too much work for diminishing returns

### Decision: Interpolation syntax `#{expr}`

**Rationale**: Uses `#{}` syntax (similar to Ruby string interpolation) to embed Rust expressions:
- Visually distinct from JavaScript syntax
- Doesn't conflict with template literals (which use `${}`)
- Easy to parse and escape

```rust
let id = "my-id";
js!{ document.getElementById(#{id}) }
// Expands to: "document.getElementById(\"my-id\")"
```

### Decision: Support both expressions and statement blocks

**Rationale**: JavaScript in browser automation can be:
- Simple expressions: `1 + 2`
- Arrow functions: `() => window.innerWidth`
- IIFEs: `(() => { ... })()`
- Statement blocks for side effects

The macro will accept any valid JavaScript expression or statement sequence.

## Architecture

```
viewpoint-js/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Crate root, re-exports macro
│   ├── parser.rs        # JavaScript parsing logic
│   ├── interpolation.rs # Rust interpolation handling
│   └── escape.rs        # String escaping utilities
└── tests/
    ├── valid_js.rs      # Tests for valid JavaScript
    ├── invalid_js.rs    # Tests for error messages
    └── interpolation.rs # Tests for interpolation
```

### Macro Expansion

Input:
```rust
js!{ () => window.innerWidth }
```

Expansion:
```rust
"() => window.innerWidth"
```

Input with interpolation:
```rust
let selector = ".my-class";
js!{ document.querySelector(#{selector}) }
```

Expansion:
```rust
{
    let __js_arg_0 = viewpoint_js::to_js_value(&selector);
    format!("document.querySelector({})", __js_arg_0)
}
```

## Risks / Trade-offs

### Risk: Compile time increase
- **Mitigation**: swc is very fast; parsing small JS snippets is negligible
- **Mitigation**: Make the crate optional; users can stick with strings

### Risk: Interpolation type safety
- **Trade-off**: We can validate that interpolated values implement `ToJsValue`, but can't validate they make sense in context
- **Mitigation**: Clear documentation about what types can be interpolated

### Risk: Maintenance burden of JS parser updates
- **Mitigation**: swc is actively maintained and follows ECMAScript standards
- **Mitigation**: We only need parsing, not code generation, so updates are low-risk

## Open Questions

1. **Should we support TypeScript syntax?** - swc supports it, but may add confusion
2. **Should interpolation support complex expressions?** - Start simple, expand if needed
3. **Should the macro produce `&'static str` or `String`?** - Static for simple cases, String for interpolation
