# javascript-evaluation Specification

## Purpose
TBD - created by archiving change add-page-operations. Update Purpose after archive.
## Requirements
### Requirement: Page Evaluate

The system SHALL execute JavaScript in the page context and return results.

#### Scenario: Evaluate simple expression

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate::<i32>("1 + 2").await` is called
- **THEN** the result `3` is returned

#### Scenario: Evaluate with string return

- **GIVEN** a page with a title
- **WHEN** `page.evaluate::<String>("document.title").await` is called
- **THEN** the document title string is returned

#### Scenario: Evaluate function expression

- **GIVEN** a page with content
- **WHEN** `page.evaluate::<i32>("() => window.innerWidth").await` is called
- **THEN** the viewport width is returned

#### Scenario: Evaluate with argument

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate_with_arg::<i32, _>("x => x * 2", 21).await` is called
- **THEN** the result `42` is returned

#### Scenario: Evaluate with multiple arguments

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate_with_arg::<i32, _>("([a, b]) => a + b", (5, 7)).await` is called
- **THEN** the result `12` is returned

#### Scenario: Evaluate with object argument

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate_with_arg::<String, _>("obj => obj.name", json!({"name": "test"})).await` is called
- **THEN** the result `"test"` is returned

#### Scenario: Evaluate returning object

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate::<serde_json::Value>("() => ({x: 1, y: 2})").await` is called
- **THEN** the JSON object `{"x": 1, "y": 2}` is returned

#### Scenario: Evaluate returning array

- **GIVEN** a page with list items
- **WHEN** `page.evaluate::<Vec<String>>("() => [...document.querySelectorAll('li')].map(e => e.textContent)").await` is called
- **THEN** an array of text contents is returned

#### Scenario: Evaluate async function

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate::<String>("async () => { return await Promise.resolve('done'); }").await` is called
- **THEN** the promise resolves and `"done"` is returned

#### Scenario: Evaluate with Promise return

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate::<i32>("() => new Promise(r => setTimeout(() => r(42), 100))").await` is called
- **THEN** the method waits for the promise and returns `42`

#### Scenario: Evaluate with error

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate::<()>("() => { throw new Error('test error'); }").await` is called
- **THEN** an error is returned containing the JavaScript error message

#### Scenario: Evaluate with custom timeout

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate::<i32>("() => slowOperation()").timeout(Duration::from_secs(60)).await` is called
- **THEN** the evaluation uses the specified 60 second timeout

#### Scenario: Evaluate with default timeout

- **GIVEN** a page with default timeout of 30 seconds
- **WHEN** `page.evaluate::<i32>("() => verySlowOperation()").await` is called
- **AND** the operation takes longer than 30 seconds
- **THEN** a timeout error is returned

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

### Requirement: Evaluate Handle

The system SHALL support returning JavaScript object handles for complex objects.

#### Scenario: Get handle to DOM element

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate_handle("document.body").await` is called
- **THEN** a JsHandle referencing the body element is returned

#### Scenario: Get handle to window object

- **GIVEN** a page with content loaded
- **WHEN** `page.evaluate_handle("window").await` is called
- **THEN** a JsHandle referencing the window object is returned

#### Scenario: Use handle as argument

- **GIVEN** a JsHandle from a previous evaluation
- **WHEN** `page.evaluate_with_arg::<String, _>("el => el.tagName", handle).await` is called
- **THEN** the element's tag name is returned

#### Scenario: Dispose handle

- **GIVEN** a JsHandle from a previous evaluation
- **WHEN** `handle.dispose().await` is called
- **THEN** the JavaScript object reference is released

#### Scenario: Get JSON value from handle

- **GIVEN** a JsHandle referencing a serializable object
- **WHEN** `handle.json_value::<T>().await` is called
- **THEN** the JSON representation is returned

### Requirement: Locator Evaluate

The system SHALL support JavaScript evaluation on located elements.

#### Scenario: Evaluate on single element

- **GIVEN** a locator matching one element
- **WHEN** `locator.evaluate::<String>("el => el.textContent").await` is called
- **THEN** the element's text content is returned

#### Scenario: Evaluate on element with argument

- **GIVEN** a locator matching one element
- **WHEN** `locator.evaluate_with_arg::<(), _>("(el, cls) => el.classList.add(cls)", "active").await` is called
- **THEN** the class is added to the element

#### Scenario: Evaluate on all matching elements

- **GIVEN** a locator matching multiple elements
- **WHEN** `locator.evaluate_all::<Vec<String>>("els => els.map(e => e.id)").await` is called
- **THEN** an array of all element IDs is returned

#### Scenario: Get element handle

- **GIVEN** a locator matching one element
- **WHEN** `locator.element_handle().await` is called
- **THEN** an ElementHandle for the element is returned

### Requirement: Exposed Functions

The system SHALL allow exposing Rust functions to page JavaScript.

#### Scenario: Expose simple function

- **GIVEN** a page instance
- **WHEN** `page.expose_function("greet", |name: String| format!("Hello, {}!", name)).await` is called
- **AND** page JavaScript calls `await window.greet("World")`
- **THEN** the function executes and returns `"Hello, World!"`

#### Scenario: Expose async function

- **GIVEN** a page instance
- **WHEN** an async function is exposed via `page.expose_function`
- **AND** page JavaScript calls the exposed function
- **THEN** the async function executes and the result is returned

#### Scenario: Exposed function survives navigation

- **GIVEN** a page with an exposed function
- **WHEN** the page navigates to a new URL
- **THEN** the exposed function remains available on `window`

#### Scenario: Context-level exposed function

- **GIVEN** a browser context
- **WHEN** `context.expose_function("util", func).await` is called
- **THEN** the function is available in all pages of the context

