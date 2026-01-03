# Design: Fix Locator Operations Ref Resolution

## Context

The `Selector::Ref` variant was added to support interaction with elements identified from ARIA snapshots. When `page.locator_from_ref("c0p0f0e1")` is called, it creates a `Locator` with `Selector::Ref(ref_str)`.

Core locator operations (click, hover, focus, fill, type, etc.) have been updated to handle `Selector::Ref` by:
1. Looking up the `backendNodeId` from the Page's ref map
2. Using CDP `DOM.resolveNode` to get a `RemoteObject`
3. Using `Runtime.callFunctionOn` to execute operations on that object

However, multiple functions were missed during this update. They call `self.selector.to_js_expression()` directly, which for `Selector::Ref` returns JavaScript that throws an error:

```javascript
throw new Error('Ref selectors must be resolved via Page ref map');
```

## Affected Functions

### viewpoint-core

1. **select/mod.rs**: `select_option_internal`, `select_options_internal`
2. **files/mod.rs**: `set_input_files`, `set_input_files_from_buffer`
3. **evaluation/mod.rs**: `evaluate`, `evaluate_all`, `element_handle`, `scroll_into_view_if_needed`
4. **debug/mod.rs**: `highlight_for`
5. **queries/mod.rs**: `is_checked`, `all_inner_texts`, `all_text_contents`, `inner_text`, `get_attribute`, `input_value`
6. **aria_snapshot_impl.rs**: `aria_snapshot`
7. **screenshot_element/mod.rs**: `get_element_bounding_box`

### viewpoint-test

1. **locator_helpers.rs**: `get_input_value`, `get_selected_values`, `get_attribute`, `is_enabled`

## Goals

- Enable all locator operations to work with ref-based locators
- Maintain consistency with existing ref handling pattern from helpers/mod.rs
- No changes to public API

## Non-Goals

- Changing how refs are stored or looked up
- Optimizing ref resolution performance
- Adding new locator operations

## Decisions

### Decision: Follow existing pattern from helpers/mod.rs

All implementations will follow the same pattern used by `query_element_info` and `focus_element` in `helpers/mod.rs`:

```rust
// Handle Ref selector - lookup in ref map and resolve via CDP
if let Selector::Ref(ref_str) = &self.selector {
    let backend_node_id = self.page.get_backend_node_id_for_ref(ref_str)?;
    return self.operation_by_backend_id(backend_node_id, ...).await;
}

// Handle BackendNodeId selector  
if let Selector::BackendNodeId(backend_node_id) = &self.selector {
    return self.operation_by_backend_id(*backend_node_id, ...).await;
}

// Fall through to normal selector handling
let selector_expr = self.selector.to_js_expression();
// ... existing JS-based implementation
```

**Alternatives considered:**

1. **Modify `Selector::Ref::to_js_expression()` to return valid JS** - Rejected because refs must be resolved via CDP's `DOM.resolveNode` to get a valid object reference. There's no way to query by backendNodeId from JavaScript.

2. **Pre-resolve refs in the builder** - Rejected because it would duplicate ref resolution logic and break the pattern established by other operations.

### Decision: Create shared helper for CDP-based operations

For functions that need to call JavaScript on a resolved element, we'll use `Runtime.callFunctionOn` with the resolved element's object ID. This is the same pattern used by `query_element_info_by_backend_id`.

## Implementation Strategy

### Phase 1: Create Helper Functions (if not already existing)

Each module may need a `*_by_backend_id` helper function that:
1. Resolves the backend node ID to a `RemoteObject` via `DOM.resolveNode`
2. Uses `Runtime.callFunctionOn` to execute the operation on that object
3. Handles the result appropriately

### Phase 2: Add Ref/BackendNodeId Checks

Add the standard pattern at the beginning of each affected function:
1. Check for `Selector::Ref`, resolve to backend ID, call helper
2. Check for `Selector::BackendNodeId`, call helper directly
3. Fall through to existing JS expression-based implementation

### Phase 3: Add Integration Tests

Add integration tests that:
1. Capture an aria snapshot
2. Get a ref from the snapshot
3. Use `page.locator_from_ref(ref)` to create a locator
4. Call the affected operation
5. Verify it works correctly

## Risks / Trade-offs

- **Risk**: Additional CDP round-trip for ref-based operations
  - **Mitigation**: This is unavoidable and consistent with other ref-based operations. Performance impact is minimal (single additional call).

- **Risk**: Code duplication across modules
  - **Mitigation**: The pattern is simple and consistent. A shared utility could be added later if needed, but for now maintaining consistency with the existing helpers/mod.rs pattern is preferred.

## Open Questions

None - this is a straightforward bug fix following established patterns.
