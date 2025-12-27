# Change: Fix Audit Gaps

## Why

A comprehensive audit of Viewpoint specs against the implementation revealed several gaps where specs promise functionality that is not fully implemented. This proposal addresses the high and medium priority gaps to bring the implementation into alignment with specifications.

## What Changes

### High Priority

1. **HAR Recording** - Implement active HAR capture during navigation (currently only replay/update mode exists)
2. **Redirect Chain Tracking** - Populate `redirected_from`/`redirected_to` on Request objects
3. **Route Fallback** - Fix `route.fallback()` to pass to next matching handler (currently just continues request)

### Medium Priority

4. **File Upload from Buffer** - Add `set_input_files_from_buffer()` API for in-memory file uploads
5. **Request Failure Details** - Populate `request.failure()` with error information from loadingFailed events
6. **Security Details** - Populate `SecurityDetails` from CDP response for HTTPS requests

## Impact

- **Affected specs**: `har-support`, `network-events`, `network-routing`, `file-uploads`
- **Affected code**:
  - `viewpoint-core/src/network/har.rs` - HAR recording
  - `viewpoint-core/src/network/har_replay.rs` - HAR recording integration
  - `viewpoint-core/src/network/request.rs` - Redirect chain, failure details
  - `viewpoint-core/src/network/response.rs` - Security details
  - `viewpoint-core/src/network/route.rs` - Route fallback
  - `viewpoint-core/src/network/handler.rs` - Route fallback chaining
  - `viewpoint-core/src/page/locator/actions.rs` - File upload from buffer
- **Breaking changes**: None (all additive)
