# Test Assertions

Fluent async assertion API for verifying page and element state.

## ADDED Requirements

### Requirement: Expect Function

The `expect` function SHALL create an assertion builder for the given subject, supporting both locators and pages.

#### Scenario: Expect with locator
- **GIVEN** a locator targeting an element
- **WHEN** `expect(&locator)` is called
- **THEN** a `LocatorAssertions` builder SHALL be returned
- **AND** assertion methods SHALL be available

#### Scenario: Expect with page
- **GIVEN** a page reference
- **WHEN** `expect(&page)` is called
- **THEN** a `PageAssertions` builder SHALL be returned
- **AND** page-level assertion methods SHALL be available

### Requirement: Locator Visibility Assertions

The framework SHALL provide assertions for element visibility state.

#### Scenario: Assert element is visible
- **GIVEN** a locator targeting a visible element
- **WHEN** `expect(&locator).to_be_visible().await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Assert element is hidden
- **GIVEN** a locator targeting a hidden element
- **WHEN** `expect(&locator).to_be_hidden().await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Visibility assertion with timeout
- **GIVEN** a locator targeting an element that becomes visible after 1 second
- **WHEN** `expect(&locator).to_be_visible().await` is called with default timeout
- **THEN** the assertion SHALL wait and pass when the element becomes visible

#### Scenario: Visibility assertion timeout exceeded
- **GIVEN** a locator targeting an element that never becomes visible
- **WHEN** `expect(&locator).to_be_visible().await` is called
- **THEN** the assertion SHALL fail with a timeout error after the default timeout

### Requirement: Locator Text Assertions

The framework SHALL provide assertions for element text content.

#### Scenario: Assert exact text
- **GIVEN** a locator targeting an element with text "Hello World"
- **WHEN** `expect(&locator).to_have_text("Hello World").await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Assert text contains
- **GIVEN** a locator targeting an element with text "Hello World"
- **WHEN** `expect(&locator).to_contain_text("World").await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Text assertion failure
- **GIVEN** a locator targeting an element with text "Hello"
- **WHEN** `expect(&locator).to_have_text("Goodbye").await` is called
- **THEN** the assertion SHALL fail with an error showing expected vs actual text

### Requirement: Locator Attribute Assertions

The framework SHALL provide assertions for element attributes.

#### Scenario: Assert attribute value
- **GIVEN** a locator targeting an element with `data-status="active"`
- **WHEN** `expect(&locator).to_have_attribute("data-status", "active").await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Assert element has class
- **GIVEN** a locator targeting an element with class "btn primary"
- **WHEN** `expect(&locator).to_have_class("primary").await` is called
- **THEN** the assertion SHALL pass

### Requirement: Locator State Assertions

The framework SHALL provide assertions for element interactive state.

#### Scenario: Assert element is enabled
- **GIVEN** a locator targeting an enabled button
- **WHEN** `expect(&locator).to_be_enabled().await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Assert element is disabled
- **GIVEN** a locator targeting a disabled input
- **WHEN** `expect(&locator).to_be_disabled().await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Assert element is checked
- **GIVEN** a locator targeting a checked checkbox
- **WHEN** `expect(&locator).to_be_checked().await` is called
- **THEN** the assertion SHALL pass

### Requirement: Page URL Assertions

The framework SHALL provide assertions for page URL state.

#### Scenario: Assert page URL
- **GIVEN** a page at "https://example.com/path"
- **WHEN** `expect(&page).to_have_url("https://example.com/path").await` is called
- **THEN** the assertion SHALL pass

#### Scenario: Assert page URL contains
- **GIVEN** a page at "https://example.com/path?query=value"
- **WHEN** `expect(&page).to_have_url_containing("/path").await` is called
- **THEN** the assertion SHALL pass

### Requirement: Page Title Assertions

The framework SHALL provide assertions for page title.

#### Scenario: Assert page title
- **GIVEN** a page with title "Example Domain"
- **WHEN** `expect(&page).to_have_title("Example Domain").await` is called
- **THEN** the assertion SHALL pass

### Requirement: Assertion Configuration

Assertions SHALL support configuration for timeout and soft mode.

#### Scenario: Custom assertion timeout
- **GIVEN** an assertion with `.timeout(Duration::from_secs(10))`
- **WHEN** the assertion is awaited
- **THEN** it SHALL wait up to 10 seconds before failing

#### Scenario: Soft assertion
- **GIVEN** an assertion created with `expect.soft(&locator)`
- **WHEN** the assertion fails
- **THEN** it SHALL record the failure but not immediately return an error
- **AND** the test can continue to run
