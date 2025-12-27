## MODIFIED Requirements

### Requirement: Trace Content

The system SHALL capture comprehensive trace data based on configuration.

#### Scenario: Trace includes actions

- **GIVEN** tracing is started with default options
- **WHEN** page actions are performed (click, fill, navigate)
- **THEN** the trace file contains records of each action

#### Scenario: Trace includes network

- **GIVEN** tracing is started with default options
- **WHEN** network requests are made
- **THEN** the trace file contains network request/response data

#### Scenario: Trace includes DOM snapshots

- **GIVEN** tracing is started with `snapshots(true)`
- **WHEN** actions are performed
- **THEN** the trace file contains DOM snapshots captured via `DOMSnapshot.captureSnapshot`

#### Scenario: Trace includes sources

- **GIVEN** tracing is started with `sources(true)`
- **WHEN** the trace is stopped
- **THEN** the trace file includes source files used by the test

#### Scenario: Trace is viewable

- **GIVEN** a completed trace file
- **WHEN** the trace is opened in a viewer
- **THEN** actions, network, screenshots, and snapshots are viewable
