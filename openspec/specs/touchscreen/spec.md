# touchscreen Specification

## Purpose
TBD - created by archiving change add-input-devices. Update Purpose after archive.
## Requirements
### Requirement: Touch Context Configuration

The system SHALL require touch capability to be enabled in browser context.

#### Scenario: Touch requires hasTouch option

- **GIVEN** a browser context created without `hasTouch: true`
- **WHEN** `page.touchscreen().tap(100.0, 100.0).await` is called
- **THEN** an error is returned indicating touch is not enabled

#### Scenario: Touch works with hasTouch enabled

- **GIVEN** a browser context created with `hasTouch: true`
- **WHEN** `page.touchscreen().tap(100.0, 100.0).await` is called
- **THEN** touch events are dispatched successfully

### Requirement: Touch Tap

The system SHALL dispatch touchstart and touchend events for taps.

#### Scenario: Tap at coordinates

- **GIVEN** a page in a touch-enabled context
- **WHEN** `page.touchscreen().tap(100.0, 200.0).await` is called
- **THEN** a touchstart event is dispatched at (100, 200)
- **AND** a touchend event is dispatched at (100, 200)

#### Scenario: Tap triggers click

- **GIVEN** a page with a button in a touch-enabled context
- **WHEN** `page.touchscreen().tap(button_x, button_y).await` is called
- **THEN** the button receives a click event

#### Scenario: Tap on input focuses element

- **GIVEN** a page with an input element in a touch-enabled context
- **WHEN** `page.touchscreen().tap(input_x, input_y).await` is called
- **THEN** the input receives focus

### Requirement: Touch Event Details

The system SHALL provide correct touch event properties.

#### Scenario: Touch event has correct coordinates

- **GIVEN** a page with touch event listeners
- **WHEN** `page.touchscreen().tap(150.0, 250.0).await` is called
- **THEN** the touch event has clientX=150 and clientY=250

#### Scenario: Touch event has identifier

- **GIVEN** a page with touch event listeners
- **WHEN** `page.touchscreen().tap(100.0, 100.0).await` is called
- **THEN** the touch event has a touch identifier

#### Scenario: TouchList has single touch

- **GIVEN** a page with touch event listeners
- **WHEN** `page.touchscreen().tap(100.0, 100.0).await` is called
- **THEN** the touch event's touches list has length 1

### Requirement: Locator Tap

The system SHALL support tapping on located elements.

#### Scenario: Tap on locator

- **GIVEN** a locator for a button in a touch-enabled context
- **WHEN** `locator.tap().await` is called
- **THEN** touch events are dispatched at the element's center

#### Scenario: Tap with position offset

- **GIVEN** a locator for an element
- **WHEN** `locator.tap().position(10, 10).await` is called
- **THEN** touch events are dispatched at (10, 10) relative to element

#### Scenario: Tap with force option

- **GIVEN** a locator for a hidden element
- **WHEN** `locator.tap().force(true).await` is called
- **THEN** the tap occurs without waiting for actionability

#### Scenario: Tap with modifiers

- **GIVEN** a locator for an element
- **WHEN** `locator.tap().modifiers(vec![Modifier::Shift]).await` is called
- **THEN** the touch event has Shift modifier active

### Requirement: Touch Viewport Coordinates

The system SHALL use viewport-relative CSS pixel coordinates.

#### Scenario: Coordinates are viewport-relative

- **GIVEN** a page with a scrolled viewport
- **WHEN** `page.touchscreen().tap(0.0, 0.0).await` is called
- **THEN** the touch occurs at the viewport origin (not document origin)

#### Scenario: Coordinates are in CSS pixels

- **GIVEN** a page with devicePixelRatio > 1
- **WHEN** `page.touchscreen().tap(100.0, 100.0).await` is called
- **THEN** the touch occurs at CSS pixel (100, 100)
- **AND** the underlying device coordinates are scaled appropriately

