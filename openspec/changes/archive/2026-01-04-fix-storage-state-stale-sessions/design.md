# Design: Fix Storage State Stale Session Handling

## Context

The `storage_state()` method collects cookies (browser-level) and localStorage (page-level) from a context. The localStorage collection requires iterating over pages and executing JavaScript via CDP using each page's session ID. If a page has been closed, its session ID becomes stale and CDP commands fail.

The `BrowserContext` tracks pages in an `Arc<RwLock<Vec<PageInfo>>>` but never removes entries when pages close, leading to stale session accumulation.

## Goals

- Prevent stale sessions from accumulating in the pages list
- `storage_state()` should only operate on valid, open pages
- Clean architectural solution that prevents the issue at the source

## Non-Goals

- Backward compatibility with code that relies on closed pages being in the list (no such code exists)

## Decision: Remove Pages from Tracking on Close

### Current Architecture

```
BrowserContext
  └── pages: Arc<RwLock<Vec<PageInfo>>>  ← shared with StorageStateBuilder
  
Page
  └── No reference back to context's pages list
```

When `Page::close()` is called, the page detaches from CDP but the `PageInfo` remains in the context's list.

### New Architecture

```
BrowserContext
  └── pages: Arc<RwLock<Vec<PageInfo>>>
  
Page
  └── pages: Arc<RwLock<Vec<PageInfo>>>  ← Clone of context's Arc
  └── target_id: String                   ← Used to identify which entry to remove
```

When `Page::close()` is called:
1. Page detaches from CDP (existing behavior)
2. Page removes its entry from the shared `pages` list using its `target_id`

### Implementation Approach

**Option A: Pass pages Arc to Page during construction**
- Page already receives `Arc<CdpConnection>` during construction
- Add `pages: Arc<RwLock<Vec<PageInfo>>>` parameter
- Page stores it and uses it in `close()`
- Pro: Simple, no new traits or callbacks
- Con: Slightly larger Page struct

**Option B: Callback/closure approach**
- Pass a closure to Page that removes from list
- Pro: More decoupled
- Con: More complex, lifetime issues with closures

**Chosen: Option A** - Pass the pages Arc directly. It's the simplest approach and matches how other shared state (like connection) is already passed.

### Code Changes

1. **Page struct** - Add `context_pages: Option<Arc<RwLock<Vec<PageInfo>>>>` field

2. **Page construction** - Modify `Page::new_with_indices()` and related constructors to accept and store the pages Arc

3. **Page::close()** - Before returning, remove self from pages list:
   ```rust
   if let Some(pages) = &self.context_pages {
       let mut pages_guard = pages.write().await;
       pages_guard.retain(|p| p.target_id != self.target_id);
   }
   ```

4. **page_factory** - Pass the pages Arc when creating Page instances

## Testing

1. Integration test: Create context, create page, close page, verify pages list is empty
2. Integration test: Create context, create page, close page, call storage_state() - should succeed
3. Integration test: Create multiple pages, close some, storage_state() collects from remaining
