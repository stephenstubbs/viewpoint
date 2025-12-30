## MODIFIED Requirements

### Requirement: Wait For Function

The system SHALL wait for a JavaScript function to return a truthy value.

#### Scenario: Wait for simple condition

- **GIVEN** a page where a condition will become true
- **WHEN** `page.wait_for_function("() => document.querySelector('.loaded')").wait().await` is called
- **THEN** the method returns `Ok(Some(JsHandle))` when the selector matches an element

#### Scenario: Wait for function with argument

- **GIVEN** a page with dynamic content
- **WHEN** `page.wait_for_function_with_arg("sel => document.querySelector(sel)", ".ready").wait().await` is called
- **THEN** the method returns `Ok(Some(JsHandle))` when the condition is met

#### Scenario: Wait with RAF polling (default)

- **GIVEN** a page with content
- **WHEN** `page.wait_for_function("() => window.ready").wait().await` is called
- **THEN** the function is evaluated on each requestAnimationFrame by default

#### Scenario: Wait with interval polling

- **GIVEN** a page with content
- **WHEN** `page.wait_for_function("() => window.ready").polling(Polling::Interval(Duration::from_millis(100))).wait().await` is called
- **THEN** the function is evaluated every 100 milliseconds

#### Scenario: Wait with custom timeout

- **GIVEN** a page with content
- **WHEN** `page.wait_for_function("() => false").timeout(Duration::from_secs(1)).wait().await` is called
- **AND** the condition never becomes true
- **THEN** a timeout error is returned after 1 second

#### Scenario: Wait with default timeout

- **GIVEN** a page with default timeout of 30 seconds
- **WHEN** `page.wait_for_function("() => false").wait().await` is called
- **AND** the condition never becomes true
- **THEN** a timeout error is returned after 30 seconds

#### Scenario: Wait returns JsHandle for objects

- **GIVEN** a page with content
- **WHEN** `page.wait_for_function("() => document.body").wait().await` is called
- **THEN** `Ok(Some(JsHandle))` referencing the body element is returned

#### Scenario: Wait returns None for primitive truthy values

- **GIVEN** a page with content
- **WHEN** `page.wait_for_function("() => true").wait().await` is called
- **THEN** `Ok(None)` is returned because primitive values have no object handle

#### Scenario: Wait returns None for truthy numbers

- **GIVEN** a page with content
- **WHEN** `page.wait_for_function("() => 42").wait().await` is called
- **THEN** `Ok(None)` is returned because numbers are primitives

#### Scenario: Wait returns None for truthy strings

- **GIVEN** a page with content
- **WHEN** `page.wait_for_function("() => 'loaded'").wait().await` is called
- **THEN** `Ok(None)` is returned because strings are primitives

#### Scenario: Wait for text content match

- **GIVEN** a page with dynamic text content
- **WHEN** `page.wait_for_function("() => document.body.innerText.includes('ready')").wait().await` is called
- **THEN** `Ok(None)` is returned when the text appears (boolean true has no handle)
