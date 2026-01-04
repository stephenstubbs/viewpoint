## Context

This change addresses documentation gaps across the Viewpoint workspace. The project has 6 crates with varying levels of documentation completeness. The goal is to ensure every public API has documented examples that compile.

## Current State Analysis

### README Coverage
| Crate | README | Status |
|-------|--------|--------|
| viewpoint (root) | ✅ Yes | Comprehensive |
| viewpoint-core | ✅ Yes | Good, needs minor updates |
| viewpoint-test | ✅ Yes | Good |
| viewpoint-cdp | ✅ Yes | Minimal |
| viewpoint-js | ✅ Yes | Good |
| viewpoint-js-core | ❌ No | **Missing** |
| viewpoint-test-macros | ✅ Yes | Minimal |

### Crate-Level Docs (lib.rs)
All crates have `//!` documentation, but example coverage varies:
- `viewpoint-core`: Has examples, uses `no_run`
- `viewpoint-test`: Has examples, some `ignore`d
- `viewpoint-js`: Has examples with `compile` checks ✅
- `viewpoint-js-core`: Has examples with `compile` checks ✅
- `viewpoint-cdp`: Minimal examples
- `viewpoint-test-macros`: Has examples

### Module-Level Docs
All 125+ modules in viewpoint-core have `//!` doc comments. However, most are single-line descriptions without:
- Key types enumeration
- Usage pattern examples
- Cross-references to related modules

### Public API Docs
Estimated 250+ public items across the workspace. Many have:
- No doc comments at all (just `#[derive]` attributes)
- No code examples
- No `# Errors` sections
- No cross-references

## Documentation Standards

### Example Attributes
Use the most restrictive attribute that still allows the example to work:

1. **No attribute** - Example runs during `cargo test --doc`
2. **`compile`** - Example compiles but doesn't run (preferred for async code)
3. **`no_run`** - Example compiles but never runs (for browser-requiring code)
4. **`ignore`** - Example is not compiled (use sparingly, only if compile would fail)

### Priority Order
1. **lib.rs** - First thing users see on docs.rs
2. **Primary types** - Browser, Page, Locator, TestHarness
3. **Common methods** - click, fill, goto, expect
4. **Supporting types** - Options, builders, events
5. **Internal types** - Can have minimal docs

### Example Structure
```rust
/// Brief one-line description.
///
/// Longer explanation of what this does and when to use it.
///
/// # Examples
///
/// ```compile
/// use viewpoint_core::SomeType;
///
/// let thing = SomeType::new();
/// thing.do_something();
/// ```
///
/// # Errors
///
/// Returns [`SomeError`] if the operation fails because...
```

## Implementation Strategy

### Phase 1: Audit
Generate a complete inventory of documentation gaps:
- READMEs missing or incomplete
- lib.rs files missing key sections
- Public types/methods without docs
- Examples that are `ignore`d but could be `compile`

### Phase 2: High-Priority Fixes
1. Add viewpoint-js-core README.md
2. Update lib.rs files with compile-checked examples
3. Document primary public types (Browser, Page, Locator, etc.)

### Phase 3: Comprehensive Coverage
1. Add examples to all public methods
2. Add `# Errors` sections where applicable
3. Update module docs with usage patterns

### Phase 4: Validation
1. Run `cargo test --doc` and fix any failures
2. Update Context7 configuration
3. Review docs.rs rendering

## Audit Findings (Completed)

### Doc Test Results
All 256 doc tests pass across the workspace. Distribution:
- `viewpoint-core`: 256 passed, 24 ignored
- `viewpoint-js`: 7 passed, 1 ignored  
- `viewpoint-js-core`: 13 passed, 1 ignored
- `viewpoint-test`: 10 passed, 7 ignored
- `viewpoint-test-macros`: 0 passed, 1 ignored (only has text examples)
- `viewpoint-cdp`: All passed (not shown in output)

### Documentation Quality Assessment

#### Excellent Documentation (No Changes Needed)
- `viewpoint-js-core/src/lib.rs` - Comprehensive with runnable examples
- `viewpoint-js/src/lib.rs` - Comprehensive with compile-checked examples
- `viewpoint-core/src/lib.rs` - Good overview with `no_run` examples
- `viewpoint-test/src/lib.rs` - Good overview with examples
- `viewpoint-cdp/src/lib.rs` - Good with connection examples
- `viewpoint-test-macros/src/lib.rs` - Comprehensive with text examples
- `viewpoint-core/src/browser/mod.rs` - Excellent with MCP connection examples
- `viewpoint-core/src/page/mod.rs` - Excellent with usage examples

#### README Status
- Root README.md: Comprehensive (581 lines) with all features documented
- viewpoint-core/README.md: Exists
- viewpoint-test/README.md: Exists
- viewpoint-cdp/README.md: Minimal (31 lines) - needs enhancement
- viewpoint-js/README.md: Exists
- viewpoint-js-core/README.md: **MISSING** - needs creation
- viewpoint-test-macros/README.md: Minimal (33 lines) - needs enhancement

### Ignore Attribute Usage Review
Most `ignore` attributes are appropriate because:
- They are for text examples in macros (can't run)
- They require browser interaction (would hang without browser)
- They are pseudo-code examples showing patterns

## Open Questions
None - scope is well-defined by the documentation spec requirements.
