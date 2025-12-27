# Design: Frame Support

## Context

Frames (iframes) are separate browsing contexts within a page. Each frame has its own DOM, JavaScript context, and URL. Playwright provides two ways to work with frames: Frame objects for direct access and FrameLocator for locator-based access.

## Goals

- Support interacting with elements inside iframes
- Handle nested frames (frames within frames)
- Provide both direct Frame access and FrameLocator patterns
- Track frame lifecycle events

## Decisions

### Decision 1: FrameLocator vs Frame

**Choice**: Support both patterns, recommend FrameLocator.

```rust
// FrameLocator pattern (recommended)
page.frame_locator("#my-iframe")
    .get_by_role(Role::Button, "Submit")
    .click()
    .await?;

// Frame pattern (for advanced use)
let frame = page.frame("iframe-name").await?;
frame.locator("button").click().await?;
```

**Rationale**:
- FrameLocator is auto-waiting and more robust
- Frame is useful for frame-level operations (navigation, content)
- Matches Playwright's dual approach

### Decision 2: Frame Identification

**Choice**: Identify frames by name, URL pattern, or selector.

```rust
// By name attribute
page.frame("payment-frame").await?;

// By URL pattern
page.frame(FrameSelector::url("**/payment/**")).await?;

// By selector (using frame_locator)
page.frame_locator("iframe.payment").locator(...);
```

### Decision 3: Nested Frames

**Choice**: FrameLocator chaining for nested frames.

```rust
page.frame_locator("#outer-frame")
    .frame_locator("#inner-frame")
    .locator("button")
    .click()
    .await?;
```

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| List frames | Page.getFrameTree | Page |
| Frame events | Page.frameAttached, frameDetached, frameNavigated | Page |
| Execute in frame | Runtime.evaluate with executionContextId | Runtime |

## Open Questions

None.
