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

#### Scenario: Snapshot includes element refs for all nodes
- **GIVEN** a page with interactive and non-interactive elements
- **WHEN** `locator.aria_snapshot().await` is called
- **THEN** every node in the snapshot has a unique `ref` field (e.g., `e12345`)

#### Scenario: Snapshot refs are stable identifiers
- **GIVEN** an aria snapshot with element refs
- **WHEN** the DOM has not been modified
- **THEN** the same ref can be used to resolve to the original element

#### Scenario: Dynamically created elements get refs
- **GIVEN** a page where JavaScript dynamically creates elements
- **WHEN** `locator.aria_snapshot().await` is called after element creation
- **THEN** the dynamically created elements have valid refs in the snapshot

### Requirement: Highlight

The system SHALL support visual element highlighting.

#### Scenario: Highlight element

- **GIVEN** a locator
- **WHEN** `locator.highlight().await` is called
- **THEN** the element is visually highlighted in the browser

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

