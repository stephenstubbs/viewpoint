# Design: Test Framework

## Context

RustRight needs a test framework that feels familiar to Playwright users while being idiomatic Rust. The framework must integrate with `cargo test` (not a custom test runner) and prioritize explicitness and debuggability.

Key inspirations:
- **Playwright Test** - Fixtures, expect API, locator-based actions
- **sqlx** - TestHarness pattern with Drop-based cleanup
- **rstest** - Optional proc macro convenience layer

## Goals / Non-Goals

**Goals:**
- Work with standard `cargo test` runner
- Explicit, debuggable test setup via `TestHarness`
- Provide built-in `page`, `context`, `browser` access
- Optional `#[rustright::test]` macro for brevity
- Fluent async assertion API (`expect(&locator).to_be_visible().await?`)
- Auto-waiting locators with configurable timeouts
- Type-safe, compile-time checked API

**Non-Goals:**
- Custom test runner or CLI (use `cargo test`)
- Parallel test execution within a single test (Playwright's `test.parallel`)
- Test generation/codegen
- Visual comparison/screenshot diffing (future work)

## Decisions

### Decision 1: TestHarness as primary API

**What:** Users create a `TestHarness` explicitly in their tests, which manages browser lifecycle via Drop.

**Why:**
- Explicit and debuggable - no hidden code generation
- Excellent IDE support (no macro expansion needed)
- Fast compile times (no syn/quote dependency for basic usage)
- Familiar to Rust developers

**Primary API example:**
```rust
#[tokio::test]
async fn my_test() -> Result<()> {
    let harness = TestHarness::new().await?;
    let page = harness.page();
    
    page.goto("https://example.com").goto().await?;
    expect(&page.locator("h1")).to_have_text("Example").await?;
    
    Ok(()) // harness drops and cleans up
}
```

**Alternatives considered:**
- Proc macro as primary - Less debuggable, slower compiles, more "magic"
- Closure-based - Awkward with async move, lifetime issues

### Decision 2: Optional proc macro for convenience

**What:** `#[rustright::test]` macro available as optional convenience layer that generates `TestHarness` setup.

**Why:**
- Some users prefer minimal boilerplate
- Playwright users expect this style
- Low maintenance cost (thin wrapper over TestHarness)

**Secondary API example:**
```rust
#[rustright::test]
async fn my_test(page: Page) -> Result<()> {
    page.goto("https://example.com").goto().await?;
    expect(&page.locator("h1")).to_have_text("Example").await?;
    Ok(())
}

// Expands to:
#[tokio::test]
async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
    let _harness = TestHarness::new().await?;
    let page = _harness.page();
    
    page.goto("https://example.com").goto().await?;
    expect(&page.locator("h1")).to_have_text("Example").await?;
    Ok(())
}
```

**Macro with scoping:**
```rust
// Module-scoped browser for faster tests
#[rustright::test(scope = "browser", browser = "shared_browser")]
async fn fast_test(page: Page) -> Result<()> {
    page.goto("https://example.com").goto().await?;
    Ok(())
}

// Expands to:
#[tokio::test]
async fn fast_test() -> Result<(), Box<dyn std::error::Error>> {
    let _harness = TestHarness::from_browser(shared_browser().await).await?;
    let page = _harness.page();
    
    page.goto("https://example.com").goto().await?;
    Ok(())
}
```

### Decision 3: Two-crate architecture

**What:** Separate `rustright-test-macros` (proc macros) from `rustright-test` (runtime).

**Why:** Rust requires proc macros to be in a separate crate. Users who don't want macros only depend on `rustright-test`.

**Dependency structure:**
```
rustright-test (primary - no proc macro deps)
├── rustright-core
└── tokio

rustright-test-macros (optional)
├── rustright-test
├── syn
├── quote
└── proc-macro2
```

### Decision 4: Locator as a lazy, chainable handle

**What:** `Locator` is a lightweight struct that stores selection criteria but doesn't query the DOM until an action is performed.

**Why:** This enables:
- Auto-waiting built into actions
- Chainable refinement (`page.locator(".list").locator(".item")`)
- No stale element references

**Structure:**
```rust
pub struct Locator<'a> {
    page: &'a Page,
    selector: Selector,
    options: LocatorOptions,
}

pub enum Selector {
    Css(String),
    Text { text: String, exact: bool },
    Role { role: AriaRole, name: Option<String> },
    TestId(String),
    Label(String),
    Placeholder(String),
    Chained(Box<Selector>, Box<Selector>),
}
```

### Decision 5: Expect returns Result, not panic

**What:** Assertions return `Result<(), AssertionError>` instead of panicking.

**Why:**
- Consistent with Rust's error handling idioms
- Enables soft assertions (collect multiple failures)
- Better integration with async code (no `catch_unwind` needed)
- User can use `?` operator for clean test code

**Example:**
```rust
expect(&locator).to_be_visible().await?;  // Returns Result
expect(&locator).to_have_text("Hello").await?;
```

### Decision 6: Actions live on Locator, not Page

**What:** User interactions (`click`, `fill`, `type`) are methods on `Locator`, not `Page`.

**Why:**
- Enforces element selection before action
- Auto-waiting is natural (wait for element, then act)
- Matches Playwright's modern API
- Reduces chance of acting on wrong element

**Example:**
```rust
// Good: Action on locator
page.locator("button").click().await?;

// Not provided: Direct page action (Playwright legacy API)
// page.click("button").await?;
```

## Risks / Trade-offs

### Risk: Proc macro complexity
**Mitigation:** Macro is optional and thin - just generates TestHarness setup. Users can always fall back to explicit harness.

### Risk: CDP command coverage for actions
**Mitigation:** Implement core actions first (click, fill, type). Defer complex actions (drag-drop, file upload) to later iterations.

### Trade-off: Two ways to write tests
**Pro:** Users choose their preferred style
**Con:** Must document both approaches clearly

## Crate Structure

```
crates/
├── rustright-test/              # Test framework runtime (PRIMARY)
│   ├── src/
│   │   ├── lib.rs               # Re-exports
│   │   ├── harness.rs           # TestHarness struct
│   │   ├── config.rs            # TestConfig builder
│   │   ├── expect/              # Assertion API
│   │   │   ├── mod.rs
│   │   │   ├── locator.rs       # Locator assertions
│   │   │   └── page.rs          # Page assertions
│   │   └── error.rs             # AssertionError
│   └── Cargo.toml
├── rustright-test-macros/       # Proc macros (OPTIONAL)
│   ├── src/
│   │   ├── lib.rs
│   │   └── test_attr.rs         # #[rustright::test]
│   └── Cargo.toml
└── rustright-core/              # Extended with locators/actions
    └── src/
        └── page/
            ├── locator.rs       # Locator type
            └── actions.rs       # click, fill, type, etc.
```

### Decision 7: Flexible fixture scoping

**What:** TestHarness supports three scoping levels via different constructors.

**Why:**
- Test-scoped (default): Maximum isolation, but slower
- Module-scoped browser: Faster, share browser across tests
- Shared context: Even faster for tests that can share cookies/storage

**API:**
```rust
// Test-scoped (default) - new browser per test
let harness = TestHarness::new().await?;

// Module-scoped - reuse browser, fresh context/page
let harness = TestHarness::from_browser(&browser).await?;

// Shared context - reuse context, fresh page only
let harness = TestHarness::from_context(&context).await?;
```

**Ownership model:**
| Constructor | Owns Browser | Owns Context | Owns Page |
|-------------|--------------|--------------|-----------|
| `new()` | Yes | Yes | Yes |
| `from_browser()` | No | Yes | Yes |
| `from_context()` | No | No | Yes |

Only owned resources are cleaned up when harness drops.

**Module-scoped example:**
```rust
use std::sync::LazyLock;
use tokio::sync::OnceCell;

static BROWSER: LazyLock<OnceCell<Browser>> = LazyLock::new(OnceCell::new);

async fn shared_browser() -> &'static Browser {
    BROWSER.get_or_init(|| async {
        Browser::launch().await.unwrap()
    }).await
}

#[tokio::test]
async fn test_one() -> Result<()> {
    let harness = TestHarness::from_browser(shared_browser().await).await?;
    // Fresh context/page, shared browser
    Ok(())
}
```

### Decision 8: Tracing integration

**What:** The framework does not manage tracing. Users configure tracing using the `Once` pattern from project.md.

**Why:**
- Consistent with existing crates (rustright-cdp, rustright-core)
- `Once` ensures subscriber initialized exactly once across parallel tests
- Framework stays simple and focused

**Recommended pattern:**
```rust
use std::sync::Once;

static TRACING_INIT: Once = Once::new();

fn init_tracing() {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with_test_writer()
            .try_init()
            .ok();
    });
}

#[tokio::test]
async fn my_test() -> Result<()> {
    init_tracing();
    
    let harness = TestHarness::new().await?;
    // ...
}
```

Benefits:
- `Once` ensures subscriber is initialized exactly once across all tests
- `with_test_writer()` integrates with `cargo test` output capture
- `EnvFilter::from_default_env()` respects `RUST_LOG` environment variable
- Default to INFO level if `RUST_LOG` not set

## Open Questions

None - all questions resolved.
