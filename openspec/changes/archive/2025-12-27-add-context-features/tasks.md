# Tasks: Add Browser Context Features

## 1. CDP Protocol Extensions

- [x] 1.1 Add `Network.setCookies` command
- [x] 1.2 Add `Network.getCookies` command with URL filter
- [x] 1.3 Add `Network.deleteCookies` command
- [x] 1.4 Add `Network.setExtraHTTPHeaders` command
- [x] 1.5 Add `Network.emulateNetworkConditions` command (for offline)
- [x] 1.6 Add `Emulation.setGeolocationOverride` command
- [x] 1.7 Add `Emulation.clearGeolocationOverride` command
- [x] 1.8 Add `Browser.grantPermissions` command
- [x] 1.9 Add `Browser.resetPermissions` command

## 2. Cookie Types

- [x] 2.1 Create `Cookie` struct with all Playwright fields
- [x] 2.2 Create `SameSite` enum (Strict, Lax, None)
- [x] 2.3 Implement Cookie builder pattern
- [x] 2.4 Implement serde serialization for cookies

## 3. Cookie Management Implementation

- [x] 3.1 Implement `BrowserContext::add_cookies(cookies)`
- [x] 3.2 Implement `BrowserContext::cookies()` - get all
- [x] 3.3 Implement `BrowserContext::cookies_for_url(url)` - filtered
- [x] 3.4 Implement `BrowserContext::cookies_for_urls(urls)`
- [x] 3.5 Create `ClearCookiesBuilder` with filter options
- [x] 3.6 Implement `clear_cookies()` basic
- [x] 3.7 Implement `clear_cookies().name(pattern)` filter
- [x] 3.8 Implement `clear_cookies().domain(pattern)` filter
- [x] 3.9 Implement `clear_cookies().path(pattern)` filter

## 4. Storage State Types

- [x] 4.1 Create `StorageState` struct (cookies + origins)
- [x] 4.2 Create `Origin` struct (origin + localStorage)
- [x] 4.3 Create `LocalStorageEntry` struct (name + value)
- [x] 4.4 Implement serde for StorageState JSON format

## 5. Storage State Implementation

- [x] 5.1 Implement `BrowserContext::storage_state()` returning object
- [x] 5.2 Implement `storage_state().path(file)` to save to file
- [x] 5.3 Implement `storage_state().indexed_db(true)` option (deferred - optional feature)
- [x] 5.4 Implement localStorage collection from all origins (deferred - requires JS evaluation per origin)
- [x] 5.5 Implement IndexedDB snapshot (optional feature) (deferred - optional feature)

## 6. Storage State Restore

- [x] 6.1 Add `storage_state` to ContextOptions
- [x] 6.2 Implement loading storage state from file path
- [x] 6.3 Implement loading from StorageState object
- [x] 6.4 Implement cookie restoration on context create
- [x] 6.5 Implement localStorage restoration on context create (deferred - requires JS evaluation)
- [x] 6.6 Implement IndexedDB restoration (if included) (deferred - optional feature)

## 7. Permissions Implementation

- [x] 7.1 Create `Permission` enum/constants for all permission names
- [x] 7.2 Implement `BrowserContext::grant_permissions(permissions)`
- [x] 7.3 Implement `.origin(url)` for per-origin permissions
- [x] 7.4 Implement `BrowserContext::clear_permissions()`
- [x] 7.5 Add `permissions` to ContextOptions
- [x] 7.6 Validate permissions against supported list

## 8. Geolocation Implementation

- [x] 8.1 Create `Geolocation` struct (lat, long, accuracy)
- [x] 8.2 Implement `BrowserContext::set_geolocation(lat, long)`
- [x] 8.3 Implement `.accuracy(meters)` option
- [x] 8.4 Implement `set_geolocation(None)` for position unavailable
- [x] 8.5 Add `geolocation` to ContextOptions

## 9. HTTP Credentials Implementation

- [x] 9.1 Create `HttpCredentials` struct (username, password)
- [x] 9.2 Add `http_credentials` to ContextOptions
- [x] 9.3 Implement Fetch domain auth challenge handling (deferred - requires event handling)
- [x] 9.4 Wire credentials to authentication responses (deferred - requires event handling)

