# Tasks: Track Page Activation Events

## 1. Add CDP Event Type

- [x] 1.1 Add `TargetInfoChangedEvent` struct to `viewpoint-cdp/src/protocol/target_domain/mod.rs`
  - Contains `target_info: TargetInfo`
  - Uses `#[serde(rename_all = "camelCase")]`

## 2. Extend Event Infrastructure

- [x] 2.1 Add `PageActivatedEventHandler` type alias in `context/events/mod.rs`
- [x] 2.2 Add `page_activated_handlers: EventEmitter<PageActivatedEventHandler>` to `ContextEventManager`
- [x] 2.3 Implement `on_page_activated()` method (returns `HandlerId`)
- [x] 2.4 Implement `off_page_activated()` method
- [x] 2.5 Implement `emit_page_activated()` method
- [x] 2.6 Update `clear()` to clear page_activated handlers

## 3. Add Public API

- [x] 3.1 Add `on_page_activated()` to `BrowserContext` in `context/page_events/mod.rs`
- [x] 3.2 Add `off_page_activated()` to `BrowserContext`
- [x] 3.3 Export `on_page_activated` / `off_page_activated` from context module

## 4. Implement Event Listener

- [x] 4.1 Add `Target.targetInfoChanged` case to `start_target_event_listener()` match
- [x] 4.2 Create `handle_target_info_changed()` function
  - Filter by context ID
  - Filter for page-type targets
  - Look up existing Page from pages list
  - Emit `page_activated` event with the Page

## 5. Testing

- [x] 5.1 Add unit tests for `TargetInfoChangedEvent` deserialization
- [x] 5.2 Add unit tests for `on_page_activated` / `off_page_activated` in event manager
- [x] 5.3 Add integration test: handler registration and removal, context isolation
- [x] 5.4 Run `cargo test --workspace`
- [x] 5.5 Run `cargo test --workspace --features integration`
