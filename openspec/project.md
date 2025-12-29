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
- **Hexagonal Architecture**: Core logic independent of external concerns
- **Separate crates**: domain logic, protocol adapters (CDP), CLI, public API

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

**Requirements**:
- New features must include integration tests with real Chromium
- Test both success and failure paths
- Use `tracing` + `tracing-subscriber` with `env-filter` for test output

### Version Control
- **VCS**: jj (Jujutsu), not git
- **Commits**: Conventional commits (feat:, fix:, refactor:, docs:, test:, chore:)

## Domain

### Core Concepts
- **Browser**: Running browser instance
- **Context**: Isolated session (like incognito)
- **Page**: Single tab/window
- **Element**: DOM element for interaction
- **CDP**: Chrome DevTools Protocol for browser communication

### Key Operations
- Lifecycle: launch, connect, close
- Navigation: goto, reload, back, forward
- Interaction: click, type, select, hover
- Evaluation: execute JavaScript
- Waiting: selectors, navigation, network idle
- Capture: screenshots, PDFs

## Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `thiserror` | Library error types |
| `serde` / `serde_json` | CDP serialization |
| `tokio-tungstenite` | WebSocket communication |
| `tracing` | Structured logging |
| `tracing-subscriber` | Test output (dev) |

## Workspace Structure
```
viewpoint/
├── Cargo.toml
├── crates/
│   ├── viewpoint-cdp/          # CDP protocol
│   ├── viewpoint-core/         # Core domain
│   ├── viewpoint-test/         # Test framework
│   └── viewpoint-test-macros/  # Proc macros
├── tests/                      # Integration tests
└── examples/
```
