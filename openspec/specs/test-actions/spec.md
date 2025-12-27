# test-actions Specification

## Purpose
TBD - created by archiving change add-test-framework. Update Purpose after archive.
## Requirements
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

### Requirement: Fill Action

The framework SHALL support filling input fields with text.

#### Scenario: Fill text input
- **GIVEN** a locator targeting a text input
- **WHEN** `.fill("hello").await` is called
- **THEN** the input SHALL be cleared
- **AND** "hello" SHALL be entered into the input

#### Scenario: Fill textarea
- **GIVEN** a locator targeting a textarea
- **WHEN** `.fill("multi\nline").await` is called
- **THEN** the textarea SHALL contain "multi\nline"

#### Scenario: Fill clears existing content
- **GIVEN** a locator targeting an input with existing text "old"
- **WHEN** `.fill("new").await` is called
- **THEN** the input SHALL contain only "new"

### Requirement: Type Action

The framework SHALL support typing text character by character.

#### Scenario: Type text
- **GIVEN** a locator targeting a focused input
- **WHEN** `.type_text("hello").await` is called
- **THEN** each character SHALL be typed sequentially
- **AND** keydown/keyup events SHALL fire for each character

#### Scenario: Type with delay
- **GIVEN** a locator targeting an input
- **WHEN** `.type_text("abc").delay(Duration::from_millis(100)).await` is called
- **THEN** there SHALL be a 100ms delay between each keystroke

### Requirement: Press Key Action

The framework SHALL support pressing keyboard keys.

#### Scenario: Press Enter key
- **GIVEN** a locator targeting a form input
- **WHEN** `.press("Enter").await` is called
- **THEN** the Enter key SHALL be pressed

#### Scenario: Press key combination
- **GIVEN** a locator targeting an element
- **WHEN** `.press("Control+a").await` is called
- **THEN** Ctrl+A (select all) SHALL be performed

#### Scenario: Press special keys
- **GIVEN** a locator targeting an input
- **WHEN** `.press("Backspace").await` is called
- **THEN** the Backspace key SHALL be pressed

### Requirement: Select Option Action

The framework SHALL support selecting options in dropdowns.

#### Scenario: Select by value
- **GIVEN** a locator targeting a select element
- **WHEN** `.select_option().value("option1").await` is called
- **THEN** the option with value "option1" SHALL be selected

#### Scenario: Select by label
- **GIVEN** a locator targeting a select element with `<option>Blue</option>`
- **WHEN** `.select_option().label("Blue").await` is called
- **THEN** the "Blue" option SHALL be selected

#### Scenario: Select multiple options
- **GIVEN** a locator targeting a multi-select element
- **WHEN** `.select_option().values(&["a", "b"]).await` is called
- **THEN** options "a" and "b" SHALL be selected

### Requirement: Check/Uncheck Action

The framework SHALL support checking and unchecking checkboxes and radio buttons.

#### Scenario: Check checkbox
- **GIVEN** a locator targeting an unchecked checkbox
- **WHEN** `.check().await` is called
- **THEN** the checkbox SHALL become checked

#### Scenario: Check already checked
- **GIVEN** a locator targeting an already checked checkbox
- **WHEN** `.check().await` is called
- **THEN** the checkbox SHALL remain checked (no action)

#### Scenario: Uncheck checkbox
- **GIVEN** a locator targeting a checked checkbox
- **WHEN** `.uncheck().await` is called
- **THEN** the checkbox SHALL become unchecked

### Requirement: Hover Action

The framework SHALL support hovering over elements.

#### Scenario: Hover over element
- **GIVEN** a locator targeting an element with a hover tooltip
- **WHEN** `.hover().await` is called
- **THEN** the mouse SHALL move to the element's center
- **AND** hover events SHALL fire

#### Scenario: Hover with position
- **GIVEN** a locator targeting an element
- **WHEN** `.hover().position(5, 5).await` is called
- **THEN** the mouse SHALL move to offset (5, 5) from top-left

### Requirement: Focus Action

The framework SHALL support focusing elements.

#### Scenario: Focus input
- **GIVEN** a locator targeting an input element
- **WHEN** `.focus().await` is called
- **THEN** the input SHALL receive focus
- **AND** focus events SHALL fire

### Requirement: Clear Action

The framework SHALL support clearing input content.

#### Scenario: Clear input
- **GIVEN** a locator targeting an input with text "hello"
- **WHEN** `.clear().await` is called
- **THEN** the input SHALL become empty

### Requirement: Action Actionability

All actions SHALL wait for elements to be actionable before proceeding.

#### Scenario: Wait for visible
- **GIVEN** a locator targeting a hidden element
- **WHEN** an action is performed
- **THEN** the action SHALL wait for the element to become visible

#### Scenario: Wait for enabled
- **GIVEN** a locator targeting a disabled button
- **WHEN** `.click().await` is called
- **THEN** the action SHALL wait for the button to become enabled

#### Scenario: Wait for stable
- **GIVEN** a locator targeting an element that is animating
- **WHEN** an action is performed
- **THEN** the action SHALL wait for the element position to stabilize

