## ADDED Requirements

### Requirement: JavaScript Validation Macro

The system SHALL provide a `js!` macro that validates JavaScript syntax at compile time.

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

#### Scenario: Unexpected token

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ let x = @ }` is compiled (invalid token)
- **THEN** compilation fails with an error indicating unexpected token

### Requirement: Rust Variable Interpolation

The system SHALL support embedding Rust expressions into JavaScript via `#{}` syntax.

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

### Requirement: JavaScript Value Conversion

The system SHALL provide a `ToJsValue` trait for converting Rust types to JavaScript representations.

#### Scenario: Convert string to JS

- **GIVEN** a Rust `String` or `&str`
- **WHEN** converted via `ToJsValue`
- **THEN** produces a properly quoted and escaped JavaScript string literal

#### Scenario: Convert integer to JS

- **GIVEN** a Rust integer type (i32, u64, etc.)
- **WHEN** converted via `ToJsValue`
- **THEN** produces the number as-is (no quotes)

#### Scenario: Convert float to JS

- **GIVEN** a Rust floating point type (f32, f64)
- **WHEN** converted via `ToJsValue`
- **THEN** produces the number, handling special values (NaN, Infinity)

#### Scenario: Convert bool to JS

- **GIVEN** a Rust `bool`
- **WHEN** converted via `ToJsValue`
- **THEN** produces `true` or `false` (no quotes)

#### Scenario: Convert Option to JS

- **GIVEN** a Rust `Option<T>` where T: ToJsValue
- **WHEN** converted via `ToJsValue`
- **THEN** produces the value for `Some` or `null` for `None`

#### Scenario: Convert serde_json::Value to JS

- **GIVEN** a `serde_json::Value`
- **WHEN** converted via `ToJsValue`
- **THEN** produces the corresponding JavaScript literal

### Requirement: Static vs Dynamic Output

The system SHALL produce static strings when possible and dynamic strings when interpolation is used.

#### Scenario: No interpolation produces static str

- **GIVEN** `js!{ window.location.href }` with no interpolation
- **WHEN** the macro is expanded
- **THEN** the result is `&'static str`

#### Scenario: Interpolation produces String

- **GIVEN** `js!{ document.getElementById(#{id}) }` with interpolation
- **WHEN** the macro is expanded
- **THEN** the result is `String` (built at runtime)

### Requirement: Integration with Page Methods

The system SHALL work seamlessly with existing `page.evaluate()` and similar methods.

#### Scenario: Use with page.evaluate

- **GIVEN** a page instance
- **WHEN** `page.evaluate(js!{ document.title }).await` is called
- **THEN** the JavaScript is executed and result returned

#### Scenario: Use with locator.evaluate

- **GIVEN** a locator instance
- **WHEN** `locator.evaluate(js!{ el => el.textContent }).await` is called
- **THEN** the JavaScript is executed on the element

#### Scenario: Use with wait_for_function

- **GIVEN** a page instance
- **WHEN** `page.wait_for_function(js!{ () => document.querySelector('.loaded') }).await` is called
- **THEN** the function is polled until it returns truthy
