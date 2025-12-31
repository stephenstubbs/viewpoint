## ADDED Requirements

### Requirement: Single-Quoted JavaScript Strings

The system SHALL support single-quoted string literals in the `js!` macro input.

#### Scenario: Simple single-quoted string

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ 'hello world' }` is compiled
- **THEN** compilation succeeds and produces the string `'hello world'`

#### Scenario: Single-quoted string with escaped quote

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ 'it\'s working' }` is compiled
- **THEN** compilation succeeds and produces the string with escaped quote preserved

#### Scenario: CSS attribute selector with double quotes inside single quotes

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ document.querySelector('[data-testid="submit-btn"]') }` is compiled
- **THEN** compilation succeeds preserving the mixed quote pattern

### Requirement: Template Literal Support

The system SHALL support JavaScript template literals (backtick strings) in the `js!` macro input.

#### Scenario: Simple template literal

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ `hello world` }` is compiled
- **THEN** compilation succeeds and produces the template literal

#### Scenario: Template literal with JavaScript interpolation

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ `Hello, ${name}!` }` is compiled
- **THEN** compilation succeeds preserving the `${name}` JavaScript interpolation

#### Scenario: Template literal with Rust interpolation

- **GIVEN** a Rust source file with `let greeting = "Hello";`
- **WHEN** `js!{ `${#{greeting}} World` }` is compiled
- **THEN** the Rust value is interpolated and the result is a valid template literal

#### Scenario: Multi-line template literal

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** a template literal spanning multiple lines is compiled
- **THEN** compilation succeeds preserving the newlines

#### Scenario: Nested template literals

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ `outer ${`inner`}` }` is compiled
- **THEN** compilation succeeds with nested template literals preserved

### Requirement: Regex Literal Support

The system SHALL support JavaScript regex literals in the `js!` macro input.

#### Scenario: Simple regex literal

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ /^hello/.test(str) }` is compiled
- **THEN** compilation succeeds and produces the regex test expression

#### Scenario: Regex with flags

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ /pattern/gi.test(str) }` is compiled
- **THEN** compilation succeeds preserving the regex flags

#### Scenario: Regex with special characters

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ /^https?:\/\/[^\/]+\//.test(url) }` is compiled
- **THEN** compilation succeeds with escaped characters preserved

#### Scenario: Regex with character class containing slash

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ /[/\\]/.test(path) }` is compiled
- **THEN** compilation succeeds (slash in character class doesn't end regex)

#### Scenario: Regex vs division disambiguation

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ a / b / c }` is compiled
- **THEN** compilation succeeds recognizing this as division, not regex

#### Scenario: Regex after operator

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ x = /pattern/ }` is compiled
- **THEN** compilation succeeds recognizing this as regex assignment

#### Scenario: Regex after return keyword

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ return /pattern/.test(s) }` is compiled
- **THEN** compilation succeeds recognizing the regex after return

### Requirement: XPath Expression Support

The system SHALL support XPath expressions with complex quoting patterns in the `js!` macro input.

#### Scenario: XPath with single quotes in double-quoted string

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** the following is compiled:
  ```rust
  js!{ document.evaluate("//div[@class='container']", document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null) }
  ```
- **THEN** compilation succeeds preserving the XPath expression

#### Scenario: XPath with double quotes in single-quoted string

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** the following is compiled:
  ```rust
  js!{ document.evaluate('//div[@class="container"]', document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null) }
  ```
- **THEN** compilation succeeds preserving the XPath expression

#### Scenario: XPath with text predicate

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ document.evaluate("//div[text()='Hello']", document, null, 9, null) }` is compiled
- **THEN** compilation succeeds preserving the text predicate

#### Scenario: XPath with multiple predicates

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ document.evaluate("//tr[position() > 1][@class='active']", document, null, 7, null) }` is compiled
- **THEN** compilation succeeds preserving all predicates

#### Scenario: XPath via value interpolation

