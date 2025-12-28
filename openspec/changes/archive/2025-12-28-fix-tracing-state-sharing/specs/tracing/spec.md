## MODIFIED Requirements

### Requirement: Start Tracing

The system SHALL allow starting trace recording when at least one page exists in the context.

#### Scenario: Start with defaults

- **GIVEN** a browser context with at least one page
- **WHEN** `context.tracing().start(TracingOptions::new()).await` is called
- **THEN** tracing begins

#### Scenario: Start with screenshots

- **GIVEN** a browser context with at least one page
- **WHEN** `context.tracing().start(TracingOptions::new().screenshots(true)).await` is called
- **THEN** screenshots are captured during tracing

#### Scenario: Start with snapshots

- **GIVEN** a browser context with at least one page
- **WHEN** `context.tracing().start(TracingOptions::new().snapshots(true)).await` is called
- **THEN** DOM snapshots are captured

#### Scenario: Start with sources

- **GIVEN** a browser context with at least one page
- **WHEN** `context.tracing().start(TracingOptions::new().sources(true)).await` is called
- **THEN** source files are included in trace

#### Scenario: Start without pages fails

- **GIVEN** a browser context with no pages
- **WHEN** `context.tracing().start(TracingOptions::new()).await` is called
- **THEN** an error is returned indicating no pages exist

### Requirement: Stop Tracing

The system SHALL allow stopping and saving traces, with state persisted across `tracing()` calls.

#### Scenario: Stop and save

- **GIVEN** tracing is active (started via a previous `tracing()` call)
- **WHEN** `context.tracing().stop("trace.zip").await` is called
- **THEN** the trace is saved to the file

#### Scenario: Stop without saving

- **GIVEN** tracing is active
- **WHEN** `context.tracing().stop_discard().await` is called
- **THEN** tracing stops without saving

#### Scenario: Stop without start fails

- **GIVEN** tracing has not been started
- **WHEN** `context.tracing().stop("trace.zip").await` is called
- **THEN** an error is returned indicating tracing is not active

## ADDED Requirements

### Requirement: Tracing State Persistence

The system SHALL persist tracing state across multiple `context.tracing()` calls within the same context.

#### Scenario: State shared between tracing() calls

- **GIVEN** a browser context with a page
- **WHEN** `context.tracing().start(opts).await` succeeds
- **AND** `context.tracing().stop("trace.zip").await` is called (separate `tracing()` call)
- **THEN** the stop succeeds because state is shared

#### Scenario: Recording state accessible from any tracing() call

- **GIVEN** tracing has been started via `context.tracing().start()`
- **WHEN** `context.tracing().is_recording().await` is called
- **THEN** it returns `true`
