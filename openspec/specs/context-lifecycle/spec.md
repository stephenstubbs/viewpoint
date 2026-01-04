# context-lifecycle Specification

## Purpose
TBD - created by archiving change add-context-features. Update Purpose after archive.
## Requirements
### Requirement: List Context Pages

The system SHALL provide access to all pages in a browser context by storing and returning actual `Page` objects.

Implementation note: The internal storage changes from `Vec<PageInfo>` to `Vec<Page>` to support returning fully-functional Page instances.

#### Scenario: Get all pages returns Page objects

- **GIVEN** a browser context with multiple pages open
- **WHEN** `context.pages()` is called
- **THEN** a `Vec<Page>` is returned (not `Vec<PageInfo>`)
- **AND** each `Page` object is fully functional (can call `url()`, `click()`, etc.)

#### Scenario: Externally-opened pages included

- **GIVEN** a browser context
- **WHEN** a page is opened via `window.open()`, `target="_blank"`, or Ctrl+Click
- **THEN** the new page appears in `context.pages()`
- **AND** the returned `Page` object is fully functional

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

### Requirement: Context Close Events

The system SHALL emit events when the context is closed.

#### Scenario: Close event on explicit close

- **GIVEN** a browser context with a close event listener
- **WHEN** `context.close().await` is called
- **THEN** the 'close' event is emitted before the context closes

#### Scenario: Close event on browser close

- **GIVEN** a browser context with a close event listener
- **WHEN** the browser is closed
- **THEN** the 'close' event is emitted

### Requirement: Context Close

The system SHALL allow closing a browser context and all its pages.

#### Scenario: Close context

- **GIVEN** a browser context with pages open
- **WHEN** `context.close().await` is called
- **THEN** all pages in the context are closed
- **AND** the context is no longer usable

#### Scenario: Operations after close

- **GIVEN** a closed browser context
- **WHEN** any method is called on the context
- **THEN** an error is returned indicating the context is closed

### Requirement: Browser Context Proxy Configuration

The system SHALL support proxy configuration when creating a browser context.

#### Scenario: Create context with HTTP proxy

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("http://proxy:8080")).build().await` is called
- **THEN** all network traffic from pages in that context routes through the proxy

#### Scenario: Create context with SOCKS5 proxy

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("socks5://proxy:1080")).build().await` is called
- **THEN** all network traffic from pages in that context routes through the SOCKS5 proxy

#### Scenario: Proxy with authentication

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("http://proxy:8080").credentials("user", "pass")).build().await` is called
- **THEN** the proxy connection uses the provided credentials

#### Scenario: Proxy with bypass list

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("http://proxy:8080").bypass("localhost,*.local")).build().await` is called
- **THEN** requests to localhost and *.local domains bypass the proxy

#### Scenario: Proxy configuration via ContextOptions

- **GIVEN** a ContextOptionsBuilder
- **WHEN** `ContextOptionsBuilder::new().proxy(config).build()` is called
- **THEN** the resulting ContextOptions contains the proxy configuration

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

