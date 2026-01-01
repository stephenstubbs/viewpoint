# Change: Fix ARIA Snapshot Paragraph Text Capture

## Why

Paragraph elements (`<p>`) with text content are completely omitted from ARIA snapshots. This causes important content like "Score: 82" or descriptive text to be invisible in accessibility snapshots. Discovered while evaluating Context7's benchmark page where question scores in `<p>` elements were missing from the snapshot.

The root cause is that `<p>` elements are not included in the `getImplicitRole` function's role map. When an element has no role (implicit or explicit) and no child elements, the snapshot code returns `null`, discarding the text content entirely.

## W3C ARIA 1.2 Spec Consideration

According to [WAI-ARIA 1.2](https://www.w3.org/TR/wai-aria-1.2/#namefromcontent):

1. The `paragraph` role exists and maps to `<p>` elements
2. However, `paragraph` is listed under **"Roles which cannot be named (Name prohibited)"** - meaning it should NOT have an accessible name per the spec
3. The `paragraph` role is NOT in the "Roles Supporting Name from Content" list

This is intentional in the accessibility world because paragraphs are considered "static text" - assistive technologies read the text content directly rather than treating it as a named accessible object.

**However, for browser automation purposes**, we need to capture paragraph content in snapshots to enable:
- Verifying page content in tests
- Finding elements by text for automation
- Providing complete page context to AI/LLM tools

## What Changes

1. **Add `"p": "paragraph"`** to the `getImplicitRole` roleMap in both JavaScript snapshot functions. This aligns with the W3C spec's role mapping.

2. **Add `"paragraph"` to `nameFromContentRoles`** array. This is a **deviation from the strict W3C spec** for practical automation purposes. We capture the text content as the "name" field in the snapshot, even though technically paragraphs have "name prohibited" in the ARIA spec.

3. **Add integration tests** to verify paragraph elements appear in ARIA snapshots with their text content.

## Design Rationale

This is a pragmatic decision that prioritizes automation utility over strict ARIA compliance:

- **Playwright does this too** - they include static text content in their accessibility snapshots for the same practical reasons
- **Our snapshot is for automation, not AT** - We're building a snapshot for test automation and browser control, not implementing a screen reader
- **Text content is essential** - Without paragraph text, the snapshot loses significant page context

## Impact

- Affected specs: `advanced-locators` (Aria Snapshot requirement)
- Affected code:
  - `crates/viewpoint-core/src/page/locator/aria_js/snapshot_with_refs.rs`
  - `crates/viewpoint-core/src/page/locator/aria_js/snapshot_basic.rs`
  - Test files for ARIA snapshot functionality

## Alternative Considered

We could strictly follow the W3C spec and NOT give paragraphs a name, but instead include them in the tree with their text content as child text nodes. However, this would:
- Require more complex snapshot structure
- Make it harder to match/find paragraphs by content
- Differ from how other roles with text content are handled

The simpler approach of treating paragraph like other text-bearing roles provides a consistent, usable API.
