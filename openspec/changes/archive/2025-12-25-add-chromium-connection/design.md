# Design: Chromium Connection with Wait System and Navigation

## Context

RustRight aims to be a high-performance Rust-native Playwright clone. The foundation requires:
1. CDP (Chrome DevTools Protocol) communication with Chromium
2. Browser lifecycle management (launch/connect/close)
3. Navigation with configurable wait states
4. Auto-waiting infrastructure for future element interactions

Playwright's architecture separates browser process management from protocol communication, which we will replicate using hexagonal architecture.

## Goals

- Launch Chromium and connect via CDP WebSocket
- Connect to already-running Chromium instances
- Navigate to URLs with Playwright-compatible wait states
- Provide builder-pattern API for ergonomic usage
- Optimize for test completion speed (minimal latency, efficient memory usage)

## Non-Goals

- Firefox/WebKit support (future work)
- Element selectors and interactions (future work)
- Network interception (future work)
- Browser installation/management (future work)

## Architecture

### Crate Structure

```
crates/
├── rustright-cdp/          # Low-level CDP protocol
│   └── src/
│       ├── mod.rs
│       ├── error.rs        # CdpError using thiserror
│       ├── connection.rs   # WebSocket connection management
│       ├── transport.rs    # CDP message framing
│       └── protocol/       # CDP domain types (generated or manual)
│           ├── mod.rs
│           ├── target.rs   # Target domain
│           ├── page.rs     # Page domain
│           └── runtime.rs  # Runtime domain
│
└── rustright-core/         # Core domain (hexagonal architecture)
    └── src/
        ├── mod.rs
        ├── error.rs        # CoreError using thiserror
        ├── browser/
        │   ├── mod.rs
        │   ├── error.rs
        │   ├── launcher.rs # Browser launching (port: BrowserLauncher trait)
        │   └── browser.rs  # Browser type
        ├── context/
        │   ├── mod.rs
        │   ├── error.rs
        │   └── context.rs  # BrowserContext type
        ├── page/
        │   ├── mod.rs
        │   ├── error.rs
        │   ├── page.rs     # Page type
        │   └── navigation.rs
        └── wait/
            ├── mod.rs
            ├── error.rs
            ├── load_state.rs   # DocumentLoadState enum
            └── waiter.rs       # Wait coordination
```

### Hexagonal Architecture Boundaries

```
┌─────────────────────────────────────────────────────────────┐
│                     rustright-core                          │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   Domain Layer                       │   │
│  │   Browser, BrowserContext, Page, Navigation          │   │
│  │   DocumentLoadState, WaitCondition                   │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   Port Layer (Traits)                │   │
│  │   BrowserLauncher, ProtocolTransport, PageDriver    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     rustright-cdp                           │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Adapter Layer (Implementations)         │   │
│  │   CdpTransport, ChromiumLauncher                    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### CDP Connection Flow

```
1. Launch Chromium with --remote-debugging-port=0
2. Read stderr for "DevTools listening on ws://..."
3. Connect WebSocket to CDP endpoint
4. Send Target.getBrowserContexts to verify connection
5. Create new context via Target.createBrowserContext
6. Create page via Target.createTarget with browserContextId
7. Attach to page via Target.attachToTarget
8. Page ready for navigation
```

### Wait States (Playwright-Compatible)

| State | Description | CDP Events |
|-------|-------------|------------|
| `commit` | Navigation response received | `Network.responseReceived` for main frame |
| `domcontentloaded` | DOM parsed | `Page.domContentEventFired` |
| `load` | Full page load (default) | `Page.loadEventFired` |
| `networkidle` | No network for 500ms | Custom tracking of `Network.*` events |

### Navigation Flow

```
page.goto("https://example.com", WaitUntil::Load)
    │
    ├─► Page.navigate(url)
    │       └─► Returns frameId, loaderId
    │
    ├─► Wait for specified load state
    │       ├─► Listen for Page.frameNavigated
    │       ├─► Listen for Page.loadEventFired (if WaitUntil::Load)
    │       ├─► Listen for Page.domContentEventFired (if WaitUntil::DomContentLoaded)
    │       └─► Track Network.* events (if WaitUntil::NetworkIdle)
    │
    └─► Return Response or timeout error
```

## Decisions

### Decision: Use tokio-tungstenite for WebSocket

**Why**: Native async Rust, zero-copy frame handling, well-maintained, minimal dependencies.

**Alternatives considered**:
- `async-tungstenite`: Similar but less popular
- `websocket`: Older, less async-friendly
- `reqwest` with WebSocket: Overkill for our needs

### Decision: Builder pattern for API

**Why**: Matches user preference, provides discoverable API, allows optional parameters without Option explosion.

**Example**:
```rust
let browser = Browser::launch()
    .headless(true)
    .args(["--no-sandbox"])
    .launch()
    .await?;
```

### Decision: Separate CDP message ID tracking per session

**Why**: Enables parallel operations across multiple pages without contention.

**Implementation**: Each `CdpSession` maintains its own atomic message counter.

### Decision: Use serde_json::RawValue for CDP responses

**Why**: Defer JSON parsing until the specific response type is needed, reducing overhead for events we don't care about.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| CDP protocol changes | Pin to specific Chromium version initially; add protocol version negotiation later |
| WebSocket connection drops | Implement reconnection logic with exponential backoff |
| networkidle timing sensitivity | Use configurable idle timeout (default 500ms like Playwright) |
| Memory usage for many pages | Lazy initialization of page resources; explicit cleanup on close |

## Open Questions

1. Should we generate CDP types from the protocol JSON or hand-write them?
   - **Recommendation**: Hand-write essential types first (Target, Page, Network, Runtime domains), generate later if needed

2. Should Browser::launch() auto-detect Chromium path?
   - **Recommendation**: Yes, check common paths and CHROMIUM_PATH env var; fall back to error with helpful message

3. Should we support connecting to remote Chromium (different host)?
   - **Recommendation**: Yes, `Browser::connect(ws_url)` should work for any reachable WebSocket URL
