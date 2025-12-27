# advanced-assertions Specification

## Purpose
TBD - created by archiving change add-advanced-locators-assertions. Update Purpose after archive.
## Requirements
### Requirement: Count Assertions

The system SHALL provide count-based assertions.

#### Scenario: Assert element count

- **GIVEN** a locator matching elements
- **WHEN** `expect(locator).to_have_count(5).await` is called
- **THEN** the assertion passes if exactly 5 elements match

#### Scenario: Assert minimum count

- **GIVEN** a locator matching elements
- **WHEN** `expect(locator).to_have_count_greater_than(2).await` is called
- **THEN** the assertion passes if more than 2 elements match

### Requirement: Value Assertions

The system SHALL provide value-based assertions.

#### Scenario: Assert select values

- **GIVEN** a multi-select element
- **WHEN** `expect(locator).to_have_values(["a", "b"]).await` is called
- **THEN** the assertion passes if those values are selected

#### Scenario: Assert element ID

- **GIVEN** an element with ID
- **WHEN** `expect(locator).to_have_id("my-id").await` is called
- **THEN** the assertion passes if ID matches

### Requirement: Class Assertions

The system SHALL provide class-based assertions.

#### Scenario: Assert has class

- **GIVEN** an element with classes
- **WHEN** `expect(locator).to_have_class("active").await` is called
- **THEN** the assertion passes if element has that class

#### Scenario: Assert multiple classes

- **GIVEN** an element with classes
- **WHEN** `expect(locator).to_have_class(["btn", "primary"]).await` is called
- **THEN** the assertion passes if element has all classes

### Requirement: Aria Snapshot Assertions

The system SHALL provide accessibility assertions.

#### Scenario: Match aria snapshot

- **GIVEN** a navigation element
- **WHEN** `expect(locator).to_match_aria_snapshot(snapshot).await` is called
- **THEN** the assertion passes if accessibility tree matches

#### Scenario: Snapshot with regex

- **GIVEN** an element with dynamic content
- **WHEN** snapshot contains regex patterns
- **THEN** dynamic parts can be matched flexibly

### Requirement: Text Collection Assertions

The system SHALL provide text collection assertions.

#### Scenario: Assert all texts contain

- **GIVEN** a locator matching multiple elements
- **WHEN** `expect(locator).to_have_texts(["A", "B", "C"]).await` is called
- **THEN** the assertion passes if texts match in order

### Requirement: Negative Assertions

The system SHALL provide negative assertion variants.

#### Scenario: Not to have count

- **GIVEN** a locator
- **WHEN** `expect(locator).not().to_have_count(0).await` is called
- **THEN** the assertion passes if elements exist

#### Scenario: Not to match aria snapshot

- **GIVEN** a locator
- **WHEN** `expect(locator).not().to_match_aria_snapshot(bad_snapshot).await` is called
- **THEN** the assertion passes if snapshot doesn't match

### Requirement: Soft Assertions

The system SHALL support soft assertions.

#### Scenario: Soft assertion continues

- **GIVEN** a soft assertion context
- **WHEN** `expect.soft(locator).to_have_text("wrong").await` fails
- **THEN** the test continues executing

#### Scenario: Soft assertions collected

- **GIVEN** multiple soft assertions fail
- **WHEN** the test completes
- **THEN** all failures are reported together

