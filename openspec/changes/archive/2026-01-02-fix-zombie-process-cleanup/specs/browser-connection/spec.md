## MODIFIED Requirements

### Requirement: Browser Lifecycle

The system SHALL manage browser lifecycle including graceful shutdown and proper process cleanup.

The browser shutdown SHALL:
- Kill the child process if launched by us (owned browser)
- Wait/reap the child process to prevent zombie processes
- Clean up resources even if the browser process has already died
- Handle both async (`close()`) and sync (`Drop`) cleanup contexts

#### Scenario: Close launched browser
- **GIVEN** a browser was launched via `Browser::launch()`
- **WHEN** `browser.close().await` is called
- **THEN** the WebSocket connection is closed
- **AND** the Chromium process is terminated via `kill()`
- **AND** the process is reaped via `wait()` to prevent zombie
- **AND** no `<defunct>` chromium process remains

#### Scenario: Close connected browser
- **GIVEN** a browser was connected via `Browser::connect()`
- **WHEN** `browser.close().await` is called
- **THEN** the WebSocket connection is closed AND the Chromium process continues running

#### Scenario: Browser dropped without explicit close
- **GIVEN** a browser was launched via `Browser::launch()`
- **WHEN** the Browser instance is dropped without calling `close()`
- **THEN** the Chromium process is terminated
- **AND** `try_wait()` is called to attempt to reap the process
- **AND** no zombie process remains (best effort in sync context)

#### Scenario: Browser process dies before close
- **GIVEN** a browser was launched via `Browser::launch()`
- **AND** the Chromium process has crashed or been killed externally
- **WHEN** `browser.close().await` is called
- **THEN** no error is raised for the already-dead process
- **AND** `wait()` is called to reap the zombie process
- **AND** no `<defunct>` chromium process remains

#### Scenario: Browser process dies before drop
- **GIVEN** a browser was launched via `Browser::launch()`
- **AND** the Chromium process has crashed or been killed externally
- **WHEN** the Browser instance is dropped
- **THEN** `try_wait()` is called to reap the zombie process
- **AND** no `<defunct>` chromium process remains

#### Scenario: Browser context creation
- **GIVEN** an active browser connection
- **WHEN** `browser.new_context().await` is called
- **THEN** a new isolated BrowserContext is created via `Target.createBrowserContext`
