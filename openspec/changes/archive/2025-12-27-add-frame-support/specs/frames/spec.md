# Frames

## ADDED Requirements

### Requirement: Frame Locator

The system SHALL provide a FrameLocator for interacting with iframe content.

#### Scenario: Create frame locator by selector

- **GIVEN** a page with an iframe
- **WHEN** `page.frame_locator("#my-iframe")` is called
- **THEN** a FrameLocator for that iframe is returned

#### Scenario: Locate element within frame

- **GIVEN** a FrameLocator for an iframe
- **WHEN** `frame_locator.locator("button").click().await` is called
- **THEN** the button inside the iframe is clicked

#### Scenario: Use get_by methods in frame

- **GIVEN** a FrameLocator for an iframe
- **WHEN** `frame_locator.get_by_role(Role::Button, "Submit")` is called
- **THEN** a Locator for the button inside the frame is returned

#### Scenario: Nested frame locators

- **GIVEN** a page with nested iframes
- **WHEN** `page.frame_locator("#outer").frame_locator("#inner").locator("input")` is called
- **THEN** the input in the nested iframe is located

#### Scenario: Frame locator auto-waits

- **GIVEN** an iframe that loads dynamically
- **WHEN** `page.frame_locator("#dynamic-iframe").locator("button").click().await` is called
- **THEN** the operation waits for the iframe to load

### Requirement: Frame Access

The system SHALL provide direct access to Frame objects.

#### Scenario: Get frame by name

- **GIVEN** a page with a named iframe
- **WHEN** `page.frame("payment-frame").await` is called
- **THEN** the Frame object is returned

#### Scenario: Get frame by URL

- **GIVEN** a page with iframes having different URLs
- **WHEN** `page.frame_by_url("**/checkout/**").await` is called
- **THEN** the Frame matching the URL pattern is returned

#### Scenario: List all frames

- **GIVEN** a page with multiple iframes
- **WHEN** `page.frames()` is called
- **THEN** a list of all Frame objects is returned

#### Scenario: Get main frame

- **GIVEN** a page
- **WHEN** `page.main_frame()` is called
- **THEN** the main (top-level) Frame is returned

#### Scenario: Get child frames

- **GIVEN** a Frame with nested iframes
- **WHEN** `frame.child_frames()` is called
- **THEN** a list of child Frame objects is returned

#### Scenario: Get parent frame

- **GIVEN** a child Frame
- **WHEN** `frame.parent_frame()` is called
- **THEN** the parent Frame is returned

#### Scenario: Main frame has no parent

- **GIVEN** the main Frame
- **WHEN** `frame.parent_frame()` is called
- **THEN** None is returned

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
- **THEN** the frame's HTML content is returned

#### Scenario: Check if frame is detached

- **GIVEN** a Frame reference
- **WHEN** `frame.is_detached()` is called
- **THEN** true is returned if the frame was removed from DOM

### Requirement: Frame Navigation

The system SHALL support navigation within frames.

#### Scenario: Navigate frame to URL

- **GIVEN** a Frame
- **WHEN** `frame.goto("https://example.com").await` is called
- **THEN** the frame navigates to the URL

#### Scenario: Set frame content

- **GIVEN** a Frame
- **WHEN** `frame.set_content("<h1>Test</h1>").await` is called
- **THEN** the frame's content is replaced

#### Scenario: Frame title

- **GIVEN** a Frame with a title
- **WHEN** `frame.title().await` is called
- **THEN** the frame's document title is returned

### Requirement: Frame Events

The system SHALL emit events for frame lifecycle changes.

#### Scenario: Frame attached event

- **GIVEN** a page with frameattached listener
- **WHEN** an iframe is added to the DOM
- **THEN** a frameattached event is emitted

#### Scenario: Frame navigated event

- **GIVEN** a page with framenavigated listener
- **WHEN** an iframe navigates to a new URL
- **THEN** a framenavigated event is emitted

#### Scenario: Frame detached event

- **GIVEN** a page with framedetached listener
- **WHEN** an iframe is removed from the DOM
- **THEN** a framedetached event is emitted

### Requirement: Frame Locator Methods

The system SHALL provide all locator methods on FrameLocator.

#### Scenario: Frame locator get_by_text

- **GIVEN** a FrameLocator
- **WHEN** `frame_locator.get_by_text("Submit")` is called
- **THEN** a Locator for text within the frame is returned

#### Scenario: Frame locator get_by_role

- **GIVEN** a FrameLocator
- **WHEN** `frame_locator.get_by_role(Role::Button, opts)` is called
- **THEN** a Locator for the role within the frame is returned

#### Scenario: Frame locator get_by_label

- **GIVEN** a FrameLocator
- **WHEN** `frame_locator.get_by_label("Email")` is called
- **THEN** a Locator for the labeled input is returned

#### Scenario: Frame locator get_by_placeholder

- **GIVEN** a FrameLocator
- **WHEN** `frame_locator.get_by_placeholder("Enter email")` is called
- **THEN** a Locator for the input is returned

#### Scenario: Frame locator get_by_test_id

- **GIVEN** a FrameLocator
- **WHEN** `frame_locator.get_by_test_id("submit-btn")` is called
- **THEN** a Locator for the test ID is returned
