## MODIFIED Requirements

### Requirement: HTTP Authentication

The system SHALL support automatic HTTP Basic/Digest authentication.

#### Scenario: Create context with HTTP credentials

- **GIVEN** a site requiring HTTP authentication
- **WHEN** `browser.new_context().http_credentials("user", "pass").build().await` is called
- **THEN** the context automatically authenticates requests

#### Scenario: Authentication without prompt

- **GIVEN** a context with HTTP credentials
- **WHEN** a page navigates to a protected resource
- **THEN** no authentication dialog appears
- **AND** the page loads successfully

#### Scenario: Credentials sent with requests

- **GIVEN** a context with HTTP credentials
- **WHEN** requests are made to authenticated endpoints
- **THEN** the Authorization header is included

#### Scenario: Basic authentication scheme

- **GIVEN** a context with HTTP credentials
- **WHEN** a server responds with `WWW-Authenticate: Basic realm="..."`
- **THEN** the system provides credentials using Base64-encoded `user:pass` format

#### Scenario: Digest authentication scheme

- **GIVEN** a context with HTTP credentials
- **WHEN** a server responds with `WWW-Authenticate: Digest ...`
- **THEN** the system provides credentials using the Digest authentication protocol

#### Scenario: Auth challenge via CDP Fetch.authRequired

- **GIVEN** a context with HTTP credentials and network interception enabled
- **WHEN** the browser receives an auth challenge (401/407)
- **THEN** the system handles the `Fetch.authRequired` CDP event
- **AND** responds with `Fetch.continueWithAuth` containing stored credentials
