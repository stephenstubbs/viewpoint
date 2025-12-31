## ADDED Requirements

### Requirement: Action Navigation Auto-Wait

Actions that may trigger navigation SHALL automatically wait for the navigation to complete before returning.

The following actions SHALL support auto-wait:
- `click()` / `dblclick()`
- `press()` (on locator)
- `fill()`
- `select_option()`
- `check()` / `uncheck()`

Navigation detection SHALL:
- Listen for `Page.frameNavigated` CDP events on the main frame
- Use a short detection window (50ms) after the action completes
- If navigation detected, wait for `DocumentLoadState::Load` by default
- If no navigation detected, return immediately without additional delay

#### Scenario: Click triggers navigation
- **GIVEN** a locator targeting a link element
- **WHEN** `.click().await` is called
- **AND** the click triggers a page navigation
- **THEN** the method SHALL wait for the new page to reach `Load` state
- **AND** then return successfully

#### Scenario: Click does not trigger navigation
- **GIVEN** a locator targeting a button that shows a modal
- **WHEN** `.click().await` is called
- **AND** the click does not trigger navigation
- **THEN** the method SHALL return within ~50ms of the click completing

#### Scenario: Press Enter triggers form submission
- **GIVEN** a locator targeting a search input in a form
- **WHEN** `.press("Enter").await` is called
- **AND** the form submission triggers navigation
- **THEN** the method SHALL wait for the results page to reach `Load` state
- **AND** then return successfully

#### Scenario: Fill with auto-submit form
- **GIVEN** a locator targeting an input with JavaScript auto-submit on change
- **WHEN** `.fill("value").await` is called
- **AND** the fill triggers navigation via JavaScript
- **THEN** the method SHALL wait for navigation to complete

#### Scenario: Navigation timeout
- **GIVEN** a locator targeting an element that triggers slow navigation
- **WHEN** an action is performed that triggers navigation
- **AND** the navigation does not complete within the timeout period
- **THEN** a timeout error SHALL be returned

### Requirement: No Wait After Option

Actions SHALL support a `no_wait_after` option to skip navigation waiting.

When `no_wait_after(true)` is specified:
- The action SHALL return immediately after the DOM event is dispatched
- No navigation detection or waiting SHALL occur
- This matches Playwright's `noWaitAfter` option behavior

#### Scenario: Skip navigation wait on click
- **GIVEN** a locator targeting a link element
- **WHEN** `.click().no_wait_after(true).await` is called
- **THEN** the method SHALL return immediately after the click
- **AND** SHALL NOT wait for any triggered navigation

#### Scenario: Skip navigation wait on press
- **GIVEN** a locator targeting a form input
- **WHEN** `.press("Enter").no_wait_after(true).await` is called
- **THEN** the method SHALL return immediately after the key press
- **AND** SHALL NOT wait for form submission navigation

#### Scenario: Default behavior without no_wait_after
- **GIVEN** a locator targeting an element
- **WHEN** an action is performed without `.no_wait_after()`
- **THEN** the action SHALL use auto-wait behavior (wait for navigation if triggered)

## MODIFIED Requirements

### Requirement: Click Action

The framework SHALL support clicking elements via locators.

#### Scenario: Basic click
- **GIVEN** a locator targeting a button element
- **WHEN** `.click().await` is called
- **THEN** the element SHALL be scrolled into view
- **AND** the element SHALL be clicked in its center
- **AND** if navigation is triggered, the method SHALL wait for it to complete

#### Scenario: Click with position
- **GIVEN** a locator targeting an element
- **WHEN** `.click().position(10, 20).await` is called
- **THEN** the click SHALL occur at offset (10, 20) from the element's top-left corner

#### Scenario: Double click
- **GIVEN** a locator targeting an element
- **WHEN** `.dblclick().await` is called
- **THEN** a double-click SHALL be performed on the element
- **AND** if navigation is triggered, the method SHALL wait for it to complete

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

#### Scenario: Click with no_wait_after
- **GIVEN** a locator targeting a link element
- **WHEN** `.click().no_wait_after(true).await` is called
- **THEN** the click SHALL be performed
- **AND** the method SHALL return immediately without waiting for navigation

### Requirement: Press Key Action

The framework SHALL support pressing keyboard keys.

#### Scenario: Press Enter key
- **GIVEN** a locator targeting a form input
- **WHEN** `.press("Enter").await` is called
- **THEN** the Enter key SHALL be pressed
- **AND** if navigation is triggered (e.g., form submission), the method SHALL wait for it to complete

#### Scenario: Press key combination
- **GIVEN** a locator targeting an element
- **WHEN** `.press("Control+a").await` is called
- **THEN** Ctrl+A (select all) SHALL be performed

#### Scenario: Press special keys
- **GIVEN** a locator targeting an input
- **WHEN** `.press("Backspace").await` is called
- **THEN** the Backspace key SHALL be pressed

#### Scenario: Press with no_wait_after
- **GIVEN** a locator targeting a form input
- **WHEN** `.press("Enter").no_wait_after(true).await` is called
- **THEN** the Enter key SHALL be pressed
- **AND** the method SHALL return immediately without waiting for navigation

### Requirement: Fill Action

The framework SHALL support filling input fields with text.

#### Scenario: Fill text input
- **GIVEN** a locator targeting a text input
- **WHEN** `.fill("hello").await` is called
- **THEN** the input SHALL be cleared
- **AND** "hello" SHALL be entered into the input
- **AND** if navigation is triggered, the method SHALL wait for it to complete

#### Scenario: Fill textarea
- **GIVEN** a locator targeting a textarea
- **WHEN** `.fill("multi\nline").await` is called
- **THEN** the textarea SHALL contain "multi\nline"

#### Scenario: Fill clears existing content
- **GIVEN** a locator targeting an input with existing text "old"
- **WHEN** `.fill("new").await` is called
- **THEN** the input SHALL contain only "new"

#### Scenario: Fill with no_wait_after
- **GIVEN** a locator targeting an input with auto-submit behavior
- **WHEN** `.fill("value").no_wait_after(true).await` is called
- **THEN** the input SHALL be filled
- **AND** the method SHALL return immediately without waiting for navigation

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

### Requirement: Check/Uncheck Action

The framework SHALL support checking and unchecking checkboxes and radio buttons.

#### Scenario: Check checkbox
- **GIVEN** a locator targeting an unchecked checkbox
- **WHEN** `.check().await` is called
- **THEN** the checkbox SHALL become checked
- **AND** if navigation is triggered, the method SHALL wait for it to complete

#### Scenario: Check already checked
- **GIVEN** a locator targeting an already checked checkbox
- **WHEN** `.check().await` is called
- **THEN** the checkbox SHALL remain checked (no action)

#### Scenario: Uncheck checkbox
- **GIVEN** a locator targeting a checked checkbox
- **WHEN** `.uncheck().await` is called
- **THEN** the checkbox SHALL become unchecked

#### Scenario: Check with no_wait_after
- **GIVEN** a locator targeting a checkbox with onChange navigation
- **WHEN** `.check().no_wait_after(true).await` is called
- **THEN** the checkbox SHALL be checked
- **AND** the method SHALL return immediately without waiting for navigation
