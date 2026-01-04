# Project Context

## Purpose
Viewpoint is a high-performance browser automation and E2E testing framework in Rust—the Rust equivalent of Playwright.

### Goals
- Native Rust API for browser automation
- High-performance E2E testing without JavaScript/Node.js
- Chromium as initial target, extensible to other browsers

## Tech Stack
- **Language**: Rust (stable, via rust-overlay, edition 2024)
- **Async Runtime**: Tokio
- **Error Handling**: `thiserror` (libraries), `anyhow` (binaries)
- **Build Environment**: Nix flakes + direnv
- **TLS / SSL**: Rusttls
- **Browser**: Chromium via CDP (Chrome DevTools Protocol)

## Conventions

### Code Style
- `rustfmt` default configuration
- Pedantic clippy lints enabled
- Prefer explicit over implicit

### Module Structure
- **Folder modules only** (directories, not single `.rs` files)
- **No inline tests** (`#[cfg(test)] mod tests` blocks)
- **Maximum 500 lines per file** — refactor into smaller modules if exceeded

```
module_name/
├── mod.rs        # Public exports
├── error.rs      # Module-specific errors (thiserror)
├── tests/        # Unit tests (folder module)
│   ├── mod.rs
│   └── *.rs
└── ...
```

```rust
// In mod.rs
#[cfg(test)]
mod tests;
```

### Naming
- Error types: `{Module}Error` (e.g., `BrowserError`)
- Result aliases: `type Result<T> = std::result::Result<T, {Module}Error>`
- Async: prefer `async fn` over `impl Future`

### Architecture
- **Workspace required**: Always use a Cargo workspace with multiple crates, never a single-crate project
- **Separate crates**: domain logic, protocol adapters, database adapters, CLI, public API, etc
- **Hexagonal Architecture (Ports & Adapters)**: Core logic independent of external concerns

### JavaScript Code
- **Always use `viewpoint_js::js!` macro** for inline JavaScript code
- The macro provides **compile-time syntax validation** catching JS errors before runtime
- **Full JavaScript syntax support**: single-quoted strings, template literals, regex, XPath, etc.
- Use `#{expr}` for **value interpolation** (quoted/escaped Rust values)
- Use `@{expr}` for **raw interpolation** (inject pre-built JS expressions as-is)
- JavaScript `${expr}` in template literals is preserved (not Rust interpolation)
- **Use `viewpoint_js_core` utilities** for string escaping when building dynamic JS
- Never use raw string literals or `format!` for JavaScript code

```rust
use viewpoint_js::js;

// Simple expression (returns &'static str)
let code = js!{ document.title };

// Single-quoted strings
let code = js!{ console.log('hello') };

// Template literals with JS interpolation (${} is preserved)
let code = js!{ `Hello, ${userName}!` };

// Template literals with Rust interpolation
let name = "world";
let code = js!{ `Hello, #{name}!` };

// Regex literals
let code = js!{ /^test/.test(str) };

// XPath with mixed quotes
let code = js!{ document.evaluate("//div[@class='item']", doc) };

// Value interpolation - Rust values are quoted/escaped
let selector = ".my-class";
let code = js!{ document.querySelector(#{selector}) };

// Raw interpolation - inject JS expression as-is
let selector_expr = "document.body";
let code = js!{ @{selector_expr}.getAttribute("id") };

// Multi-line with both interpolation types
let js_fn = some_js_function();
let attr = "data-id";
let code = js!{
    (function() {
        const fn = @{js_fn};
        return fn(document).getAttribute(#{attr});
    })()
};
```

#### JavaScript String Escaping Utilities

When building JavaScript strings dynamically outside the `js!` macro, use `viewpoint_js_core`:

```rust
use viewpoint_js_core::{
    escape_js_string,         // "hello" -> "\"hello\""
    escape_js_string_single,  // "hello" -> "'hello'"
    escape_js_contents,       // For double-quoted strings (no outer quotes)
    escape_js_contents_single,// For single-quoted strings (no outer quotes)  
    escape_for_css_attr,      // For CSS attribute selectors in JS
    ToJsValue,                // Trait for value interpolation
};

// Escape for double-quoted JS string
let s = escape_js_string("say \"hi\"");  // "\"say \\\"hi\\\"\""

// Escape for CSS attribute selector inside JS
let id = escape_for_css_attr("my-id");   // "\\\"my-id\\\""
let selector = format!(r#"document.querySelector('[data-id={}]')"#, id);

// ToJsValue trait for type-safe conversion
let num = 42.to_js_value();      // "42"
let flag = true.to_js_value();   // "true"  
let text = "hello".to_js_value(); // "\"hello\""
```

### Testing

| Type | Location | Chromium? | Command |
|------|----------|-----------|---------|
| Unit | `src/**/tests/` | No (mocked) | `cargo test` |
| Integration | `tests/` (crate root) | Yes | `cargo test --features integration` |

**Integration tests** require the `integration` feature flag:
```toml
[features]
integration = []
```
```rust
#![cfg(feature = "integration")]
```

**IMPORTANT: Always run BOTH test commands when implementing changes:**
```bash
# Unit tests (fast, no browser)
cargo test --workspace

# Integration tests (requires Chromium)
cargo test --workspace --features integration
```

Integration tests are NOT run by default. Failing to run integration tests will miss real browser interaction bugs.

**Requirements**:
- New features must include integration tests with real Chromium
- Test both success and failure paths
- Use `tracing` + `tracing-subscriber` with `env-filter` for test output

### Version Control
- **VCS**: jj (Jujutsu), not git
- **Commits**: Conventional commits (feat:, fix:, refactor:, docs:, test:, chore:)
