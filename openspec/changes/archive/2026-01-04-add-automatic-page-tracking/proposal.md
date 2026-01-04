# Change: Unified CDP Event-Driven Page Tracking

## Why
When a user Ctrl+clicks a link or a page opens via `target="_blank"`, the browser creates a new tab/page. However, viewpoint-core currently has a hybrid approach that's messy and prone to race conditions:

1. `new_page()` creates, attaches, tracks, and emits events directly
2. Target event listener only handles external pages (with `opener_id`)
3. Multiple deduplication checks scattered across code
4. Race conditions possible in edge cases

## What Changes

### Unified Approach
All page lifecycle management happens through CDP events:

1. **`Target.targetCreated`** - Single entry point for ALL page creation
   - Attach to target, enable domains, create Page instance
   - Track page in context
   - Emit `on_page` event (always, for all pages)

2. **`Target.targetDestroyed`** - Single entry point for ALL page removal
   - Remove page from tracking
   - Clean up resources

### How `new_page()` Works
```rust
pub async fn new_page(&self) -> Result<Page, ContextError> {
    // 1. Set up listener BEFORE creating target
    let page_future = self.wait_for_page();
    
    // 2. Create the target (CDP event listener handles the rest)
    self.connection.send_command("Target.createTarget", ...).await?;
    
    // 3. Wait for the event listener to complete page setup
    let page = page_future.await?;
    
    Ok(page)
}
```

### Event Listener Logic
```rust
fn handle_target_created(target_id: &str, ...) {
    // All pages handled identically:
    // 1. Attach to target
    // 2. Enable domains
    // 3. Apply emulation settings
    // 4. Create Page instance
    // 5. Track in context
    // 6. Emit on_page event
    event_manager.emit_page(page);
}

fn handle_target_destroyed(target_id: &str, ...) {
    // Remove from tracking
    pages.retain(|p| p.target_id != target_id);
}
```

### Benefits
1. **Single code path** - All pages go through the same initialization
2. **No race conditions** - CDP event listener is single source of truth
3. **No deduplication needed** - Only one place creates/tracks pages
4. **Consistent behavior** - `new_page()` and external pages initialized identically
5. **Symmetric design** - Creation and destruction both CDP event-driven
6. **Simpler code** - No pending pages map, no special cases

### Behavior
- `new_page()` returns the Page AND triggers `on_page` handlers
- External pages (popups, Ctrl+click, etc.) trigger `on_page` handlers
- Page close (via `page.close()` or user action) triggers cleanup via `Target.targetDestroyed`

## Why Not Context Events?
CDP does not emit events for browser context lifecycle (`browserContextCreated`, `browserContextDestroyed` don't exist). Contexts are managed purely via commands. Since viewpoint-mcp already creates/closes contexts explicitly, context tracking is correct. Only page tracking needs CDP event listeners.

## Impact
- Affected specs: `context-lifecycle`
- Affected code:
  - `crates/viewpoint-core/src/context/target_events/mod.rs` - rewrite as single source of truth
  - `crates/viewpoint-core/src/context/page_management/mod.rs` - simplify `new_page()` to use `wait_for_page()`
  - `crates/viewpoint-core/src/context/page_factory/mod.rs` - keep utilities, remove direct tracking
