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
