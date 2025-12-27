# Locator Handlers

## ADDED Requirements

### Requirement: Add Locator Handler

The system SHALL allow registering handlers for overlay elements that may block actions.

#### Scenario: Register handler for overlay

- **GIVEN** a page with cookie consent banner that appears during tests
- **WHEN** `page.add_locator_handler(page.get_by_role("button", "Accept"), |locator| locator.click()).await` is called
- **THEN** the handler is registered for that locator

#### Scenario: Handler triggered on blocked action

- **GIVEN** a locator handler is registered for a dismiss button
- **WHEN** an action is blocked by the overlay matching the handler's locator
- **THEN** the handler is invoked to dismiss the overlay
- **AND** the original action is retried

#### Scenario: Handler with no_wait_after option

- **GIVEN** a locator handler is registered with `no_wait_after: true`
- **WHEN** the handler completes
- **THEN** the system does not wait for navigation or animations

#### Scenario: Handler with times limit

- **GIVEN** a locator handler is registered with `times: 1`
- **WHEN** the handler runs once
- **THEN** the handler is automatically removed

### Requirement: Remove Locator Handler

The system SHALL allow removing registered locator handlers.

#### Scenario: Remove handler by locator

- **GIVEN** a locator handler is registered
- **WHEN** `page.remove_locator_handler(locator).await` is called with the same locator
- **THEN** the handler is removed and no longer triggered

#### Scenario: Handler not found

- **GIVEN** no handler is registered for a locator
- **WHEN** `page.remove_locator_handler(locator).await` is called
- **THEN** the operation succeeds silently (no error)
