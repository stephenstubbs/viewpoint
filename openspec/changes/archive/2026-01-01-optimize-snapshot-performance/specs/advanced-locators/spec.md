## ADDED Requirements

### Requirement: Snapshot Performance Options

The system SHALL support configuration options for snapshot capture performance tuning.

#### Scenario: Configure max concurrency for node resolution
- **GIVEN** a page with many elements
- **WHEN** `page.aria_snapshot_with_options(SnapshotOptions { max_concurrency: Some(25), ..Default::default() }).await` is called
- **THEN** at most 25 concurrent CDP calls SHALL be made for node resolution
- **AND** the snapshot SHALL complete successfully

#### Scenario: Disable ref resolution for faster snapshots
- **GIVEN** a page with many elements
- **WHEN** `page.aria_snapshot_with_options(SnapshotOptions { include_refs: false, ..Default::default() }).await` is called
- **THEN** the snapshot SHALL be captured without resolving element refs
- **AND** all `node_ref` fields SHALL be `None`
- **AND** the capture SHALL complete faster than with refs enabled

#### Scenario: Default options match existing behavior
- **GIVEN** a page with elements
- **WHEN** `page.aria_snapshot().await` is called
- **THEN** the behavior SHALL match `page.aria_snapshot_with_options(SnapshotOptions::default()).await`
- **AND** element refs SHALL be included in the snapshot

## MODIFIED Requirements

### Requirement: Aria Snapshot

The system SHALL support accessibility tree snapshots with frame boundary tracking and element references.

The ARIA snapshot system SHALL capture accessibility tree structure including:
- Element roles (explicit or implicit from HTML semantics)
- Accessible names computed per W3C Accessible Name Computation spec
- Accessible descriptions
- State attributes (disabled, checked, expanded, selected, pressed)
- Heading levels
- Value attributes for range widgets

The accessible name computation SHALL:
1. Check `aria-labelledby` first (concatenate referenced element text)
2. Check `aria-label` attribute
3. For form inputs, check associated `<label>` elements
4. For images, use `alt` attribute
5. For elements with roles that allow "name from content", use text content
6. Use `title` attribute as final fallback

Roles that allow name from content include:
- `heading` (h1-h6)
- `link` (a with href)
- `button`
- `listitem` (li)
- `cell`, `columnheader`, `rowheader` (td, th)
- `option` (option)
- `tab`, `menuitem`, `treeitem`
- `legend`, `caption`
- Any element with explicit role allowing name from content

Node resolution for element refs SHALL be performed concurrently to optimize performance for large DOMs.

#### Scenario: Heading accessible name from text content
- **GIVEN** a page with `<h2>Page Title</h2>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `heading (level 2) "Page Title"`

#### Scenario: List item accessible name from text content
- **GIVEN** a page with `<li>List Item Text</li>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `listitem "List Item Text"`

#### Scenario: Table cell accessible name from text content
- **GIVEN** a page with `<td>Cell Value</td>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `cell "Cell Value"`

#### Scenario: aria-label takes precedence over text content
- **GIVEN** a page with `<h2 aria-label="Custom Name">Visible Text</h2>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `heading (level 2) "Custom Name"`

#### Scenario: Large DOM performance
- **GIVEN** a page with 100+ elements
- **WHEN** capturing an ARIA snapshot with refs
- **THEN** node resolution SHALL use concurrent CDP calls
- **AND** the snapshot SHALL complete within a reasonable time

### Requirement: Multi-Frame Aria Snapshot

The system SHALL support capturing complete accessibility snapshots across all frames with element references.

Frame snapshots SHALL be captured concurrently when capturing multi-frame snapshots to optimize performance.

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

#### Scenario: Parallel frame capture
- **GIVEN** a page with multiple same-origin iframes
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** child frame snapshots SHALL be captured concurrently
- **AND** the total capture time SHALL be approximately the time of the slowest frame (not cumulative)
