# Change: Fix Storage State Stale Session Handling

## Why

When calling `context.storage_state()`, the method fails with "CDP protocol error -32001: Session with given id not found" if any page in the context's internal `pages` list has a stale session ID.

The root cause is that `BrowserContext.pages` stores `PageInfo` with session IDs when pages are created, but this list is not updated when pages are closed. When `storage_state()` iterates over this list to collect localStorage from each page, it attempts CDP commands using stale session IDs, causing failures.

## What Changes

**Remove pages from tracking list when closed**: When a page is closed, remove its entry from `BrowserContext.pages`. This prevents stale sessions from accumulating and ensures `storage_state()` only operates on valid pages.

## Impact

- Affected specs: `storage-state`
- Affected code:
  - `crates/viewpoint-core/src/context/mod.rs` - Add method to remove page from tracking
  - `crates/viewpoint-core/src/page/mod.rs` - Call context's untrack method when page closes
  - May need to pass context reference or callback to Page

## Root Cause Analysis

Pages are tracked in `BrowserContext.pages` via `track_page()` in `page_factory/mod.rs`:

```rust
pub(crate) async fn track_page(
    pages: &RwLock<Vec<PageInfo>>,
    target_id: String,
    session_id: String,
) {
    let mut pages_guard = pages.write().await;
    pages_guard.push(PageInfo { target_id, session_id });
}
```

But when `Page::close()` is called, there's no corresponding removal from this list. The fix adds an `untrack_page()` function and ensures it's called when pages close.
