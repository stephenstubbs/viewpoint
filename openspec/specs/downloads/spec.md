# downloads Specification

## Purpose
TBD - created by archiving change add-dialog-file-handling. Update Purpose after archive.
## Requirements
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

### Requirement: Download Properties

The system SHALL expose download properties.

#### Scenario: Get suggested filename

- **GIVEN** a Download object
- **WHEN** `download.suggested_filename()` is called
- **THEN** the browser-suggested filename is returned

#### Scenario: Get download URL

- **GIVEN** a Download object
- **WHEN** `download.url()` is called
- **THEN** the download URL is returned

### Requirement: Download Path

The system SHALL provide access to downloaded file path.

#### Scenario: Get download path

- **GIVEN** a completed download
- **WHEN** `download.path().await` is called
- **THEN** the path to the downloaded file is returned

#### Scenario: Path waits for completion

- **GIVEN** a download in progress
- **WHEN** `download.path().await` is called
- **THEN** the method waits for download to complete

### Requirement: Save Download

The system SHALL allow saving downloads to custom locations.

#### Scenario: Save to custom path

- **GIVEN** a Download object
- **WHEN** `download.save_as("./my-file.pdf").await` is called
- **THEN** the file is copied to the specified path

#### Scenario: Save during download

- **GIVEN** a download in progress
- **WHEN** `download.save_as(path).await` is called
- **THEN** the method waits and saves when complete

### Requirement: Cancel Download

The system SHALL allow canceling downloads.

#### Scenario: Cancel in-progress download

- **GIVEN** a download in progress
- **WHEN** `download.cancel().await` is called
- **THEN** the download is cancelled

#### Scenario: Failure after cancel

- **GIVEN** a cancelled download
- **WHEN** `download.failure().await` is called
- **THEN** "cancelled" is returned

### Requirement: Download Failure

The system SHALL report download failures.

#### Scenario: Get failure reason

- **GIVEN** a failed download
- **WHEN** `download.failure().await` is called
- **THEN** the error message is returned

#### Scenario: Successful download has no failure

- **GIVEN** a successful download
- **WHEN** `download.failure().await` is called
- **THEN** None is returned

### Requirement: Wait For Download

The system SHALL provide convenient download waiting.

#### Scenario: Wait for download with action

- **GIVEN** a page with a download link
- **WHEN** `page.wait_for_download(|| click_action).await` is called
- **THEN** the download is returned after the action triggers it

