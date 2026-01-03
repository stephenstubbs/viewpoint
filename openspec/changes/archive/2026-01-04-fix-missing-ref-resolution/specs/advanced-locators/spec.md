## MODIFIED Requirements

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

#### Scenario: Is checked via ref from aria snapshot

- **GIVEN** an aria snapshot containing a checkbox element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").is_checked().await` is called
- **THEN** the checkbox's checked state is returned

#### Scenario: Inner text via ref from aria snapshot

- **GIVEN** an aria snapshot containing an element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").inner_text().await` is called
- **THEN** the element's inner text is returned

#### Scenario: Get attribute via ref from aria snapshot

- **GIVEN** an aria snapshot containing an element with ref `c0p0e5`
- **AND** the element has a `data-id` attribute
- **WHEN** `page.locator_from_ref("c0p0e5").get_attribute("data-id").await` is called
- **THEN** the attribute value is returned

#### Scenario: Input value via ref from aria snapshot

- **GIVEN** an aria snapshot containing an input element with ref `c0p0e5`
- **AND** the input has a value
- **WHEN** `page.locator_from_ref("c0p0e5").input_value().await` is called
- **THEN** the input's value is returned

### Requirement: Highlight

The system SHALL support visual element highlighting.

#### Scenario: Highlight element

- **GIVEN** a locator
- **WHEN** `locator.highlight().await` is called
- **THEN** the element is visually highlighted in the browser

#### Scenario: Highlight element via ref from aria snapshot

- **GIVEN** an aria snapshot containing an element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").highlight().await` is called
- **THEN** the element is visually highlighted in the browser

### Requirement: Element Ref Resolution

The system SHALL support resolving snapshot refs back to elements for interaction.

Resolution SHALL validate that the ref's context index matches the target context and the page index matches the target page.

When a ref cannot be resolved, the system SHALL return a clear error suggesting the user capture a new snapshot.

#### Scenario: Resolve ref to element handle

- **GIVEN** a ref string from an aria snapshot (e.g., `c0p0e1`)
- **WHEN** `page.element_from_ref("c0p0e1").await` is called on context 0, page 0
- **THEN** an `ElementHandle` for that element is returned

#### Scenario: Resolve ref to locator

- **GIVEN** a ref string from an aria snapshot
- **WHEN** `page.locator_from_ref("c0p0e1")` is called
- **THEN** a `Locator` targeting that element is returned with auto-waiting behavior

#### Scenario: Click element via ref

- **GIVEN** a button's ref from an aria snapshot
- **WHEN** `page.locator_from_ref(button_ref).click().await` is called
- **THEN** the button is clicked

#### Scenario: Context index mismatch returns error

- **GIVEN** a ref with context index 0 (`c0p0e1`)
- **WHEN** `page.element_from_ref("c0p0e1").await` is called on a page in context 1
- **THEN** an error is returned indicating context index mismatch

#### Scenario: Page index mismatch returns error

- **GIVEN** a ref with page index 0 (`c0p0e1`)
- **WHEN** `page.element_from_ref("c0p0e1").await` is called on page index 1
- **THEN** an error is returned indicating page index mismatch

#### Scenario: Stale ref returns helpful error

- **GIVEN** a ref for an element that no longer exists
- **WHEN** `page.element_from_ref(stale_ref).await` is called
- **THEN** an error is returned with message suggesting to capture a new snapshot

#### Scenario: Invalid ref format returns error

- **GIVEN** an invalid or malformed ref string
- **WHEN** `page.element_from_ref("invalid").await` is called
- **THEN** an appropriate error is returned indicating invalid format

#### Scenario: Scroll into view via ref from aria snapshot

- **GIVEN** an aria snapshot containing an off-screen element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").scroll_into_view_if_needed().await` is called
- **THEN** the element is scrolled into view

#### Scenario: Aria snapshot on locator via ref

- **GIVEN** an aria snapshot containing a container element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").aria_snapshot().await` is called
- **THEN** an aria snapshot of that element's subtree is returned

#### Scenario: Element screenshot via ref

- **GIVEN** an aria snapshot containing an element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").screenshot().capture().await` is called
- **THEN** a screenshot of that element is captured
