## MODIFIED Requirements

### Requirement: HTTP Authentication

The system SHALL support automatic HTTP Basic/Digest authentication.

#### Scenario: Create context with HTTP credentials

- **GIVEN** a site requiring HTTP authentication
- **WHEN** `browser.new_context().http_credentials("user", "pass").build().await` is called
- **THEN** the context stores the credentials
- **AND** pages created in the context have access to the credentials

#### Scenario: Authentication without prompt

- **GIVEN** a context with HTTP credentials
- **WHEN** a page navigates to a protected resource
- **THEN** no authentication dialog appears
- **AND** the page loads successfully with HTTP 200

#### Scenario: Credentials sent with requests

- **GIVEN** a context with HTTP credentials
- **WHEN** requests are made to authenticated endpoints
- **THEN** the Authorization header is included after auth challenge

#### Scenario: Auth challenge via CDP Fetch.authRequired

- **GIVEN** a context with HTTP credentials
- **WHEN** the browser receives an auth challenge (401/407)
- **THEN** the `Fetch.authRequired` CDP event is received
- **AND** the system responds with `Fetch.continueWithAuth` containing stored credentials
- **AND** the request succeeds with the authenticated response

#### Scenario: Page enables Fetch with auth handling

- **GIVEN** a context with HTTP credentials
- **WHEN** a new page is created
- **THEN** `Fetch.enable` is called with `handleAuthRequests: true`
- **AND** the page's RouteHandlerRegistry receives auth events

#### Scenario: Auth credentials flow to page network handler

- **GIVEN** a context with HTTP credentials
- **WHEN** a page is created in that context
- **THEN** the page's auth handler is initialized with the context credentials
- **AND** `Fetch.authRequired` events for that page trigger credential response
