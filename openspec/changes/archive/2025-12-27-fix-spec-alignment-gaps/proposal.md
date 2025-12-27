# Change: Fix Spec Alignment Gaps

## Why

When archiving the `complete-spec-gaps` change, spec updates were skipped due to header conflicts. A comparison of archived deltas against current specs revealed several gaps where implemented functionality is not fully documented in the specifications.

## What Changes

### http-credentials (Material Gaps)
- Add explicit `Fetch.authRequired` CDP event handling scenario
- Add dedicated Digest authentication scenario
- Add `WWW-Authenticate` header response handling

### test-actions (Minor Gap)
- Add middle-click (`MouseButton::Middle`) scenario to Click Action

### network-routing (Minor Gap)
- Add `FetchedResponse` type documentation with `status`, `headers`, `body` access
- Add `fetched.text()` and `fetched.json()` helper methods

## Impact

- **Affected specs**: `http-credentials`, `test-actions`, `network-routing`
- **Affected code**: None (documentation-only change)
- **Breaking changes**: None
