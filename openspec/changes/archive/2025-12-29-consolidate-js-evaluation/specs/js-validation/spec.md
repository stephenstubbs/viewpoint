## ADDED Requirements

### Requirement: Raw Expression Interpolation

The system SHALL support raw JavaScript expression interpolation via `@{expr}` syntax that injects pre-built JavaScript code without quoting or escaping.

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

#### Scenario: Invalid raw interpolation syntax

- **GIVEN** a Rust source file using the `js!` macro
- **WHEN** `js!{ @{unclosed }` is compiled (missing closing brace)
- **THEN** compilation fails with a descriptive error message

### Requirement: JavaScript Evaluation Trait

The system SHALL provide a `JsEvaluator` trait for standardized JavaScript evaluation across different contexts.

#### Scenario: Trait definition

- **GIVEN** a Rust project depending on `viewpoint-js-core`
- **WHEN** the `JsEvaluator` trait is imported
- **THEN** it provides an `evaluate_js` async method returning `Result<serde_json::Value, Self::Error>`

#### Scenario: Implement for custom type

- **GIVEN** a custom struct that can execute JavaScript
- **WHEN** `JsEvaluator` is implemented for the struct
- **THEN** the struct can be used with generic code expecting `JsEvaluator`

## MODIFIED Requirements

### Requirement: Integration with Page Methods

The system SHALL work seamlessly with existing `page.evaluate()` and similar methods, and internal evaluation helpers SHALL use the `js!` macro for compile-time validation.

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

#### Scenario: Internal locator queries use validated JS

- **GIVEN** internal locator helper methods (query_element_info, focus_element, etc.)
- **WHEN** these methods construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with compile-time validation

#### Scenario: Internal frame locator queries use validated JS

- **GIVEN** internal frame locator helper methods
- **WHEN** these methods construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with compile-time validation
