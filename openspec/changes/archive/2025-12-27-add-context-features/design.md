# Design: Browser Context Features

## Context

Browser contexts provide isolated sessions with their own cookies, cache, and storage. This proposal adds APIs to control context-level state, which is essential for authentication testing, permission testing, and state management.

## Goals

- Complete cookie management matching Playwright's API
- Storage state save/restore for test isolation
- Permission granting per context and per origin
- Geolocation mocking for location-based features
- HTTP credentials for basic auth testing

## Non-Goals

- Service Worker management (Chromium-only, complex)
- Browser-level settings (handled at launch)
- CDP session management (advanced use case)

## Decisions

### Decision 1: Cookie Type Structure

**Choice**: Use a Cookie struct matching Playwright's cookie object.

```rust
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub url: Option<String>,  // Either url OR domain+path
    pub expires: Option<f64>, // Unix timestamp in seconds
    pub http_only: Option<bool>,
    pub secure: Option<bool>,
    pub same_site: Option<SameSite>,
}

pub enum SameSite {
    Strict,
    Lax,
    None,
}
```

**Rationale**:
- Matches Playwright's cookie structure
- All fields optional except name/value
- URL is convenience alternative to domain+path

### Decision 2: Storage State Format

**Choice**: Use JSON format matching Playwright for storage state files.

```json
{
  "cookies": [...],
  "origins": [
    {
      "origin": "https://example.com",
      "localStorage": [
        { "name": "key", "value": "value" }
      ]
    }
  ]
}
```

**Rationale**:
- Matches Playwright's format for interoperability
- JSON is human-readable and easy to edit
- Supports cookies + localStorage (IndexedDB optional)

### Decision 3: Permission Names

**Choice**: Use string permission names matching web standards and Playwright.

**Supported Permissions**:
- `geolocation` - location access
- `notifications` - push notifications
- `camera` - video capture
- `microphone` - audio capture
- `clipboard-read` - read clipboard
- `clipboard-write` - write clipboard
- `background-sync` - background sync
- `midi` - MIDI device access
- `midi-sysex` - MIDI system exclusive
- `accelerometer`, `gyroscope`, `magnetometer` - sensors
- `ambient-light-sensor` - light sensor
- `payment-handler` - payment API
- `storage-access` - storage access API

**Note**: Permission support varies by browser.

### Decision 4: Geolocation Accuracy

**Choice**: Default accuracy to 0 (exact), matching Playwright.

```rust
pub struct Geolocation {
    pub latitude: f64,   // -90 to 90
    pub longitude: f64,  // -180 to 180
    pub accuracy: Option<f64>, // defaults to 0
}
```

**Rationale**:
- Tests typically want exact location
- Accuracy is optional for simplicity
- None/null means position unavailable

### Decision 5: Context Options Builder

**Choice**: Extend context creation with builder pattern for all options.

```rust
let context = browser.new_context()
    .storage_state("state.json")
    .geolocation(59.95, 30.31667)
    .permissions(vec!["geolocation"])
    .http_credentials("user", "pass")
    .extra_http_headers(headers)
    .offline(false)
    .has_touch(true)
    .build()
    .await?;
```

**Rationale**:
- Many optional configuration options
- Builder pattern is idiomatic and readable
- Can validate options before context creation

### Decision 6: Clear Cookies Filtering

**Choice**: Support optional filtering when clearing cookies.

```rust
// Clear all
context.clear_cookies().await?;

// Clear by name
context.clear_cookies().name("session").await?;

// Clear by domain
context.clear_cookies().domain("example.com").await?;

// Clear by domain regex
context.clear_cookies().domain(Regex::new(r".*\.example\.com")?).await?;

// Combine filters
context.clear_cookies().name("token").domain("api.example.com").await?;
```

**Rationale**:
- Matches Playwright's filtering options
- Selective clearing is common need
- Builder pattern for optional filters

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| Add cookies | Network.setCookies | Network |
| Get cookies | Network.getCookies | Network |
| Clear cookies | Network.deleteCookies | Network |
| Set geolocation | Emulation.setGeolocationOverride | Emulation |
| Clear geolocation | Emulation.clearGeolocationOverride | Emulation |
| Grant permissions | Browser.grantPermissions | Browser |
| Reset permissions | Browser.resetPermissions | Browser |
| Set offline | Network.emulateNetworkConditions | Network |
| Extra headers | Network.setExtraHTTPHeaders | Network |
| HTTP credentials | Fetch domain for auth challenges | Fetch |

## Risks / Trade-offs

### Risk: Permission Support Varies by Browser

**Mitigation**:
- Document which permissions are supported per browser
- Return error for unsupported permissions
- Test on target browser

### Risk: Storage State Size

**Mitigation**:
- Consider size limits for IndexedDB snapshots
- Document that large localStorage may be slow
- Optional IndexedDB inclusion

### Risk: HTTP Credentials Caching

**Mitigation**:
- Document that browsers may cache credentials
- Recommend new context for different credentials
- Match Playwright's behavior (deprecated setHTTPCredentials)

## Open Questions

None - all design decisions resolved.
