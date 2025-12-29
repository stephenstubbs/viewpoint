# browser-connection Specification

## Purpose
TBD - created by archiving change add-chromium-connection. Update Purpose after archive.
## Requirements
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

### Requirement: Connect Over CDP

The system SHALL connect to an already-running Chromium instance using an HTTP endpoint URL that auto-discovers the WebSocket connection.

The `connect_over_cdp` method SHALL:
- Accept both HTTP URLs (e.g., `http://localhost:9222`) and WebSocket URLs
- For HTTP URLs, fetch `/json/version` to discover `webSocketDebuggerUrl`
- Support configurable timeout (default 30 seconds)
- Support custom headers for the connection request

#### Scenario: Connect via HTTP endpoint

- **GIVEN** Chromium is running with `--remote-debugging-port=9222`
- **WHEN** `Browser::connect_over_cdp("http://localhost:9222").connect().await` is called
- **THEN** the system fetches `/json/version` to get the WebSocket URL
- **AND** a Browser instance is returned with an active CDP connection

#### Scenario: Connect via HTTP with custom port

- **GIVEN** Chromium is running with `--remote-debugging-port=9333`
- **WHEN** `Browser::connect_over_cdp("http://localhost:9333").connect().await` is called
- **THEN** a Browser instance is returned with an active CDP connection

#### Scenario: Connect via remote HTTP endpoint

- **GIVEN** Chromium is running on a remote host with exposed debugging port
- **WHEN** `Browser::connect_over_cdp("http://remote-host:9222").connect().await` is called
- **THEN** a Browser instance is returned with an active CDP connection

#### Scenario: Connect via WebSocket URL directly

- **GIVEN** Chromium is running with known WebSocket URL
- **WHEN** `Browser::connect_over_cdp("ws://localhost:9222/devtools/browser/...").connect().await` is called
- **THEN** the system connects directly without HTTP discovery
- **AND** a Browser instance is returned

#### Scenario: Connection with timeout

- **GIVEN** Chromium is running but responding slowly
- **WHEN** `Browser::connect_over_cdp(url).timeout(Duration::from_secs(5)).connect().await` is called
- **AND** connection takes longer than 5 seconds
- **THEN** a `BrowserError::ConnectionTimeout` error is returned

#### Scenario: Connection with custom headers

- **GIVEN** a browser endpoint requiring authentication
- **WHEN** `Browser::connect_over_cdp(url).header("Authorization", "Bearer token").connect().await` is called
- **THEN** the Authorization header is included in the WebSocket upgrade request

#### Scenario: Invalid HTTP endpoint

- **GIVEN** the HTTP endpoint does not expose CDP
- **WHEN** `Browser::connect_over_cdp("http://localhost:8080").connect().await` is called
- **THEN** a `BrowserError::EndpointDiscoveryFailed` error is returned

#### Scenario: Unreachable endpoint

- **GIVEN** no service is listening on the specified address
- **WHEN** `Browser::connect_over_cdp("http://localhost:9999").connect().await` is called
- **THEN** a `BrowserError::ConnectionFailed` error is returned

### Requirement: List Browser Contexts

The system SHALL provide access to all browser contexts in a connected browser.

#### Scenario: Get contexts from connected browser

- **GIVEN** a browser connected via `connect_over_cdp` with existing contexts
- **WHEN** `browser.contexts().await` is called
- **THEN** a list of all BrowserContext instances is returned
- **AND** the list includes the default context

#### Scenario: Get contexts from launched browser

- **GIVEN** a browser launched via `Browser::launch()`
- **WHEN** contexts are created via `browser.new_context()`
- **AND** `browser.contexts().await` is called
- **THEN** the created contexts are returned

#### Scenario: Default context handling

- **GIVEN** a browser connected to an existing Chromium instance
- **WHEN** `browser.contexts().await` is called
- **THEN** the default context (browser's main profile) is accessible
- **AND** existing pages in the default context can be enumerated

### Requirement: Access Existing Pages

The system SHALL allow accessing pages/tabs that existed before connection.

#### Scenario: List pages in default context

- **GIVEN** Chromium is running with tabs already open
- **AND** a browser connection is established via `connect_over_cdp`
- **WHEN** `context.pages().await` is called on the default context
- **THEN** existing pages are returned as Page instances

#### Scenario: Interact with existing page

- **GIVEN** Chromium has a tab open to `https://example.com`
- **AND** a browser connection is established via `connect_over_cdp`
- **WHEN** the existing page is retrieved from the default context
- **THEN** standard Page operations (navigate, evaluate, click, etc.) work

### Requirement: Context Ownership

The system SHALL track whether browser contexts are owned (created by us) or external (existed before connection).

#### Scenario: Close owned context

- **GIVEN** a browser context created via `browser.new_context()`
- **WHEN** `context.close().await` is called
- **THEN** the context is disposed via `Target.disposeBrowserContext`

#### Scenario: Close external context behavior

- **GIVEN** a browser context obtained via `browser.contexts()` on a connected browser
- **AND** the context existed before our connection
- **WHEN** `context.close().await` is called
- **THEN** the context is NOT disposed (preserves user's browser state)
- **AND** only our connection to it is closed

#### Scenario: Close connected browser

- **GIVEN** a browser connected via `connect_over_cdp`
- **WHEN** `browser.close().await` is called
- **THEN** the WebSocket connection is closed
- **AND** the Chromium process continues running with all tabs intact

