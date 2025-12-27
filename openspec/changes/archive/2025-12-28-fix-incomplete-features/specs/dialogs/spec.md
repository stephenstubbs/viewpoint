## MODIFIED Requirements

### Requirement: Dialog Events

The system SHALL emit events for browser dialogs.

#### Scenario: Alert dialog event

- **GIVEN** a page with dialog listener registered via `page.on_dialog(handler).await`
- **WHEN** JavaScript calls `alert("message")`
- **THEN** a dialog event is emitted with type "alert"
- **AND** the handler receives the Dialog object
- **AND** the page does not hang waiting for the dialog

#### Scenario: Confirm dialog event

- **GIVEN** a page with dialog listener
- **WHEN** JavaScript calls `confirm("question")`
- **THEN** a dialog event is emitted with type "confirm"
- **AND** the handler can call `dialog.accept()` or `dialog.dismiss()`

#### Scenario: Prompt dialog event

- **GIVEN** a page with dialog listener
- **WHEN** JavaScript calls `prompt("question")`
- **THEN** a dialog event is emitted with type "prompt"
- **AND** the handler can call `dialog.accept_with_text("answer")`

#### Scenario: Beforeunload dialog event

- **GIVEN** a page with beforeunload handler and dialog listener
- **WHEN** navigation is triggered
- **THEN** a dialog event is emitted with type "beforeunload"

#### Scenario: CDP Page.javascriptDialogOpening event

- **GIVEN** a page is created
- **WHEN** the Page CDP domain is enabled
- **THEN** `Page.javascriptDialogOpening` events are subscribed
- **AND** events are routed to the PageEventManager

### Requirement: Auto-Dismiss

The system SHALL auto-dismiss dialogs when no listener is registered.

#### Scenario: Auto-dismiss alert

- **GIVEN** a page without dialog listener
- **WHEN** JavaScript calls `alert("message")`
- **THEN** the dialog is automatically dismissed via `Page.handleJavaScriptDialog`

#### Scenario: Auto-dismiss does not block

- **GIVEN** a page without dialog listener
- **WHEN** JavaScript calls multiple alerts
- **THEN** the page does not freeze
- **AND** subsequent actions can proceed
