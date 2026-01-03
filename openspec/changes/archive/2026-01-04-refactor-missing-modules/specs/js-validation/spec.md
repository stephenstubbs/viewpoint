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

- **GIVEN** internal locator helper methods (query_element_info, focus_element, is_checked, get_attribute, etc.)
- **WHEN** these methods construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with `@{expr}` raw interpolation for selector expressions

#### Scenario: Internal frame locator queries use validated JS

- **GIVEN** internal frame locator helper methods
- **WHEN** these methods construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with compile-time validation

#### Scenario: Internal select option methods use validated JS

- **GIVEN** internal select option methods (select_option_internal, select_options_internal)
- **WHEN** these methods construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with `#{expr}` value interpolation

#### Scenario: Internal file input methods use validated JS

- **GIVEN** internal file input methods (set_input_files, set_input_files_from_buffer)
- **WHEN** these methods construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with `@{expr}` raw interpolation

#### Scenario: Internal storage state methods use validated JS

- **GIVEN** internal storage state methods (save_storage_state, restore_storage_state)
- **WHEN** these methods construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with compile-time validation

#### Scenario: Internal assertion helpers use validated JS

- **GIVEN** internal assertion helper functions in viewpoint-test (get_input_value, is_enabled, get_attribute)
- **WHEN** these functions construct JavaScript for evaluation
- **THEN** the JavaScript is constructed using the `js!` macro with `@{expr}` raw interpolation for selector expressions
