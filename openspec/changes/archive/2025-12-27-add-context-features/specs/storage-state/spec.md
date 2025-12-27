# Storage State

## ADDED Requirements

### Requirement: Get Storage State

The system SHALL export the browser context's storage state.

#### Scenario: Get storage state as object

- **GIVEN** a browser context with cookies and localStorage
- **WHEN** `context.storage_state().await` is called
- **THEN** a StorageState object is returned with cookies and origins

#### Scenario: Storage state includes cookies

- **GIVEN** a browser context with cookies
- **WHEN** `context.storage_state().await` is called
- **THEN** the cookies array contains all context cookies

#### Scenario: Storage state includes localStorage

- **GIVEN** a page has set localStorage items
- **WHEN** `context.storage_state().await` is called
- **THEN** the origins array contains localStorage for each origin

#### Scenario: Save storage state to file

- **GIVEN** a browser context with state
- **WHEN** `context.storage_state().path("state.json").await` is called
- **THEN** the storage state is written to the file
- **AND** the StorageState object is still returned

#### Scenario: Storage state includes IndexedDB

- **GIVEN** a page has stored data in IndexedDB
- **WHEN** `context.storage_state().indexed_db(true).await` is called
- **THEN** the IndexedDB snapshot is included in the state

### Requirement: Restore Storage State

The system SHALL restore browser context state from a saved state.

#### Scenario: Create context with storage state file

- **GIVEN** a saved storage state file
- **WHEN** `browser.new_context().storage_state("state.json").build().await` is called
- **THEN** the new context has the saved cookies and localStorage

#### Scenario: Create context with storage state object

- **GIVEN** a StorageState object
- **WHEN** `browser.new_context().storage_state_object(state).build().await` is called
- **THEN** the new context has the cookies and localStorage from the object

#### Scenario: Restored cookies are functional

- **GIVEN** a context created with storage state containing auth cookies
- **WHEN** a page navigates to the authenticated site
- **THEN** the user is authenticated without logging in

#### Scenario: Restored localStorage is accessible

- **GIVEN** a context created with storage state containing localStorage
- **WHEN** a page accesses localStorage for that origin
- **THEN** the saved values are available

#### Scenario: Storage state with IndexedDB

- **GIVEN** a saved storage state with IndexedDB snapshot
- **WHEN** a context is created with that state
- **THEN** the IndexedDB data is restored

### Requirement: Storage State Format

The system SHALL use a JSON format compatible with Playwright.

#### Scenario: JSON format structure

- **GIVEN** a storage state is exported
- **WHEN** the JSON is examined
- **THEN** it has "cookies" array and "origins" array at the top level

#### Scenario: Cookie format in JSON

- **GIVEN** a storage state with cookies
- **WHEN** the JSON is examined
- **THEN** each cookie has name, value, domain, path, expires, httpOnly, secure, sameSite

#### Scenario: LocalStorage format in JSON

- **GIVEN** a storage state with localStorage
- **WHEN** the JSON is examined
- **THEN** origins contains objects with "origin" and "localStorage" array of name/value pairs

### Requirement: Authentication State Reuse

The system SHALL support the common pattern of reusing authentication.

#### Scenario: Save authenticated state

- **GIVEN** a test has logged into an application
- **WHEN** `context.storage_state().path("auth.json").await` is called
- **THEN** the authentication state is saved

#### Scenario: Reuse authenticated state in new test

- **GIVEN** a saved authentication state file
- **WHEN** a new test creates a context with that state
- **THEN** the test starts already authenticated

#### Scenario: Multiple user states

- **GIVEN** storage states saved for different users
- **WHEN** tests create contexts with different states
- **THEN** each test runs as the appropriate user
