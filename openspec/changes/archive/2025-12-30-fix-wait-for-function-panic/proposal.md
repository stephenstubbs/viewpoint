# Change: Fix wait_for_function panic on primitive truthy values

## Why

`wait_for_function` panics when the JavaScript function returns a primitive truthy value (like `true`, `42`, or `"hello"`). The code at line 95 of `wait/mod.rs` assumes all truthy values have an `object_id`, but JavaScript primitives returned via CDP don't have object handles.

This panic breaks common use cases like:
```rust
page.wait_for_function("() => document.body.innerText.includes('loaded')")
    .wait()
    .await?;
```

The condition `document.body.innerText.includes('loaded')` returns `true` (a primitive boolean), causing:
```
panicked at 'truthy result has handle', viewpoint-core-0.2.7/src/page/evaluate/wait/mod.rs:95:41
```

## What Changes

- **MODIFIED** `WaitForFunctionBuilder::wait()` to return `Option<JsHandle>` instead of `JsHandle`
- Primitive truthy values (booleans, numbers, strings) return `None`
- Object truthy values (DOM elements, objects, arrays) return `Some(JsHandle)`
- **BREAKING**: Return type changes from `Result<JsHandle, PageError>` to `Result<Option<JsHandle>, PageError>`

## Impact

- Affected specs: `javascript-evaluation`
- Affected code: `crates/viewpoint-core/src/page/evaluate/wait/mod.rs`
- Breaking change: Users expecting `JsHandle` must handle `Option<JsHandle>`

## Alternatives Considered

1. **Always return JsHandle by boxing primitives**: Would require wrapping primitives in objects on the JS side, adding complexity and overhead.

2. **Create synthetic handles for primitives**: Not possible—CDP `object_id` is required for `Runtime.callFunctionOn` and other handle operations.

3. **Return `Option<JsHandle>` (chosen)**: Natural representation—primitives have no handle, objects do. Callers who need the handle can match on `Some`, callers who just want to wait can ignore the return value.
