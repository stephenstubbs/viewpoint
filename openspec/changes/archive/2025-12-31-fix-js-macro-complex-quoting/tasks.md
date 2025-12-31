# Tasks: Fix js! Macro for Complex JavaScript Quoting

## 1. Source Text Extraction

- [x] 1.1 Add source text extraction from span
  - Implement `extract_source_text()` function
  - Handle `Span::source_text()` return value
  - Strip surrounding delimiters (`{` `}`)
  - Add fallback detection for when source text is unavailable

- [x] 1.2 Add fallback path for token-based parsing
  - Keep existing `tokens_to_js_string()` as fallback
  - Route to fallback when `source_text()` returns `None`
  - Add warning/note when fallback is used with problematic syntax

## 2. JavaScript-Aware Scanner

- [x] 2.1 Implement scanner state machine
  - Define `ScanState` enum with all states
  - Implement state transitions for string literals (single, double, template)
  - Implement state transitions for regex literals
  - Implement state transitions for comments (line and block)

- [x] 2.2 Handle escape sequences
  - Track backslash escapes in strings
  - Don't end string on escaped quote characters
  - Handle template literal escapes

- [x] 2.3 Implement regex vs division disambiguation
  - Track preceding tokens/characters
  - Apply heuristics to determine if `/` starts regex
  - Handle edge cases (return /regex/, etc.)

- [x] 2.4 Handle regex character classes
  - Track `[` and `]` inside regex
  - Don't end regex on `/` inside character class

- [x] 2.5 Handle template literal nesting
  - Track `${` and `}` depth inside template literals
  - Allow nested template literals inside `${}`

## 3. Interpolation Detection

- [x] 3.1 Update interpolation parsing to use scanner
  - Detect `#{` and `@{` only in code context (not inside JS strings)
  - Extract Rust expressions between braces
  - Handle nested braces in Rust expressions
  - Parse expressions with `syn`

- [x] 3.2 Handle interpolation inside template literals
  - `#{expr}` inside `` `...` `` is still our interpolation
  - Distinguish from JavaScript's `${expr}`

## 4. Integration

- [x] 4.1 Connect scanner to existing validation
  - Create validation source (replace interpolations with `null`)
  - Pass to swc parser for validation
  - Preserve error position mapping

- [x] 4.2 Connect scanner to existing code generation
  - Build segment list (Literal, ValueInterpolation, RawInterpolation)
  - Use existing `generate_interpolated_code()`

## 5. Testing

- [x] 5.1 Add unit tests for scanner state machine
  - Test state transitions for each string type
  - Test regex detection and boundaries
  - Test comment handling
  - Test escape sequence handling

- [x] 5.2 Add unit tests for regex vs division
  - Test division: `a / b / c`
  - Test regex after operators: `x = /pattern/`
  - Test regex after keywords: `return /pattern/`
  - Test regex after punctuation: `(/pattern/)`

- [x] 5.3 Add integration tests for JavaScript patterns
  - Single-quoted strings: `js!{ 'hello' }`
  - Template literals: `js!{ `hello ${name}` }`
  - Template with our interpolation: `js!{ `value: #{x}` }`
  - Regex literals: `js!{ /^test/.test(s) }`
  - XPath single quotes: `js!{ document.evaluate("//div[@class='x']", ...) }`
  - XPath double quotes: `js!{ document.querySelector('[data-id="x"]') }`
  - CSS selectors mixed quotes
  - JSON strings in JavaScript
  - Nested template literals

- [x] 5.4 Add integration tests for interpolation
  - Value interpolation with all JS string types present
  - Raw interpolation with complex JS
  - Mixed interpolation in complex code

- [x] 5.5 Add compile-fail tests
  - Invalid JavaScript still fails
  - Unclosed string literal
  - Unclosed template literal
  - Invalid regex
  - Unclosed interpolation

- [x] 5.6 Add fallback behavior tests
  - Test behavior when source_text() unavailable
  - Verify warning/error messaging

## 6. Documentation

- [x] 6.1 Update `viewpoint-js/README.md`
  - Document newly supported syntax
  - Add examples for XPath, CSS selectors, templates, regex
  - Note any limitations

- [x] 6.2 Update `viewpoint-js/src/lib.rs` doc comments
  - Add examples showing complex quoting
  - Document any edge cases

- [x] 6.3 Update `openspec/project.md` JavaScript section
  - Add examples of newly supported patterns

## 7. Validation

- [x] 7.1 Run full test suite
  - `cargo test --workspace`
  - `cargo test --workspace --features integration`

- [x] 7.2 Verify backward compatibility
  - All existing tests pass unchanged
  - Existing usage in codebase still works

- [x] 7.3 Test in real-world scenarios
  - XPath selectors in browser automation
  - CSS attribute selectors
  - Template literal generation

## Dependencies

```
1.1 ──► 1.2
         │
         ▼
2.1 ──► 2.2 ──► 2.3 ──► 2.4 ──► 2.5
                                 │
                                 ▼
                        3.1 ──► 3.2
                                 │
                                 ▼
                        4.1 ──► 4.2
                                 │
                                 ▼
                    5.1 through 5.6 (parallel)
                                 │
                                 ▼
                    6.1 through 6.3 (parallel)
                                 │
                                 ▼
                    7.1 ──► 7.2 ──► 7.3
```

## Complexity Estimate

| Task Group | Complexity | Estimated Effort |
|------------|------------|------------------|
| 1. Source extraction | Low | Small |
| 2. Scanner | High | Medium-Large |
| 3. Interpolation | Medium | Small-Medium |
| 4. Integration | Low | Small |
| 5. Testing | Medium | Medium |
| 6. Documentation | Low | Small |
| 7. Validation | Low | Small |

**Total**: Medium-Large effort, primarily in the scanner implementation.
