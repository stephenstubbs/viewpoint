## MODIFIED Requirements

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
