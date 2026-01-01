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
