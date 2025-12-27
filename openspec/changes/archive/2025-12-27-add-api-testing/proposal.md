# Change: Add API Testing

## Why

Many tests need to make HTTP requests outside of browser context:
- **Setup/Teardown**: Create test data via API
- **Verification**: Check backend state after UI actions
- **Authentication**: Get tokens for authenticated tests
- **API-only tests**: Test APIs without browser overhead

This is proposal 10 of 12 in the Playwright feature parity series.

## What Changes

### New Capabilities

1. **APIRequestContext** - Make HTTP requests
   - `context.request()` - get request context with cookies
   - `playwright.request.new_context()` - standalone context
   - `request.get()`, `post()`, `put()`, `patch()`, `delete()`
   - `request.fetch()` - generic request
   - `request.head()` - HEAD request

2. **Request Options** - Configure requests
   - Headers, query params, body
   - JSON, form data, multipart
   - Timeout, ignore HTTPS errors
   - HTTP credentials

3. **Response Handling** - Process responses
   - `response.status()`, `response.headers()`
   - `response.json()`, `response.text()`, `response.body()`
   - `response.ok()` - check success

4. **Context Integration** - Share state with browser
   - API context shares cookies with browser context
   - Requests go through proxy settings
   - Extra headers applied

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1-9 | Previous | Complete |
| **10** | **API Testing** (this) | **Current** |
| 11-12 | Remaining | Pending |

## Impact

- **New specs**: `api-request-context`
- **Affected code**: 
  - `viewpoint-core/src/api/` - new APIRequestContext, APIResponse
  - Uses reqwest or similar HTTP client
- **Breaking changes**: None
- **Dependencies**: Proposal 4 (cookies, credentials)
