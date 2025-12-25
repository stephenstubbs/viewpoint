# test-runner Specification

## Purpose
TBD - created by archiving change add-test-framework. Update Purpose after archive.
## Requirements
### Requirement: TestHarness Primary API

The `TestHarness` struct SHALL provide explicit test setup with browser, context, and page access, using Drop for automatic cleanup.

#### Scenario: Basic test with harness
- **GIVEN** a test function using `#[tokio::test]`
- **WHEN** `TestHarness::new().await?` is called
- **THEN** a browser SHALL be launched
- **AND** a context SHALL be created
- **AND** a page SHALL be created
- **AND** the harness SHALL provide access via `.page()`, `.context()`, `.browser()`

#### Scenario: Harness cleanup on success
- **GIVEN** a test using TestHarness
- **WHEN** the test completes successfully and harness is dropped
- **THEN** the page SHALL be closed
- **AND** the context SHALL be closed
- **AND** the browser SHALL be closed

#### Scenario: Harness cleanup on error
- **GIVEN** a test using TestHarness that returns an error
- **WHEN** the harness is dropped
- **THEN** all resources SHALL still be cleaned up

#### Scenario: Harness cleanup on panic
- **GIVEN** a test using TestHarness that panics
- **WHEN** the panic unwinds
- **THEN** all resources SHALL still be cleaned up via Drop

### Requirement: TestHarness Configuration

The `TestHarness` SHALL support a builder pattern for configuration options.

#### Scenario: Headless mode configuration
- **GIVEN** a harness created with `.headless(false)`
- **WHEN** the browser launches
- **THEN** the browser SHALL run in headed mode

#### Scenario: Timeout configuration
- **GIVEN** a harness created with `.timeout(Duration::from_secs(60))`
- **WHEN** operations are performed
- **THEN** the configured timeout SHALL be used as default

#### Scenario: Default configuration
- **GIVEN** a harness created with `TestHarness::new()`
- **WHEN** the browser launches
- **THEN** the browser SHALL run in headless mode
- **AND** the default timeout of 30 seconds SHALL apply

### Requirement: Test Attribute Macro (Optional)

The `#[rustright::test]` attribute macro SHALL provide optional convenience by generating TestHarness setup code.

#### Scenario: Basic test with page fixture
- **GIVEN** a test function annotated with `#[rustright::test]`
- **AND** the function has a `page: Page` parameter
- **WHEN** the test is executed via `cargo test`
- **THEN** the macro SHALL generate TestHarness setup
- **AND** the page SHALL be extracted and passed to the test body

#### Scenario: Test with multiple fixtures
- **GIVEN** a test function with `page: Page` and `context: BrowserContext` parameters
- **WHEN** the macro expands
- **THEN** it SHALL generate code to extract both from the same harness

#### Scenario: Macro with configuration
- **GIVEN** a test with `#[rustright::test(headless = false, timeout = 60000)]`
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::builder().headless(false).timeout(...).build()`

### Requirement: Macro Fixture Scoping

The `#[rustright::test]` macro SHALL support fixture scoping via the `scope` attribute parameter.

#### Scenario: Test-scoped fixtures (default)
- **GIVEN** a test with `#[rustright::test]` and no scope parameter
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::new().await?`
- **AND** each test SHALL get a fresh browser, context, and page

#### Scenario: Module-scoped browser
- **GIVEN** a test with `#[rustright::test(scope = "browser", browser = "shared_browser")]`
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::from_browser(shared_browser().await).await?`
- **AND** the test SHALL reuse the browser from the specified function
- **AND** the test SHALL get a fresh context and page

#### Scenario: Shared context
- **GIVEN** a test with `#[rustright::test(scope = "context", context = "shared_context")]`
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::from_context(shared_context().await).await?`
- **AND** the test SHALL reuse the context from the specified function
- **AND** the test SHALL get a fresh page

#### Scenario: Missing source function error
- **GIVEN** a test with `#[rustright::test(scope = "browser")]` but no `browser` parameter
- **WHEN** the macro is compiled
- **THEN** a compile error SHALL be emitted indicating the missing source function

