# Change: Add Auto-Wait for Navigation After Actions

## Why

Actions like `click()`, `press()`, and form submissions can trigger page navigations. Currently, these actions return immediately after the DOM event is dispatched, without waiting for any resulting navigation to complete. This causes race conditions where subsequent operations fail with "Connection closed" or similar errors because the page context is being destroyed/recreated during navigation.

Playwright's default behavior is to automatically wait for navigation after actions that might trigger it. The `noWaitAfter` option exists to opt out of this behavior. Viewpoint should match this behavior to provide reliable browser automation.

**Real-world example**: Typing into a DuckDuckGo search box with `submit: true` presses Enter, which triggers navigation to search results. Without auto-waiting, subsequent snapshot or interaction calls fail because the page is mid-navigation.

## What Changes

- Actions (`click`, `dblclick`, `press`, `fill`, `select_option`, `check`, `uncheck`) SHALL automatically wait for any triggered navigation to complete
- A new `noWaitAfter` option SHALL allow opting out of this behavior
- Navigation detection SHALL use CDP frame navigation events
- Default wait state SHALL be `Load` (matching Playwright)
- **BREAKING**: Actions will now wait longer by default if they trigger navigation

## Impact

- Affected specs: `test-actions`
- Affected code: `viewpoint-core` action implementations (click, press, fill, etc.)
- Breaking change: Actions that previously returned immediately will now wait if navigation is triggered (this is the correct/expected behavior)
