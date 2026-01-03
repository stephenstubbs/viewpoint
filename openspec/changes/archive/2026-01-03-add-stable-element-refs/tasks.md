# Tasks

## 1. Context Index Tracking

- [x] 1.1 Add `context_index: usize` field to `BrowserContext` struct
- [x] 1.2 Assign context index when context is created by Browser
- [x] 1.3 Add `context.index()` getter method
- [x] 1.4 Add integration test for context index assignment

## 2. Page Index Tracking

- [x] 2.1 Add `page_index: usize` field to `Page` struct
- [x] 2.2 Assign page index when page is added to context
- [x] 2.3 Add `page.index()` getter method
- [x] 2.4 Add integration test for page index assignment

## 3. Frame Index Tracking

- [x] 3.1 Add `frame_index: usize` field to Frame struct
- [x] 3.2 Assign frame index when frame is created (0 = main, 1+ = child)
- [x] 3.3 Update Frame constructors to accept frame_index
- [x] 3.4 Pass frame_index through Page.frames() and Page.main_frame()

## 4. Ref Format and Parsing

- [x] 4.1 Update `format_ref()` to generate `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}` format
- [x] 4.2 Update `parse_ref()` to extract context index, page index, frame index, and element counter
- [x] 4.3 Add unit tests for ref format parsing
- [x] 4.4 Remove legacy `e{backendNodeId}` format support from `parse_ref()`

## 5. Ref Map on Page

- [x] 5.1 Add `ref_map: HashMap<String, BackendNodeId>` to Page
- [x] 5.2 Populate ref map during snapshot capture
- [x] 5.3 Include context, page, and frame index in generated refs
- [x] 5.4 Clear ref_map at start of aria_snapshot() to remove stale refs

## 6. Ref Resolution

- [x] 6.1 Update `element_from_ref()` to validate context index
- [x] 6.2 Update `element_from_ref()` to validate page index
- [x] 6.3 Update `element_from_ref()` to use ref map lookup (O(1))
- [x] 6.4 Return clear error message when ref not found
- [x] 6.5 Update `locator_from_ref()` to use new parsing
- [x] 6.6 Add `Selector::Ref` handling in query_element_info and focus_element

## 7. Integration Tests

- [x] 7.1 Test: resolve ref on correct context and page
- [x] 7.2 Test: reject ref on wrong context
- [x] 7.3 Test: reject ref on wrong page
- [x] 7.4 Test: stale ref returns helpful error
- [x] 7.5 Test: context indices are assigned correctly across multiple contexts
- [x] 7.6 Test: page indices are assigned correctly within a context
- [x] 7.7 Test: refs from different contexts have different prefixes
- [x] 7.8 Test: refs from different pages have different prefixes

## 8. Cleanup

- [x] 8.1 Remove legacy `e{backendNodeId}` parsing from `parse_ref()`
- [x] 8.2 Remove `format_ref_legacy()` function
- [x] 8.3 Update any tests still using legacy format
