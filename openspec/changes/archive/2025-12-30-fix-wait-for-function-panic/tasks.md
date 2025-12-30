# Tasks: Fix wait_for_function panic on primitive truthy values

## 1. Implementation

- [x] 1.1 Change `WaitForFunctionBuilder::wait()` return type to `Result<Option<JsHandle>, PageError>`
- [x] 1.2 Remove the `.expect("truthy result has handle")` and return the `Option<JsHandle>` directly
- [x] 1.3 Update doc comments to explain when `Some` vs `None` is returned

## 2. Testing

- [x] 2.1 Add unit test for wait_for_function with boolean `true` return
- [x] 2.2 Add unit test for wait_for_function with number return (e.g., `42`)
- [x] 2.3 Add unit test for wait_for_function with string return (e.g., `"loaded"`)
- [x] 2.4 Add unit test for wait_for_function with object return (e.g., `document.body`)
- [x] 2.5 Add integration test verifying no panic on `() => true`
- [x] 2.6 Add integration test verifying handle returned for `() => document.body`
