# Design: Network Interception

## Context

Network interception requires intercepting requests at the CDP level, allowing user code to decide how to handle each request, and managing the lifecycle of intercepted requests. This is one of Playwright's most powerful features for testing.

## Goals

- Full request/response interception matching Playwright's capabilities
- Ergonomic Rust API with async handlers
- Efficient handling of binary response bodies
- Support for both page-level and context-level routing

## Non-Goals

- Service Worker interception (complex, defer to later)
- Automatic retry logic (user responsibility)
- Built-in caching beyond HAR replay

## Decisions

### Decision 1: CDP Domain Choice

**Choice**: Use the Fetch domain for request interception, Network domain for monitoring.

**Rationale**:
- Fetch domain provides `Fetch.requestPaused` for interception
- Fetch domain allows modifying requests before they're sent
- Network domain provides passive monitoring (events only)
- This matches how Playwright implements interception

**CDP Flow**:
1. Enable `Fetch.enable` with patterns to intercept
2. Receive `Fetch.requestPaused` events
3. Respond with `Fetch.fulfillRequest`, `Fetch.continueRequest`, or `Fetch.failRequest`

### Decision 2: Route Handler Architecture

**Choice**: Use async closures with `Route` parameter, stored in a handler registry.

```rust
page.route("**/*.css", |route| async move {
    route.abort().await
}).await?;

page.route("**/api/**", |route| async move {
    route.fulfill()
        .status(200)
        .json(json!({"mock": true}))
        .send()
        .await
}).await?;
```

**Rationale**:
- Async closures allow I/O in handlers (e.g., reading files)
- Route object provides builder pattern for fulfill options
- Registry allows multiple handlers with fallback chain
- Matches Playwright's API semantically

**Implementation**:
- Store handlers in `Vec<(Pattern, Box<dyn Fn(Route) -> Future>)>`
- Match in reverse order (last registered = first tried)
- `route.fallback()` passes to next matching handler

### Decision 3: URL Pattern Matching

**Choice**: Support both glob patterns and regex, matching Playwright's syntax.

**Glob Patterns**:
- `*` matches any characters except `/`
- `**` matches any characters including `/`
- `?` matches literal `?` (not single character)
- `{a,b}` matches alternatives

**Implementation**:
```rust
enum UrlPattern {
    Glob(String),
    Regex(Regex),
    Predicate(Box<dyn Fn(&str) -> bool>),
}
```

### Decision 4: Request/Response Types

**Choice**: Create `Request` and `Response` types that wrap CDP data with convenient accessors.

```rust
pub struct Request {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    post_data: Option<Vec<u8>>,
    resource_type: ResourceType,
    // ... timing, frame, redirect info
}

pub struct Response {
    status: u16,
    status_text: String,
    headers: HashMap<String, String>,
    request: Request,
    // body fetched lazily
}
```

**Rationale**:
- Owned types avoid lifetime complexity
- Lazy body fetching avoids unnecessary data transfer
- Methods like `json()`, `text()`, `body()` are async

### Decision 5: HAR File Format

**Choice**: Use standard HAR 1.2 format with serde for parsing.

**Rationale**:
- HAR is a well-documented standard
- Playwright uses HAR 1.2
- Existing Rust crates can help (or implement minimal parser)
- Allows interoperability with other tools

**API**:
```rust
// Replay from HAR
page.route_from_har("network.har", HarOptions::default()).await?;

// Record to HAR (via context tracing, proposal 8)
context.tracing().start(TracingOptions::new().har(true)).await?;
```

### Decision 6: Event System

**Choice**: Use async channels for network events, with typed event enums.

```rust
// Subscribe to events
let mut requests = page.on_request();
while let Some(request) = requests.recv().await {
    println!("Request: {}", request.url());
}

// Or use callback style
page.on("request", |request| {
    println!("Request: {}", request.url());
});
```

**Rationale**:
- Channels work well with async Rust
- Typed events provide compile-time safety
- Callback style available for convenience

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| Enable interception | Fetch.enable | Fetch |
| Request paused event | Fetch.requestPaused | Fetch |
| Fulfill request | Fetch.fulfillRequest | Fetch |
| Continue request | Fetch.continueRequest | Fetch |
| Fail request | Fetch.failRequest | Fetch |
| Get response body | Fetch.getResponseBody | Fetch |
| Network events | Network.requestWillBeSent, etc. | Network |
| Enable network | Network.enable | Network |

## Risks / Trade-offs

### Risk: Handler Ordering Complexity

**Mitigation**:
- Document LIFO (last-in-first-out) ordering clearly
- Provide `route.fallback()` for explicit chaining
- Consider adding priority parameter in future if needed

### Risk: Memory Usage with Large Responses

**Mitigation**:
- Lazy body fetching (only fetch when accessed)
- Streaming support for very large bodies (future enhancement)
- Document memory implications

### Risk: Async Handler Complexity

**Mitigation**:
- Provide sync convenience methods where possible
- Good documentation with examples
- Consider `route.fulfill_sync()` variants

## Open Questions

None - all design decisions resolved.
