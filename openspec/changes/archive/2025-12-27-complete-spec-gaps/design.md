# Design: Complete Spec Gaps

## Context

A comprehensive audit identified gaps between specifications and implementation. This document captures technical decisions for addressing the most impactful gaps.

## Goals / Non-Goals

### Goals
- Bring implementation to full spec compliance
- Maintain backward compatibility (all changes are additive)
- Keep implementation simple and focused

### Non-Goals
- Adding new features not already in specs
- Refactoring existing working code
- Performance optimization (unless required by spec)

## Decisions

### 1. Navigation Response with Status/Headers

**Decision**: Capture response from `Network.responseReceived` during navigation and attach to `NavigationResponse`.

**Rationale**: The navigation waiter already subscribes to network events. We extend it to capture the main document response and extract status/headers.

**Implementation**:
- Add `status: Option<u16>` and `headers: Option<HashMap<String, String>>` to `NavigationResponse`
- In `wait_for_navigation`, listen for `Network.responseReceived` matching the navigation request ID
- Store the response data when the main document response is received

### 2. Route Fetch Implementation

**Decision**: Use CDP `Fetch.continueRequest` with `interceptResponse: true` followed by `Fetch.getResponseBody`.

**Rationale**: CDP's Fetch domain supports request interception with response access. This is the standard approach used by Playwright.

**Implementation**:
- When `route.fetch()` is called, issue `Fetch.continueRequest` with response interception
- Wait for `Fetch.requestPaused` with `responseStatusCode`
- Call `Fetch.getResponseBody` to get body
- Construct `FetchedResponse` with actual data
- Allow caller to modify and fulfill or continue

### 3. Storage State Restoration

**Decision**: Apply localStorage via `Page.addScriptToEvaluateOnNewDocument` and IndexedDB via JavaScript evaluation on first navigation.

**Rationale**: 
- localStorage must be set before page scripts run (init script)
- IndexedDB requires async JavaScript and an active page context
- This matches Playwright's approach

**Implementation**:
- On context creation with storage state, generate JS that populates localStorage per origin
- Add as init script to apply before page loads
- For IndexedDB, after first page in context navigates, run restoration script

### 4. HTTP Credentials Application

**Decision**: Handle `Fetch.authRequired` CDP event to provide credentials automatically.

**Rationale**: The infrastructure for `Fetch.authRequired` handling already exists in `auth.rs`. We need to wire it to actually respond with credentials.

**Implementation**:
- Ensure `Fetch.enable` is called with `handleAuthRequests: true`
- In auth handler, when `Fetch.authRequired` fires, call `Fetch.continueWithAuth` with stored credentials
- Support both Basic and Digest auth schemes

### 5. Context Route Propagation

**Decision**: Store context routes in `BrowserContext` and apply them to new pages during `new_page()`.

**Rationale**: Context-level routes should behave like page-level routes but automatically apply to all pages.

**Implementation**:
- `BrowserContext` already has `ContextRouteRegistry`
- In `new_page()`, after creating the page, copy context routes to page's handler registry
- Maintain precedence: page routes checked before context routes

### 6. ARIA Snapshot Assertion

**Decision**: Create `to_match_aria_snapshot()` assertion wrapper around existing `AriaSnapshot::matches()`.

**Rationale**: The core matching logic exists in `aria.rs`. We just need the assertion API.

**Implementation**:
- Add `to_match_aria_snapshot(expected: &str)` to `LocatorAssertions`
- Parse expected string into `AriaSnapshot`
- Call `AriaSnapshot::matches()` for comparison
- Generate meaningful diff on failure

### 7. Frame Events

**Decision**: Subscribe to CDP `Page.frameAttached`, `Page.frameNavigated`, `Page.frameDetached` and emit corresponding events.

**Rationale**: CDP provides these events directly. We need to propagate them to user-facing API.

**Implementation**:
- Add event handler callbacks to `Page` struct
- In page event loop, handle CDP frame events
- Emit events with `Frame` reference

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Route fetch changes network timing | Document that fetch() adds latency; test thoroughly |
| Storage restoration timing | Use init scripts for localStorage; document IndexedDB limitations |
| Frame events increase event loop complexity | Keep handlers lightweight; use channels for async |

## Open Questions

1. Should `route.fetch()` support timeouts? (Recommend: yes, use page default timeout)
2. Should context routes auto-update on page after registration? (Recommend: no, only at creation time)
