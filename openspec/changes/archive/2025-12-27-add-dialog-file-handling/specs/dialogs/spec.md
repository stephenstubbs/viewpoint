# Dialogs

## ADDED Requirements

### Requirement: Dialog Events

The system SHALL emit events for browser dialogs.

#### Scenario: Alert dialog event

- **GIVEN** a page with dialog listener
- **WHEN** JavaScript calls `alert("message")`
- **THEN** a dialog event is emitted with type "alert"

#### Scenario: Confirm dialog event

- **GIVEN** a page with dialog listener
- **WHEN** JavaScript calls `confirm("question")`
- **THEN** a dialog event is emitted with type "confirm"

#### Scenario: Prompt dialog event

- **GIVEN** a page with dialog listener
- **WHEN** JavaScript calls `prompt("question")`
- **THEN** a dialog event is emitted with type "prompt"

#### Scenario: Beforeunload dialog event

- **GIVEN** a page with beforeunload handler and dialog listener
- **WHEN** navigation is triggered
- **THEN** a dialog event is emitted with type "beforeunload"

### Requirement: Dialog Properties

The system SHALL expose dialog properties.

#### Scenario: Get dialog message

- **GIVEN** a dialog event
- **WHEN** `dialog.message()` is called
- **THEN** the dialog message text is returned

#### Scenario: Get dialog type

- **GIVEN** a dialog event
- **WHEN** `dialog.type_()` is called
- **THEN** the dialog type (alert, confirm, prompt, beforeunload) is returned

#### Scenario: Get default prompt value

- **GIVEN** a prompt dialog with default value
- **WHEN** `dialog.default_value()` is called
- **THEN** the default value is returned

### Requirement: Dialog Actions

The system SHALL allow accepting or dismissing dialogs.

#### Scenario: Accept alert

- **GIVEN** an alert dialog
- **WHEN** `dialog.accept().await` is called
- **THEN** the dialog is closed

#### Scenario: Accept confirm

- **GIVEN** a confirm dialog
- **WHEN** `dialog.accept().await` is called
- **THEN** the dialog returns true

#### Scenario: Dismiss confirm

- **GIVEN** a confirm dialog
- **WHEN** `dialog.dismiss().await` is called
- **THEN** the dialog returns false

#### Scenario: Accept prompt with text

- **GIVEN** a prompt dialog
- **WHEN** `dialog.accept_with_text("answer").await` is called
- **THEN** the dialog returns the provided text

#### Scenario: Dismiss prompt

- **GIVEN** a prompt dialog
- **WHEN** `dialog.dismiss().await` is called
- **THEN** the dialog returns null

### Requirement: Auto-Dismiss

The system SHALL auto-dismiss dialogs when no listener is registered.

#### Scenario: Auto-dismiss alert

- **GIVEN** a page without dialog listener
- **WHEN** JavaScript calls `alert("message")`
- **THEN** the dialog is automatically dismissed

#### Scenario: Auto-dismiss does not block

- **GIVEN** a page without dialog listener
- **WHEN** JavaScript calls multiple alerts
- **THEN** the page does not freeze
