# Design: Custom JavaScript Tokenizer for js! Macro

## Context

The `js!` macro currently works by:
1. Accepting Rust tokens as input (`js!{ code }`)
2. Converting those tokens back to a JavaScript string
3. Validating the JavaScript with swc parser
4. Generating code that either returns `&'static str` or `String`

The problem is that Rust's tokenizer rejects valid JavaScript syntax:
- `'string'` → Rust sees a multi-character char literal (error)
- `` `template` `` → Backticks aren't valid Rust tokens (error)
- `/regex/` → Parsed as division operators (wrong output)
- `'[data-id="x"]'` → Invalid Rust syntax (error)

## Goals

1. Support all JavaScript string literal types (single, double, template)
2. Support regex literals
3. Support complex quoting patterns (XPath, CSS selectors)
4. Maintain compile-time JavaScript validation
5. Keep Rust interpolation working (`#{expr}` and `@{expr}`)
6. **Preserve IDE syntax highlighting** (keep `js!{ }` syntax)
7. Backward compatible - existing code continues to work
8. Support JavaScript template literal `${expr}` syntax (JS-side interpolation)

## Non-Goals

- Changing the macro invocation syntax
- Runtime JavaScript validation
- Supporting JavaScript features beyond ES2020
- Full JavaScript semantic analysis
- Replacing Rust interpolation with JS interpolation (they serve different purposes)

## Decisions

### Decision 1: Extract Source Text from Span Instead of Tokens

**What**: Use `proc_macro2::Span` to get the original source text, bypassing Rust's tokenizer.

**How**:
```rust
// Instead of converting tokens to string:
fn tokens_to_js_string(tokens: &TokenStream2) -> String { ... }

// Extract the raw source text from the span:
fn extract_source_from_span(input: TokenStream) -> String {
    // Get the span covering the entire input
    // Use span.source_text() or fallback to positional extraction
    // This gives us the raw characters the user typed
}
```

**Why**:
- Preserves exact user input including single quotes, backticks, etc.
- No tokenization errors for valid JavaScript
- IDE still sees `js!{ }` and can provide highlighting
- Simpler than building a full JavaScript lexer

**Challenges**:
- `Span::source_text()` is only available with `proc_macro` (not `proc_macro2` in all cases)
- Need fallback for when source text isn't available (e.g., macro-generated code)
- Must handle the delimiters (`{` `}`) correctly

### Decision 2: Implement Minimal JavaScript Lexer for Interpolation Detection

**What**: Build a simple state machine that understands JavaScript string/regex boundaries to correctly identify `#{expr}` and `@{expr}` interpolation markers.

**Why**: We need to distinguish between:
- `#{expr}` in code context → Rust interpolation
- `#{expr}` inside a JS string → Literal characters (unlikely but possible)
- Template literal `${expr}` → JavaScript interpolation, not ours

