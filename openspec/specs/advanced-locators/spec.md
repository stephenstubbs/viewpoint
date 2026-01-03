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

**Performance Requirements:**
- Snapshot capture SHALL use CDP's `Accessibility.getFullAXTree` to retrieve the complete tree in a single round-trip
- Tree transformation SHALL be performed in Rust using parallel processing (Rayon)
- Element refs SHALL be assigned during tree construction, not as a post-processing step
- No JavaScript execution SHALL be required for snapshot capture

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

Element refs SHALL use a context and page-scoped format: `c{contextIndex}p{pageIndex}e{counter}` where:
- `contextIndex` is the zero-based index of the browser context
- `pageIndex` is the zero-based index of the page/tab within the context
- `counter` is an incrementing number assigned during snapshot capture

Refs are generated fresh on each snapshot capture and do not persist across snapshots.

#### Scenario: Heading accessible name from text content
- **GIVEN** a page with `<h2>Page Title</h2>`
- **WHEN** capturing an ARIA snapshot
- **THEN** the snapshot SHALL include `heading (level 2) "Page Title"`

#### Scenario: Element refs include context and page index
- **GIVEN** a browser context at index 0 with a page at index 0
- **WHEN** `page.aria_snapshot().await` is called
- **THEN** each node's `node_ref` field uses format `c0p0e{n}` (e.g., `c0p0e1`, `c0p0e2`)

#### Scenario: Refs are sequential within a snapshot
- **GIVEN** a page with multiple elements
- **WHEN** `page.aria_snapshot().await` is called
- **THEN** refs are assigned sequentially: `c0p0e1`, `c0p0e2`, `c0p0e3`, etc.

#### Scenario: Multi-context ref disambiguation
- **GIVEN** two browser contexts at indices 0 and 1, each with a page
- **WHEN** snapshots are captured for pages in each context
- **THEN** context 0 refs have prefix `c0` (e.g., `c0p0e1`)
- **AND** context 1 refs have prefix `c1` (e.g., `c1p0e1`)

#### Scenario: Multi-tab ref disambiguation
- **GIVEN** a browser context at index 0 with pages at indices 0 and 1
- **WHEN** snapshots are captured for each page
- **THEN** page 0 refs have format `c0p0e{n}` (e.g., `c0p0e1`)
- **AND** page 1 refs have format `c0p1e{n}` (e.g., `c0p1e1`)

#### Scenario: Large DOM performance
- **GIVEN** a page with 100+ elements
- **WHEN** capturing an ARIA snapshot with refs
- **THEN** the snapshot SHALL be captured via a single CDP call
- **AND** tree transformation SHALL use parallel processing
- **AND** the snapshot SHALL complete in under 50ms for 100 elements

#### Scenario: No JavaScript execution for snapshot
- **GIVEN** a page with elements
- **WHEN** `page.aria_snapshot().await` is called
- **THEN** no `Runtime.evaluate` or JavaScript execution SHALL occur
- **AND** the snapshot SHALL be built entirely from CDP Accessibility domain data

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

Resolution SHALL validate that the ref's context index matches the target context and the page index matches the target page.

When a ref cannot be resolved, the system SHALL return a clear error suggesting the user capture a new snapshot.

#### Scenario: Resolve ref to element handle
- **GIVEN** a ref string from an aria snapshot (e.g., `c0p0e1`)
- **WHEN** `page.element_from_ref("c0p0e1").await` is called on context 0, page 0
- **THEN** an `ElementHandle` for that element is returned

#### Scenario: Resolve ref to locator
- **GIVEN** a ref string from an aria snapshot
- **WHEN** `page.locator_from_ref("c0p0e1")` is called
- **THEN** a `Locator` targeting that element is returned with auto-waiting behavior

#### Scenario: Click element via ref
- **GIVEN** a button's ref from an aria snapshot
- **WHEN** `page.locator_from_ref(button_ref).click().await` is called
- **THEN** the button is clicked

#### Scenario: Context index mismatch returns error
- **GIVEN** a ref with context index 0 (`c0p0e1`)
- **WHEN** `page.element_from_ref("c0p0e1").await` is called on a page in context 1
- **THEN** an error is returned indicating context index mismatch

#### Scenario: Page index mismatch returns error
- **GIVEN** a ref with page index 0 (`c0p0e1`)
- **WHEN** `page.element_from_ref("c0p0e1").await` is called on page index 1
- **THEN** an error is returned indicating page index mismatch

#### Scenario: Stale ref returns helpful error
- **GIVEN** a ref for an element that no longer exists
- **WHEN** `page.element_from_ref(stale_ref).await` is called
- **THEN** an error is returned with message suggesting to capture a new snapshot

#### Scenario: Invalid ref format returns error
- **GIVEN** an invalid or malformed ref string
- **WHEN** `page.element_from_ref("invalid").await` is called
- **THEN** an appropriate error is returned indicating invalid format

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

### Requirement: Snapshot Versioning

The system SHALL track snapshot versions for incremental update support.

Each snapshot capture SHALL increment the version number stored on the Page.

The version number SHALL be included in the returned snapshot.

#### Scenario: Snapshot includes version number
- **GIVEN** a page with elements
- **WHEN** `page.aria_snapshot().await` is called
- **THEN** the returned snapshot includes a `version` field with a positive integer

#### Scenario: Version increments on each capture
- **GIVEN** a page with a previous snapshot at version N
- **WHEN** `page.aria_snapshot().await` is called again
- **THEN** the returned snapshot has version N+1

