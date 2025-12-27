# video-recording Specification

## Purpose
TBD - created by archiving change add-tracing-debugging. Update Purpose after archive.
## Requirements
### Requirement: Record Video

The system SHALL record video of page activity.

#### Scenario: Enable video recording

- **GIVEN** video recording options
- **WHEN** `browser.new_context().record_video(opts).build().await` is called
- **THEN** video recording is enabled for the context

#### Scenario: Video saved on page close

- **GIVEN** a page with video recording
- **WHEN** the page is closed
- **THEN** the video file is finalized

### Requirement: Video Access

The system SHALL provide access to video files.

#### Scenario: Get video path

- **GIVEN** a page with video recording
- **WHEN** `page.video().path().await` is called
- **THEN** the video file path is returned

#### Scenario: Save video to custom path

- **GIVEN** a page with video recording
- **WHEN** `page.video().save_as("./my-video.webm").await` is called
- **THEN** the video is copied to the path

#### Scenario: Delete video

- **GIVEN** a page with video recording
- **WHEN** `page.video().delete().await` is called
- **THEN** the video file is deleted

### Requirement: Video Options

The system SHALL support video configuration.

#### Scenario: Set video directory

- **GIVEN** video options
- **WHEN** `VideoOptions::new("./videos")` is used
- **THEN** videos are saved to that directory

#### Scenario: Set video size

- **GIVEN** video options
- **WHEN** `VideoOptions::new(path).size(1280, 720)` is used
- **THEN** video is recorded at that resolution

