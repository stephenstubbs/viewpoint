# Change: Complete Spec Gaps

## Why

A comprehensive audit of the Viewpoint codebase against specifications revealed several gaps where functionality is specified but not fully implemented. These gaps affect core workflows like navigation response handling, storage state restoration, HTTP authentication, and network interception. Completing these gaps will bring Viewpoint to full spec compliance.

## What Changes

### High Priority (Core Functionality)

1. **Navigation Response** - Add HTTP status code and headers to `NavigationResponse`
2. **Route Fetch** - Implement actual network fetch in `route.fetch()` instead of placeholder
3. **Storage State Restoration** - Restore localStorage and IndexedDB on context creation (not just cookies)
4. **HTTP Credentials** - Apply stored credentials to network requests via Authorization header
5. **Context Route Propagation** - Propagate context-level routes to newly created pages

### Medium Priority (Test Framework)

6. **Advanced Assertions** - Add `to_have_count_greater_than()` and `to_match_aria_snapshot()`
7. **Frame Events** - Add `frameattached`, `framenavigated`, `framedetached` events
8. **Action Builders** - Add `.position()`, `.button()`, `.modifiers()`, `.force()` to click action
9. **Custom Test ID** - Make test ID attribute configurable (not hardcoded to `data-testid`)

### Lower Priority (Completeness)

10. **Tracing Snapshots** - Actually capture DOM snapshots and sources when options are set

## Impact

- **Affected specs**: `navigation`, `network-routing`, `storage-state`, `http-credentials`, `advanced-assertions`, `frames`, `test-actions`, `test-locators`, `tracing`
- **Affected code**:
  - `viewpoint-core/src/page/mod.rs` - navigation response
  - `viewpoint-core/src/network/route.rs` - route fetch
  - `viewpoint-core/src/browser/mod.rs` - storage restoration
  - `viewpoint-core/src/network/handler.rs` - HTTP auth
  - `viewpoint-core/src/context/routing.rs` - route propagation
  - `viewpoint-test/src/expect/locator.rs` - new assertions
  - `viewpoint-core/src/page/` - frame events, action builders
- **Breaking changes**: None (all additive)
- **Dependencies**: Each area can be implemented independently
