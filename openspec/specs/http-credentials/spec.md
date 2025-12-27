# http-credentials Specification

## Purpose
TBD - created by archiving change add-context-features. Update Purpose after archive.
## Requirements
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

### Requirement: Geolocation Override

The system SHALL allow mocking browser geolocation.

#### Scenario: Set geolocation

- **GIVEN** a browser context
- **WHEN** `context.set_geolocation(59.95, 30.31667).await` is called
- **THEN** navigator.geolocation returns that location

#### Scenario: Set geolocation with accuracy

- **GIVEN** a browser context
- **WHEN** `context.set_geolocation(59.95, 30.31667).accuracy(100.0).await` is called
- **THEN** the location accuracy is 100 meters

#### Scenario: Clear geolocation (position unavailable)

- **GIVEN** a context with geolocation set
- **WHEN** `context.set_geolocation(None).await` is called
- **THEN** navigator.geolocation returns position unavailable error

#### Scenario: Geolocation in context options

- **GIVEN** geolocation in context creation
- **WHEN** `browser.new_context().geolocation(lat, long).build().await` is called
- **THEN** the context starts with that location

#### Scenario: Geolocation requires permission

- **GIVEN** geolocation is set but permission not granted
- **WHEN** a page requests geolocation
- **THEN** the browser's default permission behavior applies

### Requirement: Extra HTTP Headers

The system SHALL allow setting extra HTTP headers for all requests.

#### Scenario: Set extra headers

- **GIVEN** a browser context
- **WHEN** `context.set_extra_http_headers(headers).await` is called
- **THEN** all subsequent requests include those headers

#### Scenario: Headers apply to all pages

- **GIVEN** extra headers are set on context
- **WHEN** multiple pages make requests
- **THEN** all requests include the extra headers

#### Scenario: Page headers override context headers

- **GIVEN** context has extra header "X-Custom: context"
- **AND** page has extra header "X-Custom: page"
- **WHEN** the page makes a request
- **THEN** the request has "X-Custom: page"

#### Scenario: Extra headers in context options

- **GIVEN** headers in context creation
- **WHEN** `browser.new_context().extra_http_headers(headers).build().await` is called
- **THEN** the context starts with those headers

### Requirement: Offline Mode

The system SHALL allow simulating offline network conditions.

#### Scenario: Go offline

- **GIVEN** a browser context
- **WHEN** `context.set_offline(true).await` is called
- **THEN** all network requests fail as if offline

#### Scenario: Go back online

- **GIVEN** an offline context
- **WHEN** `context.set_offline(false).await` is called
- **THEN** network requests work normally

#### Scenario: Offline applies to all pages

- **GIVEN** a context is set to offline
- **WHEN** any page in the context makes a request
- **THEN** the request fails with network error

#### Scenario: Offline in context options

- **GIVEN** offline mode in context creation
- **WHEN** `browser.new_context().offline(true).build().await` is called
- **THEN** the context starts offline

### Requirement: Default Timeouts

The system SHALL allow configuring default timeouts.

#### Scenario: Set default timeout

- **GIVEN** a browser context
- **WHEN** `context.set_default_timeout(Duration::from_secs(10))` is called
- **THEN** all actions use 10 second timeout by default

#### Scenario: Set default navigation timeout

- **GIVEN** a browser context
- **WHEN** `context.set_default_navigation_timeout(Duration::from_secs(30))` is called
- **THEN** all navigation operations use 30 second timeout

#### Scenario: Page timeout overrides context

- **GIVEN** context has default timeout of 10s
- **AND** page sets default timeout to 5s
- **THEN** the page uses 5s timeout

#### Scenario: Navigation timeout overrides default timeout

- **GIVEN** context has default timeout of 5s
- **AND** context has navigation timeout of 30s
- **WHEN** a navigation occurs
- **THEN** the 30s navigation timeout is used