**State Machine States**:
```
Normal → String (on " or ' or `)
       → Regex (on / in expression context)
       → LineComment (on //)
       → BlockComment (on /*)
       → Interpolation (on #{ or @{)

String → Normal (on closing quote, respecting escapes)
Regex → Normal (on closing /, respecting escapes and char classes)
```

### Decision 3: Handle Regex vs Division Ambiguity

**What**: Track enough context to distinguish `/regex/` from `a / b / c`.

**Heuristic**: A `/` starts a regex if preceded by:
- Start of input
- `(`, `[`, `{`, `,`, `;`, `:`
- Operators: `=`, `+=`, `-=`, `!`, `?`, etc.
- Keywords: `return`, `case`, `throw`, `in`, `of`, `typeof`, `void`, `delete`, `new`

This matches how JavaScript engines disambiguate.

### Decision 4: Preserve Whitespace and Formatting

**What**: Keep the original whitespace and formatting from the source.

**Why**: 
- JavaScript is whitespace-insensitive for most purposes
- But template literals preserve whitespace
- And users expect their code to look the same in output (for debugging)

## Architecture

```
js!{ user's JavaScript code }
           │
           ▼
┌─────────────────────────────┐
│  Extract source from span   │  ← Get raw text, bypass Rust tokenizer
└─────────────────────────────┘
           │
           ▼
┌─────────────────────────────┐
│  JavaScript-aware scanner   │  ← State machine for strings/regex/comments
│  - Find interpolations      │
│  - Track string boundaries  │
└─────────────────────────────┘
           │
           ├─── Interpolations found ───► Parse Rust expressions with syn
           │
           ▼
┌─────────────────────────────┐
│  Create validation source   │  ← Replace interpolations with `null`
└─────────────────────────────┘
           │
           ▼
┌─────────────────────────────┐
│  Validate with swc parser   │  ← Existing validation logic
└─────────────────────────────┘
           │
           ▼
┌─────────────────────────────┐
│  Generate output code       │  ← Existing code generation
└─────────────────────────────┘
```

## Implementation Details

### Source Text Extraction

```rust
pub fn js_impl(input: TokenStream) -> TokenStream {
    // Try to get source text directly
    let source = match extract_source_text(&input) {
        Some(s) => s,
        None => {
            // Fallback to token-based approach for macro-generated code
            return js_impl_from_tokens(input);
        }
    };
    
    // Remove surrounding braces if present
    let js_source = strip_delimiters(&source);
    
    // Continue with JavaScript-aware parsing...
}

fn extract_source_text(input: &TokenStream) -> Option<String> {
    // proc_macro::Span::source_text() returns Option<String>
    // We need the span of the entire input
    let span = input.span();
    span.source_text()
}
```

### JavaScript Scanner States

```rust
enum ScanState {
    Normal,
    DoubleString,      // Inside "..."
    SingleString,      // Inside '...'
    TemplateString,    // Inside `...`
    TemplatExpr,       // Inside ${...} within template
    Regex,             // Inside /.../
    RegexCharClass,    // Inside [...] within regex
    LineComment,       // After //
    BlockComment,      // Inside /* ... */
    ValueInterpolation,  // Inside #{...}
    RawInterpolation,    // Inside @{...}
}
```

### Handling Edge Cases

1. **Escaped characters**: Track `\` and don't end string on `\"`, `\'`, `` \` ``
2. **Template nesting**: `${...}` can contain more template literals
3. **Regex character classes**: `[/]` doesn't end the regex
4. **Interpolation in template**: `#{x}` inside `` `...` `` is still our interpolation
5. **Newlines**: Allowed in template literals, not in regular strings

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| `source_text()` not available in all contexts | Fallback to token-based parsing with clear error message |
| Edge cases in regex/division disambiguation | Use well-tested heuristics from JS parsers; swc validates the result |
| Complex state machine has bugs | Comprehensive test suite; swc catches syntax errors anyway |
| Performance overhead of scanning | Negligible for typical macro inputs (< 1KB of JS) |

## Fallback Strategy

If `Span::source_text()` returns `None` (e.g., in some proc-macro test harnesses or when code is macro-generated):

1. Fall back to current token-based implementation
2. Emit a warning if JavaScript syntax that requires raw access is detected
3. User can work around by using interpolation: `@{r#"'string'"#}`

## Testing Strategy

1. **Unit tests for scanner**: Test each state transition
2. **Integration tests for all JS patterns**:
   - Single-quoted strings
   - Template literals with `${}`
   - Regex literals
   - XPath expressions
   - CSS selectors with mixed quotes
3. **Edge case tests**:
   - Escaped quotes
   - Nested templates
   - Regex with character classes
   - Division vs regex
4. **Compile-fail tests**: Invalid JS still fails

## Interpolation Design: Rust vs JavaScript

A key design decision is maintaining **both** Rust interpolation (`#{expr}`, `@{expr}`) and JavaScript interpolation (`${expr}`). They serve fundamentally different purposes:

### Why Both Are Necessary

| Interpolation | Language Boundary | Use Case |
|---------------|-------------------|----------|
| `#{expr}` | Rust → JS value | Convert Rust value to JS literal via `ToJsValue` trait |
| `@{expr}` | Rust → JS code | Inject pre-built JavaScript expression string |
| `${expr}` | JS → JS value | JavaScript template literal interpolation |

### Example: The Three Interpolations Working Together

```rust
let rust_name = "Alice";  // Rust &str
let js_selector = "document.querySelector('#user')";  // Pre-built JS

// This macro call:
js!{
    (function() {
        const el = @{js_selector};           // @{} injects JS code
        el.setAttribute('data-name', #{rust_name});  // #{} converts Rust value
        return `Hello, ${el.textContent}!`;  // ${} is JS runtime interpolation
    })()
}

// Produces:
// (function() {
//     const el = document.querySelector('#user');
//     el.setAttribute('data-name', "Alice");
//     return `Hello, ${el.textContent}!`;
// })()
```

### Scanner Handling

The JavaScript-aware scanner must:
1. Recognize `#{expr}` and `@{expr}` as Rust interpolation markers (process at macro expansion)
2. Preserve `${expr}` inside template literals as JavaScript code (pass through to output)
3. NOT confuse `#{expr}` inside a JavaScript string as Rust interpolation (e.g., `"color: #{foo}"` is literal CSS)

### State Machine Additions for Template Literals

```
TemplateString state:
  - On `${` → Push to TemplatExpr state (track depth)
  - On `#{` or `@{` in template → Rust interpolation (NOT inside ${} though)
  - On `` ` `` → Return to Normal

TemplatExpr state (inside ${...}):
  - Track brace depth
  - Can contain nested template literals
  - `#{` and `@{` here are still Rust interpolation
  - On matching `}` at depth 0 → Return to TemplateString
```

## Syntax Safety Analysis: No Conflicts with JavaScript

The Rust interpolation syntax (`#{expr}` and `@{expr}`) does **not** conflict with any valid JavaScript syntax:

### `#{` Analysis

| JS Feature | Syntax | Conflict? |
|------------|--------|-----------|
| Private class fields | `#name`, `this.#value` | No - `#` followed by identifier, never `{` |
| Record & Tuple proposal | `#{ a: 1 }`, `#[1, 2]` | **Withdrawn** - never became part of JS |

**Conclusion**: `#{` is not valid JavaScript syntax as a language construct.

### `@{` Analysis

| JS Feature | Syntax | Conflict? |
|------------|--------|-----------|
| Decorators (Stage 3) | `@decorator`, `@obj.method`, `@decorator()` | No - `@` followed by identifier/call/member, never `{` |

**Conclusion**: `@{` is not valid JavaScript syntax as a language construct.

### Where These Sequences CAN Appear (as literal characters)

These sequences can appear inside:
1. **String literals**: `"#{foo}"` or `"@{bar}"` - just string content
2. **Template literals**: `` `#{foo}` `` or `` `@{bar}` `` - just template content  
3. **Regular expressions**: `/#{/` or `/@{/` - regex patterns
4. **Comments**: `// #{` or `/* @{ */` - comment content

**Design Decision**: In all these cases, the sequences are treated as **literal characters**, not Rust interpolation. This matches user expectations (e.g., `"color: #{foo}"` is a CSS color, not Rust variable).

## Open Questions

1. **Nightly features**: `Span::source_text()` stabilization status?
   - As of Rust 1.73+, it's stable in proc_macro
   - proc_macro2 mirrors this

2. **Template literal interpolation conflict**: What if user writes `` `${#{x}}` ``?
   - The `#{x}` is our interpolation, `${}` is JS template
   - Should work: scan finds `#{x}` first, replaces it, JS sees `${value}`

3. **Rust interpolation inside JS string**: Is `"#{foo}"` literal or interpolation?
   - **Decision**: Literal. Rust interpolation only in JS code context, not inside strings.
   - This matches user expectation: `"color: #{foo}"` is CSS, not Rust variable
