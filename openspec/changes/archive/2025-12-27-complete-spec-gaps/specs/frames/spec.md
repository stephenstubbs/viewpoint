## ADDED Requirements

### Requirement: Frame Events

The system SHALL emit events when frames are attached, navigated, or detached.

#### Scenario: Frame attached event

- **GIVEN** a page with a frame attached event listener
- **WHEN** an iframe is added to the page
- **THEN** the `frameattached` event is emitted with the new Frame

#### Scenario: Frame navigated event

- **GIVEN** a page with a frame navigated event listener
- **WHEN** an iframe navigates to a new URL
- **THEN** the `framenavigated` event is emitted with the Frame

#### Scenario: Frame detached event

- **GIVEN** a page with a frame detached event listener
- **WHEN** an iframe is removed from the page
- **THEN** the `framedetached` event is emitted with the Frame

### Requirement: Frame Hierarchy

The system SHALL provide access to frame parent-child relationships.

#### Scenario: Get child frames

- **GIVEN** a frame containing nested iframes
- **WHEN** `frame.child_frames()` is called
- **THEN** a list of child Frame instances is returned

#### Scenario: Get parent frame

- **GIVEN** a nested iframe
- **WHEN** `frame.parent_frame()` is called
- **THEN** the parent Frame is returned

#### Scenario: Main frame has no parent

- **GIVEN** the main frame of a page
- **WHEN** `frame.parent_frame()` is called
- **THEN** `None` is returned
