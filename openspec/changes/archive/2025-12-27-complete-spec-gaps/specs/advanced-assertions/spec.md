## ADDED Requirements

### Requirement: Count Comparison Assertions

The system SHALL provide assertions for comparing element counts.

#### Scenario: Assert element count greater than

- **GIVEN** a locator matching multiple elements
- **WHEN** `expect(&locator).to_have_count_greater_than(2).await` is called
- **THEN** the assertion passes if more than 2 elements match
- **AND** fails with a descriptive error if 2 or fewer match

#### Scenario: Assert element count less than

- **GIVEN** a locator matching multiple elements
- **WHEN** `expect(&locator).to_have_count_less_than(5).await` is called
- **THEN** the assertion passes if fewer than 5 elements match

#### Scenario: Assert element count at least

- **GIVEN** a locator matching multiple elements
- **WHEN** `expect(&locator).to_have_count_at_least(3).await` is called
- **THEN** the assertion passes if 3 or more elements match

#### Scenario: Assert element count at most

- **GIVEN** a locator matching multiple elements
- **WHEN** `expect(&locator).to_have_count_at_most(10).await` is called
- **THEN** the assertion passes if 10 or fewer elements match

### Requirement: Aria Snapshot Assertions

The system SHALL provide assertions for ARIA accessibility tree structure.

#### Scenario: Match aria snapshot

- **GIVEN** an element with an accessibility tree
- **WHEN** `expect(&locator).to_match_aria_snapshot(expected).await` is called
- **THEN** the assertion passes if the ARIA snapshot matches the expected structure

#### Scenario: Snapshot with regex pattern

- **GIVEN** an expected snapshot containing `/pattern/` regex markers
- **WHEN** `expect(&locator).to_match_aria_snapshot(expected).await` is called
- **THEN** regex patterns in the expected snapshot match against actual values

#### Scenario: Snapshot mismatch shows diff

- **GIVEN** an ARIA snapshot that does not match
- **WHEN** the assertion fails
- **THEN** the error message includes a meaningful diff showing the difference
