# Change: Add Advanced Locators & Assertions

## Why

Complex testing scenarios require advanced element location and assertions:
- **Complex Selectors**: Combining multiple conditions
- **Aria Snapshots**: Accessibility tree assertions
- **Additional Locators**: getByAltText, getByTitle, nth, first, last
- **Enhanced Assertions**: toHaveCount, toHaveValues, toMatchAriaSnapshot

This is proposal 11 of 11 in the Playwright feature parity series (Chromium only).

## What Changes

### New Capabilities

1. **Locator Composition** - Combine locator conditions
   - `locator.and(other)` - match both conditions
   - `locator.or(other)` - match either condition
   - `locator.filter()` - filter by text, has, hasNot

2. **Additional Locators** - More ways to locate elements
   - `get_by_alt_text()` - by alt attribute
   - `get_by_title()` - by title attribute
   - `locator.nth(n)` - nth matching element
   - `locator.first()` - first matching
   - `locator.last()` - last matching

3. **Locator Properties** - Query locator state
   - `locator.count()` - number of matches
   - `locator.all()` - all matching locators
   - `locator.all_inner_texts()` - all inner texts
   - `locator.all_text_contents()` - all text contents

4. **Aria Snapshots** - Accessibility assertions
   - `expect(locator).to_match_aria_snapshot(snapshot)`
   - `locator.aria_snapshot()` - get aria tree

5. **Additional Assertions** - More expect methods
   - `to_have_count(n)` - assert element count
   - `to_have_values([...])` - assert select values
   - `to_have_class([...])` - assert multiple classes
   - `to_have_id(id)` - assert element ID

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1-10 | Previous | Complete |
| **11** | **Advanced Locators & Assertions** (this) | **Current** |

This completes Playwright feature parity for Chromium!

## Impact

- **New specs**: `advanced-locators`, `advanced-assertions`
- **Affected code**: 
  - `viewpoint-core/src/page/locator/` - add new methods
  - `viewpoint-test/src/expect/` - add new assertions
- **Breaking changes**: None
- **Dependencies**: Existing locator and assertion specs
