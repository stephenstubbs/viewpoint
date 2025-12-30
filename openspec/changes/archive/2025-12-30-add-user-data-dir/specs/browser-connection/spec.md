## MODIFIED Requirements

### Requirement: Browser Launching

The system SHALL launch a Chromium browser process and establish a CDP WebSocket connection.

The launcher SHALL:
- Spawn Chromium with `--remote-debugging-port=0` to get an available port
- Parse the WebSocket URL from Chromium's stderr output
- Establish WebSocket connection within a configurable timeout (default 30 seconds)
- Support headless and headed modes via builder configuration
- Support custom user data directory for persistent browser profiles

#### Scenario: Launch headless browser successfully
- **GIVEN** Chromium is installed and accessible
- **WHEN** `Browser::launch().headless(true).launch().await` is called
- **THEN** a Browser instance is returned with an active CDP connection

#### Scenario: Launch headed browser
- **GIVEN** Chromium is installed and a display is available
- **WHEN** `Browser::launch().headless(false).launch().await` is called
- **THEN** a visible browser window opens and Browser instance is returned

#### Scenario: Launch with custom arguments
- **GIVEN** Chromium is installed
- **WHEN** `Browser::launch().args(["--no-sandbox", "--disable-gpu"]).launch().await` is called
- **THEN** Chromium is launched with the specified arguments

#### Scenario: Launch with user data directory
- **GIVEN** Chromium is installed
- **WHEN** `Browser::launch().user_data_dir("/path/to/profile").launch().await` is called
- **THEN** Chromium is launched with `--user-data-dir=/path/to/profile`
- **AND** browser state (cookies, localStorage, settings) persists in that directory

#### Scenario: Persistent profile across sessions
- **GIVEN** a browser was launched with a user data directory and cookies were set
- **AND** the browser was closed
- **WHEN** a new browser is launched with the same user data directory
- **THEN** the previously set cookies are available

#### Scenario: Launch timeout
- **GIVEN** Chromium fails to start within timeout period
- **WHEN** `Browser::launch().timeout(Duration::from_secs(5)).launch().await` is called
- **THEN** a `BrowserError::LaunchTimeout` error is returned

#### Scenario: Chromium not found
- **GIVEN** Chromium is not installed or not in expected paths
- **WHEN** `Browser::launch().launch().await` is called
- **THEN** a `BrowserError::ChromiumNotFound` error is returned with helpful message
