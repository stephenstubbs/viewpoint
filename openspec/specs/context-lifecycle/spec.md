# context-lifecycle Specification

## Purpose
TBD - created by archiving change add-context-features. Update Purpose after archive.
## Requirements
### Requirement: List Context Pages

The system SHALL provide access to all pages in a browser context.

#### Scenario: Get all pages

- **GIVEN** a browser context with multiple pages open
- **WHEN** `context.pages()` is called
- **THEN** a list of all Page instances in the context is returned

#### Scenario: Empty context

- **GIVEN** a browser context with no pages
- **WHEN** `context.pages()` is called
- **THEN** an empty list is returned

#### Scenario: Pages reflect current state

- **GIVEN** a browser context
- **WHEN** pages are opened and closed
- **THEN** `context.pages()` reflects the current set of open pages

### Requirement: Page Creation Events

The system SHALL emit events when new pages are created in the context.

#### Scenario: New page event

- **GIVEN** a browser context with a page event listener
- **WHEN** `context.new_page().await` is called
- **THEN** the 'page' event is emitted with the new Page instance

#### Scenario: Popup triggers page event

- **GIVEN** a browser context with a page event listener
- **WHEN** a page opens a popup
- **THEN** the 'page' event is emitted with the popup Page instance

#### Scenario: Wait for new page

- **GIVEN** a browser context
- **WHEN** `context.wait_for_page(action).await` is called
- **AND** the action creates a new page
- **THEN** the new Page instance is returned

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

