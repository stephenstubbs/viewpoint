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
- `blockquote` (blockquote) - NOTE: Deviates from W3C spec for automation purposes
- `code` (code) - NOTE: Deviates from W3C spec for automation purposes
- `emphasis` (em) - NOTE: Deviates from W3C spec for automation purposes
- `strong` (strong) - NOTE: Deviates from W3C spec for automation purposes
- `deletion` (del) - NOTE: Deviates from W3C spec for automation purposes
- `insertion` (ins) - NOTE: Deviates from W3C spec for automation purposes
- `subscript` (sub) - NOTE: Deviates from W3C spec for automation purposes
- `superscript` (sup) - NOTE: Deviates from W3C spec for automation purposes
- `term` (dfn) - NOTE: Deviates from W3C spec for automation purposes
- `time` (time) - NOTE: Deviates from W3C spec for automation purposes
- `status` (output) - Per W3C spec
- `figure` (figure) - NOTE: Deviates from W3C spec for automation purposes
- Any element with explicit role allowing name from content

Implicit HTML element roles SHALL include:
- `p` -> `paragraph` role (per W3C ARIA 1.2 spec)
- `blockquote` -> `blockquote` role (per W3C ARIA in HTML spec)
- `code` -> `code` role (per W3C ARIA in HTML spec)
- `em` -> `emphasis` role (per W3C ARIA in HTML spec)
- `strong` -> `strong` role (per W3C ARIA in HTML spec)
- `del` -> `deletion` role (per W3C ARIA in HTML spec)
- `ins` -> `insertion` role (per W3C ARIA in HTML spec)
- `sub` -> `subscript` role (per W3C ARIA in HTML spec)
- `sup` -> `superscript` role (per W3C ARIA in HTML spec)
- `figure` -> `figure` role (per W3C ARIA in HTML spec)
- `details` -> `group` role (per W3C ARIA in HTML spec)
- `fieldset` -> `group` role (per W3C ARIA in HTML spec)
- `address` -> `group` role (per W3C ARIA in HTML spec)
- `hgroup` -> `group` role (per W3C ARIA in HTML spec)
- `search` -> `search` role (per W3C ARIA in HTML spec)
- `menu` -> `list` role (per W3C ARIA in HTML spec)
- `datalist` -> `listbox` role (per W3C ARIA in HTML spec)
- `dfn` -> `term` role (per W3C ARIA in HTML spec)
- `time` -> `time` role (per W3C ARIA in HTML spec)
- `meter` -> `meter` role (per W3C ARIA in HTML spec)
- `output` -> `status` role (per W3C ARIA in HTML spec)
- `hr` -> `separator` role (per W3C ARIA in HTML spec)
- `math` -> `math` role (per W3C ARIA in HTML spec)
- `thead` -> `rowgroup` role (per W3C ARIA in HTML spec)
- `tfoot` -> `rowgroup` role (per W3C ARIA in HTML spec)
- `caption` -> `caption` role (per W3C ARIA in HTML spec)
- `optgroup` -> `group` role (per W3C ARIA in HTML spec)

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

#### Scenario: Blockquote text content is captured
- **GIVEN** a page with `<blockquote>Famous quote here</blockquote>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `blockquote "Famous quote here"`

#### Scenario: Code element text content is captured
- **GIVEN** a page with `<code>console.log("hello")</code>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `code "console.log("hello")"`

#### Scenario: Emphasis element text content is captured
- **GIVEN** a page with `<em>Important text</em>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `emphasis "Important text"`

#### Scenario: Strong element text content is captured
- **GIVEN** a page with `<strong>Bold statement</strong>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `strong "Bold statement"`

#### Scenario: Deletion element text content is captured
- **GIVEN** a page with `<del>Removed text</del>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `deletion "Removed text"`

#### Scenario: Insertion element text content is captured
- **GIVEN** a page with `<ins>Added text</ins>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `insertion "Added text"`

#### Scenario: Subscript element text content is captured
- **GIVEN** a page with `H<sub>2</sub>O`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `subscript "2"`

#### Scenario: Superscript element text content is captured
- **GIVEN** a page with `E=mc<sup>2</sup>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `superscript "2"`

#### Scenario: Figure element text content is captured
- **GIVEN** a page with `<figure><img src="..." alt="Chart"><figcaption>Sales data</figcaption></figure>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `figure` with child content

#### Scenario: Definition term element text content is captured
- **GIVEN** a page with `<dfn>API</dfn> stands for Application Programming Interface`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `term "API"`

#### Scenario: Time element text content is captured
- **GIVEN** a page with `<time datetime="2024-01-01">January 1st, 2024</time>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `time "January 1st, 2024"`

#### Scenario: Output element text content is captured
- **GIVEN** a page with `<output>42</output>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `status "42"`

#### Scenario: Meter element is captured
- **GIVEN** a page with `<meter value="0.6">60%</meter>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `meter` with value attributes

#### Scenario: Horizontal rule is captured as separator
- **GIVEN** a page with `<hr>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `separator`

#### Scenario: Search element is captured
- **GIVEN** a page with `<search><input type="search"></search>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `search` containing the input

#### Scenario: Details element is captured as group
- **GIVEN** a page with `<details><summary>More info</summary>Content here</details>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `group` with content

#### Scenario: Fieldset element is captured as group
- **GIVEN** a page with `<fieldset><legend>User Info</legend>...</fieldset>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `group` with the fieldset content

#### Scenario: Address element is captured as group
- **GIVEN** a page with `<address>123 Main St</address>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `group`