- **GIVEN** a Rust source file with `let xpath = "//div[@id='main']";`
- **WHEN** `js!{ document.evaluate(#{xpath}, document, null, 9, null) }` is compiled
- **THEN** the XPath string is properly quoted and escaped in the output

### Requirement: Comment Handling

The system SHALL correctly handle JavaScript comments without interfering with string or interpolation parsing.

#### Scenario: Line comment

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ x = 1 // comment with 'quotes' }` is compiled
- **THEN** compilation succeeds and the comment content is preserved as-is

#### Scenario: Block comment

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ x = /* comment with 'quotes' */ 1 }` is compiled
- **THEN** compilation succeeds and the comment content is preserved as-is

#### Scenario: Interpolation marker in comment is ignored

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ x = 1 // #{not_interpolated} }` is compiled
- **THEN** the `#{...}` in the comment is not treated as interpolation

## MODIFIED Requirements

### Requirement: JavaScript Validation Macro

The system SHALL provide a `js!` macro that validates JavaScript syntax at compile time, supporting the full range of JavaScript literal syntax including single-quoted strings, template literals, and regex literals.

#### Scenario: Valid simple expression

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ 1 + 2 }` is compiled
- **THEN** compilation succeeds and produces the string `"1 + 2"`

#### Scenario: Valid arrow function

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ () => window.innerWidth }` is compiled
- **THEN** compilation succeeds and produces the string `"() => window.innerWidth"`

#### Scenario: Valid multi-line function

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** the following is compiled:
  ```rust
  js!{
      (() => {
          const items = document.querySelectorAll('li');
          return items.length;
      })()
  }
  ```
- **THEN** compilation succeeds and produces the equivalent JavaScript string

#### Scenario: Invalid JavaScript syntax

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ function( }` is compiled (missing closing paren and body)
- **THEN** compilation fails with a descriptive error message indicating the syntax error

#### Scenario: Unclosed string literal

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ "unclosed string }` is compiled
- **THEN** compilation fails with an error indicating unclosed string

#### Scenario: Unclosed single-quoted string

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ 'unclosed string }` is compiled
- **THEN** compilation fails with an error indicating unclosed string

#### Scenario: Unclosed template literal

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ `unclosed template }` is compiled
- **THEN** compilation fails with an error indicating unclosed template literal

#### Scenario: Invalid regex

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ /[invalid }` is compiled
- **THEN** compilation fails with an error indicating invalid regex

### Requirement: Rust Variable Interpolation

The system SHALL support embedding Rust expressions into JavaScript via `#{}` syntax, correctly detecting interpolation markers only in JavaScript code context (not inside strings or comments).

#### Scenario: Interpolate string variable

- **GIVEN** a Rust source file with `let id = "my-id";`
- **WHEN** `js!{ document.getElementById(#{id}) }` is compiled
- **THEN** compilation succeeds and produces code that formats the string properly

#### Scenario: Interpolate numeric variable

- **GIVEN** a Rust source file with `let count = 42;`
- **WHEN** `js!{ Array(#{count}).fill(0) }` is compiled
- **THEN** compilation succeeds and the number is embedded without quotes

#### Scenario: Interpolate expression

- **GIVEN** a Rust source file
- **WHEN** `js!{ console.log(#{1 + 2}) }` is compiled
- **THEN** compilation succeeds and the expression result is embedded

#### Scenario: Multiple interpolations

- **GIVEN** a Rust source file with `let x = 1; let y = 2;`
- **WHEN** `js!{ [#{x}, #{y}] }` is compiled
- **THEN** compilation succeeds with both values properly interpolated

#### Scenario: Interpolation with proper escaping

- **GIVEN** a Rust source file with `let s = "hello \"world\"";`
- **WHEN** `js!{ console.log(#{s}) }` is compiled
- **THEN** the string is properly escaped in the JavaScript output

#### Scenario: Interpolation inside template literal

