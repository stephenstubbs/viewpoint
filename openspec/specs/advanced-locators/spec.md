# advanced-locators Specification

## Purpose
TBD - created by archiving change add-advanced-locators-assertions. Update Purpose after archive.
## Requirements
### Requirement: Locator Composition

The system SHALL support combining locators.

#### Scenario: And composition

- **GIVEN** two locators
- **WHEN** `locator1.and(locator2)` is called
- **THEN** a locator matching both conditions is returned

#### Scenario: Or composition

- **GIVEN** two locators
- **WHEN** `locator1.or(locator2)` is called
- **THEN** a locator matching either condition is returned

#### Scenario: Filter by text

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.filter().has_text("specific")` is called
- **THEN** only elements containing that text match

#### Scenario: Filter by has

- **GIVEN** a locator for container elements
- **WHEN** `locator.filter().has(child_locator)` is called
- **THEN** only containers with matching children match

#### Scenario: Filter by has_not

- **GIVEN** a locator for container elements
- **WHEN** `locator.filter().has_not(child_locator)` is called
- **THEN** only containers without matching children match

### Requirement: Additional Locator Methods

The system SHALL provide additional ways to locate elements.

#### Scenario: Get by alt text

- **GIVEN** a page with images
- **WHEN** `page.get_by_alt_text("Logo")` is called
- **THEN** images with that alt text are matched

#### Scenario: Get by title

- **GIVEN** a page with titled elements
- **WHEN** `page.get_by_title("Help")` is called
- **THEN** elements with that title are matched

#### Scenario: Nth element

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.nth(2)` is called
- **THEN** only the third element (0-indexed) matches

#### Scenario: First element

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.first()` is called
- **THEN** only the first element matches

#### Scenario: Last element

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.last()` is called
- **THEN** only the last element matches

### Requirement: Locator Queries

The system SHALL provide locator query methods.

#### Scenario: Count elements

- **GIVEN** a locator
- **WHEN** `locator.count().await` is called
- **THEN** the number of matching elements is returned

#### Scenario: Get all locators

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.all().await` is called
- **THEN** a Vec of locators (one per element) is returned

#### Scenario: All inner texts

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.all_inner_texts().await` is called
- **THEN** a Vec of inner text strings is returned

#### Scenario: All text contents

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.all_text_contents().await` is called
- **THEN** a Vec of text content strings is returned

### Requirement: Aria Snapshot

The system SHALL support accessibility tree snapshots.

#### Scenario: Get aria snapshot

- **GIVEN** a locator
- **WHEN** `locator.aria_snapshot().await` is called
- **THEN** the accessibility tree for that element is returned

#### Scenario: Snapshot includes roles

- **GIVEN** an aria snapshot
- **WHEN** the snapshot is examined
- **THEN** element roles are included

#### Scenario: Snapshot includes names

- **GIVEN** an aria snapshot
- **WHEN** the snapshot is examined
- **THEN** accessible names are included

### Requirement: Highlight

The system SHALL support visual element highlighting.

#### Scenario: Highlight element

- **GIVEN** a locator
- **WHEN** `locator.highlight().await` is called
- **THEN** the element is visually highlighted in the browser

