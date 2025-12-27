# Change: Add Browser Context Features

## Why

Browser contexts provide isolation between test sessions, but many testing scenarios require controlling context-level state:
- **Authentication Testing**: Set cookies, storage state, HTTP credentials
- **Geolocation Testing**: Mock user location for location-based features
- **Permission Testing**: Grant or deny browser permissions (camera, notifications, etc.)
- **State Management**: Save and restore authentication state between tests
- **Offline Testing**: Simulate network offline conditions
- **Header Injection**: Add custom headers to all requests

This is proposal 4 of 11 in the Playwright feature parity series (Chromium only).

## What Changes

### New Capabilities

1. **Cookie Management** - Control browser cookies
   - `context.add_cookies(cookies)` - add cookies to context
   - `context.cookies()` - get all cookies
   - `context.cookies(urls)` - get cookies for specific URLs
   - `context.clear_cookies()` - clear all or filtered cookies

2. **Storage State** - Save and restore browser state
   - `context.storage_state()` - get cookies + localStorage snapshot
   - `context.storage_state().path(file)` - save to file
   - Context options: `storage_state: path` to restore state
   - IndexedDB snapshot support

3. **Permissions** - Control browser permissions
   - `context.grant_permissions(permissions)` - grant permissions
   - `context.grant_permissions(permissions).origin(url)` - per-origin
   - `context.clear_permissions()` - reset permissions
   - Support: geolocation, notifications, camera, microphone, clipboard, etc.

4. **HTTP Credentials** - Basic/Digest authentication
   - Context option: `http_credentials: { username, password }`
   - Automatic authentication for matching requests

5. **Geolocation** - Mock browser location
   - `context.set_geolocation(lat, long)` - set location
   - `context.set_geolocation(None)` - position unavailable
   - Context option: `geolocation: { latitude, longitude, accuracy }`

6. **Extra HTTP Headers** - Add headers to all requests
   - `context.set_extra_http_headers(headers)` - set headers
   - Headers merged with page-specific headers

7. **Offline Mode** - Simulate network conditions
   - `context.set_offline(true)` - go offline
   - `context.set_offline(false)` - go online

8. **Timeout Configuration** - Default timeouts
   - `context.set_default_timeout(ms)` - action timeout
   - `context.set_default_navigation_timeout(ms)` - navigation timeout

9. **Context Lifecycle** - Manage context and pages
   - `context.pages()` - get all pages in context
   - `context.on('page')` - listen for new page creation
   - `context.on('close')` - listen for context close
   - `context.close()` - close context and all pages

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1 | Core Page Operations | Complete |
| 2 | Network Interception | Complete |
| 3 | Input Devices | Complete |
| **4** | **Browser Context Features** (this) | **Current** |
| 5-11 | ... | Pending |

## Impact

- **New specs**: `cookies`, `storage-state`, `permissions`, `http-credentials`, `geolocation`
- **Affected code**: 
  - `viewpoint-cdp/src/protocol/` - Network, Browser, Emulation domains
  - `viewpoint-core/src/context/` - expand BrowserContext with new methods
  - `viewpoint-core/src/browser/` - context options handling
- **Breaking changes**: None
- **Dependencies**: None (can be implemented in parallel with proposals 1-3)
