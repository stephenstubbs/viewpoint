## Context

The download handling infrastructure was scaffolded but never wired up to CDP events. The current event listener handles:
- `Runtime.consoleAPICalled`
- `Runtime.exceptionThrown`
- `Page.frameAttached` / `Page.frameNavigated` / `Page.frameDetached`
- `Page.javascriptDialogOpening`

But does NOT handle:
- `Browser.downloadWillBegin`
- `Browser.downloadProgress`

Additionally, `Browser.setDownloadBehavior` is called to enable download events, but the events themselves are never processed.

## Goals / Non-Goals

**Goals:**
- Wire up download events to existing handler infrastructure
- Fix parameter order bug in Download construction
- Enable all download tests to pass

**Non-Goals:**
- Changing the Download API surface
- Adding new download features beyond what's specified

## Decisions

### Decision 1: Event Routing Strategy

**Decision:** Add download event handling directly in the existing `event_listener` module.

**Alternatives considered:**
1. Create a separate download event listener - Rejected because it fragments event handling
2. Handle at Browser level with event forwarding - More complex, defer until needed

**Rationale:** The event listener already has the infrastructure for handling CDP events. Adding download events follows the existing pattern.

### Decision 2: Session ID Filtering for Browser Events

**Decision:** Browser-domain events may not include session_id. We need to either:
a) Not filter by session_id for Browser.download* events, OR
b) Track which downloads belong to which page by frame_id from `downloadWillBegin`

**Research needed:** Check if `Browser.downloadWillBegin` includes session_id in the CDP event envelope.

### Decision 3: Parameter Order Fix

**Decision:** Fix the parameter order in `handle_download_begin` to match `Download::new` signature.

Current (wrong):
```rust
let download = Download::new(guid.clone(), suggested_filename, url, state_rx, path_rx);
```

Correct:
```rust
let download = Download::new(guid.clone(), url, suggested_filename, state_rx, path_rx);
```

## Risks / Trade-offs

- **Risk:** Browser.download* events may require browser-level subscription
  - **Mitigation:** Test with simple case first; if events don't arrive, investigate Browser-level event subscription

- **Risk:** Multiple pages may receive download events meant for other pages
  - **Mitigation:** Use frame_id from downloadWillBegin to route to correct page

## Open Questions

1. Does `Browser.setDownloadBehavior` need to be called at browser level instead of page session level?
2. Should download event handling be moved to browser context level instead of page level?
