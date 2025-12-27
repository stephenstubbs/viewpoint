# Network Routing

## ADDED Requirements

### Requirement: Page Route Registration

The system SHALL allow registering route handlers to intercept network requests.

#### Scenario: Route with glob pattern

- **GIVEN** a page instance
- **WHEN** `page.route("**/*.css", handler).await` is called
- **THEN** all CSS requests are intercepted by the handler

#### Scenario: Route with regex pattern

- **GIVEN** a page instance
- **WHEN** `page.route(Regex::new(r"\.png$")?, handler).await` is called
- **THEN** all PNG requests are intercepted by the handler

#### Scenario: Route with predicate function

- **GIVEN** a page instance
- **WHEN** `page.route(|url| url.contains("/api/"), handler).await` is called
- **THEN** requests matching the predicate are intercepted

#### Scenario: Multiple routes with fallback

- **GIVEN** multiple routes registered on a page
- **WHEN** a request matches multiple patterns
- **THEN** handlers are tried in reverse registration order (LIFO)
- **AND** `route.fallback()` passes to the next matching handler

#### Scenario: Unroute specific handler

- **GIVEN** a route registered on a page
- **WHEN** `page.unroute("**/*.css").await` is called
- **THEN** the route handler is removed

#### Scenario: Unroute all handlers

- **GIVEN** multiple routes registered on a page
- **WHEN** `page.unroute_all().await` is called
- **THEN** all route handlers are removed

### Requirement: Context Route Registration

The system SHALL allow registering route handlers at the browser context level.

#### Scenario: Context-level route

- **GIVEN** a browser context
- **WHEN** `context.route("**/api/**", handler).await` is called
- **THEN** matching requests in all pages of the context are intercepted

#### Scenario: Context route applies to new pages

- **GIVEN** a context with a registered route
- **WHEN** a new page is created in the context
- **THEN** the route handler applies to the new page

#### Scenario: Page route takes precedence

- **GIVEN** both context-level and page-level routes for the same pattern
- **WHEN** a matching request is made
- **THEN** the page-level route is tried first

### Requirement: Route Fulfill

The system SHALL allow fulfilling requests with custom responses.

#### Scenario: Fulfill with status and body

- **GIVEN** an intercepted request
- **WHEN** `route.fulfill().status(200).body("OK").send().await` is called
- **THEN** the request completes with the specified status and body

#### Scenario: Fulfill with JSON

- **GIVEN** an intercepted request
- **WHEN** `route.fulfill().json(json!({"data": 123})).send().await` is called
- **THEN** the request completes with JSON body and appropriate content-type

#### Scenario: Fulfill with headers

- **GIVEN** an intercepted request
- **WHEN** `route.fulfill().header("X-Custom", "value").send().await` is called
- **THEN** the response includes the custom header

#### Scenario: Fulfill from file

- **GIVEN** an intercepted request
- **WHEN** `route.fulfill().path("mock/response.json").send().await` is called
- **THEN** the request completes with the file contents

#### Scenario: Fulfill with binary body

- **GIVEN** an intercepted request
- **WHEN** `route.fulfill().body_bytes(image_data).content_type("image/png").send().await` is called
- **THEN** the request completes with the binary data

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
- **THEN** the request fails with "failed" error

#### Scenario: Abort with connection refused

- **GIVEN** an intercepted request
- **WHEN** `route.abort_with(AbortError::ConnectionRefused).await` is called
- **THEN** the request fails with "connectionrefused" error

#### Scenario: Abort with timeout

- **GIVEN** an intercepted request
- **WHEN** `route.abort_with(AbortError::TimedOut).await` is called
- **THEN** the request fails with "timedout" error

#### Scenario: Abort blocks page resources

- **GIVEN** a route that aborts image requests
- **WHEN** the page loads with images
- **THEN** images fail to load without blocking page load

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
