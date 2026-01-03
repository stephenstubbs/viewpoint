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

