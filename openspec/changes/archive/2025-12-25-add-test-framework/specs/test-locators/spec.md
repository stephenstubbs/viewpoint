# Test Locators

Element selection strategies with auto-waiting capabilities.

## ADDED Requirements

### Requirement: CSS Selector Locator

The framework SHALL support locating elements via CSS selectors.

#### Scenario: Locate by CSS selector
- **GIVEN** a page with an element matching `button.submit`
- **WHEN** `page.locator("button.submit")` is called
- **THEN** a `Locator` targeting that element SHALL be returned

#### Scenario: CSS selector with multiple matches
- **GIVEN** a page with multiple elements matching `.item`
- **WHEN** `page.locator(".item")` is called
- **THEN** a `Locator` targeting all matching elements SHALL be returned
- **AND** actions on this locator SHALL apply to the first match by default

### Requirement: Text Locator

The framework SHALL support locating elements by their text content.

#### Scenario: Locate by exact text
- **GIVEN** a page with a button containing text "Submit"
- **WHEN** `page.get_by_text("Submit")` is called
- **THEN** a `Locator` targeting that button SHALL be returned

#### Scenario: Locate by partial text
- **GIVEN** a page with a paragraph containing "Hello World"
- **WHEN** `page.get_by_text("Hello")` is called with partial match option
- **THEN** a `Locator` targeting that paragraph SHALL be returned

#### Scenario: Text locator case sensitivity
- **GIVEN** a page with text "Submit"
- **WHEN** `page.get_by_text("submit")` is called with case-insensitive option
- **THEN** a `Locator` targeting that element SHALL be returned

### Requirement: Role Locator

The framework SHALL support locating elements by their ARIA role.

#### Scenario: Locate by role
- **GIVEN** a page with a `<button>` element
- **WHEN** `page.get_by_role(AriaRole::Button)` is called
- **THEN** a `Locator` targeting that button SHALL be returned

#### Scenario: Locate by role with name
- **GIVEN** a page with buttons "Cancel" and "Submit"
- **WHEN** `page.get_by_role(AriaRole::Button).with_name("Submit")` is called
- **THEN** a `Locator` targeting only the "Submit" button SHALL be returned

#### Scenario: Locate by role with accessibility name
- **GIVEN** a page with `<button aria-label="Close dialog">X</button>`
- **WHEN** `page.get_by_role(AriaRole::Button).with_name("Close dialog")` is called
- **THEN** a `Locator` targeting that button SHALL be returned

### Requirement: Test ID Locator

The framework SHALL support locating elements by test ID attribute.

#### Scenario: Locate by test ID
- **GIVEN** a page with `<div data-testid="user-profile">`
- **WHEN** `page.get_by_test_id("user-profile")` is called
- **THEN** a `Locator` targeting that div SHALL be returned

#### Scenario: Custom test ID attribute
- **GIVEN** a page with `<div data-test="user-profile">`
- **AND** test ID attribute configured as "data-test"
- **WHEN** `page.get_by_test_id("user-profile")` is called
- **THEN** a `Locator` targeting that div SHALL be returned

### Requirement: Label Locator

The framework SHALL support locating form controls by their associated label.

#### Scenario: Locate input by label
- **GIVEN** a page with `<label for="email">Email</label><input id="email">`
- **WHEN** `page.get_by_label("Email")` is called
- **THEN** a `Locator` targeting the input element SHALL be returned

### Requirement: Placeholder Locator

The framework SHALL support locating inputs by their placeholder text.

#### Scenario: Locate by placeholder
- **GIVEN** a page with `<input placeholder="Enter your email">`
- **WHEN** `page.get_by_placeholder("Enter your email")` is called
- **THEN** a `Locator` targeting that input SHALL be returned

### Requirement: Locator Chaining

Locators SHALL support chaining to narrow down element selection.

#### Scenario: Chain locators
- **GIVEN** a page with nested structure `.list > .item > button`
- **WHEN** `page.locator(".list").locator(".item").locator("button")` is called
- **THEN** a `Locator` targeting buttons within items within list SHALL be returned

#### Scenario: Filter locator results
- **GIVEN** a locator matching multiple elements
- **WHEN** `.filter(|l| l.has_text("Active"))` is called
- **THEN** the locator SHALL be narrowed to elements containing "Active"

### Requirement: Locator Nth Selection

Locators SHALL support selecting a specific element by index.

#### Scenario: Select first element
- **GIVEN** a locator matching multiple elements
- **WHEN** `.first()` is called
- **THEN** a `Locator` targeting only the first match SHALL be returned

#### Scenario: Select last element
- **GIVEN** a locator matching multiple elements
- **WHEN** `.last()` is called
- **THEN** a `Locator` targeting only the last match SHALL be returned

#### Scenario: Select nth element
- **GIVEN** a locator matching 5 elements
- **WHEN** `.nth(2)` is called
- **THEN** a `Locator` targeting the third element (0-indexed) SHALL be returned

### Requirement: Locator Auto-Waiting

Locators SHALL automatically wait for elements before performing actions.

#### Scenario: Wait for element to appear
- **GIVEN** a locator targeting an element that appears after 1 second
- **WHEN** an action is performed on the locator
- **THEN** the action SHALL wait for the element to appear
- **AND** then perform the action

#### Scenario: Wait timeout exceeded
- **GIVEN** a locator targeting a non-existent element
- **WHEN** an action is performed with default timeout
- **THEN** the action SHALL fail with a timeout error after 30 seconds

#### Scenario: Custom wait timeout
- **GIVEN** a locator with `.timeout(Duration::from_secs(5))`
- **WHEN** targeting a non-existent element
- **THEN** the action SHALL fail after 5 seconds
