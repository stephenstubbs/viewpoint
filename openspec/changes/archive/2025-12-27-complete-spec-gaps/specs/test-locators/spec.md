## ADDED Requirements

### Requirement: Custom Test ID Attribute

The system SHALL support configuring the test ID attribute name.

#### Scenario: Set custom test ID attribute

- **GIVEN** a browser context
- **WHEN** `context.set_test_id_attribute("data-cy")` is called
- **THEN** subsequent `get_by_test_id()` calls use `data-cy` attribute

#### Scenario: Default test ID attribute

- **GIVEN** a browser context without custom configuration
- **WHEN** `page.get_by_test_id("login")` is called
- **THEN** the selector looks for `[data-testid="login"]`

#### Scenario: Custom test ID in selectors

- **GIVEN** a context with `set_test_id_attribute("data-qa")` called
- **WHEN** `page.get_by_test_id("submit-btn")` is called
- **THEN** the selector looks for `[data-qa="submit-btn"]`
