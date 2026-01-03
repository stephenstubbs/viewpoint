# Design: Context and Page-Scoped Element References

## Context

MCP servers present accessibility snapshots to AI agents who interact with elements using refs. The current implementation uses `e{backendNodeId}` format which:

1. Doesn't indicate which context or page a ref belongs to
2. Leaks CDP implementation details through the API
3. Can cause confusion in multi-context/multi-tab scenarios

## How Playwright MCP Handles This

Playwright MCP uses simple incrementing refs (`e1`, `e2`, etc.) that are:
- Generated fresh on each snapshot
- Stored on DOM elements via a symbol property
- Resolved via a custom `aria-ref` selector engine
- Not designed to survive DOM mutations

When a ref becomes stale, they return: "Ref not found in the current page snapshot. Try capturing new snapshot."

## Our Approach

Match Playwright MCP's simplicity for refs, but with:
1. **Context, page, and frame scoping** for multi-context/multi-tab/multi-frame disambiguation
2. **Opaque ref format** that doesn't expose `backendNodeId`
3. **Clear validation** with helpful error messages

### Ref Format

```
c{contextIndex}p{pageIndex}f{frameIndex}e{counter}
```

Examples:
- `c0p0f0e1` - Context 0, Page 0, Frame 0 (main), element 1
- `c0p0f0e2` - Context 0, Page 0, Frame 0 (main), element 2  
- `c0p0f1e1` - Context 0, Page 0, Frame 1 (child iframe), element 1
- `c0p1f0e1` - Context 0, Page 1, Frame 0 (main), element 1
- `c1p0f0e1` - Context 1, Page 0, Frame 0 (main), element 1

### Why Context, Page, and Frame Prefix?

Unlike Playwright MCP (which operates on a single page context at a time), our MCP server may manage multiple browser contexts, each with multiple tabs, each with multiple frames:

- **Context prefix (`c`)**: Prevents cross-context ref misuse. Different contexts are isolated (separate cookies, storage, sessions), so refs should clearly belong to a specific context.
- **Page prefix (`p`)**: Prevents cross-tab ref misuse within the same context. Multi-tab automation is common, and refs should identify which tab they came from.
- **Frame prefix (`f`)**: Prevents cross-frame ref collisions within a page. When capturing `aria_snapshot_with_frames()`, each frame's elements get unique refs. Frame 0 is always the main frame, with child iframes numbered 1, 2, etc. in capture order.

### Generation

During snapshot capture:
1. Clear the page's ref_map (invalidates all previous refs)
2. Reset counter to 0
3. Get context index from the page's parent context
4. Get page index from the page
5. Get frame index (0 for main frame, 1+ for child frames in capture order)
6. For each element in the accessibility tree:
   - Assign ref `c{contextIndex}p{pageIndex}f{frameIndex}e{++counter}`
   - Store mapping: ref → backendNodeId in the page's ref_map
7. Return snapshot with refs

### Resolution

```rust
fn resolve_ref(ref_str: &str, page: &Page) -> Result<ElementHandle> {
    let parsed = parse_ref(ref_str)?;
    
    if parsed.context_index != page.context_index() {
        return Err("Context index mismatch: ref is for context X, but this page is in context Y");
    }
    
    if parsed.page_index != page.page_index() {
        return Err("Page index mismatch: ref is for page X, but this is page Y");
    }
    
    // Frame index is part of the ref key - no separate validation needed
    // The ref_map stores all refs from all frames captured in the snapshot
    let backend_node_id = page.ref_map().get(&ref_str)
        .ok_or("Ref not found. Capture a new snapshot.")?;
    
    resolve_by_backend_node_id(backend_node_id)
}
```

### Ref Map Lifecycle

- Created fresh on each `aria_snapshot()` call
- Stored on the Page object
- Cleared/replaced when a new snapshot is taken
- Not persisted across page navigations

---

## Implementation Architecture

### Snapshot Capture Flow

```
JavaScript Traversal          CDP Calls              Rust Processing
─────────────────────────────────────────────────────────────────────
Runtime.evaluate ────────────►│                     │
  (JS builds tree +           │                     │
   collects elements)         │                     │
  (~50ms)                     │                     │
                              │                     │
◄─────────────────────────────│ {snapshot, elements}│
                              │                     │
                              │ DOM.describeNode x N│──────────────►
                              │ (parallel, ~100ms)  │
                              │                     │
                              │◄────────────────────│ backendNodeIds
                              │                     │
                              │                     │ Assign scoped refs
                              │                     │ Store in ref_map
                              │                     │ (~5ms)
─────────────────────────────────────────────────────────────────────
Total: ~155ms for 100 elements
```

### Why Not CDP Accessibility Domain?

We evaluated using `Accessibility.getFullAXTree` for a pure Rust implementation:

**Advantages (theoretical):**
- Single CDP call returns full tree
- `backendDOMNodeId` included in nodes
- No JavaScript execution

**Problems (actual):**
- Many AX nodes lack `backendDOMNodeId` (especially text nodes, some containers)
- Would require additional CDP calls to resolve missing IDs
- Negates the performance benefit
- Less reliable than JS traversal

**Decision**: Keep the JS traversal + parallel resolution approach. It's reliable, well-tested, and performant enough.

### Key Components

1. **`BrowserContext.context_index`**: Unique index assigned when context is created (global counter)
2. **`Page.page_index`**: Unique index within the context (per-context counter)
3. **`Frame.frame_index`**: Index within the snapshot (0 = main, 1+ = child frames in capture order)
4. **`Page.ref_map`**: HashMap<String, BackendNodeId> storing ref → backendNodeId mappings
5. **`parse_ref()`**: Parses `c{ctx}p{page}f{frame}e{counter}` format
6. **`format_ref()`**: Generates ref strings

### Validation Flow

```rust
// When resolving c0p1f0e5 on page with context_index=0, page_index=0:
parse_ref("c0p1f0e5")  // → ParsedRef { context_index: 0, page_index: 1, frame_index: 0, element_counter: 5 }

// Validation fails:
// "Page index mismatch: ref 'c0p1f0e5' is for page 1, but this is page 0"
```

---

## Trade-offs

| Aspect | Our Approach | Alternative (Playwright-style) |
|--------|--------------|--------------------------------|
| Multi-context support | ✅ Built-in | ❌ Not supported |
| Multi-tab support | ✅ Built-in | ❌ Not supported |
| Multi-frame support | ✅ Built-in | ❌ Not supported |
| Ref format | Longer (`c0p0f0e1`) | Shorter (`e1`) |
| Validation | Clear error messages | Silent failures possible |

---

## Note: Impact on Testing Framework

This ref system is primarily designed for MCP/AI agent use cases, not for writing tests.

**Tests should use locators, not refs:**

```rust
// Preferred for tests - locators re-query on each action
page.get_by_role("button", "Submit").click().await?;
page.locator("css=.my-class").fill("text").await?;

// Not recommended for tests - refs are snapshot-specific
let snapshot = page.aria_snapshot().await?;
page.locator_from_ref("c0p0f0e1").click().await?; // Fragile if DOM changes
```

Locators capture *how to find* an element and re-query the DOM on each action, making them resilient to DOM mutations. Refs point to a specific element from a specific snapshot and become stale when the DOM changes.

The ref system exists so AI agents can reference elements they saw in a snapshot without needing to construct locators. For test code, always prefer locators.
