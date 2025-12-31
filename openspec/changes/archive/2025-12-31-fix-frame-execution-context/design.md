# Design: Fix Frame JavaScript execution context targeting

## Context

Each frame (main frame and iframes) can have multiple JavaScript execution contexts:
- **Main world context**: The default context where page scripts run
- **Isolated world contexts**: Separate contexts for extensions, content scripts, or explicitly created isolated worlds

Each context has a unique `ExecutionContextId`. CDP's `Runtime.evaluate` and `Runtime.callFunctionOn` accept an optional `context_id` parameter to target a specific execution context.

Currently, Frame methods pass `context_id: None`, which defaults to the main frame's context. This breaks Frame-specific JavaScript execution.

**CDP Events for context tracking:**
- `Runtime.executionContextCreated` - Emitted when a new context is created (includes frame ID and world name via `auxData`)
- `Runtime.executionContextDestroyed` - Emitted when a context is destroyed

**CDP Methods for isolated worlds:**
- `Page.createIsolatedWorld` - Creates an isolated world for a frame, returns the execution context ID

## Goals / Non-Goals

**Goals:**
- Frame methods (`content()`, `title()`, `aria_snapshot()`) execute JavaScript in the correct frame context
- Support for isolated world contexts (for internal use and future extension support)
- Execution context mapping is maintained automatically as frames load/unload
- Minimal changes to existing code structure

**Non-Goals:**
- Exposing execution context IDs in the public API (keep internal)
- Full extension content script API (just infrastructure)
- Changing how Page-level methods work (they already target main frame correctly)

## Decisions

### Decision: Track multiple contexts per frame

**What:** Store a map of world name to execution context ID per frame, not just a single context ID.

**Why:**
- Each frame can have multiple worlds: the main world plus any isolated worlds
- `Page.createIsolatedWorld` returns a context for a specific frame
- Future extension/content script support requires isolated world tracking

**Data structure:**
```rust
struct FrameData {
    url: String,
    name: String,
    detached: bool,
    // Map of world name -> ExecutionContextId
    // "" (empty string) = main world
    // "viewpoint-isolated" = our isolated world
    // Other names = extension content scripts, etc.
    execution_contexts: HashMap<String, ExecutionContextId>,
}
```

### Decision: Store execution contexts in Frame's internal data

**What:** Add execution context tracking to the existing `FrameData` struct inside `Frame`.

**Why:**
- Keeps context data co-located with other mutable frame state
- Already using `RwLock<FrameData>` for thread-safe updates
- No new synchronization primitives needed

### Decision: Track contexts via event subscription in Page

**What:** When a Page is created, subscribe to `Runtime.executionContextCreated` events and update the corresponding Frame's context map.

**Why:**
- Page already owns the CDP connection and manages frames
- Context events include frame ID and world name in `auxData`, allowing direct mapping

**Implementation approach:**
1. Enable Runtime domain: `Runtime.enable`
2. Subscribe to `Runtime.executionContextCreated` events
3. Extract frame ID from `context.auxData.frameId`
4. Extract world name from `context.auxData.isDefault` (true = main world, use "") and `context.name`
5. Find matching Frame and update its `execution_contexts` map

### Decision: Use main world context by default in Frame methods

**What:** `Frame.content()`, `Frame.title()`, and `Frame.aria_snapshot()` use the main world context (empty string key).

**Why:** 
- Main world is where page scripts and DOM live
- Consistent with Playwright's behavior
- Isolated worlds are opt-in for specific use cases

**Fallback:** If main world context is not yet available, fall back to `context_id: None`.

### Decision: Provide method to get/create isolated world context

**What:** Add `Frame::get_or_create_isolated_world(&self, name: &str) -> Result<ExecutionContextId>` for internal use.

**Why:**
- Needed for safe script injection that won't interfere with page scripts
- `Page.createIsolatedWorld` creates a context on demand
- Cache the context ID to avoid recreating

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Race condition: context created before Frame exists | Context events may arrive before Frame is added to tree. Store pending contexts in a temporary map and reconcile when Frame appears. |
| Context destroyed mid-operation | Operations will fail with CDP error. Return appropriate PageError. Clear context from map on destroy event. |
| Performance overhead from HashMap per frame | Minimal - typically 1-2 contexts per frame, HashMap is overkill but simple. Could use SmallVec if needed. |
| Isolated world context invalidated on navigation | Track via `executionContextDestroyed` and recreate on next use. |

## Migration Plan

1. Update `FrameData` to include `execution_contexts: HashMap<String, ExecutionContextId>`
2. Add getter/setter methods for execution contexts on Frame
3. Enable `Runtime` domain and subscribe to context events in Page initialization
4. Handle `executionContextDestroyed` to remove stale contexts
5. Update Frame evaluate methods to use main world context ID
6. Add `get_or_create_isolated_world` for future isolated world needs
7. Add integration tests verifying Frame-specific JavaScript execution
8. No breaking API changes - existing code continues to work

**Rollback:** Revert to `context_id: None` if issues discovered.

## Open Questions

None - all questions resolved.
