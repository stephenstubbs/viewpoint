# Design: Auto-Wait for Navigation After Actions

## Context

Playwright's actions (click, press, fill, etc.) automatically wait for any navigation they trigger to complete before returning. This is documented in their API:

> "Actions that initiate navigations normally wait for these navigations to happen and for pages to start loading."
> - Playwright `noWaitAfter` option documentation

This behavior is crucial for reliable browser automation because:
1. Form submissions via Enter key trigger navigation
2. Link clicks trigger navigation  
3. JavaScript handlers on buttons/inputs may trigger `window.location` changes
4. Without waiting, subsequent operations race against page load

## Goals

- Match Playwright's default auto-wait behavior for actions
- Provide `no_wait_after` escape hatch for advanced use cases
- Minimal performance impact when navigation is not triggered
- Clear timeout behavior when navigation stalls

## Non-Goals

- Changing navigation behavior for explicit `page.goto()` (already waits)
- Auto-waiting for non-navigation side effects (AJAX, animations)
- Detecting client-side routing in SPAs (requires different approach)

## Decisions

### Decision: Use Frame Navigation Events for Detection

**What**: Listen for `Page.frameNavigated` CDP events to detect navigation.

**Why**: This is the same mechanism used by Playwright. It fires when:
- Full page navigation occurs
- iframe navigation occurs (we filter to main frame)

**Alternatives considered**:
- URL change polling: Unreliable, misses same-URL navigations
- Load event only: Misses early navigation detection, can't distinguish "no navigation" from "navigation in progress"

### Decision: Wait for Load State by Default

**What**: After detecting navigation, wait for `DocumentLoadState::Load` by default.

**Why**: 
- Matches Playwright's default behavior
- Most actions that trigger navigation expect the new page to be ready
- `DomContentLoaded` is too early for many use cases (images, scripts not loaded)

**Alternatives considered**:
- `NetworkIdle`: Too slow for default, users can explicitly wait if needed
- `DomContentLoaded`: Faster but less reliable for general use

### Decision: No-Op for Actions That Don't Navigate

**What**: If no navigation is detected within a short window (e.g., 50ms after action), return immediately without waiting.

**Why**:
- Most actions don't trigger navigation
- We shouldn't add latency to every action
- Playwright uses similar heuristics

**Implementation approach**:
1. Before action: subscribe to `Page.frameNavigated` events
2. Perform action
3. Wait up to 50ms for navigation event
4. If navigation detected: wait for load state (with full timeout)
5. If no navigation: return immediately

### Decision: Builder Pattern for no_wait_after

**What**: Add `.no_wait_after(true)` to action builders.

**Why**:
- Consistent with existing builder pattern (`.force()`, `.timeout()`, etc.)
- Matches Playwright's API naming
- Optional - defaults to false (auto-wait enabled)

```rust
// Default: waits for navigation
locator.click().await?;

// Opt out: returns immediately after click
locator.click().no_wait_after(true).await?;
```

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Added latency for non-navigating actions | Short detection window (50ms), then return immediately |
| Breaking change for existing code | This is the *correct* behavior; old code was racing |
| Complex state management for navigation detection | Encapsulate in `NavigationWaiter` struct |
| Timeout confusion (action timeout vs navigation timeout) | Document clearly; navigation timeout is separate |

## Migration Plan

1. Implement `NavigationWaiter` infrastructure
2. Add to click action first as proof of concept
3. Roll out to other actions
4. Update documentation
5. No explicit migration needed - behavior change is a bug fix

## Open Questions

- Should `keyboard.press()` also auto-wait? (Playwright does this)
  - **Answer**: Yes, pressing Enter in a form is a common navigation trigger
- Should we add `wait_until` option to customize the load state waited for?
  - **Tentative**: No, use explicit `page.wait_for_load_state()` if needed
