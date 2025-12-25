# Browser Connection

Chrome DevTools Protocol connection management for Chromium browser automation.

## ADDED Requirements

### Requirement: Browser Launching

The system SHALL launch a Chromium browser process and establish a CDP WebSocket connection.

The launcher SHALL:
- Spawn Chromium with `--remote-debugging-port=0` to get an available port
- Parse the WebSocket URL from Chromium's stderr output
- Establish WebSocket connection within a configurable timeout (default 30 seconds)
- Support headless and headed modes via builder configuration

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

#### Scenario: Launch timeout
- **GIVEN** Chromium fails to start within timeout period
- **WHEN** `Browser::launch().timeout(Duration::from_secs(5)).launch().await` is called
- **THEN** a `BrowserError::LaunchTimeout` error is returned

#### Scenario: Chromium not found
- **GIVEN** Chromium is not installed or not in expected paths
- **WHEN** `Browser::launch().launch().await` is called
- **THEN** a `BrowserError::ChromiumNotFound` error is returned with helpful message

### Requirement: Browser Connection

The system SHALL connect to an already-running Chromium instance via CDP WebSocket URL.

#### Scenario: Connect to running browser
- **GIVEN** Chromium is running with `--remote-debugging-port=9222`
- **WHEN** `Browser::connect("ws://localhost:9222/devtools/browser/...").await` is called
- **THEN** a Browser instance is returned with an active CDP connection

#### Scenario: Connect to remote browser
- **GIVEN** Chromium is running on a remote host with exposed debugging port
- **WHEN** `Browser::connect("ws://remote-host:9222/devtools/browser/...").await` is called
- **THEN** a Browser instance is returned with an active CDP connection

#### Scenario: Connection refused
- **GIVEN** no Chromium is listening on the specified address
- **WHEN** `Browser::connect("ws://localhost:9999/...").await` is called
- **THEN** a `BrowserError::ConnectionFailed` error is returned

### Requirement: Browser Lifecycle

The system SHALL manage browser lifecycle including graceful shutdown.

#### Scenario: Close launched browser
- **GIVEN** a browser was launched via `Browser::launch()`
- **WHEN** `browser.close().await` is called
- **THEN** the WebSocket connection is closed AND the Chromium process is terminated

#### Scenario: Close connected browser
- **GIVEN** a browser was connected via `Browser::connect()`
- **WHEN** `browser.close().await` is called
- **THEN** the WebSocket connection is closed AND the Chromium process continues running

#### Scenario: Browser context creation
- **GIVEN** an active browser connection
- **WHEN** `browser.new_context().await` is called
- **THEN** a new isolated BrowserContext is created via `Target.createBrowserContext`

### Requirement: CDP Transport

The system SHALL provide reliable CDP message transport over WebSocket.

The transport SHALL:
- Use atomic message ID generation per session
- Support concurrent command execution
- Broadcast CDP events to all subscribers
- Handle connection drops with appropriate errors

#### Scenario: Send command and receive response
- **GIVEN** an active CDP connection
- **WHEN** a CDP command is sent
- **THEN** the response is matched to the request by message ID

#### Scenario: Receive CDP event
- **GIVEN** an active CDP connection with event subscriptions
- **WHEN** a CDP event is received (e.g., `Page.loadEventFired`)
- **THEN** all subscribers receive the event

#### Scenario: Connection lost during command
- **GIVEN** an active CDP connection
- **WHEN** the WebSocket connection is lost while awaiting a response
- **THEN** a `CdpError::ConnectionLost` error is returned to the caller
