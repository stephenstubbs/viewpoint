# network-routing Specification

## Purpose
TBD - created by archiving change add-network-interception. Update Purpose after archive.
## Requirements
### Requirement: Page Route Registration

The system SHALL allow registering route handlers to intercept network requests.

#### Scenario: Route with glob pattern

- **GIVEN** a page instance
- **WHEN** `page.route("**/*.css", handler).await` is called
- **THEN** `Fetch.enable` is called if not already enabled
- **AND** the handler is registered in the RouteHandlerRegistry
- **AND** all CSS requests trigger the handler

#### Scenario: Route handler receives requests

- **GIVEN** a page with a route handler registered
- **WHEN** a matching request is made (navigation, fetch, XHR, etc.)
- **THEN** `Fetch.requestPaused` CDP event is received
- **AND** the user handler closure is invoked with a Route object
- **AND** the handler can fulfill, continue, or abort the request

#### Scenario: Multiple routes with fallback

- **GIVEN** multiple routes registered on a page
- **WHEN** a request matches multiple patterns
- **THEN** handlers are tried in reverse registration order (LIFO)
- **AND** `route.fallback()` passes to the next matching handler

### Requirement: Context Route Registration

The system SHALL allow registering route handlers at the browser context level.

#### Scenario: Context-level route applies to pages

- **GIVEN** a browser context with a registered route
- **WHEN** a new page is created in the context
- **THEN** the page's RouteHandlerRegistry includes context routes
- **AND** matching requests in the page are intercepted

#### Scenario: Context route handler invoked

- **GIVEN** a context with route handler and a page in that context
- **WHEN** a matching request is made from the page
- **THEN** the context-level handler is invoked
- **AND** the handler can fulfill, continue, or abort

### Requirement: Route Fulfill

The system SHALL allow fulfilling requests with custom responses.

#### Scenario: Fulfill with status and body

- **GIVEN** an intercepted request
- **WHEN** `route.fulfill().status(200).body("OK").send().await` is called
- **THEN** `Fetch.fulfillRequest` CDP command is sent
- **AND** the request completes with the specified status and body

#### Scenario: Fulfill sends correct CDP parameters

- **GIVEN** an intercepted request with route.fulfill() called
- **WHEN** the fulfill builder sends the response
- **THEN** `Fetch.fulfillRequest` includes `requestId` from the paused request
- **AND** `responseCode` matches the specified status
- **AND** `body` is base64 encoded if binary

### Requirement: Route Continue

The system SHALL allow continuing requests with optional modifications.

#### Scenario: Continue unmodified

- **GIVEN** an intercepted request
- **WHEN** `route.continue_().await` is called
- **THEN** the request proceeds to the network unmodified

#### Scenario: Continue with modified headers

- **GIVEN** an intercepted request
- **WHEN** `route.continue_().header("Authorization", "Bearer token").await` is called
- **THEN** the request proceeds with the added/modified header

#### Scenario: Continue with modified URL

- **GIVEN** an intercepted request
- **WHEN** `route.continue_().url("https://other.example.com/api").await` is called
- **THEN** the request is redirected to the new URL

#### Scenario: Continue with modified method

- **GIVEN** an intercepted request
- **WHEN** `route.continue_().method("POST").await` is called
- **THEN** the request proceeds with the changed method

#### Scenario: Continue with modified post data

- **GIVEN** an intercepted POST request
- **WHEN** `route.continue_().post_data(new_body).await` is called
- **THEN** the request proceeds with the new body

### Requirement: Route Abort

The system SHALL allow aborting requests with specific error codes.

#### Scenario: Abort with default error

- **GIVEN** an intercepted request
- **WHEN** `route.abort().await` is called
- **THEN** `Fetch.failRequest` CDP command is sent with error "Failed"

#### Scenario: Abort blocks page resources

- **GIVEN** a route that aborts image requests
- **WHEN** the page loads with images
- **THEN** `Fetch.failRequest` is sent for each image
- **AND** images fail to load without blocking page load

### Requirement: Route Fetch and Modify

The system SHALL allow fetching the actual response and modifying it.

#### Scenario: Fetch and modify response body

- **GIVEN** an intercepted request
- **WHEN** `route.fetch().await` is called
- **THEN** the actual response is fetched
- **AND** `route.fulfill().response(response).body(modified).send().await` can modify it

#### Scenario: Fetch with modified request

- **GIVEN** an intercepted request
- **WHEN** `route.fetch().header("X-Test", "value").await` is called
- **THEN** the request is made with the modified header

#### Scenario: Add data to JSON response

- **GIVEN** an intercepted API request
- **WHEN** the response is fetched and parsed as JSON
- **AND** additional fields are added
- **AND** `route.fulfill().json(modified_json).send().await` is called
- **THEN** the page receives the modified JSON

#### Scenario: FetchedResponse provides response data

- **GIVEN** an intercepted request
- **WHEN** `let response = route.fetch().await` is called
- **THEN** `response.status` returns the HTTP status code
- **AND** `response.headers` returns the response headers map
- **AND** `response.body()` returns the response body bytes

#### Scenario: FetchedResponse text helper

- **GIVEN** a fetched response with text content
- **WHEN** `response.text()` is called
- **THEN** the body is returned as a UTF-8 string

#### Scenario: FetchedResponse JSON helper

- **GIVEN** a fetched response with JSON content
- **WHEN** `response.json::<T>()` is called
- **THEN** the body is parsed as JSON and deserialized to type T

### Requirement: Route Request Access

The system SHALL provide access to the intercepted request details.

#### Scenario: Access request URL

- **GIVEN** an intercepted request
- **WHEN** `route.request().url()` is called
- **THEN** the request URL is returned

#### Scenario: Access request method

- **GIVEN** an intercepted request
- **WHEN** `route.request().method()` is called
- **THEN** the HTTP method is returned

#### Scenario: Access request headers

- **GIVEN** an intercepted request
- **WHEN** `route.request().headers()` is called
- **THEN** a map of request headers is returned

#### Scenario: Access request post data

- **GIVEN** an intercepted POST request
- **WHEN** `route.request().post_data()` is called
- **THEN** the request body is returned

#### Scenario: Access resource type

- **GIVEN** an intercepted request
- **WHEN** `route.request().resource_type()` is called
- **THEN** the resource type (document, script, image, etc.) is returned

