## MODIFIED Requirements

### Requirement: Navigation Response

The system SHALL return response information after successful navigation.

#### Scenario: Response contains status code

- **GIVEN** a page navigating to a URL
- **WHEN** the navigation completes successfully
- **THEN** `response.status()` returns the HTTP status code (e.g., 200, 404, 302)

#### Scenario: Response contains headers

- **GIVEN** a page navigating to a URL
- **WHEN** the navigation completes successfully
- **THEN** `response.headers()` returns a map of HTTP response headers

#### Scenario: Response contains URL

- **GIVEN** a page navigating to a URL that redirects
- **WHEN** the navigation completes
- **THEN** `response.url()` returns the final URL after redirects

#### Scenario: Handle redirects

- **GIVEN** a page navigating to a URL that redirects
- **WHEN** the navigation completes
- **THEN** `response.status()` returns the final response status (not redirect status)
- **AND** `response.url()` returns the final URL
