## MODIFIED Requirements

### Requirement: Locator Evaluate

The system SHALL support JavaScript evaluation on located elements.

#### Scenario: Evaluate on single element

- **GIVEN** a locator matching one element
- **WHEN** `locator.evaluate::<String>("el => el.textContent").await` is called
- **THEN** the element's text content is returned

#### Scenario: Evaluate on element with argument

- **GIVEN** a locator matching one element
- **WHEN** `locator.evaluate_with_arg::<(), _>("(el, cls) => el.classList.add(cls)", "active").await` is called
- **THEN** the class is added to the element

#### Scenario: Evaluate on all matching elements

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.evaluate_all::<Vec<String>>("els => els.map(e => e.id)").await` is called
- **THEN** an array of all element IDs is returned

#### Scenario: Get element handle

- **GIVEN** a locator matching one element
- **WHEN** `locator.element_handle().await` is called
- **THEN** an ElementHandle for the element is returned

#### Scenario: Evaluate on element via ref from aria snapshot

- **GIVEN** an aria snapshot containing an element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").evaluate::<String>("el => el.textContent").await` is called
- **THEN** the element's text content is returned

#### Scenario: Evaluate all via ref from aria snapshot

- **GIVEN** an aria snapshot containing a container element with ref `c0p0e5`
- **AND** the container has multiple child elements
- **WHEN** `page.locator_from_ref("c0p0e5").evaluate_all::<Vec<String>>("els => els.map(e => e.id)").await` is called
- **THEN** an array of element IDs is returned

#### Scenario: Get element handle via ref from aria snapshot

- **GIVEN** an aria snapshot containing an element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").element_handle().await` is called
- **THEN** an ElementHandle for that element is returned
