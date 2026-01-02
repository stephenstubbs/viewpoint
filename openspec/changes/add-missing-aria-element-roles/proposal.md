# Change: Add Missing ARIA Element Role Mappings

## Why

The previous `fix-aria-snapshot-paragraph-capture` change added `<p>` element support, but many other HTML elements with implicit ARIA roles defined in the W3C ARIA in HTML spec are missing from our snapshot implementation. This causes content like blockquotes, figures, code blocks, emphasis text, and other semantic HTML elements to be invisible in accessibility snapshots.

Browser automation tools like Playwright capture these elements for practical automation purposes. We need parity to ensure ARIA snapshots accurately represent page content for MCP agents and test automation.

## Playwright Reference Implementation

Analyzed Playwright's `packages/injected/src/roleUtils.ts` (`kImplicitRoleByTagName` mapping):

**Currently in Viewpoint:**
- `a` (link), `article`, `aside` (complementary), `button`, `dialog`, `footer` (contentinfo), `form`, `h1-h6` (heading), `header` (banner), `img`, `input` (complex), `li` (listitem), `main`, `nav` (navigation), `ol` (list), `option`, `p` (paragraph), `progress` (progressbar), `section` (region), `select` (combobox), `table`, `tbody` (rowgroup), `td` (cell), `textarea` (textbox), `th` (columnheader), `tr` (row), `ul` (list)

**Missing from Viewpoint (in Playwright):**
- `area` → link (with href) | null
- `blockquote` → blockquote
- `caption` → caption
- `code` → code
- `datalist` → listbox
- `dd` → definition
- `del` → deletion
- `details` → group
- `dfn` → term
- `dt` → term
- `em` → emphasis
- `fieldset` → group
- `figure` → figure
- `hr` → separator
- `html` → document (skip - not needed for snapshots)
- `ins` → insertion
- `mark` → mark
- `math` → math
- `menu` → list
- `meter` → meter
- `optgroup` → group
- `output` → status
- `search` → search
- `strong` → strong
- `sub` → subscript
- `sup` → superscript
- `svg` → img
- `tfoot` → rowgroup
- `thead` → rowgroup
- `time` → time

## What Changes

1. **Add missing implicit role mappings** to `getImplicitRole` function in both `snapshot_with_refs.rs` and `snapshot_basic.rs`:
   - **Semantic text**: `blockquote` (blockquote), `code` (code), `em` (emphasis), `strong` (strong), `del` (deletion), `ins` (insertion), `sub` (subscript), `sup` (superscript), `mark` (mark)
   - **Definition list**: `dd` (definition), `dt` (term)
   - **Document structure**: `figure` (figure), `details` (group), `search` (search)
   - **Table/list/form**: `menu` (list), `datalist` (listbox), `fieldset` (group), `optgroup` (group), `thead` (rowgroup), `tfoot` (rowgroup), `caption` (caption)
   - **Other semantic**: `dfn` (term), `time` (time), `meter` (meter), `output` (status), `hr` (separator), `math` (math)
   - **Media**: `svg` (img), `area` (link with href)

2. **Add to `nameFromContentRoles`** for elements where text content should be captured as the accessible name (following Playwright's `allowsNameFromContent` logic):
   - Per Playwright's spec, when `targetDescendant` is true (which it is for most traversal), these roles get name from content: `caption`, `code`, `contentinfo`, `definition`, `deletion`, `emphasis`, `insertion`, `list`, `listitem`, `mark`, `none`, `paragraph`, `presentation`, `region`, `row`, `rowgroup`, `section`, `strong`, `subscript`, `superscript`, `table`, `term`, `time`
   - Add: `blockquote`, `code`, `emphasis`, `strong`, `deletion`, `insertion`, `subscript`, `superscript`, `term`, `time`, `status`, `figure`, `caption`, `definition`, `mark`

3. **Add integration tests** to verify the newly mapped elements appear in ARIA snapshots with appropriate roles and text content.

## Comparison Table

| Element | Playwright Role | Viewpoint Current | Action |
|---------|----------------|-------------------|--------|
| area | link (w/href) | ❌ missing | Add |
| blockquote | blockquote | ❌ missing | Add |
| caption | caption | ❌ missing | Add |
| code | code | ❌ missing | Add |
| datalist | listbox | ❌ missing | Add |
| dd | definition | ❌ missing | Add |
| del | deletion | ❌ missing | Add |
| details | group | ❌ missing | Add |
| dfn | term | ❌ missing | Add |
| dt | term | ❌ missing | Add |
| em | emphasis | ❌ missing | Add |
| fieldset | group | ❌ missing | Add |
| figure | figure | ❌ missing | Add |
| hr | separator | ❌ missing | Add |
| ins | insertion | ❌ missing | Add |
| mark | mark | ❌ missing | Add |
| math | math | ❌ missing | Add |
| menu | list | ❌ missing | Add |
| meter | meter | ❌ missing | Add |
| optgroup | group | ❌ missing | Add |
| output | status | ❌ missing | Add |
| search | search | ❌ missing | Add |
| strong | strong | ❌ missing | Add |
| sub | subscript | ❌ missing | Add |
| sup | superscript | ❌ missing | Add |
| svg | img | ❌ missing | Add |
| tfoot | rowgroup | ❌ missing | Add |
| thead | rowgroup | ❌ missing | Add |
| time | time | ❌ missing | Add |

**Total: 29 new element role mappings**

## Name From Content (Playwright Reference)

From Playwright's `allowsNameFromContent` function, roles that support name from content when traversing descendants:

```javascript
// Always allows name from content:
['button', 'cell', 'checkbox', 'columnheader', 'gridcell', 'heading', 'link', 
 'menuitem', 'menuitemcheckbox', 'menuitemradio', 'option', 'radio', 'row', 
 'rowheader', 'switch', 'tab', 'tooltip', 'treeitem']

// Descendant allows name from content:
['', 'caption', 'code', 'contentinfo', 'definition', 'deletion', 'emphasis', 
 'insertion', 'list', 'listitem', 'mark', 'none', 'paragraph', 'presentation', 
 'region', 'row', 'rowgroup', 'section', 'strong', 'subscript', 'superscript', 
 'table', 'term', 'time']
```

## Impact

- Affected specs: `advanced-locators`
- Affected code:
  - `crates/viewpoint-core/src/page/locator/aria_js/snapshot_with_refs.rs`
  - `crates/viewpoint-core/src/page/locator/aria_js/snapshot_basic.rs`
  - New test files for element role coverage

## Non-Goals

- Implementing conditional role logic like Playwright does for:
  - `footer`/`header` (conditional on ancestor)
  - `form`/`section` (conditional on accessible name)
  - `td` (gridcell vs cell based on table role)
  - `th` (complex column/row header logic)
  - `img` (presentation when alt="" and no global aria)
- These conditional behaviors can be added in a future change if needed