#### Scenario: Version resets after navigation
- **GIVEN** a page with a snapshot at version N
- **WHEN** the page navigates to a new URL
- **AND** `page.aria_snapshot().await` is called
- **THEN** a full snapshot is returned (not a diff)

### Requirement: Incremental Snapshot Diffing

The system SHALL support capturing incremental snapshots that return only changes since a previous version.

**Performance Requirements:**
- Content hashes SHALL be computed during tree construction (not as a separate pass)
- Diff comparison SHALL use parallel processing (Rayon)
- If root hashes match, the system SHALL return an empty diff immediately (fast-path)
- Unchanged subtrees SHALL NOT be traversed during diff computation

When `since_version` option is provided, the system SHALL compare the new snapshot against the stored previous snapshot and return a diff.

The diff SHALL include:
- `added`: Nodes that exist in the new snapshot but not in the previous
- `removed`: Refs that existed in the previous snapshot but not in the new
- `modified`: Nodes whose content (role, name, states) changed
- `unchanged_count`: Number of nodes that remained the same

#### Scenario: Incremental snapshot with no changes
- **GIVEN** a page with a snapshot at version N
- **WHEN** `page.aria_snapshot_with_options(SnapshotOptions::default().since_version(N)).await` is called
- **AND** the DOM has not changed
- **THEN** a `SnapshotResult::Diff` is returned with empty added/removed/modified and unchanged_count > 0

#### Scenario: Incremental snapshot detects added elements
- **GIVEN** a page with a snapshot at version N
- **WHEN** a new button is added to the DOM
- **AND** `page.aria_snapshot_with_options(SnapshotOptions::default().since_version(N)).await` is called
- **THEN** a `SnapshotResult::Diff` is returned with the new button in `added`

#### Scenario: Incremental snapshot detects removed elements
- **GIVEN** a page with a button in snapshot at version N
- **WHEN** the button is removed from the DOM
- **AND** `page.aria_snapshot_with_options(SnapshotOptions::default().since_version(N)).await` is called
- **THEN** a `SnapshotResult::Diff` is returned with the button's ref in `removed`

#### Scenario: Incremental snapshot detects modified elements
- **GIVEN** a page with a button named "Submit" in snapshot at version N
- **WHEN** the button's text is changed to "Send"
- **AND** `page.aria_snapshot_with_options(SnapshotOptions::default().since_version(N)).await` is called
- **THEN** a `SnapshotResult::Diff` is returned with the button in `modified`

#### Scenario: Full snapshot returned when no previous exists
- **GIVEN** a page with no previous snapshot
- **WHEN** `page.aria_snapshot_with_options(SnapshotOptions::default().since_version(1)).await` is called
- **THEN** a `SnapshotResult::Full` is returned with the complete snapshot

#### Scenario: Full snapshot returned on version mismatch
- **GIVEN** a page with snapshot at version 5
- **WHEN** `page.aria_snapshot_with_options(SnapshotOptions::default().since_version(3)).await` is called
- **THEN** a `SnapshotResult::Full` is returned (requested version doesn't match stored)

### Requirement: Ref Stability Across Incremental Snapshots

The system SHALL preserve refs for unchanged nodes across incremental snapshots.

Unchanged nodes SHALL keep the same ref from the previous snapshot.

Added nodes SHALL receive new refs with fresh counter values.

Modified nodes SHALL keep the same ref (identity preserved, content changed).

#### Scenario: Unchanged node refs remain valid after diff
- **GIVEN** a page with a button ref `c0p0e5` in snapshot at version N
- **WHEN** a different element is added to the DOM
- **AND** an incremental snapshot is captured
- **THEN** the button's ref `c0p0e5` remains valid for interaction

#### Scenario: Added nodes get new refs
- **GIVEN** a page with snapshot at version N
- **WHEN** a new button is added to the DOM
- **AND** an incremental snapshot is captured
- **THEN** the new button has a ref with a counter value higher than any existing ref

#### Scenario: Modified nodes keep their refs
- **GIVEN** a page with a button ref `c0p0e5` named "Submit" in snapshot at version N
- **WHEN** the button's text is changed to "Send"
- **AND** an incremental snapshot is captured
- **THEN** the button's ref remains `c0p0e5`
- **AND** the ref resolves to the modified button

#### Scenario: Removed node refs become invalid
- **GIVEN** a page with a button ref `c0p0e5` in snapshot at version N
- **WHEN** the button is removed from the DOM
- **AND** an incremental snapshot is captured
- **THEN** attempting to resolve `c0p0e5` returns an error suggesting to capture a new snapshot

### Requirement: Parallel Frame Capture

The system SHALL capture multi-frame snapshots with maximum parallelism.

Frame snapshots SHALL be captured concurrently using async I/O.

Tree stitching after capture SHALL use parallel processing.

#### Scenario: Parallel capture of multiple frames
- **GIVEN** a page with 5 same-origin iframes
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** all 5 frame snapshots SHALL be captured concurrently
- **AND** the total time SHALL be approximately the time of the slowest frame (not cumulative)

#### Scenario: Streaming frame results
- **GIVEN** a page with multiple iframes of varying complexity
- **WHEN** `page.aria_snapshot_with_frames().await` is called
- **THEN** frame results SHALL be processed as they complete
- **AND** fast frames SHALL NOT wait for slow frames before being processed

#### Scenario: Parallel tree stitching
- **GIVEN** captured frame snapshots ready for stitching
- **WHEN** frame content is stitched into the main snapshot
- **THEN** stitching operations SHALL use parallel processing where beneficial

