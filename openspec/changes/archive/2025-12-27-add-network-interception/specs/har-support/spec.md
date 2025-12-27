# HAR Support

## ADDED Requirements

### Requirement: HAR Replay

The system SHALL replay network responses from HAR files.

#### Scenario: Route from HAR file

- **GIVEN** a HAR file with recorded responses
- **WHEN** `page.route_from_har("network.har").await` is called
- **THEN** matching requests are fulfilled from the HAR file

#### Scenario: Context-level HAR routing

- **GIVEN** a HAR file with recorded responses
- **WHEN** `context.route_from_har("network.har").await` is called
- **THEN** matching requests in all pages are fulfilled from HAR

#### Scenario: HAR with URL matching

- **GIVEN** a HAR file with multiple entries
- **WHEN** a request URL matches an entry in the HAR
- **THEN** the corresponding response is used

#### Scenario: HAR with unmatched requests

- **GIVEN** a HAR file with recorded responses
- **WHEN** a request does not match any HAR entry
- **AND** no fallback option is set
- **THEN** the request proceeds to the network

#### Scenario: HAR with strict mode

- **GIVEN** a HAR file with recorded responses
- **WHEN** `page.route_from_har("network.har").strict(true).await` is called
- **AND** a request does not match any HAR entry
- **THEN** the request fails with an error

#### Scenario: HAR with URL filter

- **GIVEN** a HAR file with many entries
- **WHEN** `page.route_from_har("network.har").url("**/api/**").await` is called
- **THEN** only API requests are matched against the HAR

### Requirement: HAR Update Mode

The system SHALL support updating HAR files with new responses.

#### Scenario: Update HAR with missing entries

- **GIVEN** a HAR file with some recorded responses
- **WHEN** `page.route_from_har("network.har").update(true).await` is called
- **AND** a request does not match any HAR entry
- **THEN** the request proceeds to the network
- **AND** the response is added to the HAR file

#### Scenario: Update preserves existing entries

- **GIVEN** a HAR file with recorded responses
- **WHEN** update mode is enabled
- **AND** a request matches an existing HAR entry
- **THEN** the existing entry is used (not replaced)

#### Scenario: HAR file created if missing

- **GIVEN** a non-existent HAR file path
- **WHEN** `page.route_from_har("new.har").update(true).await` is called
- **AND** requests are made
- **THEN** a new HAR file is created with the responses

### Requirement: HAR Content Matching

The system SHALL match requests based on multiple criteria.

#### Scenario: Match by URL and method

- **GIVEN** a HAR file with GET and POST entries for the same URL
- **WHEN** a POST request is made
- **THEN** the POST entry is matched

#### Scenario: Match by post data

- **GIVEN** a HAR file with entries having different post data
- **WHEN** `page.route_from_har("api.har").match_post_data(true).await` is called
- **THEN** requests are matched by their post data content

#### Scenario: Match by headers

- **GIVEN** a HAR file with entries having specific headers
- **WHEN** `page.route_from_har("api.har").match_headers(["Accept"]).await` is called  
- **THEN** the Accept header is considered in matching

### Requirement: HAR Response Timing

The system SHALL optionally preserve response timing from HAR.

#### Scenario: Immediate response (default)

- **GIVEN** a HAR file with recorded timing
- **WHEN** `page.route_from_har("network.har").await` is called
- **THEN** responses are returned immediately (timing ignored)

#### Scenario: Preserve timing

- **GIVEN** a HAR file with recorded timing
- **WHEN** `page.route_from_har("network.har").timing(HarTiming::Preserved).await` is called
- **THEN** responses are delayed to match recorded timing

### Requirement: HAR Recording

The system SHALL support recording network traffic to HAR format.

#### Scenario: Record HAR during navigation

- **GIVEN** HAR recording is started on a context
- **WHEN** pages navigate and make requests
- **THEN** all network traffic is captured in HAR format

#### Scenario: Save HAR to file

- **GIVEN** HAR recording is active
- **WHEN** `context.close().await` is called
- **OR** `har_recorder.save("output.har").await` is called
- **THEN** the HAR file is written to disk

#### Scenario: HAR includes request details

- **GIVEN** a recorded HAR file
- **WHEN** the HAR is inspected
- **THEN** each entry contains URL, method, headers, and body

#### Scenario: HAR includes response details

- **GIVEN** a recorded HAR file
- **WHEN** the HAR is inspected
- **THEN** each entry contains status, headers, body, and timing

#### Scenario: HAR includes timing information

- **GIVEN** a recorded HAR file
- **WHEN** the HAR is inspected
- **THEN** timing breakdown (DNS, connect, send, wait, receive) is present

### Requirement: HAR Content Options

The system SHALL support options for HAR content handling.

#### Scenario: Omit response content

- **GIVEN** HAR recording with content option
- **WHEN** `har_recorder.omit_content(true)` is configured
- **THEN** response bodies are not included in the HAR

#### Scenario: Include only specific content types

- **GIVEN** HAR recording with content filter
- **WHEN** `har_recorder.content_types(["application/json"]).await` is configured
- **THEN** only JSON response bodies are included

#### Scenario: Limit response body size

- **GIVEN** HAR recording with size limit
- **WHEN** `har_recorder.max_body_size(1024 * 1024).await` is configured
- **THEN** response bodies larger than 1MB are truncated
