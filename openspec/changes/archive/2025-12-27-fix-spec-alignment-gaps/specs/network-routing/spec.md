## MODIFIED Requirements

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
