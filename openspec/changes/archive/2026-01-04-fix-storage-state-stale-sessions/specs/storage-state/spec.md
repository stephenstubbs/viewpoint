## MODIFIED Requirements

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

#### Scenario: Closed pages are removed from tracking

- **GIVEN** a browser context with an open page
- **WHEN** the page is closed via `page.close().await`
- **THEN** the page is removed from the context's internal pages list
- **AND** subsequent calls to `storage_state()` do not attempt to use the closed page's session

#### Scenario: Storage state succeeds after closing pages

- **GIVEN** a browser context where a page was created and then closed
- **WHEN** `context.storage_state().await` is called
- **THEN** the operation succeeds without "session not found" errors
- **AND** cookies are collected successfully
