# keyboard Specification

## Purpose
TBD - created by archiving change add-input-devices. Update Purpose after archive.
## Requirements
### Requirement: Key Press

The system SHALL dispatch key press events (keydown + keyup).

#### Scenario: Press single key

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().press("a").await` is called
- **THEN** a keydown event for 'a' is dispatched
- **AND** a keyup event for 'a' is dispatched
- **AND** the character 'a' is typed

#### Scenario: Press function key

- **GIVEN** a page with keyboard event listeners
- **WHEN** `page.keyboard().press("F1").await` is called
- **THEN** F1 key events are dispatched

#### Scenario: Press navigation key

- **GIVEN** a page with a text input containing text
- **WHEN** `page.keyboard().press("ArrowLeft").await` is called
- **THEN** the cursor moves left in the input

#### Scenario: Press with delay

- **GIVEN** a page with a focused element
- **WHEN** `page.keyboard().press("Enter").delay(Duration::from_millis(100)).await` is called
- **THEN** there is a 100ms delay between keydown and keyup

#### Scenario: Press uppercase letter

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().press("A").await` is called
- **THEN** Shift is automatically held
- **AND** the character 'A' is typed

#### Scenario: Press key combination

- **GIVEN** a page with a focused input element with text selected
- **WHEN** `page.keyboard().press("Control+C").await` is called
- **THEN** Control is held while C is pressed
- **AND** the copy action is triggered

#### Scenario: Press ControlOrMeta

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().press("ControlOrMeta+A").await` is called
- **THEN** on Windows/Linux, Control+A is pressed
- **AND** on macOS, Meta+A is pressed

### Requirement: Key Down

The system SHALL dispatch keydown events and hold modifier state.

#### Scenario: Hold modifier key

- **GIVEN** a page with a focused element
- **WHEN** `page.keyboard().down("Shift").await` is called
- **THEN** Shift keydown is dispatched
- **AND** subsequent key presses have Shift modifier

#### Scenario: Hold multiple modifiers

- **GIVEN** a page with a focused element
- **WHEN** `page.keyboard().down("Control").await` is called
- **AND** `page.keyboard().down("Shift").await` is called
- **THEN** both modifiers are active for subsequent keys

#### Scenario: Key down sets repeat flag

- **GIVEN** Shift is already held down
- **WHEN** `page.keyboard().down("Shift").await` is called again
- **THEN** the keydown event has repeat=true

### Requirement: Key Up

The system SHALL dispatch keyup events and release modifier state.

#### Scenario: Release modifier key

- **GIVEN** Shift is held down
- **WHEN** `page.keyboard().up("Shift").await` is called
- **THEN** Shift keyup is dispatched
- **AND** subsequent key presses do not have Shift modifier

#### Scenario: Release one of multiple modifiers

- **GIVEN** Control and Shift are both held down
- **WHEN** `page.keyboard().up("Control").await` is called
- **THEN** only Shift remains active

### Requirement: Type Text

The system SHALL type text character by character with key events.

#### Scenario: Type simple text

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().type_text("Hello").await` is called
- **THEN** keydown, keypress, and keyup events are dispatched for each character
- **AND** "Hello" appears in the input

#### Scenario: Type with delay

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().type_text("abc").delay(Duration::from_millis(50)).await` is called
- **THEN** there is a 50ms delay between each character

#### Scenario: Type special characters

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().type_text("Hello, World!").await` is called
- **THEN** all characters including punctuation are typed

#### Scenario: Modifier keys do not affect type

- **GIVEN** Shift is held down
- **WHEN** `page.keyboard().type_text("hello").await` is called
- **THEN** "hello" is typed in lowercase (modifiers ignored)

### Requirement: Insert Text

The system SHALL insert text without key events.

#### Scenario: Insert text directly

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().insert_text("Hello").await` is called
- **THEN** "Hello" appears in the input
- **AND** no keydown/keyup events are dispatched
- **AND** only an input event is dispatched

#### Scenario: Insert non-ASCII text

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().insert_text("你好").await` is called
- **THEN** the Chinese characters are inserted

#### Scenario: Insert emoji

- **GIVEN** a page with a focused input element
- **WHEN** `page.keyboard().insert_text("Hello 👋").await` is called
- **THEN** the text with emoji is inserted

### Requirement: Key Mapping

The system SHALL support all Playwright key names.

#### Scenario: Digit keys

- **GIVEN** a page with keyboard event listeners
- **WHEN** `page.keyboard().press("Digit5").await` is called
- **THEN** the '5' key is pressed (not numpad)

#### Scenario: Numpad keys

- **GIVEN** a page with keyboard event listeners
- **WHEN** `page.keyboard().press("Numpad5").await` is called
- **THEN** the numpad '5' key is pressed

#### Scenario: Letter keys by name

- **GIVEN** a page with keyboard event listeners
- **WHEN** `page.keyboard().press("KeyA").await` is called
- **THEN** the 'a' key is pressed (lowercase)

#### Scenario: Editing keys

- **GIVEN** a page with a focused input with text
- **WHEN** `page.keyboard().press("Backspace").await` is called
- **THEN** the character before the cursor is deleted

#### Scenario: Modifier key names

- **GIVEN** a page with keyboard event listeners
- **WHEN** `page.keyboard().press("ShiftLeft").await` is called
- **THEN** the left Shift key specifically is pressed

