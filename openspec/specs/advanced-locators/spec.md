# advanced-locators Specification

## Purpose
TBD - created by archiving change add-advanced-locators-assertions. Update Purpose after archive.
## Requirements
### Requirement: Locator Composition

The system SHALL support combining locators.

#### Scenario: And composition

- **GIVEN** two locators
- **WHEN** `locator1.and(locator2)` is called
- **THEN** a locator matching both conditions is returned

#### Scenario: Or composition

- **GIVEN** two locators
- **WHEN** `locator1.or(locator2)` is called
- **THEN** a locator matching either condition is returned

#### Scenario: Filter by text

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.filter().has_text("specific")` is called
- **THEN** only elements containing that text match

#### Scenario: Filter by has

- **GIVEN** a locator for container elements
- **WHEN** `locator.filter().has(child_locator)` is called
- **THEN** only containers with matching children match

#### Scenario: Filter by has_not

- **GIVEN** a locator for container elements
- **WHEN** `locator.filter().has_not(child_locator)` is called
- **THEN** only containers without matching children match

### Requirement: Additional Locator Methods

The system SHALL provide additional ways to locate elements.

#### Scenario: Get by alt text

- **GIVEN** a page with images
- **WHEN** `page.get_by_alt_text("Logo")` is called
- **THEN** images with that alt text are matched

#### Scenario: Get by title

- **GIVEN** a page with titled elements
- **WHEN** `page.get_by_title("Help")` is called
- **THEN** elements with that title are matched

#### Scenario: Nth element

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.nth(2)` is called
- **THEN** only the third element (0-indexed) matches

#### Scenario: First element

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.first()` is called
- **THEN** only the first element matches

#### Scenario: Last element

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.last()` is called
- **THEN** only the last element matches

### Requirement: Locator Queries

The system SHALL provide locator query methods.

#### Scenario: Count elements

- **GIVEN** a locator
- **WHEN** `locator.count().await` is called
- **THEN** the number of matching elements is returned

#### Scenario: Get all locators

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.all().await` is called
- **THEN** a Vec of locators (one per element) is returned

#### Scenario: All inner texts

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.all_inner_texts().await` is called
- **THEN** a Vec of inner text strings is returned

#### Scenario: All text contents

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.all_text_contents().await` is called
- **THEN** a Vec of text content strings is returned

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
- `paragraph` (p) - NOTE: This deviates from strict W3C ARIA 1.2 spec (which lists paragraph as "name prohibited") but is included for practical automation purposes to capture visible text content in snapshots
- Any element with explicit role allowing name from content

Implicit HTML element roles SHALL include:
- `p` -> `paragraph` role (per W3C ARIA 1.2 spec)

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

#### Scenario: Paragraph text content is captured for automation purposes
- **GIVEN** a page with `<p>Score: 82</p>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `paragraph "Score: 82"`
- **NOTE** This enables test automation to verify and interact with paragraph content

#### Scenario: Multiple paragraphs are captured
- **GIVEN** a page with multiple `<p>` elements containing different text
- **WHEN** capturing an ARIA snapshot
- **THEN** each paragraph SHALL appear in the snapshot with its text content

#### Scenario: Large DOM performance
- **GIVEN** a page with 100+ elements
- **WHEN** capturing an ARIA snapshot with refs
- **THEN** node resolution SHALL use concurrent CDP calls
- **AND** the snapshot SHALL complete within a reasonable time

### Requirement: Highlight

The system SHALL support visual element highlighting.

#### Scenario: Highlight element

- **GIVEN** a locator
- **WHEN** `locator.highlight().await` is called
- **THEN** the element is visually highlighted in the browser

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

### Requirement: Element Ref Resolution

The system SHALL support resolving snapshot refs back to elements for interaction.

#### Scenario: Resolve ref to element handle
- **GIVEN** a ref string from an aria snapshot (e.g., `e12345`)
- **WHEN** `page.element_from_ref("e12345").await` is called
- **THEN** an `ElementHandle` for that element is returned

#### Scenario: Resolve ref to locator
- **GIVEN** a ref string from an aria snapshot
- **WHEN** `page.locator_from_ref("e12345")` is called
- **THEN** a `Locator` targeting that element is returned with auto-waiting behavior

#### Scenario: Click element via ref
- **GIVEN** a button's ref from an aria snapshot
- **WHEN** `page.locator_from_ref(button_ref).click().await` is called
- **THEN** the button is clicked

#### Scenario: Invalid ref returns error
- **GIVEN** an invalid or malformed ref string
- **WHEN** `page.element_from_ref("invalid").await` is called
- **THEN** an appropriate error is returned

#### Scenario: Stale ref returns error
- **GIVEN** a ref for an element that has been removed from the DOM
- **WHEN** `page.element_from_ref(stale_ref).await` is called
- **THEN** an error indicating the element no longer exists is returned

#### Scenario: Ref format is protocol-agnostic
- **GIVEN** a ref string from an aria snapshot
- **WHEN** the ref is examined
- **THEN** it is an opaque string that does not expose CDP-specific details to users

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

