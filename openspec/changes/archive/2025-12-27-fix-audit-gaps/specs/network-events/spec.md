## MODIFIED Requirements

### Requirement: Request Object Details

The system SHALL provide comprehensive request information.

#### Scenario: Get all headers

- **GIVEN** a Request object
- **WHEN** `request.all_headers().await` is called
- **THEN** all headers including security-related ones are returned

#### Scenario: Get specific header value

- **GIVEN** a Request object
- **WHEN** `request.header_value("Content-Type").await` is called
- **THEN** the header value is returned (case-insensitive)

#### Scenario: Check if navigation request

- **GIVEN** a Request object
- **WHEN** `request.is_navigation_request()` is called
- **THEN** true is returned for document navigation requests

#### Scenario: Get redirect chain

- **GIVEN** a Request that resulted from redirects
- **WHEN** `request.redirected_from()` is called
- **THEN** the previous request in the chain is returned

#### Scenario: Get redirect target

- **GIVEN** a Request that was redirected
- **WHEN** `request.redirected_to()` is called
- **THEN** the next request in the redirect chain is returned

#### Scenario: Get post data as JSON

- **GIVEN** a POST Request with JSON body
- **WHEN** `request.post_data_json::<T>()` is called
- **THEN** the parsed JSON is returned

### Requirement: Request Failed Events

The system SHALL emit events when requests fail.

#### Scenario: Request failed on network error

- **GIVEN** a page with requestfailed event listener
- **WHEN** a request fails due to network error
- **THEN** a requestfailed event is emitted

#### Scenario: Request failed on abort

- **GIVEN** a page with requestfailed event listener
- **WHEN** a request is aborted by a route handler
- **THEN** a requestfailed event is emitted

#### Scenario: Request failed contains error

- **GIVEN** a requestfailed event is received
- **WHEN** `request.failure()` is called
- **THEN** the error message is returned (e.g., "net::ERR_CONNECTION_REFUSED")

### Requirement: Response Object Details

The system SHALL provide comprehensive response information.

#### Scenario: Check response ok

- **GIVEN** a Response object
- **WHEN** `response.ok()` is called
- **THEN** true is returned for 2xx status codes

#### Scenario: Get security details

- **GIVEN** a Response from HTTPS request
- **WHEN** `response.security_details().await` is called
- **THEN** certificate information is returned including issuer, subject, and validity dates

#### Scenario: Get server address

- **GIVEN** a Response object
- **WHEN** `response.server_addr().await` is called
- **THEN** the server IP address and port are returned

#### Scenario: Get associated request

- **GIVEN** a Response object
- **WHEN** `response.request()` is called
- **THEN** the Request object that triggered this response is returned

#### Scenario: Wait for response finished

- **GIVEN** a Response object
- **WHEN** `response.finished().await` is called
- **THEN** the method waits for the response body to complete
