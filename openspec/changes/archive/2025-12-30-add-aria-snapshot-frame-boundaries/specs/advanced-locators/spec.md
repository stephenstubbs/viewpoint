# advanced-locators Spec Delta

## MODIFIED Requirements

### Requirement: Aria Snapshot

The system SHALL support accessibility tree snapshots with frame boundary tracking.

#### Scenario: Get aria snapshot

- **GIVEN** a locator
- **WHEN** `locator.aria_snapshot().await` is called
- **THEN** the accessibility tree for that element is returned

#### Scenario: Snapshot includes roles

- **GIVEN** an aria snapshot
- **WHEN** the snapshot is examined
- **THEN** element roles are included

#### Scenario: Snapshot includes names

- **GIVEN** an aria snapshot
- **WHEN** the snapshot is examined
- **THEN** accessible names are included

#### Scenario: Snapshot marks iframe as frame boundary

- **GIVEN** a locator for an element containing an iframe
- **WHEN** `locator.aria_snapshot().await` is called
- **THEN** the iframe element has `role: "iframe"` and `is_frame: true`

#### Scenario: Snapshot includes iframe metadata

- **GIVEN** a locator for an element containing `<iframe name="payment" src="https://pay.example.com">`
- **WHEN** `locator.aria_snapshot().await` is called
- **THEN** the iframe node includes `frame_name: "payment"` and `frame_url: "https://pay.example.com"`

#### Scenario: Snapshot collects iframe refs

- **GIVEN** a page with multiple iframes
- **WHEN** `page.locator("body").aria_snapshot().await` is called
- **THEN** the root snapshot's `iframe_refs` contains refs for all iframes found

#### Scenario: Cross-origin iframe shows as boundary only

- **GIVEN** a page with a cross-origin iframe
- **WHEN** `locator.aria_snapshot().await` is called
- **THEN** the iframe is marked with `is_frame: true` but has no children content

## ADDED Requirements

### Requirement: Frame Aria Snapshot

The system SHALL support capturing accessibility snapshots for individual frames.

#### Scenario: Get frame aria snapshot

- **GIVEN** a Frame object for an iframe
- **WHEN** `frame.aria_snapshot().await` is called
- **THEN** the accessibility tree for that frame's content is returned

#### Scenario: Frame snapshot for same-origin iframe

- **GIVEN** a same-origin iframe
- **WHEN** `frame.aria_snapshot().await` is called
- **THEN** the complete accessibility tree of the iframe content is returned

#### Scenario: Nested frame snapshot

- **GIVEN** a Frame with nested iframes
- **WHEN** `frame.aria_snapshot().await` is called
- **THEN** nested iframes are marked as boundaries (not traversed automatically)

### Requirement: Multi-Frame Aria Snapshot

The system SHALL support capturing complete accessibility snapshots across all frames.

#### Scenario: Page-level multi-frame snapshot

- **GIVEN** a page with same-origin iframes
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** a complete accessibility tree with all frame content stitched together is returned

#### Scenario: Multi-frame snapshot with nested frames

- **GIVEN** a page with nested iframes (iframe inside iframe)
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** all levels of nested frame content are included

#### Scenario: Multi-frame snapshot with cross-origin frames

- **GIVEN** a page with mixed same-origin and cross-origin iframes
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** same-origin frame content is included; cross-origin frames show as boundaries only

#### Scenario: YAML output includes frame boundaries

- **GIVEN** a multi-frame snapshot
- **WHEN** `snapshot.to_yaml()` is called
- **THEN** frame boundaries are marked with `[frame-boundary]` annotation
