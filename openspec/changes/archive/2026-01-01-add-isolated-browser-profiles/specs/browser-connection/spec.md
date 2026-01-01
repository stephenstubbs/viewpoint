## MODIFIED Requirements

### Requirement: Browser Launching

The system SHALL launch a Chromium browser process and establish a CDP WebSocket connection.

The launcher SHALL:
- Spawn Chromium with `--remote-debugging-port=0` to get an available port
- Parse the WebSocket URL from Chromium's stderr output
- Establish WebSocket connection within a configurable timeout (default 30 seconds)
- Support headless and headed modes via builder configuration
- Support four user data directory modes:
  - Isolated temp directory (default) - unique per session, auto-cleanup
  - Template-based temp directory - copy from template, auto-cleanup
  - Persistent path - user-specified directory, no cleanup
  - System default - use `~/.config/chromium/`, no cleanup
- Create a unique temporary directory for each browser instance by default
- Clean up temporary directories when the browser is closed or dropped
- Support loading unpacked extensions via `--load-extension` argument

#### Scenario: Launch headless browser successfully
- **GIVEN** Chromium is installed and accessible
- **WHEN** `Browser::launch().headless(true).launch().await` is called
- **THEN** a Browser instance is returned with an active CDP connection
- **AND** a unique temporary user data directory is created

#### Scenario: Launch headed browser
- **GIVEN** Chromium is installed and a display is available
- **WHEN** `Browser::launch().headless(false).launch().await` is called
- **THEN** a visible browser window opens and Browser instance is returned
- **AND** a unique temporary user data directory is created

#### Scenario: Launch with custom arguments
- **GIVEN** Chromium is installed
- **WHEN** `Browser::launch().args(["--no-sandbox", "--disable-gpu"]).launch().await` is called
- **THEN** Chromium is launched with the specified arguments

#### Scenario: Launch with persistent user data directory
- **GIVEN** Chromium is installed
- **WHEN** `Browser::launch().user_data_dir("/path/to/profile").launch().await` is called
- **THEN** Chromium is launched with `--user-data-dir=/path/to/profile`
- **AND** browser state (cookies, localStorage, settings) persists in that directory
- **AND** the directory is NOT deleted when the browser closes

#### Scenario: Launch with system default profile
- **GIVEN** Chromium is installed
- **WHEN** `Browser::launch().user_data_dir_system().launch().await` is called
- **THEN** Chromium is launched without `--user-data-dir` flag
- **AND** Chromium uses the system default profile (`~/.config/chromium/` on Linux)

#### Scenario: Launch with template-based profile
- **GIVEN** Chromium is installed
- **AND** a template profile exists at `/path/to/template` with extensions and settings
- **WHEN** `Browser::launch().user_data_dir_template_from("/path/to/template").launch().await` is called
- **THEN** a unique temporary directory is created
- **AND** the template profile contents are copied to the temporary directory
- **AND** Chromium is launched with `--user-data-dir` pointing to the temporary directory
- **AND** extensions and settings from the template are available

#### Scenario: Template profile cleanup on close
- **GIVEN** a browser was launched with `.user_data_dir_template_from()`
- **WHEN** `browser.close().await` is called
- **THEN** the temporary user data directory is deleted
- **AND** the original template directory is unchanged

#### Scenario: Concurrent browser launches with default isolation
- **GIVEN** Chromium is installed
- **WHEN** two browsers are launched concurrently with default settings
- **THEN** both browsers launch successfully with separate temporary directories
- **AND** no profile lock conflicts occur

#### Scenario: Temporary directory cleanup on close
- **GIVEN** a browser was launched with default settings (temp directory)
- **WHEN** `browser.close().await` is called
- **THEN** the temporary user data directory is deleted

#### Scenario: Temporary directory cleanup on drop
- **GIVEN** a browser was launched with default settings (temp directory)
- **WHEN** the Browser instance is dropped without explicit close
- **THEN** the temporary user data directory is deleted

#### Scenario: Persistent profile across sessions
- **GIVEN** a browser was launched with a persistent user data directory and cookies were set
- **AND** the browser was closed
- **WHEN** a new browser is launched with the same user data directory
- **THEN** the previously set cookies are available

#### Scenario: Load unpacked extension with temp profile
- **GIVEN** Chromium is installed
- **AND** an unpacked extension exists at `/path/to/extension`
- **WHEN** `Browser::launch().args(["--load-extension=/path/to/extension"]).launch().await` is called
- **THEN** the browser launches with the extension loaded
- **AND** the extension is available in the isolated temp profile

#### Scenario: Launch timeout
- **GIVEN** Chromium fails to start within timeout period
- **WHEN** `Browser::launch().timeout(Duration::from_secs(5)).launch().await` is called
- **THEN** a `BrowserError::LaunchTimeout` error is returned

#### Scenario: Chromium not found
- **GIVEN** Chromium is not installed or not in expected paths
- **WHEN** `Browser::launch().launch().await` is called
- **THEN** a `BrowserError::ChromiumNotFound` error is returned with helpful message
