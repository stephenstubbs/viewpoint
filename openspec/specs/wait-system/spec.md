# wait-system Specification

## Purpose
TBD - created by archiving change add-chromium-connection. Update Purpose after archive.
## Requirements
### Requirement: Document Load States

The system SHALL support Playwright-compatible document load states for navigation waiting.

The following load states SHALL be supported:
- `Commit` - Navigation response received (first byte)
- `DomContentLoaded` - DOM fully parsed (DOMContentLoaded event fired)
- `Load` - Full page load complete (load event fired) - **default**
- `NetworkIdle` - No network requests for 500ms

#### Scenario: Wait for commit
- **GIVEN** a page navigation is initiated
- **WHEN** `wait_until(DocumentLoadState::Commit)` is specified
- **THEN** the wait completes when the navigation response is received

#### Scenario: Wait for DOMContentLoaded
- **GIVEN** a page navigation is initiated
- **WHEN** `wait_until(DocumentLoadState::DomContentLoaded)` is specified
- **THEN** the wait completes when `Page.domContentEventFired` CDP event is received

#### Scenario: Wait for load (default)
- **GIVEN** a page navigation is initiated
- **WHEN** no `wait_until` is specified OR `wait_until(DocumentLoadState::Load)` is specified
- **THEN** the wait completes when `Page.loadEventFired` CDP event is received

#### Scenario: Wait for network idle
- **GIVEN** a page navigation is initiated
- **WHEN** `wait_until(DocumentLoadState::NetworkIdle)` is specified
- **THEN** the wait completes when no network requests occur for 500ms after load

### Requirement: Wait Timeout

The system SHALL enforce configurable timeouts on all wait operations.

#### Scenario: Default timeout
- **GIVEN** a wait operation is initiated without explicit timeout
- **WHEN** the wait condition is not met within 30 seconds
- **THEN** a `WaitError::Timeout` error is returned

#### Scenario: Custom timeout
- **GIVEN** a wait operation is initiated with `.timeout(Duration::from_secs(5))`
- **WHEN** the wait condition is not met within 5 seconds
- **THEN** a `WaitError::Timeout` error is returned

#### Scenario: Successful wait within timeout
- **GIVEN** a wait operation is initiated with a timeout
- **WHEN** the wait condition is met before the timeout
- **THEN** the operation completes successfully without error

### Requirement: Network Activity Tracking

The system SHALL track network activity for NetworkIdle detection.

The tracker SHALL:
- Monitor `Network.requestWillBeSent` events (increment pending count)
- Monitor `Network.loadingFinished` events (decrement pending count)
- Monitor `Network.loadingFailed` events (decrement pending count)
- Consider network idle when pending count is 0 for 500ms

#### Scenario: Network becomes idle
- **GIVEN** a page has active network requests
- **WHEN** all requests complete and no new requests occur for 500ms
- **THEN** the network is considered idle

#### Scenario: New request during idle wait
- **GIVEN** a page is waiting for network idle with 0 pending requests
- **WHEN** a new network request is initiated before 500ms elapses
- **THEN** the idle timer resets

#### Scenario: Request failure counts as completion
- **GIVEN** a page has a pending network request
- **WHEN** the request fails (e.g., DNS error, connection refused)
- **THEN** the pending count decrements and idle detection continues

### Requirement: Wait State Ordering

The system SHALL respect the natural ordering of load states.

Load states occur in this order: Commit → DomContentLoaded → Load → NetworkIdle

#### Scenario: Earlier state already reached
- **GIVEN** a page has already fired the `load` event
- **WHEN** `wait_for_load_state(DocumentLoadState::DomContentLoaded)` is called
- **THEN** the wait completes immediately without waiting

#### Scenario: Wait for later state
- **GIVEN** a page has fired `DomContentLoaded` but not `load`
- **WHEN** `wait_for_load_state(DocumentLoadState::Load)` is called
- **THEN** the wait blocks until `Page.loadEventFired` is received

### Requirement: Navigation Detection Window

The system SHALL detect navigation triggered by user actions (click, press) within a reasonable detection window.

The `NavigationWaiter` SHALL wait up to 150ms after an action to detect if a `Page.frameNavigated` CDP event is triggered.

#### Scenario: Click triggers navigation
- **GIVEN** a click action is performed on a link
- **WHEN** the browser triggers `Page.frameNavigated` within 150ms
- **THEN** the click waits for navigation to complete before returning

#### Scenario: Click does not trigger navigation
- **GIVEN** a click action is performed on a button that does not navigate
- **WHEN** no `Page.frameNavigated` event occurs within 150ms
- **THEN** the click returns after the detection window expires

