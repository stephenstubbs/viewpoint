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

### Testing Dynamic WebSocket Content

When testing applications that receive real-time data via WebSocket, you can verify that data updates are correctly reflected in the DOM without polling by using Viewpoint's auto-waiting assertions:

```rust
use viewpoint_core::Browser;
use viewpoint_test::expect;
use std::sync::Arc;
use tokio::sync::Mutex;

let browser = Browser::launch().headless(true).launch().await?;
let context = browser.new_context().await?;
let page = context.new_page().await?;

// Capture WebSocket messages for verification
let messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
let messages_clone = messages.clone();

// Set up WebSocket monitoring BEFORE navigation
page.on_websocket(move |ws| {
    let msgs = messages_clone.clone();
    async move {
        ws.on_framereceived(move |frame| {
            let msgs = msgs.clone();
            async move {
                if frame.is_text() {
                    msgs.lock().await.push(frame.payload().to_string());
                }
            }
        }).await;
    }
}).await;

// Navigate to page with WebSocket-driven live data
page.goto("https://example.com/live-dashboard").goto().await?;

// Auto-waiting assertions verify DOM updates WITHOUT polling!
// These automatically wait up to 30 seconds for the condition to be true

// Verify that live data container becomes visible
expect(page.locator(".live-data-container")).to_be_visible().await?;

// Verify that WebSocket data rendered specific text content
expect(page.locator(".stock-price")).to_contain_text("$").await?;

// Verify connection status updated
expect(page.locator(".connection-status")).to_have_text("Connected").await?;

// Verify a list populated by WebSocket messages has items
expect(page.locator(".message-list li")).to_have_count_greater_than(0).await?;

// Optionally verify WebSocket messages were received
let received = messages.lock().await;
assert!(!received.is_empty(), "Should have received WebSocket data");
```

The key insight is that `expect(locator).to_have_text()`, `to_be_visible()`, and similar assertions automatically wait for the condition to become true, eliminating the need for manual polling or sleep statements when testing WebSocket-driven UI updates.

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

### Accessibility Testing at Scale

For testing accessibility across multiple pages while managing performance:

```rust
use viewpoint_core::{Browser, AriaRole};

let browser = Browser::launch().headless(true).launch().await?;

// Define pages to audit
let pages_to_audit = vec![
    "https://example.com/",
    "https://example.com/about",
    "https://example.com/contact",
];

for url in pages_to_audit {
    // Fresh context per page for isolation, reuse browser for performance
    let mut context = browser.new_context().await?;
    let page = context.new_page().await?;
    page.goto(url).goto().await?;
    
    // Validate semantic HTML landmarks
    let main_count = page.get_by_role(AriaRole::Main).build().count().await?;
    assert!(main_count >= 1, "{}: Missing <main> landmark", url);
    
    let nav_count = page.get_by_role(AriaRole::Navigation).build().count().await?;
    assert!(nav_count >= 1, "{}: Missing <nav> landmark", url);
    
    // Validate heading hierarchy
    let h1_count = page.locator("h1").count().await?;
    assert!(h1_count >= 1, "{}: Missing <h1>", url);
    
    // Validate images have alt text
    let images = page.locator("img");
    for i in 0..images.count().await? {
        let alt = images.nth(i as i32).get_attribute("alt").await?;
        assert!(alt.is_some(), "{}: Image {} missing alt", url, i);
    }
    
    // Validate buttons have accessible names
    let buttons = page.get_by_role(AriaRole::Button).build();
    for i in 0..buttons.count().await? {
        let btn = buttons.nth(i as i32);
        let text = btn.text_content().await?;
        let label = btn.get_attribute("aria-label").await?;
        assert!(
            text.map(|t| !t.trim().is_empty()).unwrap_or(false) || label.is_some(),
            "{}: Button {} missing accessible name", url, i
        );
    }
    
    // Validate form inputs have labels
    let inputs = page.locator("input:not([type='hidden'])");
    for i in 0..inputs.count().await? {
        let input = inputs.nth(i as i32);
        let has_label = input.get_attribute("id").await?.is_some()
            || input.get_attribute("aria-label").await?.is_some()
            || input.get_attribute("aria-labelledby").await?.is_some();
        assert!(has_label, "{}: Input {} missing label", url, i);
    }
    
    // Capture ARIA snapshot for review
    let snapshot = page.aria_snapshot().await?;
    println!("{}: {}", url, snapshot.to_yaml());
    
    context.close().await?;
}
```

**Performance tips**: Share `Browser` across pages, use fresh `Context` per page for isolation, and use `get_by_role()` to query the accessibility tree directly.

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
