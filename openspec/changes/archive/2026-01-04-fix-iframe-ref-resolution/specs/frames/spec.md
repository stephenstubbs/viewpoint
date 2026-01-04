## MODIFIED Requirements

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

#### Scenario: Page aria_snapshot_with_frames stores iframe element refs

- **GIVEN** a page with an iframe containing interactive elements
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** element refs from the iframe are stored in Page's ref_map
- **AND** refs use the correct frame index in their format (e.g., `c0p0f1e5` for frame index 1)
- **AND** `page.locator_from_ref()` can resolve these refs

#### Scenario: Iframe element click via ref after aria_snapshot_with_frames

- **GIVEN** a page with an iframe containing a `<button>Submit</button>`
- **WHEN** `page.aria_snapshot_with_frames().await` captures a ref like `c0p0f1e3` for the button
- **AND** `page.locator_from_ref("c0p0f1e3").click().await` is called
- **THEN** the button inside the iframe is clicked

#### Scenario: Iframe element type via ref after aria_snapshot_with_frames

- **GIVEN** a page with an iframe containing an `<input type="text">`
- **WHEN** `page.aria_snapshot_with_frames().await` captures a ref for the input
- **AND** `page.locator_from_ref(ref).fill("test text").await` is called
- **THEN** the text is typed into the input inside the iframe

#### Scenario: Nested iframe element refs are resolvable

- **GIVEN** a page with nested iframes (iframe within iframe)
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** element refs from all nested iframes are stored in Page's ref_map
- **AND** each nested frame has a distinct frame index (f0, f1, f2, etc.)
- **AND** all refs can be resolved via `page.locator_from_ref()`