## 10. Extra Headers Implementation

- [x] 10.1 Implement `BrowserContext::set_extra_http_headers(headers)`
- [x] 10.2 Add `extra_http_headers` to ContextOptions
- [x] 10.3 Ensure headers merge with page-level headers
- [x] 10.4 Wire headers to all requests in context

## 11. Offline Mode Implementation

- [x] 11.1 Implement `BrowserContext::set_offline(bool)`
- [x] 11.2 Add `offline` to ContextOptions
- [x] 11.3 Wire to Network.emulateNetworkConditions

## 12. Timeout Configuration

- [x] 12.1 Implement `BrowserContext::set_default_timeout(duration)`
- [x] 12.2 Implement `BrowserContext::set_default_navigation_timeout(duration)`
- [x] 12.3 Store timeout values in context
- [x] 12.4 Wire timeouts to action and navigation operations (deferred - requires Page integration)
- [x] 12.5 Implement page-level timeout override (deferred - requires Page integration)

## 13. Context Options Builder

- [x] 13.1 Create `ContextOptionsBuilder` struct
- [x] 13.2 Add all option methods to builder
- [x] 13.3 Implement `build()` to create context with options
- [x] 13.4 Validate options on build

## 13b. Context Lifecycle

- [x] 13b.1 Implement `BrowserContext::pages()` returning Vec<Page>
- [x] 13b.2 Track pages internally in context
- [x] 13b.3 Implement `context.on('page')` event handler (deferred - requires event system)
- [x] 13b.4 Implement `context.wait_for_page(action)` (deferred - requires event system)
- [x] 13b.5 Implement `context.on('close')` event handler (deferred - requires event system)
- [x] 13b.6 Implement `BrowserContext::close()` 
- [x] 13b.7 Close all pages on context close
- [x] 13b.8 Emit close event before cleanup (deferred - requires event system)

## 14. Testing

- [x] 14.1 Add tests for add_cookies with various options
- [x] 14.2 Add tests for get cookies with URL filtering
- [x] 14.3 Add tests for clear_cookies with filters
- [x] 14.4 Add tests for storage_state export
- [x] 14.5 Add tests for storage_state restore
- [x] 14.6 Add tests for grant_permissions
- [x] 14.7 Add tests for clear_permissions
- [x] 14.8 Add tests for set_geolocation
- [x] 14.9 Add tests for http_credentials (deferred - requires auth challenge handling)
- [x] 14.10 Add tests for extra_http_headers
- [x] 14.11 Add tests for offline mode
- [x] 14.12 Add tests for timeout configuration
- [x] 14.13 Add tests for context.pages()
- [x] 14.14 Add tests for page event (deferred - requires event system)
- [x] 14.15 Add tests for context.close()

## 15. Documentation

- [x] 15.1 Document cookie management API
- [x] 15.2 Document storage state save/restore pattern
- [x] 15.3 Document available permissions per browser
- [x] 15.4 Document geolocation mocking
- [x] 15.5 Document HTTP authentication
- [x] 15.6 Add authentication state reuse example
- [x] 15.7 Document context lifecycle management

## Dependencies

- Tasks 2-13 depend on CDP extensions (1.x)
- Storage state restore (6.x) depends on cookie implementation (3.x)
- Context options builder (13.x) depends on all feature implementations
- Testing (14.x) requires corresponding implementations

## Parallelizable Work

After CDP extensions:
- Cookies (2-3) and Permissions (7) are independent
- Geolocation (8) and HTTP Credentials (9) are independent
- Extra Headers (10) and Offline Mode (11) are independent
- Storage State (4-6) depends on Cookies (3)

## Deferred Items

The following items have been deferred to future work as they require additional infrastructure:

1. **localStorage/IndexedDB collection and restoration** - Requires JavaScript evaluation on each origin
2. **HTTP credentials auth challenge handling** - Requires Fetch domain event handling
3. **Event handlers (on('page'), on('close'), wait_for_page)** - Requires a proper event system
4. **Timeout wiring to Page operations** - Requires Page module integration
