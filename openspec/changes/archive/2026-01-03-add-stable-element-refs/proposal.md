# Change: Add Context and Page-Scoped Element References

## Why

Current element references (`e{backendNodeId}`) have an ambiguity problem: refs don't indicate which context or page/tab they belong to. In multi-context or multi-tab scenarios, `e12345` could refer to different elements across different contexts and tabs, leading to incorrect interactions.

Additionally, tying refs to CDP's `backendNodeId` is an implementation detail that leaks through the API.

## What Changes

**BREAKING**: Refs now use a scoped format with context, page, and frame prefixes.

### 1. Scoped Incrementing Refs

Format: `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}` (e.g., `c0p0f0e1`, `c0p0f0e2`, `c0p0f1e1`)

- `c{contextIndex}` - Which browser context this ref belongs to
- `p{pageIndex}` - Which page/tab within the context this ref belongs to
- `f{frameIndex}` - Which frame within the page (0 = main frame, 1+ = child frames)
- `e{counter}` - Simple incrementing counter per snapshot
- Refs are generated fresh on each snapshot capture
- No persistence across snapshots (take a new snapshot to get fresh refs)

### 2. Ref Resolution

When resolving a ref:
1. Validate context index matches the target context
2. Validate page index matches the target page
3. Look up the element from the snapshot's ref map
4. If not found, return error suggesting to capture a new snapshot

No fallback strategies - if the ref is stale, the user should take a new snapshot.

### 3. Stale Ref Handling

If a ref cannot be resolved (element removed, page changed, etc.):
- Return a clear error: "Ref not found. Capture a new snapshot."
- Don't try to guess or use fallbacks

This matches Playwright MCP's approach and keeps the implementation simple.

### 4. Implementation Approach

The implementation uses a hybrid JS/Rust approach:

1. **JavaScript traversal**: Traverse the DOM and build the accessibility snapshot structure
2. **Element collection**: Collect element references in a JavaScript array during traversal
3. **Parallel resolution**: Use parallel CDP `DOM.describeNode` calls to resolve `backendNodeId` for each element
4. **Ref assignment**: Assign scoped refs (`c{ctx}p{page}f{frame}e{counter}`) in Rust and store the mapping

This approach was chosen because:
- CDP's `Accessibility.getFullAXTree` doesn't reliably provide `backendDOMNodeId` for all nodes
- The JS traversal is fast (~50ms) and provides consistent results
- Parallel `DOM.describeNode` calls resolve all elements efficiently
- The ref format is decoupled from `backendNodeId` - refs are opaque identifiers

### 5. Why Not Pure CDP Accessibility Domain?

We evaluated using `Accessibility.getFullAXTree` for a pure Rust implementation, but found:
- Many accessibility nodes lack `backendDOMNodeId`
- This would require additional CDP calls to resolve missing IDs
- The performance benefit would be negated
- The current JS+parallel resolution approach is reliable and performant

## Impact

- Affected specs: `advanced-locators`
- Affected code: `crates/viewpoint-core/src/page/ref_resolution/`, `crates/viewpoint-core/src/context/`, `crates/viewpoint-core/src/page/aria_snapshot/`
- **BREAKING**: Ref format changes from `e{backendNodeId}` to `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}`
- Legacy `e{backendNodeId}` format is no longer supported
