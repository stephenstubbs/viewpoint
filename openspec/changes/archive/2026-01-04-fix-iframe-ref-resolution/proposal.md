# Change: Fix Iframe Element Ref Resolution

## Why

When `page.aria_snapshot_with_frames()` captures elements inside iframes, the element refs are visible in the snapshot but cannot be resolved via `page.locator_from_ref()`. This breaks MCP server integrations that need to interact with iframe elements.

The root cause is that `Frame.aria_snapshot_with_options()` captures refs and applies them to the snapshot tree, but discards the ref-to-backendNodeId mappings instead of returning them to the Page for storage in its ref_map.

Evidence from `frame/aria.rs:175-178`:
```rust
// Note: Frame doesn't have access to Page's ref_map, so we discard
// the returned mappings. Refs captured via Frame are visible in the
// snapshot but not resolvable via page.locator_from_ref().
let _ = apply_refs_to_snapshot(...);
```

## What Changes

- Modify `Frame.aria_snapshot_with_options()` to return ref mappings alongside the snapshot
- Update `Page.aria_snapshot_with_frames()` to collect child frame ref mappings and store them in Page's ref_map
- Add integration tests for clicking/typing in iframe elements after `aria_snapshot_with_frames()`

## Impact

- Affected specs: `frames`
- Affected code: 
  - `crates/viewpoint-core/src/page/frame/aria.rs`
  - `crates/viewpoint-core/src/page/aria_snapshot/mod.rs`
- API change: `Frame.aria_snapshot_with_options()` return type changes (internal API, not public breaking change)
