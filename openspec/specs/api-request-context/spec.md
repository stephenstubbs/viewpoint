# api-request-context Specification

## Purpose
TBD - created by archiving change add-api-testing. Update Purpose after archive.
## Requirements
### Requirement: Request Context Creation

The system SHALL provide API request contexts.

#### Scenario: Get context from browser context

- **GIVEN** a browser context
- **WHEN** `context.request()` is called
- **THEN** an APIRequestContext sharing cookies is returned

#### Scenario: Create standalone context

- **GIVEN** the playwright instance
- **WHEN** `playwright.request.new_context(opts).await` is called
- **THEN** an independent APIRequestContext is returned

#### Scenario: Set base URL

- **GIVEN** API context options
- **WHEN** `base_url("https://api.example.com")` is set
- **THEN** relative URLs are resolved against the base

### Requirement: HTTP Methods

The system SHALL support all HTTP methods.

#### Scenario: GET request

- **GIVEN** an API context
- **WHEN** `api.get("/users").await` is called
- **THEN** a GET request is made

#### Scenario: POST request

- **GIVEN** an API context
- **WHEN** `api.post("/users").await` is called
- **THEN** a POST request is made

#### Scenario: PUT request

- **GIVEN** an API context
- **WHEN** `api.put("/users/1").await` is called
- **THEN** a PUT request is made

#### Scenario: PATCH request

- **GIVEN** an API context
- **WHEN** `api.patch("/users/1").await` is called
- **THEN** a PATCH request is made

#### Scenario: DELETE request

- **GIVEN** an API context
- **WHEN** `api.delete("/users/1").await` is called
- **THEN** a DELETE request is made

#### Scenario: HEAD request

- **GIVEN** an API context
- **WHEN** `api.head("/users").await` is called
- **THEN** a HEAD request is made

### Requirement: Request Options

The system SHALL support request configuration.

#### Scenario: JSON body

- **GIVEN** a request
- **WHEN** `.json(&data)` is called
- **THEN** the body is JSON with Content-Type header

#### Scenario: Form data

- **GIVEN** a request
- **WHEN** `.form(&data)` is called
- **THEN** the body is form-urlencoded

#### Scenario: Custom headers

- **GIVEN** a request
- **WHEN** `.header("X-Custom", "value")` is called
- **THEN** the header is included

#### Scenario: Query parameters

- **GIVEN** a request
- **WHEN** `.query(&[("key", "value")])` is called
- **THEN** query params are appended to URL

#### Scenario: Timeout

- **GIVEN** a request
- **WHEN** `.timeout(Duration::from_secs(30))` is called
- **THEN** the request times out after 30 seconds

### Requirement: Response Handling

The system SHALL provide response access.

#### Scenario: Status code

- **GIVEN** a response
- **WHEN** `response.status()` is called
- **THEN** the HTTP status code is returned

#### Scenario: Check ok

- **GIVEN** a response
- **WHEN** `response.ok()` is called
- **THEN** true for 2xx, false otherwise

#### Scenario: JSON response

- **GIVEN** a JSON response
- **WHEN** `response.json::<T>().await` is called
- **THEN** the parsed JSON is returned

#### Scenario: Text response

- **GIVEN** a response
- **WHEN** `response.text().await` is called
- **THEN** the body as text is returned

#### Scenario: Response headers

- **GIVEN** a response
- **WHEN** `response.headers()` is called
- **THEN** the response headers are returned

### Requirement: Cookie Sharing

The system SHALL share cookies with browser context.

#### Scenario: Browser cookies in API request

- **GIVEN** browser context with cookies
- **AND** API context from that browser context
- **WHEN** an API request is made
- **THEN** browser cookies are sent

#### Scenario: API cookies in browser

- **GIVEN** API context from browser context
- **WHEN** API response sets cookies
- **THEN** cookies are available in browser

### Requirement: Context Disposal

The system SHALL support disposing API contexts.

#### Scenario: Dispose context

- **GIVEN** a standalone API context
- **WHEN** `api.dispose().await` is called
- **THEN** resources are cleaned up

#### Scenario: Context from browser auto-disposes

- **GIVEN** API context from browser context
- **WHEN** browser context closes
- **THEN** API context is disposed

