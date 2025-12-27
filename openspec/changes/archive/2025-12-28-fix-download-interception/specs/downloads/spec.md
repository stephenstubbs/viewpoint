## MODIFIED Requirements

### Requirement: Download Events

The system SHALL emit events for file downloads by subscribing to and processing `Browser.downloadWillBegin` and `Browser.downloadProgress` CDP events.

#### Scenario: Download event on link click

- **GIVEN** a page with a download link
- **WHEN** the link is clicked
- **THEN** a download event is emitted

#### Scenario: Download event contains download object

- **GIVEN** a download event
- **WHEN** the event is received
- **THEN** it contains a Download object with correct url and suggested_filename

#### Scenario: Download progress updates state

- **GIVEN** a download in progress
- **WHEN** `Browser.downloadProgress` event is received with state "completed"
- **THEN** the Download object state is updated to Completed
