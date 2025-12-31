# viewpoint-core

High-level browser automation API for Rust, inspired by Playwright.

This crate provides the core browser, context, and page abstractions for the [Viewpoint](https://github.com/stephenstubbs/viewpoint) browser automation framework.

## Features

- Browser launching and management
- Page navigation with configurable wait states
- Element locators (CSS, text, role, label, test ID, placeholder)
- Element actions (click, fill, type, hover, check, select)
- Automatic element waiting
- **Automatic navigation waiting** - Actions that trigger navigation automatically wait for the page to load
- **Browser Context Features:**
  - Cookie management (add, get, clear)
  - Storage state persistence (including IndexedDB)
  - Permission granting (geolocation, notifications, camera, etc.)
  - Geolocation mocking
  - HTTP credentials for authentication
  - Extra HTTP headers
  - Offline mode simulation
  - Configurable timeouts
  - Context-level init scripts
  - Context-level route interception
- **Network Interception:**
  - Request routing and modification
  - Response mocking
  - HAR file replay
  - WebSocket monitoring
  - HTTP authentication handling
- **Event System:**
  - Page lifecycle events
  - Popup handling
  - Console and page error events
  - Dialog handling
  - Download handling
- **Advanced Features:**
  - Exposed Rust functions callable from JavaScript
  - ARIA accessibility snapshots
  - Element highlighting for debugging
  - Tracing for test debugging

## Usage

For testing, use `viewpoint-test` which re-exports this crate with additional assertions and test fixtures.

```rust
use viewpoint_core::{Browser, DocumentLoadState};

// Launch browser
let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;

// Navigate
page.goto("https://example.com")
    .wait_until(DocumentLoadState::DomContentLoaded)
    .goto()
    .await?;

// Locate and interact with elements
let button = page.locator("button.submit");
button.click().await?;
```

### Context Features

```rust
use viewpoint_core::{Browser, Cookie, Permission, SameSite};
use std::collections::HashMap;

let browser = Browser::launch().headless(true).launch().await?;

// Create context with options
let context = browser.new_context_builder()
    .geolocation(37.7749, -122.4194)  // San Francisco
    .permissions(vec![Permission::Geolocation])
    .has_touch(true)
    .locale("en-US")
    .timezone_id("America/Los_Angeles")
    .build()
    .await?;

let page = context.new_page().await?;
page.goto("https://example.com").goto().await?;

// Cookie management
context.add_cookies(vec![
    Cookie::new("session", "abc123")
        .domain("example.com")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict),
]).await?;

let cookies = context.cookies().await?;

// Extra HTTP headers
let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token123".to_string());
context.set_extra_http_headers(headers).await?;

// Offline mode
context.set_offline(true).await?;

// Save storage state for reuse
let state = context.storage_state().await?;
state.save("auth.json").await?;
```

### Event System

```rust
use viewpoint_core::{Browser, Page};

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;

// Listen for new pages (e.g., popups)
context.on_page(|page: Page| async move {
    println!("New page: {}", page.url().await.unwrap_or_default());
}).await;

let page = context.new_page().await?;

// Wait for a popup triggered by an action
let popup = context.wait_for_page(|| async {
    page.locator("a[target=_blank]").click().await?;
    Ok(())
}).await?;
```

### Network Interception

```rust
use viewpoint_core::{Browser, Route};

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;

// Mock an API response
page.route("**/api/users", |route: Route| async move {
    route.fulfill()
        .status(200)
        .json(&serde_json::json!({"users": []}))
        .send()
        .await
}).await?;

// Block certain requests
page.route("**/*.css", |route: Route| async move {
    route.abort().await
}).await?;

// HAR replay - serve requests from recorded HAR file
page.route_from_har("recordings/api.har").await?;
```

### WebSocket Monitoring

```rust
use viewpoint_core::{Browser, WebSocket};

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;

page.on_websocket(|ws: WebSocket| async move {
    println!("WebSocket opened: {}", ws.url());
    
    ws.on_framesent(|frame| async move {
        println!("Sent: {:?}", frame.payload());
        Ok(())
    }).await?;
    
    ws.on_framereceived(|frame| async move {
        println!("Received: {:?}", frame.payload());
        Ok(())
    }).await?;
    
    Ok(())
}).await?;
```

### Exposed Functions

```rust
use viewpoint_core::Browser;

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;

// Expose a Rust function to JavaScript
page.expose_function("compute", |args| async move {
    let x = args[0].as_i64().unwrap_or(0);
    let y = args[1].as_i64().unwrap_or(0);
    Ok(serde_json::json!(x + y))
}).await?;

// Now callable from JavaScript:
// const result = await window.compute(1, 2);  // returns 3
```

### ARIA Accessibility Testing

```rust
use viewpoint_core::Browser;

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;
page.goto("https://example.com").goto().await?;

// Get ARIA snapshot of an element
let snapshot = page.locator("nav").aria_snapshot().await?;
println!("{}", snapshot); // YAML-like output

// Compare with expected structure
let expected = AriaSnapshot::from_yaml(r#"
  - navigation:
    - link "Home"
    - link "About"
    - link "Contact"
"#)?;
assert!(snapshot.matches(&expected));
```

### Init Scripts

```rust
use viewpoint_core::Browser;

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;

// Add a script that runs before every page load
context.add_init_script(
    "Object.defineProperty(navigator, 'webdriver', { get: () => false })"
).await?;

// All pages in this context will have the script
let page = context.new_page().await?;
```

### Navigation Auto-Wait

Actions like `click()`, `press()`, `fill()`, `select_option()`, and `check()`/`uncheck()` automatically wait for any triggered navigation to complete before returning. This matches Playwright's default behavior and prevents race conditions.

```rust
use viewpoint_core::Browser;

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;
page.goto("https://example.com").goto().await?;

// Click a link - automatically waits for navigation to complete
page.locator("a#nav-link").click().await?;
// After click returns, the new page is fully loaded

// Press Enter in a search form - waits for search results page
page.locator("input#search").fill("query").await?;
page.locator("input#search").press("Enter").await?;
// Results page is now loaded

// Opt out of auto-waiting when needed
page.locator("a#link").click().no_wait_after(true).await?;
// Returns immediately without waiting for navigation

// Same for keyboard
page.keyboard().press("Enter").no_wait_after(true).await?;
```

## License

MIT
