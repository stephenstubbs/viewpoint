# Change: Add Node References to ARIA Snapshots

## Why

The current ARIA snapshot implementation captures accessibility tree structure but lacks unique identifiers for each node. This prevents MCP servers and automation tools from interacting with elements discovered in the snapshot (e.g., clicking a button found in the accessibility tree).

## What Changes

- Add a `ref` field to `AriaSnapshot` containing a unique node reference for each element
- Capture `backendNodeId` from CDP during snapshot traversal for each DOM element
- Design the ref system to be protocol-agnostic, enabling future WebDriver BiDi `SharedReference` support
- Provide a method to resolve refs back to elements for interaction

## Impact

- Affected specs: `advanced-locators` (Aria Snapshot requirement)
- Affected code:
  - `crates/viewpoint-core/src/page/locator/aria/mod.rs` - Add `ref` field to `AriaSnapshot`
  - `crates/viewpoint-core/src/page/locator/aria_js/mod.rs` - Capture `backendNodeId` in JS snapshot code
  - `crates/viewpoint-core/src/page/locator/mod.rs` - Add ref resolution methods
