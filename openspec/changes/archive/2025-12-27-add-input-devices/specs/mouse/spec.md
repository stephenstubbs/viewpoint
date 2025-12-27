# Mouse

## ADDED Requirements

### Requirement: Mouse Move

The system SHALL move the mouse cursor to specified coordinates.

#### Scenario: Move to coordinates

- **GIVEN** a page with hover-sensitive elements
- **WHEN** `page.mouse().move_(100.0, 200.0).await` is called
- **THEN** the mouse cursor moves to (100, 200) in viewport coordinates
- **AND** mousemove events are dispatched

#### Scenario: Move with steps

- **GIVEN** a page with mousemove event listeners
- **WHEN** `page.mouse().move_(100.0, 200.0).steps(10).await` is called
- **THEN** 10 intermediate mousemove events are dispatched
- **AND** the cursor moves smoothly from current position to destination

#### Scenario: Move triggers hover effects

- **GIVEN** a page with CSS :hover styles
- **WHEN** `page.mouse().move_(element_x, element_y).await` is called over an element
- **THEN** the element's hover styles are applied

### Requirement: Mouse Click

The system SHALL click at specified coordinates.

#### Scenario: Left click at coordinates

- **GIVEN** a page with a button at coordinates (100, 100)
- **WHEN** `page.mouse().click(100.0, 100.0).await` is called
- **THEN** the mouse moves to (100, 100)
- **AND** mousedown and mouseup events are dispatched
- **AND** a click event is dispatched

#### Scenario: Right click at coordinates

- **GIVEN** a page with a context menu trigger
- **WHEN** `page.mouse().click(100.0, 100.0).button(MouseButton::Right).await` is called
- **THEN** a right-click (contextmenu) event is dispatched

#### Scenario: Middle click at coordinates

- **GIVEN** a page with a link
- **WHEN** `page.mouse().click(100.0, 100.0).button(MouseButton::Middle).await` is called
- **THEN** a middle-click event is dispatched

#### Scenario: Click with delay

- **GIVEN** a page with click event listeners
- **WHEN** `page.mouse().click(100.0, 100.0).delay(Duration::from_millis(100)).await` is called
- **THEN** there is a 100ms delay between mousedown and mouseup

#### Scenario: Multi-click

- **GIVEN** a page with click count detection
- **WHEN** `page.mouse().click(100.0, 100.0).click_count(3).await` is called
- **THEN** a triple-click event is dispatched

### Requirement: Mouse Double Click

The system SHALL double-click at specified coordinates.

#### Scenario: Double click at coordinates

- **GIVEN** a page with a text element
- **WHEN** `page.mouse().dblclick(100.0, 100.0).await` is called
- **THEN** mousedown, mouseup, mousedown, mouseup events are dispatched
- **AND** a dblclick event is dispatched

#### Scenario: Double click selects word

- **GIVEN** a page with a text paragraph
- **WHEN** `page.mouse().dblclick(word_x, word_y).await` is called
- **THEN** the word under the cursor is selected

### Requirement: Mouse Down and Up

The system SHALL dispatch mousedown and mouseup events separately.

#### Scenario: Mouse down

- **GIVEN** a page with mousedown event listeners
- **WHEN** `page.mouse().down().await` is called
- **THEN** a mousedown event is dispatched at the current position
- **AND** the mouse button is held

#### Scenario: Mouse down with button

- **GIVEN** a page with mousedown event listeners
- **WHEN** `page.mouse().down().button(MouseButton::Right).await` is called
- **THEN** a right mousedown event is dispatched

#### Scenario: Mouse up

- **GIVEN** the mouse button is currently held
- **WHEN** `page.mouse().up().await` is called
- **THEN** a mouseup event is dispatched

#### Scenario: Mouse up with click count

- **GIVEN** a page with mouseup event listeners
- **WHEN** `page.mouse().up().click_count(2).await` is called
- **THEN** the mouseup event has detail=2

### Requirement: Mouse Wheel

The system SHALL dispatch mouse wheel events.

#### Scenario: Scroll vertically

- **GIVEN** a page with scrollable content
- **WHEN** `page.mouse().wheel(0.0, 100.0).await` is called
- **THEN** a wheel event with deltaY=100 is dispatched
- **AND** the page scrolls down

#### Scenario: Scroll horizontally

- **GIVEN** a page with horizontally scrollable content
- **WHEN** `page.mouse().wheel(100.0, 0.0).await` is called
- **THEN** a wheel event with deltaX=100 is dispatched
- **AND** the page scrolls right

#### Scenario: Scroll diagonally

- **GIVEN** a page with scrollable content
- **WHEN** `page.mouse().wheel(50.0, 50.0).await` is called
- **THEN** a wheel event with both deltaX and deltaY is dispatched

### Requirement: Mouse Position Tracking

The system SHALL track the current mouse position.

#### Scenario: Click uses current position

- **GIVEN** the mouse was previously moved to (100, 100)
- **WHEN** `page.mouse().down().await` is called without coordinates
- **THEN** the mousedown occurs at (100, 100)

#### Scenario: Position persists across operations

- **GIVEN** `page.mouse().move_(50.0, 50.0).await` was called
- **WHEN** `page.mouse().click(100.0, 100.0).await` is called
- **AND** `page.mouse().down().await` is called
- **THEN** the mousedown occurs at (100, 100) (last click position)

### Requirement: Drag Operations

The system SHALL support drag and drop via mouse operations.

#### Scenario: Drag element with mouse

- **GIVEN** a page with a draggable element at (100, 100)
- **WHEN** the following sequence is executed:
  - `page.mouse().move_(100.0, 100.0).await`
  - `page.mouse().down().await`
  - `page.mouse().move_(200.0, 200.0).steps(10).await`
  - `page.mouse().up().await`
- **THEN** drag events are dispatched
- **AND** the element is moved to (200, 200)

#### Scenario: Page drag and drop helper

- **GIVEN** a page with a draggable source and drop target
- **WHEN** `page.drag_and_drop("#source", "#target").await` is called
- **THEN** the source element is dragged to the target
- **AND** drop events are dispatched

#### Scenario: Locator drag to helper

- **GIVEN** a locator for a draggable element
- **WHEN** `source.drag_to(target).await` is called
- **THEN** the source is dragged to the target location

#### Scenario: Drag with source position

- **GIVEN** a draggable element
- **WHEN** `page.drag_and_drop("#source", "#target").source_position(10, 10).await` is called
- **THEN** the drag starts from (10, 10) relative to source element

#### Scenario: Drag with target position

- **GIVEN** a draggable element and drop target
- **WHEN** `page.drag_and_drop("#source", "#target").target_position(5, 5).await` is called
- **THEN** the drop occurs at (5, 5) relative to target element
