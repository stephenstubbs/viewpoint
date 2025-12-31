# Design: Fix Frame aria_snapshot to include element refs

## Context

The aria snapshot system has two JavaScript functions:
1. `aria_snapshot_js()` - Captures accessibility tree WITHOUT element refs
2. `aria_snapshot_with_refs_js()` - Captures accessibility tree WITH element indices for ref resolution

Currently:
- `Page.aria_snapshot()` uses `aria_snapshot_with_refs_js()` ✓
- `Frame.aria_snapshot()` uses `aria_snapshot_js()` ✗
- `Page.aria_snapshot_with_frames()` calls `main_frame().aria_snapshot()` → no refs ✗

## Goals / Non-Goals

**Goals:**
- Make `Frame.aria_snapshot()` include element refs like `Page.aria_snapshot()` does
- Ensure `Page.aria_snapshot_with_frames()` returns snapshots with refs on all elements
- Avoid code duplication between Page and Frame implementations

**Non-Goals:**
- Changing the ref format (stays as `e{backendNodeId}`)
- Modifying the JavaScript snapshot functions
- Adding new public API methods

## Decisions

### Decision: Use the same ref resolution pattern in Frame

**What:** Update `Frame.aria_snapshot()` to:
1. Use `aria_snapshot_with_refs_js()` instead of `aria_snapshot_js()`
2. Execute with `return_by_value: false` to get RemoteObjects for element resolution
3. Iterate over the elements array and resolve each to its `backendNodeId` via `DOM.describeNode`
4. Apply refs to the snapshot tree using the same `apply_refs_to_snapshot()` function

**Why:**
- This matches exactly what `Page.capture_snapshot_with_refs()` does
- Ensures consistency between Frame and Page snapshots
- The Frame already has access to the CDP connection and session_id needed

**Trade-off:** The Frame implementation becomes more complex, but the alternative (no refs) makes the feature unusable for LLM-based automation.

### Decision: Extract shared helper methods

**What:** Consider extracting these methods to a shared location:
- `get_property_value()` - Get a property as a value from RemoteObject
- `get_property_object()` - Get a property as a RemoteObject
- `get_array_element()` - Get array element by index
- `describe_node()` - Get backendNodeId for an element
- `release_object()` - Release a RemoteObject
- `apply_refs_to_snapshot()` - Apply refs to snapshot tree

**Why:**
- Avoids duplicating ~100 lines of code between Page and Frame
- These are general CDP utilities that could be useful elsewhere

**Alternative:** Copy the implementation to Frame. Simpler but creates maintenance burden.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Frame execution context differs from Page | Frame already has session_id; CDP commands should work the same |
| Performance overhead from ref resolution | Same overhead as Page.aria_snapshot(); acceptable for the functionality gained |
| Code duplication if not extracted | Can extract in follow-up; initial fix can copy the pattern |

## Migration Plan

1. Update `Frame.aria_snapshot()` to use the ref-resolution pattern
2. Verify `Page.aria_snapshot_with_frames()` now includes refs
3. Run integration tests to confirm behavior
4. Optionally extract shared helpers in a follow-up refactor

**Rollback:** Revert to using `aria_snapshot_js()` if issues are discovered (would restore the broken behavior).

## Open Questions

None - the fix is straightforward: use the same pattern that works for Page.
