## MODIFIED Requirements

### Requirement: Select Option Action

The framework SHALL support selecting options in dropdowns.

#### Scenario: Select by value
- **GIVEN** a locator targeting a select element
- **WHEN** `.select_option().value("option1").await` is called
- **THEN** the option with value "option1" SHALL be selected
- **AND** if navigation is triggered, the method SHALL wait for it to complete

#### Scenario: Select by label
- **GIVEN** a locator targeting a select element with `<option>Blue</option>`
- **WHEN** `.select_option().label("Blue").await` is called
- **THEN** the "Blue" option SHALL be selected

#### Scenario: Select multiple options
- **GIVEN** a locator targeting a multi-select element
- **WHEN** `.select_option().values(&["a", "b"]).await` is called
- **THEN** options "a" and "b" SHALL be selected

#### Scenario: Select with no_wait_after
- **GIVEN** a locator targeting a select with onChange navigation
- **WHEN** `.select_option().value("option1").no_wait_after(true).await` is called
- **THEN** the option SHALL be selected
- **AND** the method SHALL return immediately without waiting for navigation

#### Scenario: Select option via ref from aria snapshot
- **GIVEN** an aria snapshot containing a select element with ref `c0p0f0e5`
- **WHEN** `page.locator_from_ref("c0p0f0e5").select_option().value("option1").await` is called
- **THEN** the option with value "option1" SHALL be selected

#### Scenario: Select option via ref by label
- **GIVEN** an aria snapshot containing a select element with ref `c0p0f0e5`
- **AND** the select has an option with text "Blue"
- **WHEN** `page.locator_from_ref("c0p0f0e5").select_option().label("Blue").await` is called
- **THEN** the "Blue" option SHALL be selected

#### Scenario: Select multiple options via ref
- **GIVEN** an aria snapshot containing a multi-select element with ref `c0p0f0e5`
- **WHEN** `page.locator_from_ref("c0p0f0e5").select_option().values(&["a", "b"]).await` is called
- **THEN** options "a" and "b" SHALL be selected
