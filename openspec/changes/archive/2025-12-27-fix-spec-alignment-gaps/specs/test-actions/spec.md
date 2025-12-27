## MODIFIED Requirements

### Requirement: Click Action

The framework SHALL support clicking elements via locators.

#### Scenario: Basic click
- **GIVEN** a locator targeting a button element
- **WHEN** `.click().await` is called
- **THEN** the element SHALL be scrolled into view
- **AND** the element SHALL be clicked in its center

#### Scenario: Click with position
- **GIVEN** a locator targeting an element
- **WHEN** `.click().position(10, 20).await` is called
- **THEN** the click SHALL occur at offset (10, 20) from the element's top-left corner

#### Scenario: Double click
- **GIVEN** a locator targeting an element
- **WHEN** `.dblclick().await` is called
- **THEN** a double-click SHALL be performed on the element

#### Scenario: Right click
- **GIVEN** a locator targeting an element
- **WHEN** `.click().button(MouseButton::Right).await` is called
- **THEN** a right-click SHALL be performed on the element

#### Scenario: Middle click
- **GIVEN** a locator targeting an element
- **WHEN** `.click().button(MouseButton::Middle).await` is called
- **THEN** a middle-click SHALL be performed on the element

#### Scenario: Click with modifier keys
- **GIVEN** a locator targeting a link element
- **WHEN** `.click().modifiers(&[Modifier::Control]).await` is called
- **THEN** a Ctrl+click SHALL be performed

#### Scenario: Force click
- **GIVEN** a locator targeting an element covered by another element
- **WHEN** `.click().force(true).await` is called
- **THEN** the click SHALL be performed without actionability checks
