## Context

The ARIA snapshot feature captures an accessibility tree representation of the DOM. Currently, it includes roles, names, states, and frame boundary markers, but lacks element identifiers needed for subsequent interaction with discovered elements.

MCP (Model Context Protocol) servers and similar automation tools need to:
1. Present the accessibility tree to users/AI
2. Allow interaction with any element in that tree (click, type, etc.)

Without node references, users must re-query elements by role/name, which is fragile when multiple elements share the same accessible properties.

### Protocol Considerations

**CDP (Chrome DevTools Protocol):**
- `backendNodeId`: Stable identifier that persists across DOM operations. Works without DOM.enable.
- `nodeId`: Session-specific, requires DOM.enable, invalidated on DOM changes.
- `objectId`: Runtime reference, requires the element to be resolved to a RemoteObject first.

**WebDriver BiDi (future):**
- `SharedReference` with `sharedId`: Protocol's native way to reference nodes across realms.
- Maps conceptually to CDP's `backendNodeId`.

**Decision:** Use `backendNodeId` as the underlying identifier for CDP, exposed as a generic `ref` string in the API. This provides:
- Stability across DOM mutations
- No need for DOM.enable overhead
- Clean mapping to WebDriver BiDi `sharedId` in future

## Goals / Non-Goals

**Goals:**
- Every node in the snapshot gets a unique `ref` identifier
- Refs can be used to interact with elements (click, type, etc.)
- Design is protocol-agnostic (works with CDP now, BiDi later)
- Dynamically created elements get refs when snapshot is captured

**Non-Goals:**
- Real-time DOM observation (refs are snapshot-time, not live)
- Persisting refs across page navigations
- Custom ref format requirements (implementation detail)

## Decisions

### Decision 1: Ref Format
- **What:** Use `e{backendNodeId}` format (e.g., `e12345`)
- **Why:** 
  - Short and readable
  - `e` prefix distinguishes from frame refs (`frame-0`)
  - The numeric part directly maps to CDP `backendNodeId`
- **Alternatives considered:**
  - UUID: More opaque, harder to debug
  - Raw `backendNodeId`: Less clear what type of ref it is
  - XPath-based: Too verbose, fragile

### Decision 2: Capture Method
- **What:** Use CDP `DOM.describeNode` or inject refs during JS traversal
- **Why:** `backendNodeId` is available in CDP responses and stable
- **Approach:** During JavaScript snapshot traversal, use `element.__backendNodeId` where available, or call back to Rust for resolution via CDP

### Decision 3: Resolution API
- **What:** Add `Page::element_from_ref(ref)` and `Locator::from_ref(ref)` methods
- **Why:** Clean API for MCP servers to interact with snapshot elements
- **Alternatives considered:**
  - Returning ElementHandle directly in snapshot: Lifecycle complexity
  - Requiring re-query: Defeats the purpose of refs

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| `backendNodeId` may become invalid if element is removed | Document that refs are snapshot-time; re-snapshot if DOM changes |
| Performance overhead from capturing refs | Only adds one property per node; negligible |
| CDP-specific implementation detail leaking | Abstract behind generic `ref` string; documented as opaque |

## Migration Plan

- Non-breaking addition: `ref` field is optional in `AriaSnapshot`
- Existing code continues to work without changes
- No rollback concerns as this is additive

## Open Questions

None - the approach is straightforward and follows established patterns from WebDriver BiDi spec.
