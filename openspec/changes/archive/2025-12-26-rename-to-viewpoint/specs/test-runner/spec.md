## MODIFIED Requirements

### Requirement: Test Attribute Macro (Optional)

The `#[viewpoint::test]` attribute macro SHALL provide optional convenience by generating TestHarness setup code.

#### Scenario: Basic test with page fixture
- **GIVEN** a test function annotated with `#[viewpoint::test]`
- **AND** the function has a `page: Page` parameter
- **WHEN** the test is executed via `cargo test`
- **THEN** the macro SHALL generate TestHarness setup
- **AND** the page SHALL be extracted and passed to the test body

#### Scenario: Test with multiple fixtures
- **GIVEN** a test function with `page: Page` and `context: BrowserContext` parameters
- **WHEN** the macro expands
- **THEN** it SHALL generate code to extract both from the same harness

#### Scenario: Macro with configuration
- **GIVEN** a test with `#[viewpoint::test(headless = false, timeout = 60000)]`
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::builder().headless(false).timeout(...).build()`

### Requirement: Macro Fixture Scoping

The `#[viewpoint::test]` macro SHALL support fixture scoping via the `scope` attribute parameter.

#### Scenario: Test-scoped fixtures (default)
- **GIVEN** a test with `#[viewpoint::test]` and no scope parameter
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::new().await?`
- **AND** each test SHALL get a fresh browser, context, and page

#### Scenario: Module-scoped browser
- **GIVEN** a test with `#[viewpoint::test(scope = "browser", browser = "shared_browser")]`
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::from_browser(shared_browser().await).await?`
- **AND** the test SHALL reuse the browser from the specified function
- **AND** the test SHALL get a fresh context and page

#### Scenario: Shared context
- **GIVEN** a test with `#[viewpoint::test(scope = "context", context = "shared_context")]`
- **WHEN** the macro expands
- **THEN** it SHALL generate `TestHarness::from_context(shared_context().await).await?`
- **AND** the test SHALL reuse the context from the specified function
- **AND** the test SHALL get a fresh page

#### Scenario: Missing source function error
- **GIVEN** a test with `#[viewpoint::test(scope = "browser")]` but no `browser` parameter
- **WHEN** the macro is compiled
- **THEN** a compile error SHALL be emitted indicating the missing source function
