# Design: API Testing

## Context

API testing allows making HTTP requests without browser overhead. The API context can share cookies with browser context for authenticated requests.

## Goals

- Provide full HTTP request capabilities
- Share cookies/state with browser context
- Support common patterns (REST, GraphQL)
- Lightweight without browser overhead

## Decisions

### Decision 1: HTTP Client

**Choice**: Use `reqwest` for HTTP requests.

**Rationale**:
- Mature, well-maintained Rust HTTP client
- Async support
- Cookie jar support for sharing with browser
- TLS, proxy, timeout support

### Decision 2: Context Types

**Choice**: Two ways to get request context.

```rust
// From browser context - shares cookies
let api = context.request();
let response = api.get("/api/user").await?;

// Standalone - independent
let api = playwright.request.new_context(APIContextOptions::new()
    .base_url("https://api.example.com")
).await?;
```

### Decision 3: Request Building

**Choice**: Builder pattern for requests.

```rust
let response = api.post("/api/users")
    .json(&user_data)
    .header("X-Custom", "value")
    .await?;
```

## Implementation Notes

- Does not use CDP - pure HTTP client
- Cookie sync with browser context via shared cookie jar
- Response streaming for large bodies

## Open Questions

None.
