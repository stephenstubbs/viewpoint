# Proposal: Track Page Activation Events

## Problem

When users interact with browser tabs directly (clicking on tabs, Ctrl+Tab, etc.), viewpoint-core has no way to notify consumers about which page became active. This means:

1. MCP tools like `browser_close` may operate on stale page references
2. The `active_page_index` in viewpoint-mcp gets out of sync with the actual browser state
3. Consumers cannot react to user-initiated tab switches

Currently, viewpoint-core tracks page creation (`Target.targetCreated`) and destruction (`Target.targetDestroyed`), but not activation changes.

## Solution

Add event-driven page activation tracking by:

1. **Add `Target.targetInfoChanged` CDP event type** to viewpoint-cdp
2. **Listen for `Target.targetInfoChanged`** in the target events listener
3. **Add `on_page_activated` event** to `ContextEventManager` (following the existing `on_page` pattern)
4. **Emit activation events** when a page becomes the foreground tab

The CDP `Target.targetInfoChanged` event is emitted when target info changes, including when a tab becomes visible/focused. We'll filter for relevant activation changes.

## Scope

- Add `TargetInfoChangedEvent` type to viewpoint-cdp
- Extend `ContextEventManager` with `on_page_activated` / `off_page_activated`
- Listen for `Target.targetInfoChanged` in `target_events` module
- Emit page activation events to registered handlers

## Out of Scope

- Tracking focus within a page (element focus)
- Window-level focus events (browser window activation)

## Acceptance Criteria

1. `context.on_page_activated(|page| ...)` callback fires when user clicks a tab
2. The activated page is passed to the handler
3. Handler registration returns a `HandlerId` for removal
4. Events only fire for page-type targets in the context

## Risk Assessment

- **Low risk**: Extends existing event infrastructure
- **Follows established patterns**: Same design as `on_page` / `on_close`
- **No breaking changes**: Additive API only