- **GIVEN** a Rust source file with `let name = "World";`
- **WHEN** `js!{ `Hello, #{name}!` }` is compiled
- **THEN** the Rust value is interpolated into the template literal

#### Scenario: Hash in JavaScript string is not interpolation

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ "color: #{foo}" }` is compiled (hash inside JS string)
- **THEN** the `#{foo}` is treated as literal string content, not interpolation

### Requirement: Raw Expression Interpolation

The system SHALL support raw JavaScript expression interpolation via `@{expr}` syntax that injects pre-built JavaScript code without quoting or escaping, correctly detecting interpolation markers only in JavaScript code context.

#### Scenario: Interpolate selector expression

- **GIVEN** a Rust source file with `let selector = "document.querySelectorAll('.item')";`
- **WHEN** `js!{ Array.from(@{selector}) }` is compiled
- **THEN** compilation succeeds and produces code that injects the selector expression directly

#### Scenario: Interpolate complex expression

- **GIVEN** a Rust source file with `let expr = selector.to_js_expression();`
- **WHEN** the following is compiled:
  ```rust
  js!{
      (function() {
          const elements = @{expr};
          return elements.length;
      })()
  }
  ```
- **THEN** compilation succeeds and the expression is injected without quotes

#### Scenario: Raw interpolation with function call result

- **GIVEN** a Rust source file with a function returning JS expression
- **WHEN** `js!{ const el = @{get_selector_expr()}; }` is compiled
- **THEN** the function is called at runtime and result injected into JS

#### Scenario: Mix raw and value interpolation

- **GIVEN** a Rust source file with `let expr = "document.body"; let name = "test";`
- **WHEN** `js!{ @{expr}.setAttribute("data-name", #{name}) }` is compiled
- **THEN** `expr` is injected raw and `name` is properly quoted as a string

#### Scenario: At-sign in JavaScript string is not interpolation

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ "email@{domain}" }` is compiled (at-sign inside JS string)
- **THEN** the `@{domain}` is treated as literal string content, not interpolation

### Requirement: Rust and JavaScript Interpolation Coexistence

The system SHALL support both Rust interpolation (`#{expr}`, `@{expr}`) and JavaScript template literal interpolation (`${expr}`) in the same macro invocation, with clear semantics for each.

#### Scenario: JavaScript template literal ${} preserved in output

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ `Hello, ${userName}!` }` is compiled (no Rust variables)
- **THEN** the output JavaScript contains the literal string `` `Hello, ${userName}!` ``
- **AND** `userName` is expected to be a JavaScript variable at runtime

#### Scenario: Rust #{} inside template literal produces JS value

- **GIVEN** a Rust source file with `let greeting = "Hello";`
- **WHEN** `js!{ `${#{greeting}}` }` is compiled
- **THEN** the Rust string "Hello" is embedded as a JS string value
- **AND** the output is `` `${"Hello"}` `` (which JavaScript evaluates to "Hello")

#### Scenario: Combined Rust and JS interpolation in complex expression

- **GIVEN** a Rust source file with:
  ```rust
  let rust_prefix = "item";
  let js_selector = "document.querySelector('#list')";
  ```
- **WHEN** the following is compiled:
  ```rust
  js!{
      (function() {
          const list = @{js_selector};
          const prefix = #{rust_prefix};
          return list.querySelectorAll(`[data-type="${prefix}"]`);
      })()
  }
  ```
- **THEN** `@{js_selector}` is replaced with the literal JS code `document.querySelector('#list')`
- **AND** `#{rust_prefix}` is replaced with the quoted string `"item"`
- **AND** the template literal `\`[data-type="${prefix}"]\`` is preserved for JavaScript runtime

#### Scenario: Rust interpolation in template literal expression

- **GIVEN** a Rust source file with `let count = 5;`
- **WHEN** `js!{ `You have ${#{count} + items.length} items` }` is compiled
- **THEN** the Rust value `5` is embedded in the JavaScript template expression
- **AND** the output includes `${5 + items.length}` where `items` is a JS variable
