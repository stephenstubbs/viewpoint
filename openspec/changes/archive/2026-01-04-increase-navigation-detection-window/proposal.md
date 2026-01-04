# Change: Increase navigation detection window from 50ms to 150ms

## Why

The `NavigationWaiter` uses a 50ms detection window to detect if a click action triggers page navigation. This window is too short - `Page.frameNavigated` CDP events often arrive after 50ms, causing:

1. Click returns immediately without waiting for navigation
2. Subsequent operations (e.g., snapshot capture) fail because page is mid-transition
3. `document.body` can be `null` during this window, causing deserialization failures

Debug logs confirm the issue:
```
Created NavigationWaiter session_id=... frame_id=...
No navigation detected within detection window
```

## What Changes

1. Increase `NAVIGATION_DETECTION_WINDOW` from 50ms to 150ms in `navigation_waiter/mod.rs`
2. Add defensive null handling in `aria_snapshot_with_refs_js()` to return empty snapshot when `document.body` is null

## Impact

- All clicks will wait at least 150ms before returning if no navigation is detected
- This is acceptable: reliability is more important than 100ms latency for click operations
- Navigation-triggering clicks will now properly wait for page load
