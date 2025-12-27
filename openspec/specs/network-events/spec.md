# network-events Specification

## Purpose
TBD - created by archiving change add-network-interception. Update Purpose after archive.
## Requirements
### Requirement: Request Events

The system SHALL emit events for network requests.

#### Scenario: Request event on navigation

- **GIVEN** a page with request event listener
- **WHEN** the page navigates to a URL
- **THEN** a request event is emitted for the document request

#### Scenario: Request event for subresources

- **GIVEN** a page with request event listener
- **WHEN** the page loads scripts, styles, and images
- **THEN** request events are emitted for each resource

#### Scenario: Request event contains request details

- **GIVEN** a request event is received
- **WHEN** the request object is inspected
- **THEN** URL, method, headers, and resource type are available

#### Scenario: Request event for XHR

- **GIVEN** a page with request event listener
- **WHEN** JavaScript makes an XMLHttpRequest
- **THEN** a request event is emitted with resource type "xhr"

#### Scenario: Request event for fetch

- **GIVEN** a page with request event listener
- **WHEN** JavaScript makes a fetch request
- **THEN** a request event is emitted with resource type "fetch"

### Requirement: Response Events

The system SHALL emit events for network responses.

#### Scenario: Response event on success

- **GIVEN** a page with response event listener
- **WHEN** a request receives a successful response
- **THEN** a response event is emitted

#### Scenario: Response event for error status

- **GIVEN** a page with response event listener
- **WHEN** a request receives a 404 or 500 response
- **THEN** a response event is still emitted (HTTP errors are still responses)

#### Scenario: Response event contains response details

- **GIVEN** a response event is received
- **WHEN** the response object is inspected
- **THEN** status, headers, and associated request are available

#### Scenario: Response body access

- **GIVEN** a response event is received
- **WHEN** `response.body().await` is called
- **THEN** the response body bytes are returned

#### Scenario: Response JSON access

- **GIVEN** a response event for a JSON response
- **WHEN** `response.json::<T>().await` is called
- **THEN** the parsed JSON is returned

#### Scenario: Response text access

- **GIVEN** a response event
- **WHEN** `response.text().await` is called
- **THEN** the response body as text is returned

### Requirement: Request Finished Events

The system SHALL emit events when requests complete.

#### Scenario: Request finished after response

- **GIVEN** a page with requestfinished event listener
- **WHEN** a response body is fully downloaded
- **THEN** a requestfinished event is emitted

#### Scenario: Request finished contains timing

- **GIVEN** a requestfinished event is received
- **WHEN** `request.timing()` is called
- **THEN** detailed timing information is available

#### Scenario: Request finished contains sizes

- **GIVEN** a requestfinished event is received
- **WHEN** `request.sizes().await` is called
- **THEN** request and response sizes are available

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

### Requirement: Wait For Request

The system SHALL allow waiting for specific requests.

#### Scenario: Wait for request by URL glob

- **GIVEN** a page instance
- **WHEN** `page.wait_for_request("**/api/data").await` is called in parallel with navigation
- **THEN** the method resolves when a matching request is made

#### Scenario: Wait for request by regex

- **GIVEN** a page instance
- **WHEN** `page.wait_for_request(Regex::new(r"/api/\d+")?)` is called
- **THEN** the method resolves when a matching request is made

#### Scenario: Wait for request by predicate

- **GIVEN** a page instance
- **WHEN** `page.wait_for_request(|req| req.method() == "POST").await` is called
- **THEN** the method resolves when a matching POST request is made

#### Scenario: Wait for request with timeout

- **GIVEN** a page instance
- **WHEN** `page.wait_for_request("**/never").timeout(Duration::from_secs(1)).await` is called
- **AND** no matching request occurs
- **THEN** a timeout error is returned after 1 second

### Requirement: Wait For Response

The system SHALL allow waiting for specific responses.

#### Scenario: Wait for response by URL

- **GIVEN** a page instance
- **WHEN** `page.wait_for_response("**/api/data").await` is called in parallel with a triggering action
- **THEN** the method resolves when a matching response is received

#### Scenario: Wait for response by predicate

- **GIVEN** a page instance
- **WHEN** `page.wait_for_response(|res| res.status() == 200).await` is called
- **THEN** the method resolves when a 200 response is received

#### Scenario: Wait for response returns response object

- **GIVEN** `page.wait_for_response(pattern).await` resolves
- **WHEN** the returned response is inspected
- **THEN** full response details including body are accessible

#### Scenario: Wait for response with timeout

- **GIVEN** a page instance
- **WHEN** `page.wait_for_response("**/never").timeout(Duration::from_secs(1)).await` is called
- **AND** no matching response occurs
- **THEN** a timeout error is returned

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

### Requirement: WebSocket Events

The system SHALL emit events for WebSocket connections.

#### Scenario: WebSocket opened event

- **GIVEN** a page with websocket event listener
- **WHEN** JavaScript opens a WebSocket connection
- **THEN** a websocket event is emitted with the WebSocket object

#### Scenario: WebSocket URL access

- **GIVEN** a WebSocket event is received
- **WHEN** `websocket.url()` is called
- **THEN** the WebSocket URL is returned

#### Scenario: WebSocket frame sent event

- **GIVEN** a WebSocket object with framesent listener
- **WHEN** a message is sent through the WebSocket
- **THEN** a framesent event with the payload is emitted

#### Scenario: WebSocket frame received event

- **GIVEN** a WebSocket object with framereceived listener
- **WHEN** a message is received through the WebSocket
- **THEN** a framereceived event with the payload is emitted

#### Scenario: WebSocket close event

- **GIVEN** a WebSocket object with close listener
- **WHEN** the WebSocket connection closes
- **THEN** a close event is emitted

