# Project Context

## Purpose
Viewpoint is a high-performance browser automation and end-to-end testing framework written entirely in Rust. It aims to be the Rust equivalent of Playwright, enabling developers to write browser automation and E2E tests in Rust with native performance benefits.

### Goals
- Provide a native Rust API for browser automation
- Enable high-performance E2E testing without JavaScript/Node.js overhead
- Support Chromium as the initial browser target
- Deliver a seamless developer experience for Rust-first teams

## Tech Stack
- **Language**: Rust (stable, via rust-overlay)
- **Async Runtime**: Tokio
- **Error Handling**: 
  - `thiserror` for library crates (typed errors)
  - `anyhow` for binary crates (ergonomic error propagation)
- **Build/Dev Environment**: Nix flakes + direnv
- **Browser Target**: Chromium (initial), potentially others later

## Project Conventions

### Code Style
- Use `rustfmt` for formatting (default configuration)
- Enable pedantic clippy lints across all crates
- Prefer explicit over implicit where reasonable

### Module Structure
- Modules should be organized as directories (not single files)
- Each module directory should contain:
  ```
  module_name/
  ├── mod.rs        # Public exports and module organization
  ├── error.rs      # Module-specific error types (using thiserror)
  └── ...           # Other implementation files
  ```

### Naming Conventions
- Error types: `{Module}Error` (e.g., `BrowserError`, `NavigationError`)
- Result type aliases: `type Result<T> = std::result::Result<T, {Module}Error>`
- Async functions: prefer `async fn` over returning `impl Future`

### Architecture Patterns
- **Hexagonal Architecture** (Ports and Adapters):
  - Core domain logic is independent of external concerns
  - Ports define interfaces (traits) for external interactions
  - Adapters implement ports for specific technologies (e.g., CDP for Chromium)
- Separate concerns into distinct crates:
  - Core domain/business logic
  - Protocol adapters (CDP, WebDriver, etc.)
  - CLI interface
  - Public API/SDK

### Testing Strategy
- **Unit Tests**: Colocated with source code in `#[cfg(test)]` modules
- **Integration Tests**: In `tests/` directory at crate root
- Test browser automation against real Chromium instance
- Mock protocol layer for unit testing core logic
- **Tracing in Tests**: Use `tracing` for instrumentation and `tracing-subscriber` with `env-filter` for test output
  - Add `tracing` as a regular dependency
  - Add `tracing-subscriber = { version = "0.3", features = ["env-filter"] }` as a dev-dependency
  - Initialize subscriber in tests with `RUST_LOG` environment variable support

### Version Control
- **VCS**: jj (Jujutsu) - NOT git
- **Branching**: Feature branches off main
- **Commits**: 
  - Use conventional commits style (feat:, fix:, refactor:, docs:, test:, chore:)
  - Keep commits atomic and focused

## Domain Context

### Browser Automation Domain
- **CDP (Chrome DevTools Protocol)**: Primary protocol for Chromium communication
- **Browser**: A running browser instance
- **Context**: An isolated browser session (like incognito)
- **Page**: A single tab/window in a context
- **Element**: A DOM element that can be interacted with
- **Selector**: Strategy for finding elements (CSS, XPath, text, etc.)

### Key Operations
- Browser lifecycle: launch, connect, close
- Navigation: goto, reload, back, forward
- Element interaction: click, type, select, hover
- Evaluation: execute JavaScript in page context
- Waiting: wait for selectors, navigation, network idle
- Screenshots/PDFs: visual capture of pages

## Important Constraints
- Must support async/await throughout the API
- Should not require Node.js or any JavaScript runtime
- Initial focus on Chromium; architecture should allow adding Firefox/WebKit later
- Performance is a first-class concern

## External Dependencies

### Core Dependencies
| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `thiserror` | Library error types |
| `anyhow` | Binary error handling |
| `serde` / `serde_json` | CDP message serialization |
| `tokio-tungstenite` | WebSocket communication with browser |
| `tracing` | Structured logging/diagnostics |

### Dev Dependencies
| Crate | Purpose |
|-------|---------|
| `tracing-subscriber` | Test output with `env-filter` feature for `RUST_LOG` support |

### Likely Future Dependencies
| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |

### External Services
- Chromium browser (provided via Nix flake)
- Chrome DevTools Protocol (CDP) over WebSocket

## Workspace Structure
```
viewpoint/
├── Cargo.toml              # Workspace manifest
├── crates/
│   ├── viewpoint-cdp/      # Chrome DevTools Protocol implementation
│   ├── viewpoint-core/     # Core domain logic
│   ├── viewpoint-test/     # Test framework with assertions and fixtures
│   └── viewpoint-test-macros/  # Proc macros for test setup
├── tests/                  # Workspace-level integration tests
└── examples/               # Usage examples
```
