# navigation Specification

## Purpose
TBD - created by archiving change add-chromium-connection. Update Purpose after archive.
## Requirements
### Requirement: Page Navigation

The system SHALL navigate pages to URLs with Playwright-compatible behavior.

Navigation SHALL:
- Use CDP `Page.navigate` command
- Wait for the specified load state before returning
- Return navigation response information (status, headers)
- Support configurable timeout and referer

#### Scenario: Navigate to URL with default wait
- **GIVEN** a page instance
- **WHEN** `page.goto("https://example.com").goto().await` is called
- **THEN** the page navigates to the URL AND waits for `load` event AND returns the response

#### Scenario: Navigate with custom wait state
- **GIVEN** a page instance
- **WHEN** `page.goto("https://example.com").wait_until(DocumentLoadState::DomContentLoaded).goto().await` is called
- **THEN** the page navigates AND waits only for `DomContentLoaded` event

#### Scenario: Navigate with referer
- **GIVEN** a page instance
- **WHEN** `page.goto("https://example.com").referer("https://google.com").goto().await` is called
- **THEN** the navigation request includes the specified Referer header

#### Scenario: Navigate with timeout
- **GIVEN** a page instance
- **WHEN** `page.goto("https://slow-site.com").timeout(Duration::from_secs(5)).goto().await` is called
- **AND** the page does not load within 5 seconds
- **THEN** a `NavigationError::Timeout` error is returned

### Requirement: Navigation Response

The system SHALL capture and return navigation response information.

#### Scenario: Successful navigation response
- **GIVEN** a page navigates to a URL
- **WHEN** the server responds with HTTP 200
- **THEN** the response contains status code 200, response headers, and URL

#### Scenario: Redirect navigation
- **GIVEN** a page navigates to a URL that redirects
- **WHEN** the navigation completes after following redirects
- **THEN** the response reflects the final URL and status code

#### Scenario: HTTP error response
- **GIVEN** a page navigates to a URL
- **WHEN** the server responds with HTTP 404
- **THEN** the response contains status code 404 (navigation succeeds, not an error)

### Requirement: Navigation Error Handling

The system SHALL handle navigation failures appropriately.

#### Scenario: Network error
- **GIVEN** a page instance
- **WHEN** navigating to an unreachable URL (DNS failure, connection refused)
- **THEN** a `NavigationError::NetworkError` is returned with the error description

#### Scenario: SSL certificate error
- **GIVEN** a page instance (without ignore HTTPS errors enabled)
- **WHEN** navigating to a site with invalid SSL certificate
- **THEN** a `NavigationError::SslError` is returned

#### Scenario: Navigation cancelled
- **GIVEN** a page is navigating to a URL
- **WHEN** the page is closed or another navigation is initiated
- **THEN** the original navigation returns `NavigationError::Cancelled`

### Requirement: Page Creation

The system SHALL create pages within browser contexts.

#### Scenario: Create new page
- **GIVEN** an active browser context
- **WHEN** `context.new_page().await` is called
- **THEN** a new Page is created AND attached via CDP AND ready for navigation

#### Scenario: Multiple pages in context
- **GIVEN** an active browser context
- **WHEN** `context.new_page().await` is called multiple times
- **THEN** each call creates a separate Page instance with its own CDP session

#### Scenario: Close page
- **GIVEN** an active page
- **WHEN** `page.close().await` is called
- **THEN** the page's CDP session is closed AND the browser tab is closed

### Requirement: Builder Pattern API

The system SHALL provide a fluent builder pattern for navigation.

#### Scenario: Chained builder methods
- **GIVEN** a page instance
- **WHEN** building a navigation with multiple options
- **THEN** the builder allows chaining: `page.goto(url).wait_until(state).timeout(dur).referer(ref).goto().await`

#### Scenario: Builder reuse
- **GIVEN** a goto builder instance
- **WHEN** `.goto().await` is called
- **THEN** the builder is consumed and cannot be reused (enforced by Rust ownership)

