## MODIFIED Requirements

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

## ADDED Requirements

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
