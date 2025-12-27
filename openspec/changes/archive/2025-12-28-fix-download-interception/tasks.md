## 1. Fix Download Event Handling

- [x] 1.1 Add CDP protocol types for Browser download events if missing
- [x] 1.2 Add `Browser.downloadWillBegin` event handling to `event_listener/mod.rs`
- [x] 1.3 Add `Browser.downloadProgress` event handling to `event_listener/mod.rs`
- [x] 1.4 Pass download handler references to `start_event_listener`
- [x] 1.5 Fix parameter order bug in `handle_download_begin` (`url` and `suggested_filename` swapped)

## 2. Handle Browser-Level Events

- [x] 2.1 Research whether Browser.download* events include session_id
- [x] 2.2 Implement appropriate event filtering for download events
- [x] 2.3 Ensure download events are routed to correct page/context

## 3. Testing

- [x] 3.1 Enable `test_download_event_on_link_click` test
- [x] 3.2 Enable `test_download_suggested_filename` test
- [x] 3.3 Enable `test_download_url` test
- [x] 3.4 Enable `test_download_guid` test
- [x] 3.5 Enable `test_expect_download` test
- [x] 3.6 Enable `test_expect_download_csv` test
- [x] 3.7 Enable `test_download_cancel` test
- [x] 3.8 Run all download tests and verify they pass
- [x] 3.9 Verify existing non-ignored download tests still pass
