## 1. Change Internal Storage
- [x] 1.1 Change `pages: Arc<RwLock<Vec<PageInfo>>>` to `Arc<RwLock<Vec<Page>>>` in BrowserContext
- [x] 1.2 Update `handle_target_created` to store the `Page` object after creation
- [x] 1.3 Update `handle_target_destroyed` to remove page by target_id
- [x] 1.4 Update any other code that reads from the pages Vec

## 2. Fix pages() Return Type
- [x] 2.1 Change `pages()` to return `Result<Vec<Page>, ContextError>`
- [x] 2.2 Return cloned pages from internal storage instead of querying CDP
- [x] 2.3 Add `page_count()` convenience method if not present

## 3. Testing
- [x] 3.1 Update existing tests for new return type
- [x] 3.2 Verify externally-opened pages appear in `pages()`
- [x] 3.3 Run `cargo test --workspace`
- [x] 3.4 Run `cargo test --workspace --features integration`

## 4. Version Bump
- [x] 4.1 Bump version (breaking change to public API)
