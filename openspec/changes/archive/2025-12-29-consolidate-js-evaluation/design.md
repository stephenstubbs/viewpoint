## Context

Currently, JavaScript evaluation in viewpoint-core is fragmented:

1. **Duplicate `evaluate_js` functions** exist in 3+ locations with nearly identical implementations
2. **Inline JavaScript** is built using `format!()` with complex template patterns
3. The `js!` macro exists but isn't used consistently

This creates maintenance burden and bypasses compile-time validation.

## Goals / Non-Goals

**Goals:**
- Single source of truth for low-level JS evaluation utilities
- Compile-time validation for all JavaScript via `js!` macro
- Cleaner, more maintainable code

**Non-Goals:**
- Changing the public API for `page.evaluate()` or `locator.evaluate()`
- Adding new evaluation features beyond consolidation
- Runtime JS validation (compile-time only)

## Decisions

### Decision 1: Add `JsEvaluator` trait to `viewpoint-js-core`

Create a trait that standardizes JavaScript evaluation:

```rust
// In viewpoint-js-core
pub trait JsEvaluator {
    type Error;
    
    /// Evaluate JavaScript and return JSON result
    async fn evaluate_js(&self, expression: &str) -> Result<serde_json::Value, Self::Error>;
}
```

**Why:** This provides a common interface without coupling to CDP specifics. The trait lives in `viewpoint-js-core` so it can be used by any crate.

**Alternatives considered:**
- Put in `viewpoint-core`: Would create circular dependency issues
- Free function: Doesn't capture the execution context (page, session)

### Decision 2: Extend `js!` macro for complex interpolation

The current `js!` macro supports `#{expr}` interpolation. We need to support:

1. **Value interpolation** (existing): `#{expr}` - converts Rust values to JS (quoted/escaped)
2. **Raw interpolation** (new): `@{expr}` - injects pre-built JS expressions without quoting

Example use case:
```rust
let selector_expr = selector.to_js_expression(); // Returns JS code like "document.querySelectorAll('.foo')"
let js = js!{
    (function() {
        const elements = Array.from(@{selector_expr});
        return elements.length;
    })()
};
```

**Why:** Many current `format!()` usages inject JS expressions (not values) into templates. The `@{expr}` syntax is visually distinct from both:
- `#{expr}` (value interpolation)
- `#[attr]` (Rust attributes)

**Alternatives considered:**
- Keep using `format!()` for complex cases: Loses compile-time validation
- Use only `#{}`: Ambiguous whether to quote/escape the value
- Use `#[expr]`: Confusing with Rust attribute syntax

### Decision 3: Incremental migration approach

Migrate files in order of complexity:
1. Simple cases first (debug, mouse_drag)
2. Medium complexity (queries, select)
3. Complex cases (helpers, frame_locator_actions)

This allows validating the approach before tackling complex cases.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| `js!` macro changes break existing usage | Extensive test coverage; new syntax is additive |
| Performance regression from macro overhead | Macro produces static strings when possible; benchmark if needed |
| Complex JS harder to read in macro form | Use `js!` for validation but can format across lines |

## Migration Plan

1. Add `JsEvaluator` trait and raw interpolation to `viewpoint-js-core`/`viewpoint-js`
2. Update `viewpoint-core` to use the trait
3. Migrate inline JS to `js!` macro file by file
4. Remove duplicate `evaluate_js` implementations
5. Update all affected tests

**Rollback:** Each file migration is independent; can revert individual files if issues arise.

## Open Questions

None - all major decisions resolved.
