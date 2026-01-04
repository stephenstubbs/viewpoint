# wait-system Delta

## ADDED Requirements

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
