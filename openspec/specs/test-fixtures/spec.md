# test-fixtures Specification

## Purpose
TBD - created by archiving change add-test-framework. Update Purpose after archive.
## Requirements
### Requirement: Fixture Scoping

The TestHarness SHALL support different scoping levels to balance test isolation against performance.

#### Scenario: Test-scoped fixtures (default)
- **GIVEN** `TestHarness::new().await` is called
- **WHEN** the harness initializes
- **THEN** a new browser SHALL be launched
- **AND** a new context SHALL be created
- **AND** a new page SHALL be created
- **AND** all resources SHALL be owned by this harness

#### Scenario: Module-scoped browser
- **GIVEN** an existing Browser instance
- **WHEN** `TestHarness::from_browser(&browser).await` is called
- **THEN** the existing browser SHALL be reused
- **AND** a new context SHALL be created
- **AND** a new page SHALL be created
- **AND** only context and page SHALL be owned by this harness

#### Scenario: Shared context
- **GIVEN** an existing BrowserContext instance
- **WHEN** `TestHarness::from_context(&context).await` is called
- **THEN** the existing context SHALL be reused
- **AND** a new page SHALL be created
- **AND** only the page SHALL be owned by this harness

#### Scenario: Cleanup respects ownership
- **GIVEN** a TestHarness created with `from_browser()`
- **WHEN** the harness is dropped
- **THEN** the page SHALL be closed
- **AND** the context SHALL be closed
- **AND** the browser SHALL NOT be closed (not owned)

### Requirement: Page Access

The TestHarness SHALL provide access to a Page for browser automation.

#### Scenario: Get page from harness
- **GIVEN** a TestHarness that has been created
- **WHEN** `.page()` is called
- **THEN** a reference to the Page SHALL be returned
- **AND** the page SHALL be ready for navigation

#### Scenario: Page is reusable
- **GIVEN** a TestHarness with an active page
- **WHEN** `.page()` is called multiple times
- **THEN** the same Page instance SHALL be returned each time

### Requirement: Context Access

The TestHarness SHALL provide access to the BrowserContext for context-level operations.

#### Scenario: Get context from harness
- **GIVEN** a TestHarness that has been created
- **WHEN** `.context()` is called
- **THEN** a reference to the BrowserContext SHALL be returned

#### Scenario: Context owns the page
- **GIVEN** a TestHarness
- **WHEN** both `.context()` and `.page()` are called
- **THEN** the page SHALL belong to that context

### Requirement: Browser Access

The TestHarness SHALL provide access to the Browser for browser-level operations.

#### Scenario: Get browser from harness
- **GIVEN** a TestHarness that has been created
- **WHEN** `.browser()` is called
- **THEN** a reference to the Browser SHALL be returned

#### Scenario: Browser owns the context
- **GIVEN** a TestHarness
- **WHEN** `.browser()` and `.context()` are called
- **THEN** the context SHALL belong to that browser

### Requirement: Create Additional Pages

The TestHarness SHALL allow creating additional pages for multi-tab testing.

#### Scenario: Create new page
- **GIVEN** a TestHarness with an existing page
- **WHEN** `.new_page().await` is called
- **THEN** a new Page SHALL be created in the same context
- **AND** the new page SHALL be independent of the original

#### Scenario: Multiple pages cleanup
- **GIVEN** a TestHarness with multiple pages created
- **WHEN** the harness is dropped
- **THEN** all pages SHALL be closed

### Requirement: Fixture Lifecycle

The TestHarness SHALL manage fixture lifecycle with deterministic setup and teardown order.

#### Scenario: Setup order
- **GIVEN** `TestHarness::new().await` is called
- **WHEN** the harness initializes
- **THEN** browser SHALL be launched first
- **AND** context SHALL be created second
- **AND** page SHALL be created third

#### Scenario: Teardown order
- **GIVEN** a TestHarness being dropped
- **WHEN** Drop runs
- **THEN** resources SHALL be scheduled for cleanup in reverse order
- **AND** cleanup SHALL be best-effort (log errors, don't panic)

#### Scenario: Async cleanup
- **GIVEN** a TestHarness in an async context
- **WHEN** `.close().await` is called explicitly
- **THEN** all owned resources SHALL be closed gracefully
- **AND** the method SHALL return any cleanup errors
- **AND** non-owned resources (shared browser/context) SHALL NOT be closed

