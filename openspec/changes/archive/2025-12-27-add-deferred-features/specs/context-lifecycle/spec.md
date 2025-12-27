## MODIFIED Requirements

### Requirement: Context Event Handlers

The context SHALL support event handlers for lifecycle events.

#### Scenario: Listen for new page events
- **WHEN** `context.on_page(handler)` is called
- **AND** a new page is created in the context
- **THEN** the handler is invoked with the new Page

#### Scenario: Remove page event handler
- **WHEN** `context.on_page(handler)` returns a handler ID
- **AND** `context.off_page(handler_id)` is called
- **THEN** the handler is no longer invoked for new pages

#### Scenario: Listen for context close events
- **WHEN** `context.on_close(handler)` is called
- **AND** the context is closed
- **THEN** the handler is invoked before cleanup

#### Scenario: Wait for new page during action
- **WHEN** `context.wait_for_page(action)` is called
- **AND** the action triggers creation of a new page
- **THEN** the method returns the new Page

### Requirement: Context Init Scripts

The context SHALL support init scripts applied to all pages.

#### Scenario: Add context-level init script
- **WHEN** `context.add_init_script(script)` is called
- **AND** a new page is created
- **THEN** the script runs before any page scripts

#### Scenario: Add init script from file
- **WHEN** `context.add_init_script_path(path)` is called
- **AND** a new page is created
- **THEN** the script from the file runs before any page scripts

### Requirement: Context Timeout Propagation

The context default timeouts SHALL propagate to page operations.

#### Scenario: Context timeout applies to pages
- **WHEN** `context.set_default_timeout(duration)` is called
- **AND** a page action is performed without explicit timeout
- **THEN** the context timeout is used

#### Scenario: Page timeout overrides context
- **WHEN** context has a default timeout set
- **AND** page action specifies explicit timeout
- **THEN** the explicit timeout is used
