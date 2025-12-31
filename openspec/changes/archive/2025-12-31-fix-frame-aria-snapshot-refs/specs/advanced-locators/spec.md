## MODIFIED Requirements

### Requirement: Frame Aria Snapshot

The system SHALL support capturing accessibility snapshots for individual frames with element references.

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

#### Scenario: Frame snapshot includes element refs
- **GIVEN** a Frame with interactive elements
- **WHEN** `frame.aria_snapshot().await` is called
- **THEN** every node in the snapshot has a unique `ref` field (e.g., `e12345`)

#### Scenario: Frame refs are resolvable
- **GIVEN** a ref from a frame's aria snapshot
- **WHEN** `page.locator_from_ref(ref)` is called
- **THEN** a Locator targeting that element is returned

### Requirement: Multi-Frame Aria Snapshot

The system SHALL support capturing complete accessibility snapshots across all frames with element references.

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

#### Scenario: Multi-frame snapshot includes element refs
- **GIVEN** a page with iframes containing interactive elements
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** every node in the snapshot (including those from frames) has a unique `ref` field

#### Scenario: Multi-frame refs are resolvable
- **GIVEN** a ref from a multi-frame snapshot
- **WHEN** `page.locator_from_ref(ref)` is called
- **THEN** a Locator targeting that element is returned (even if the element is in an iframe)
