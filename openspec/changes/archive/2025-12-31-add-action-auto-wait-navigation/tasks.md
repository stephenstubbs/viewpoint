# Tasks

## 1. Core Infrastructure

- [x] 1.1 Create `NavigationWaiter` struct that listens for frame navigation events via CDP
- [x] 1.2 Implement navigation detection using `Page.frameNavigated` and `Page.navigatedWithinDocument` events
- [x] 1.3 Add `wait_for_load_state()` method to wait for navigation to reach specified state
- [x] 1.4 Add configurable timeout for navigation waiting (default 30s, matching navigation timeout)

## 2. Action Builder Updates

- [x] 2.1 Add `no_wait_after(bool)` method to `ClickBuilder`
- [x] 2.2 Add `no_wait_after(bool)` method to `PressBuilder` (locator press)
- [x] 2.3 Add `no_wait_after(bool)` method to `FillBuilder`
- [x] 2.4 Add `no_wait_after(bool)` method to `SelectOptionBuilder`
- [x] 2.5 Add `no_wait_after(bool)` method to `CheckBuilder`/`UncheckBuilder`
- [x] 2.6 Add `no_wait_after(bool)` method to `Keyboard::press()`

## 3. Auto-Wait Implementation

- [x] 3.1 Wrap action execution to set up navigation listener before action
- [x] 3.2 After action completes, check if navigation was triggered
- [x] 3.3 If navigation triggered and `no_wait_after` is false, wait for `Load` state
- [x] 3.4 Handle timeout gracefully - return timeout error if navigation doesn't complete

## 4. Testing

- [x] 4.1 Add unit tests for `NavigationWaiter`
- [x] 4.2 Add integration test: click link that navigates, verify page loaded
- [x] 4.3 Add integration test: press Enter in search form, verify results page loaded
- [x] 4.4 Add integration test: `no_wait_after(true)` returns immediately
- [x] 4.5 Add integration test: action that doesn't trigger navigation returns quickly
- [x] 4.6 Add integration test: navigation timeout is respected (via check and select tests)

## 5. Documentation

- [x] 5.1 Update README with auto-wait behavior explanation
- [x] 5.2 Add examples showing `no_wait_after` usage
- [x] 5.3 Document differences from Playwright (if any) - None significant, behavior matches
