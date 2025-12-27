# Design: Clock Mocking

## Context

Clock mocking allows controlling JavaScript's time-related functions (Date, setTimeout, setInterval, requestAnimationFrame) for deterministic testing.

## Goals

- Control Date.now() and new Date()
- Control setTimeout/setInterval
- Control requestAnimationFrame
- Enable fast-forwarding through time

## Decisions

### Decision 1: Implementation Approach

**Choice**: Use CDP to inject clock mocking library.

**Rationale**:
- Playwright uses this approach
- Works across all browsers
- Complete control over time functions

### Decision 2: Clock Scope

**Choice**: Clock is per-context.

```rust
let clock = context.clock();
clock.install().await?;
clock.set_fixed_time("2024-01-01T00:00:00Z").await?;
```

### Decision 3: Time Formats

**Choice**: Support multiple time formats.

```rust
// ISO string
clock.set_fixed_time("2024-01-01T00:00:00Z").await?;

// Unix timestamp (ms)
clock.set_fixed_time(1704067200000).await?;

// DateTime type
clock.set_fixed_time(datetime).await?;
```

## CDP Commands Required

Uses Runtime.evaluate to inject and control clock mocking.

## Open Questions

None.
