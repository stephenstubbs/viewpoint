# Proposal: Fix pages() Return Type

## Problem

`context.pages()` doesn't conform to the context-lifecycle spec.

**Spec says (context-lifecycle/spec.md, "List Context Pages"):**
> - **THEN** a list of all **Page instances** in the context is returned

**Implementation returns:**
```rust
pub async fn pages(&self) -> Result<Vec<PageInfo>, ContextError>
```

`PageInfo` only contains `target_id` and `session_id` - not usable `Page` objects with methods like `url()`, `click()`, `type()`, etc.

## Solution

1. Store actual `Page` objects internally instead of `PageInfo`
2. Return `Vec<Page>` from `pages()` to conform to spec

## Scope

- Change internal storage from `Vec<PageInfo>` to `Vec<Page>`
- Update `pages()` return type
- Update target event handlers to store `Page` objects

## Acceptance Criteria

1. `context.pages()` returns `Vec<Page>`
2. Externally-opened pages (popups, target="_blank", Ctrl+Click) appear in `pages()`
3. Returned `Page` objects are fully functional
4. Existing tests pass

## Risk Assessment

- **Breaking change**: `pages()` return type changes
- **Low risk**: Aligns implementation with existing spec
- **Memory**: `Page` objects are larger than `PageInfo`, but negligible for typical use
