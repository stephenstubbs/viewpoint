# Change: Add Frame Boundary Support to Aria Snapshots

## Why

MCP (Model Context Protocol) servers building on viewpoint need to capture accessibility snapshots that include frame boundaries. The current `AriaSnapshot` implementation only captures the accessibility tree within a single frame context and does not:
1. Mark iframe elements as frame boundaries
2. Track iframe references for later frame content retrieval
3. Provide mechanisms to stitch multi-frame accessibility trees together

This prevents MCP implementations from providing complete page accessibility information to LLMs, which is critical for AI-driven browser automation.

## What Changes

- **Extend AriaSnapshot struct** with frame boundary tracking fields (`is_frame`, `frame_url`, `frame_name`, `iframe_refs`)
- **Update aria snapshot JavaScript** to detect and mark iframe elements with the `iframe` role
- **Add Page-level multi-frame snapshot method** that captures all frames and stitches results together
- **Add Frame.aria_snapshot()** method to capture accessibility tree for individual frames
- **Support cross-origin frame handling** via CDP frame targeting (with documented limitations)

## Impact

- Affected specs: `advanced-locators` (Aria Snapshot requirement)
- Affected code:
  - `crates/viewpoint-core/src/page/locator/aria/mod.rs` - AriaSnapshot struct
  - `crates/viewpoint-core/src/page/locator/aria_js/mod.rs` - JavaScript capture code
  - `crates/viewpoint-core/src/page/mod.rs` - Page-level snapshot method
  - `crates/viewpoint-core/src/page/frame/mod.rs` - Frame snapshot method
- New integration tests required for frame boundary scenarios
