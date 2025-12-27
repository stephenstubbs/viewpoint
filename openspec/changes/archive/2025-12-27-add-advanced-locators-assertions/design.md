# Design: Advanced Locators & Assertions

## Context

This proposal completes Playwright feature parity by adding advanced locator composition, additional locator methods, and comprehensive assertions.

## Goals

- Enable complex element selection with and/or composition
- Provide all Playwright locator methods
- Enable accessibility testing with aria snapshots
- Complete the assertion library

## Decisions

### Decision 1: Locator Composition

**Choice**: Use method chaining for composition.

```rust
// Match button with specific text
let locator = page.get_by_role(Role::Button)
    .and(page.get_by_text("Submit"));

// Match either selector
let locator = page.locator(".primary")
    .or(page.locator(".secondary"));
```

### Decision 2: Aria Snapshot Format

**Choice**: Use YAML-like format matching Playwright.

```rust
expect(page.locator("nav")).to_match_aria_snapshot(r#"
  - navigation:
    - link "Home"
    - link "About"
    - link "Contact"
"#).await?;
```

### Decision 3: Count vs All

**Choice**: `count()` returns number, `all()` returns Vec of locators.

```rust
let count = page.locator("li").count().await?;

for item in page.locator("li").all().await? {
    // Each item is a Locator for single element
}
```

## Implementation Notes

- Composition creates new locator with combined selectors
- Aria snapshots use CDP Accessibility domain
- All methods maintain lazy evaluation

## Open Questions

None.
