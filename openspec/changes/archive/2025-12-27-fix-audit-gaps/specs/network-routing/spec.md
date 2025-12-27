## MODIFIED Requirements

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

#### Scenario: Fallback chains through all matching handlers

- **GIVEN** three route handlers registered for "**/api/**"
- **WHEN** the first handler calls `route.fallback().await`
- **THEN** the second handler is invoked
- **AND** if the second handler also calls `route.fallback().await`
- **THEN** the third handler is invoked

#### Scenario: Fallback with no more handlers continues to network

- **GIVEN** a single route handler registered
- **WHEN** the handler calls `route.fallback().await`
- **THEN** the request proceeds to the network

#### Scenario: Unroute specific handler

- **GIVEN** a route registered on a page
- **WHEN** `page.unroute("**/*.css").await` is called
- **THEN** the route handler is removed

#### Scenario: Unroute all handlers

- **GIVEN** multiple routes registered on a page
- **WHEN** `page.unroute_all().await` is called
- **THEN** all route handlers are removed
