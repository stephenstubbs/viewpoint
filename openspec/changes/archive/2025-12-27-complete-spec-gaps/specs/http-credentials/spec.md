## MODIFIED Requirements

### Requirement: HTTP Authentication

The system SHALL automatically handle HTTP authentication challenges using stored credentials.

#### Scenario: Create context with HTTP credentials

- **GIVEN** credentials configured via `http_credentials("user", "pass")`
- **WHEN** the context is created
- **THEN** credentials are stored for use with authentication challenges

#### Scenario: Authentication without prompt

- **GIVEN** a context with HTTP credentials configured
- **WHEN** a page navigates to a URL requiring HTTP Basic authentication
- **THEN** credentials are automatically provided via `Fetch.authRequired` handling
- **AND** no browser authentication dialog is shown
- **AND** the page loads successfully

#### Scenario: Basic authentication

- **GIVEN** a context with HTTP credentials
- **WHEN** the server responds with `WWW-Authenticate: Basic realm="..."` 
- **THEN** credentials are provided using Basic authentication scheme

#### Scenario: Digest authentication

- **GIVEN** a context with HTTP credentials
- **WHEN** the server responds with `WWW-Authenticate: Digest ...`
- **THEN** credentials are provided using Digest authentication scheme
