# context-lifecycle Spec Delta

## ADDED Requirements

### Requirement: Page Activation Events

The system SHALL emit events when a page becomes the active/foreground tab in the browser, including when the user clicks on a tab in the browser UI.

#### Scenario: User clicks tab emits activation event

- **GIVEN** a browser context with two pages open
- **AND** an `on_page_activated` event listener is registered
- **WHEN** the user clicks on the second tab in the browser UI
- **THEN** the `page_activated` event is emitted
- **AND** the event contains the `Page` instance that was activated

#### Scenario: Register activation event handler

- **GIVEN** a browser context
- **WHEN** `context.on_page_activated(|page| async { ... }).await` is called
- **THEN** a `HandlerId` is returned
- **AND** the handler will be called for future page activation events

#### Scenario: Remove activation event handler

- **GIVEN** a browser context with a registered activation event handler
- **WHEN** `context.off_page_activated(handler_id).await` is called
- **THEN** the handler is removed
- **AND** future activation events do not trigger the removed handler

#### Scenario: Activation event only for context's pages

- **GIVEN** two browser contexts, each with pages
- **AND** context A has an `on_page_activated` listener
- **WHEN** a page in context B becomes active
- **THEN** context A's listener is NOT triggered
- **AND** only context B's listeners (if any) are triggered

#### Scenario: Programmatic tab switch emits activation event

- **GIVEN** a browser context with two pages open
- **AND** an `on_page_activated` event listener is registered
- **WHEN** a page is brought to front via `page.bring_to_front().await`
- **THEN** the `page_activated` event is emitted for that page

### Requirement: Target.targetInfoChanged CDP Event Handling

The system SHALL listen for `Target.targetInfoChanged` CDP events to detect page activation changes.

#### Scenario: targetInfoChanged listener registered on context creation

- **GIVEN** a browser instance
- **WHEN** a new browser context is created
- **THEN** a listener for CDP `Target.targetInfoChanged` events is registered
- **AND** the listener filters events to only process page-type targets in this context

#### Scenario: targetInfoChanged triggers page lookup

- **GIVEN** a browser context with tracked pages
- **WHEN** a `Target.targetInfoChanged` event is received for a tracked page
- **THEN** the system looks up the `Page` instance by target ID
- **AND** emits the `page_activated` event with that `Page`
