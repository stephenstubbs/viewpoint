# Change: Fix Frame aria_snapshot to include element refs

## Why

The `Frame.aria_snapshot()` method uses `aria_snapshot_js()` which does NOT include element refs (`node_ref`), while `Page.aria_snapshot()` uses `aria_snapshot_with_refs_js()` which DOES include refs. This inconsistency causes `Page.aria_snapshot_with_frames()` to return snapshots without refs, because it internally calls `main_frame().aria_snapshot()`.

This is a bug that breaks downstream consumers (like viewpoint-mcp) that rely on refs being present in multi-frame snapshots for element interaction.

**Error observed in viewpoint-mcp:**
```
# Page.aria_snapshot() works - includes refs:
- button "Submit" [ref=e12345]

# Page.aria_snapshot_with_frames() broken - no refs:
- button "Submit"
```

## What Changes

- **Update `Frame.aria_snapshot()`** to use `aria_snapshot_with_refs_js()` and resolve `backendNodeId` for each element, matching the behavior of `Page.aria_snapshot()`
- **Extract shared ref resolution logic** into a reusable helper to avoid code duplication between Page and Frame implementations

## Impact

- Affected specs: `advanced-locators` (Multi-Frame Aria Snapshot requirement)
- Affected code:
  - `page/frame/mod.rs` - Update `aria_snapshot()` implementation
  - `page/aria_snapshot/mod.rs` - Potentially extract shared helper functions
