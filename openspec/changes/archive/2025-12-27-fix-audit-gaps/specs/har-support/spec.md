## MODIFIED Requirements

### Requirement: HAR Recording

The system SHALL support recording network traffic to HAR format.

#### Scenario: Start HAR recording on context

- **GIVEN** a browser context
- **WHEN** `context.record_har("output.har").await` is called
- **THEN** HAR recording is started for all pages in the context

#### Scenario: Start HAR recording with options

- **GIVEN** a browser context
- **WHEN** `context.record_har("output.har").url_filter("**/api/**").await` is called
- **THEN** only API requests are recorded to the HAR

#### Scenario: Record HAR during navigation

- **GIVEN** HAR recording is started on a context
- **WHEN** pages navigate and make requests
- **THEN** all network traffic is captured in HAR format

#### Scenario: Save HAR to file

- **GIVEN** HAR recording is active
- **WHEN** `context.close().await` is called
- **OR** `har_recorder.save("output.har").await` is called
- **THEN** the HAR file is written to disk

#### Scenario: Auto-save HAR on context close

- **GIVEN** HAR recording was started with a path
- **WHEN** the context is closed
- **THEN** the HAR file is automatically saved to the specified path

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
