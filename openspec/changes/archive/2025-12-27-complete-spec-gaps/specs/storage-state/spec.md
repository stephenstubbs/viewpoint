## MODIFIED Requirements

### Requirement: Restore Storage State

The system SHALL restore full storage state when creating a new context.

#### Scenario: Create context with storage state file

- **GIVEN** a storage state file with cookies and localStorage
- **WHEN** `browser.new_context().storage_state_path("auth.json").build().await` is called
- **THEN** the context has all cookies from the file
- **AND** localStorage is restored for each origin

#### Scenario: Restored localStorage is accessible

- **GIVEN** a storage state with localStorage entries
- **WHEN** the context is created with that state
- **AND** a page navigates to the origin
- **THEN** `localStorage.getItem(key)` returns the stored value

#### Scenario: Storage state with IndexedDB

- **GIVEN** a storage state with IndexedDB data
- **WHEN** the context is created with `indexed_db(true)` option
- **AND** a page navigates to the origin
- **THEN** IndexedDB databases and object stores are restored
