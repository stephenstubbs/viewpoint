## MODIFIED Requirements

### Requirement: Click Action

The system SHALL provide a configurable click action with builder pattern options.

#### Scenario: Basic click

- **GIVEN** a visible, enabled element
- **WHEN** `locator.click().await` is called
- **THEN** the element is scrolled into view and clicked at center

#### Scenario: Click with position offset

- **GIVEN** a visible element
- **WHEN** `locator.click().position(10, 20).await` is called
- **THEN** the click occurs at offset (10, 20) from the element's top-left corner

#### Scenario: Right click

- **GIVEN** a visible element
- **WHEN** `locator.click().button(MouseButton::Right).await` is called
- **THEN** a right-click (context menu) is performed

#### Scenario: Middle click

- **GIVEN** a visible element
- **WHEN** `locator.click().button(MouseButton::Middle).await` is called
- **THEN** a middle-click is performed

#### Scenario: Click with modifier keys

- **GIVEN** a visible element
- **WHEN** `locator.click().modifiers(&[Modifier::Shift]).await` is called
- **THEN** Shift is held during the click

#### Scenario: Force click

- **GIVEN** an element that may be covered or disabled
- **WHEN** `locator.click().force(true).await` is called
- **THEN** actionability checks are skipped and click is performed

### Requirement: Type Action

The system SHALL provide configurable text typing.

#### Scenario: Type with delay

- **GIVEN** a focused input element
- **WHEN** `locator.type_text("hello").delay(Duration::from_millis(100)).await` is called
- **THEN** each character is typed with a 100ms delay between keystrokes

### Requirement: Hover Action

The system SHALL provide configurable hover action.

#### Scenario: Hover with position

- **GIVEN** a visible element
- **WHEN** `locator.hover().position(5, 5).await` is called
- **THEN** the mouse hovers at offset (5, 5) from the element's top-left corner
