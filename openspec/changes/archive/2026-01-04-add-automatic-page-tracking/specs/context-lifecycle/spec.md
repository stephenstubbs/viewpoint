## MODIFIED Requirements

### Requirement: Page Creation Events

The system SHALL emit events when new pages are created in the context, including pages opened externally (popups, new tabs from links, `window.open()`).

#### Scenario: New page event
- **GIVEN** a browser context with a page event listener
- **WHEN** `context.new_page().await` is called
- **THEN** the 'page' event is emitted with the new Page instance

#### Scenario: Popup triggers page event
- **GIVEN** a browser context with a page event listener
- **WHEN** a page opens a popup via `window.open()`
- **THEN** the 'page' event is emitted with the popup Page instance
- **AND** the popup is added to `context.pages()` automatically

#### Scenario: Target blank link triggers page event
- **GIVEN** a browser context with a page event listener
- **WHEN** a link with `target="_blank"` is clicked
- **THEN** the 'page' event is emitted with the new Page instance
- **AND** the new page is added to `context.pages()` automatically

#### Scenario: Ctrl+click link triggers page event
- **GIVEN** a browser context with a page event listener
- **WHEN** a link is Ctrl+clicked (or Cmd+clicked on macOS)
- **THEN** the 'page' event is emitted with the new Page instance
- **AND** the new page is added to `context.pages()` automatically

#### Scenario: Wait for new page
- **GIVEN** a browser context
- **WHEN** `context.wait_for_page(action).await` is called
- **AND** the action creates a new page
- **THEN** the new Page instance is returned

#### Scenario: Automatic tracking does not duplicate explicitly created pages
- **GIVEN** a browser context with a page event listener
- **WHEN** `context.new_page().await` is called
- **THEN** the page appears exactly once in `context.pages()`
- **AND** only one 'page' event is emitted

## ADDED Requirements

### Requirement: CDP Target Event Registration

The system SHALL register CDP event listeners for target lifecycle events when a browser context is created.

#### Scenario: Target.targetCreated listener registered on context creation
- **GIVEN** a browser instance
- **WHEN** a new browser context is created
- **THEN** a listener for CDP `Target.targetCreated` events is registered
- **AND** the listener filters events to only process page-type targets in this context

#### Scenario: Target.targetDestroyed listener registered on context creation
- **GIVEN** a browser instance
- **WHEN** a new browser context is created
- **THEN** a listener for CDP `Target.targetDestroyed` events is registered
- **AND** the listener updates page tracking when pages are closed

#### Scenario: Listeners cleaned up on context close
- **GIVEN** a browser context with registered CDP listeners
- **WHEN** `context.close().await` is called
- **THEN** the CDP event listeners are cleaned up
- **AND** no memory leaks occur from orphaned listeners

### Requirement: Automatic Page Initialization

The system SHALL automatically initialize pages detected via CDP target events.

#### Scenario: New page target triggers full page initialization
- **GIVEN** a browser context with CDP listeners
- **WHEN** a `Target.targetCreated` event is received for a page target in this context
- **THEN** the system attaches to the target via `Target.attachToTarget`
- **AND** enables required CDP domains (Page, Network, Runtime)
- **AND** creates a fully-functional `Page` instance
- **AND** the `Page` can be used for navigation, clicking, typing, etc.

#### Scenario: Opener tracking for popups
- **GIVEN** a browser context with CDP listeners
- **WHEN** a `Target.targetCreated` event is received with an `opener_id`
- **THEN** the created `Page` records the opener information
- **AND** `page.opener()` returns the opener's target ID

#### Scenario: Page tracking survives rapid opens
- **GIVEN** a browser context
- **WHEN** multiple pages are opened in rapid succession (e.g., 5 links clicked quickly)
- **THEN** all pages are tracked in `context.pages()`
- **AND** all 'page' events are emitted
- **AND** no pages are lost due to race conditions
