# Tasks

## 1. Add Missing Implicit Role Mappings

- [x] 1.1 Add semantic text element roles to `getImplicitRole` roleMap in `snapshot_with_refs.rs`:
  - `"blockquote": "blockquote"`
  - `"code": "code"`
  - `"em": "emphasis"`
  - `"strong": "strong"`
  - `"del": "deletion"`
  - `"ins": "insertion"`
  - `"sub": "subscript"`
  - `"sup": "superscript"`
  - `"mark": "mark"`
  
- [x] 1.2 Add definition list element roles to `getImplicitRole` roleMap in `snapshot_with_refs.rs`:
  - `"dd": "definition"`
  - `"dt": "term"`

- [x] 1.3 Add document structure element roles to `getImplicitRole` roleMap in `snapshot_with_refs.rs`:
  - `"figure": "figure"`
  - `"details": "group"`
  - `"search": "search"`
  
- [x] 1.4 Add table/list/form element roles to `getImplicitRole` roleMap in `snapshot_with_refs.rs`:
  - `"menu": "list"`
  - `"datalist": "listbox"`
  - `"fieldset": "group"`
  - `"thead": "rowgroup"`
  - `"tfoot": "rowgroup"`
  - `"caption": "caption"`
  - `"optgroup": "group"`

- [x] 1.5 Add other semantic element roles to `getImplicitRole` roleMap in `snapshot_with_refs.rs`:
  - `"dfn": "term"`
  - `"time": "time"`
  - `"meter": "meter"`
  - `"output": "status"`
  - `"hr": "separator"`
  - `"math": "math"`

- [x] 1.6 Add media/link element roles to `getImplicitRole` roleMap in `snapshot_with_refs.rs`:
  - `"svg": "img"`
  - `"area": el.hasAttribute("href") ? "link" : null` (conditional like `a`)

- [x] 1.7 Mirror all changes from 1.1-1.6 to `snapshot_basic.rs`

## 2. Update Name From Content Roles

- [x] 2.1 Add roles that should capture text content to `nameFromContentRoles` in `snapshot_with_refs.rs` (following Playwright's `allowsNameFromContent` logic):
  - `"blockquote"` (for practical automation purposes)
  - `"code"` (per Playwright descendant list)
  - `"emphasis"` (per Playwright descendant list)
  - `"strong"` (per Playwright descendant list)
  - `"deletion"` (per Playwright descendant list)
  - `"insertion"` (per Playwright descendant list)
  - `"subscript"` (per Playwright descendant list)
  - `"superscript"` (per Playwright descendant list)
  - `"term"` (per Playwright descendant list)
  - `"time"` (per Playwright descendant list)
  - `"status"` (per W3C spec)
  - `"figure"` (for practical automation purposes)
  - `"caption"` (per Playwright descendant list)
  - `"definition"` (per Playwright descendant list)
  - `"mark"` (per Playwright descendant list)
  - `"separator"` (for hr with text alternative)

- [x] 2.2 Mirror name from content changes to `snapshot_basic.rs`

## 3. Integration Tests

- [x] 3.1 Create `aria_snapshot_semantic_text_tests.rs` for semantic text elements:
  - Test `<blockquote>` captures with role and text
  - Test `<code>` captures with role and text
  - Test `<em>` captures with role and text
  - Test `<strong>` captures with role and text
  - Test `<del>` captures with role and text
  - Test `<ins>` captures with role and text
  - Test `<sub>` captures with role and text
  - Test `<sup>` captures with role and text
  - Test `<mark>` captures with role and text

- [x] 3.2 Create `aria_snapshot_definition_tests.rs` for definition list elements:
  - Test `<dl>` with `<dt>` and `<dd>` captures correctly
  - Test `<dt>` captures as term with text
  - Test `<dd>` captures as definition with text

- [x] 3.3 Create `aria_snapshot_structure_tests.rs` for document structure elements:
  - Test `<figure>` with `<figcaption>` captures correctly
  - Test `<details>` captures as group
  - Test `<search>` captures as search
  - Test `<fieldset>` with `<legend>` captures correctly
  - Test `<dfn>` captures as term with text

- [x] 3.4 Create `aria_snapshot_widget_tests.rs` for widget/form elements:
  - Test `<meter>` captures as meter with value attributes
  - Test `<output>` captures as status with text
  - Test `<time>` captures as time with text
  - Test `<datalist>` captures as listbox
  - Test `<optgroup>` captures as group

- [x] 3.5 Create `aria_snapshot_table_tests.rs` for table structure elements:
  - Test `<thead>` captures as rowgroup
  - Test `<tfoot>` captures as rowgroup
  - Test `<caption>` captures as caption with text

- [x] 3.6 Create `aria_snapshot_media_tests.rs` for media elements:
  - Test `<svg>` captures as img
  - Test `<area href="...">` captures as link
  - Test `<hr>` captures as separator

- [x] 3.7 Verify existing ARIA snapshot tests still pass

## 4. Documentation

- [x] 4.1 Update comments in snapshot JS code to document all supported elements
- [x] 4.2 Add note about Playwright parity and automation-specific name-from-content deviations

## Validation

After completing all tasks:
```bash
# Run unit tests
cargo test --workspace

# Run integration tests (requires Chromium)
cargo test --workspace --features integration
```

## Summary

Total new element role mappings: **29 elements**

Categories:
- Semantic text: 9 elements (blockquote, code, em, strong, del, ins, sub, sup, mark)
- Definition list: 2 elements (dd, dt)
- Document structure: 3 elements (figure, details, search)
- Table/list/form: 7 elements (menu, datalist, fieldset, thead, tfoot, caption, optgroup)
- Other semantic: 6 elements (dfn, time, meter, output, hr, math)
- Media/link: 2 elements (svg, area)
