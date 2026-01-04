# Tasks: Fix Storage State Stale Session Handling

## 1. Implementation

- [x] 1.1 Add `context_pages: Option<Arc<RwLock<Vec<PageInfo>>>>` field to `Page` struct
- [x] 1.2 Modify `Page::new_with_indices()` to accept and store the pages Arc
- [x] 1.3 Modify `Page::with_video_and_indices()` to accept and store the pages Arc
- [x] 1.4 Update `page_factory::create_page_instance()` to pass pages Arc to Page constructors
- [x] 1.5 Modify `Page::close()` to remove self from pages list before returning

## 2. Testing

- [x] 2.1 Add integration test: close page, verify it's removed from context's pages list
- [x] 2.2 Add integration test: close page, then call storage_state() - should succeed
- [x] 2.3 Add integration test: create multiple pages, close some, storage_state() works on remaining

## Dependencies

- No external dependencies

## Validation

```bash
# Run unit tests
cargo test --workspace

# Run integration tests (requires Chromium)
cargo test --workspace --features integration
```
