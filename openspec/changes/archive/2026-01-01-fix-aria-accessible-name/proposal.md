# Change: Fix ARIA snapshot accessible name extraction

## Why

The ARIA snapshot functionality is not extracting accessible names for many element types. Currently, the `getAccessibleName()` function only handles buttons, links, images, and form inputs, missing headings and many other elements. This causes the snapshot to show elements without their text content (e.g., `heading (level 2)` instead of `heading (level 2) "Page Title"`).

Per the W3C Accessible Name and Description Computation spec, accessible names should be computed for all elements using a well-defined algorithm that considers:
1. `aria-labelledby`
2. `aria-label`
3. Native element semantics (labels, alt text, text content, etc.)
4. `title` attribute as fallback

## What Changes

- **Fix**: Implement proper accessible name computation following the W3C algorithm
- Elements that should derive accessible name from text content:
  - Headings (`<h1>` through `<h6>`)
  - List items (`<li>`)
  - Table cells (`<td>`, `<th>`)
  - Paragraphs and other text containers
  - `<summary>`, `<legend>`, `<caption>`, `<figcaption>`
  - `<option>`, `<optgroup>`
  - Any element with a role that allows name from content

## Impact

- Affected specs: `advanced-locators` (ARIA snapshot functionality)
- Affected code: `crates/viewpoint-core/src/page/locator/aria_js/snapshot_basic.rs` and `snapshot_with_refs.rs`
- This is a bug fix that restores expected behavior for accessibility snapshots
