# Console Events

## ADDED Requirements

### Requirement: Console Message Events

The system SHALL emit events for console messages.

#### Scenario: Console log event

- **GIVEN** a page with console listener
- **WHEN** JavaScript calls `console.log("message")`
- **THEN** a console event with type "log" is emitted

#### Scenario: Console error event

- **GIVEN** a page with console listener
- **WHEN** JavaScript calls `console.error("error")`
- **THEN** a console event with type "error" is emitted

#### Scenario: Console warn event

- **GIVEN** a page with console listener
- **WHEN** JavaScript calls `console.warn("warning")`
- **THEN** a console event with type "warning" is emitted

### Requirement: Console Message Properties

The system SHALL expose console message properties.

#### Scenario: Get message text

- **GIVEN** a ConsoleMessage
- **WHEN** `message.text()` is called
- **THEN** the formatted message text is returned

#### Scenario: Get message type

- **GIVEN** a ConsoleMessage
- **WHEN** `message.type_()` is called
- **THEN** the message type is returned

#### Scenario: Get message arguments

- **GIVEN** a ConsoleMessage with multiple arguments
- **WHEN** `message.args()` is called
- **THEN** JSHandle array for each argument is returned

#### Scenario: Get message location

- **GIVEN** a ConsoleMessage
- **WHEN** `message.location()` is called
- **THEN** the source location (URL, line, column) is returned

### Requirement: Page Error Events

The system SHALL emit events for uncaught errors.

#### Scenario: Page error event

- **GIVEN** a page with pageerror listener
- **WHEN** an uncaught exception occurs
- **THEN** a pageerror event with the error is emitted

#### Scenario: Error message

- **GIVEN** a pageerror event
- **WHEN** `error.message()` is called
- **THEN** the error message is returned

### Requirement: Context Error Events

The system SHALL emit error events at context level.

#### Scenario: Web error event

- **GIVEN** a context with weberror listener
- **WHEN** any page has an uncaught exception
- **THEN** a weberror event is emitted

#### Scenario: Web error includes page

- **GIVEN** a weberror event
- **WHEN** `web_error.page()` is called
- **THEN** the page where the error occurred is returned
