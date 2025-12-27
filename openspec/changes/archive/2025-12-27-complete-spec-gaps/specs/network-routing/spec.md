## MODIFIED Requirements

### Requirement: Route Fetch and Modify

The system SHALL allow fetching the actual response and modifying it before delivery.

#### Scenario: Fetch and modify response body

- **GIVEN** a route handler is registered
- **WHEN** `route.fetch().await` is called
- **THEN** the system makes the actual network request
- **AND** returns a `FetchedResponse` with real status, headers, and body

#### Scenario: Fetch with modified request

- **GIVEN** a route handler is registered
- **WHEN** `route.fetch().header("Authorization", "Bearer token").await` is called
- **THEN** the modified header is sent with the request
- **AND** the real response is returned

#### Scenario: Modify fetched response

- **GIVEN** a route handler has fetched the response
- **WHEN** `fetched.json::<Value>().await` is called
- **THEN** the actual response body is returned as JSON
- **AND** the caller can modify and fulfill with new data

### Requirement: Context Route Registration

The system SHALL support route registration at the browser context level.

#### Scenario: Context-level route

- **GIVEN** a browser context
- **WHEN** `context.route("**/api/**", handler).await` is called
- **THEN** the handler receives matching requests from all pages

#### Scenario: Context route applies to new pages

- **GIVEN** a browser context with registered routes
- **WHEN** a new page is created via `context.new_page().await`
- **THEN** the new page inherits the context routes
- **AND** matching requests trigger the context route handlers

#### Scenario: Page route takes precedence

- **GIVEN** a context route for "**/api/**"
- **AND** a page route for "**/api/users"
- **WHEN** a request matches both patterns
- **THEN** the page route handler is called
- **AND** the context route handler is NOT called unless fallback is used
