# Design: Frame Boundary Support for Aria Snapshots

## Context

Accessibility snapshots are essential for MCP servers that provide browser automation capabilities to LLMs. The current implementation captures accessibility trees within a single frame but does not handle multi-frame pages. This design addresses how to extend the system to support frame boundaries while maintaining compatibility with cross-origin security restrictions.

### Stakeholders
- MCP server developers building on viewpoint
- LLM-based browser automation tools
- Accessibility testing frameworks

### Constraints
- Browser Same-Origin Policy prevents JavaScript access to cross-origin iframe content
- Out-of-process iframes (OOPIF) require separate CDP sessions
- Must maintain backward compatibility with existing AriaSnapshot API

## Goals / Non-Goals

### Goals
- Mark iframe elements as frame boundaries in snapshots
- Track iframe references for MCP to request frame content separately
- Provide Page-level method to capture complete multi-frame accessibility trees
- Support same-origin iframe content traversal
- Document cross-origin limitations clearly

### Non-Goals
- Bypass browser security restrictions for cross-origin content
- Automatic cross-origin frame content injection (security risk)
- CDP Accessibility domain integration (future enhancement)

## Decisions

### Decision 1: Server-Side Frame Stitching (Playwright's Approach)

**What**: Iframes are marked as boundaries with refs in the parent snapshot. Frame content is captured separately per-frame and stitched together server-side.

**Why**: This approach:
- Works with out-of-process iframes via CDP frame targeting
- Maintains security by not attempting JavaScript cross-origin access
- Allows MCP servers to request only the frames they need
- Matches Playwright's proven implementation

**Alternatives considered**:
- JavaScript-level traversal: Only works for same-origin, fails silently for cross-origin
- CDP Accessibility.getFullAXTree: Higher complexity, different data format, limited cross-origin support

### Decision 2: AriaSnapshot Struct Extensions

**What**: Add optional fields to AriaSnapshot:
```rust
pub struct AriaSnapshot {
    // ... existing fields ...
    pub is_frame: Option<bool>,       // True for iframe nodes
    pub frame_url: Option<String>,    // iframe src URL
    pub frame_name: Option<String>,   // iframe name attribute
    pub iframe_refs: Vec<String>,     // Collected iframe refs for enumeration
}
```

**Why**: 
- Backward compatible (all new fields are optional/defaulted)
- Enables MCP to identify frame boundaries and request content
- Aligns with Playwright's AriaSnapshot type structure

### Decision 3: Two-Tier API

**What**: Provide both low-level and high-level APIs:
1. `locator.aria_snapshot()` - Single frame, marks iframes as boundaries
2. `page.aria_snapshot_with_frames()` - All frames stitched together

**Why**:
- Low-level API for MCP servers that want control over frame fetching
- High-level API for simple use cases that need complete tree
- MCP can choose based on performance/completeness tradeoffs

### Decision 4: JavaScript iframe Detection

**What**: Update `getAriaSnapshot` JS function to detect `IFRAME` elements and return them with role `iframe`:

```javascript
if (el.tagName === 'IFRAME') {
  return { 
    role: 'iframe',
    name: el.name || el.title || '',
    isFrame: true,
    frameUrl: el.src || null,
    frameName: el.name || null,
  };
}
```

**Why**:
- Does not attempt to access `contentDocument` (would fail for cross-origin)
- Provides metadata MCP needs to identify and request frame content
- Simple, reliable detection

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| Cross-origin frames show as empty boundaries | MCP gets incomplete tree | Document limitation; MCP can detect via `is_frame` + empty children |
| OOPIF requires CDP session per frame | Performance overhead | Lazy frame fetching; MCP controls which frames to expand |
| Same-origin detection complexity | Edge cases with sandbox, CSP | Use try/catch in JS; mark failures as cross-origin |
| Breaking change if fields are required | API compatibility | All new fields optional with defaults |

## Migration Plan

1. **Phase 1**: Add new optional fields to AriaSnapshot struct
2. **Phase 2**: Update JavaScript to detect and mark iframes
3. **Phase 3**: Add `Frame.aria_snapshot()` method
4. **Phase 4**: Add `Page.aria_snapshot_with_frames()` method
5. **Phase 5**: Add integration tests with iframe scenarios

No breaking changes - existing code continues to work unchanged.

## Open Questions (Resolved)

### 1. Should `iframe_refs` be populated at the root level only, or at each level?

**Decision: Root level only**

**Rationale**:
- MCP needs to enumerate all frames to decide which to fetch - a flat list at the root is sufficient
- Each iframe node already has `is_frame: true` inline for identification during tree traversal
- Collecting refs at root avoids redundant data in nested snapshots
- Simpler implementation and smaller payload size

**Implementation**: The `iframe_refs` field will be collected during traversal and stored only at the root AriaSnapshot. Individual iframe nodes are marked with `is_frame: true` inline.

### 2. Should we provide a callback-based API for streaming frame snapshots?

**Decision: No, not in initial implementation**

**Rationale**:
- Adds significant API complexity (async iterators, cancellation, backpressure)
- Current MCP implementations work with complete snapshots
- Most pages have few frames; memory is not a concern
- Can be added later as `aria_snapshot_with_frames_streaming()` without breaking changes

**Future consideration**: If MCP use cases emerge requiring streaming (e.g., pages with 50+ frames, real-time progressive rendering), add a separate streaming API.

### 3. How to handle `<frame>` and `<frameset>` (legacy)?

**Decision: Treat `<frame>` identically to `<iframe>`**

**Rationale**:
- `<frame>` and `<frameset>` are deprecated HTML4 elements but still work in all browsers
- They function identically to iframes (separate browsing context, same security model)
- Some legacy enterprise applications still use them
- Consistent handling reduces edge cases

**Implementation**: The JavaScript detection code will check for both `IFRAME` and `FRAME` tag names. `<frameset>` itself doesn't need special handling (it's just a container like `<div>`).
