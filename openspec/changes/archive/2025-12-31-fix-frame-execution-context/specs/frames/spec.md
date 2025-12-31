## MODIFIED Requirements

### Requirement: Frame Properties

The system SHALL expose frame properties.

#### Scenario: Get frame URL

- **GIVEN** a Frame
- **WHEN** `frame.url()` is called
- **THEN** the frame's current URL is returned

#### Scenario: Get frame name

- **GIVEN** a Frame with a name attribute
- **WHEN** `frame.name()` is called
- **THEN** the frame's name is returned

#### Scenario: Get frame content

- **GIVEN** a Frame with content
- **WHEN** `frame.content().await` is called
- **THEN** the frame's HTML content is returned (not the parent frame's content)

#### Scenario: Get iframe content returns iframe HTML

- **GIVEN** an iframe with content `<h1>Iframe Content</h1>`
- **WHEN** `frame.content().await` is called on the iframe's Frame object
- **THEN** the returned HTML contains `<h1>Iframe Content</h1>`

#### Scenario: Check if frame is detached

- **GIVEN** a Frame reference
- **WHEN** `frame.is_detached()` is called
- **THEN** true is returned if the frame was removed from DOM

#### Scenario: Get frame title

- **GIVEN** a Frame with a document title
- **WHEN** `frame.title().await` is called
- **THEN** the frame's document title is returned (not the parent frame's title)

#### Scenario: Get iframe title returns iframe document title

- **GIVEN** an iframe with document title "Iframe Title"
- **WHEN** `frame.title().await` is called on the iframe's Frame object
- **THEN** "Iframe Title" is returned

## ADDED Requirements

### Requirement: Frame JavaScript Execution Context

The system SHALL execute JavaScript in the correct frame context.

#### Scenario: Frame aria snapshot targets correct context

- **GIVEN** an iframe containing a `<button>Frame Button</button>`
- **WHEN** `frame.aria_snapshot().await` is called on the iframe's Frame object
- **THEN** the snapshot contains the button from the iframe, not elements from the parent

#### Scenario: Frame evaluate targets correct context

- **GIVEN** an iframe with `window.frameMarker = "iframe"`
- **WHEN** JavaScript is evaluated in that Frame
- **THEN** `window.frameMarker` returns "iframe"

#### Scenario: Main frame evaluate is not affected

- **GIVEN** a page with main frame and iframes
- **WHEN** JavaScript is evaluated on the main Frame
- **THEN** JavaScript executes in the main frame's context

### Requirement: Isolated World Contexts

The system SHALL support isolated world contexts for frames.

#### Scenario: Create isolated world for frame

- **GIVEN** a Frame
- **WHEN** an isolated world is requested for that frame
- **THEN** a separate execution context is created that shares the DOM but not global variables

#### Scenario: Isolated world does not share page globals

- **GIVEN** a page with `window.pageSecret = "secret"` in main world
- **WHEN** JavaScript is evaluated in an isolated world
- **THEN** `window.pageSecret` is undefined

#### Scenario: Isolated world shares DOM

- **GIVEN** a page with a `<button id="btn">` element
- **WHEN** `document.getElementById("btn")` is evaluated in an isolated world
- **THEN** the button element is returned

#### Scenario: Isolated world persists until navigation

- **GIVEN** an isolated world created for a frame
- **WHEN** multiple evaluations are performed in that isolated world
- **THEN** the same execution context is reused

#### Scenario: Isolated world is recreated after navigation

- **GIVEN** an isolated world created for a frame
- **WHEN** the frame navigates to a new URL
- **THEN** the old isolated world is destroyed and a new one can be created
