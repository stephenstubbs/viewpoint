# Design: Complete Test Coverage

## Overview

This document outlines the approach for achieving complete test coverage across all Viewpoint specifications, following project conventions for test organization, feature gating, and code quality.

## Test Organization

### File Structure

All new integration tests follow the established pattern:

```
crates/viewpoint-core/tests/
├── dialog_tests.rs          # NEW - Dialog handling tests
├── download_tests.rs        # NEW - Download handling tests
├── pdf_tests.rs             # NEW - PDF generation tests
├── emulation_tests.rs       # NEW - Media/device emulation tests
├── ... (existing files)

crates/viewpoint-test-macros/tests/
├── macro_tests.rs           # NEW - Macro fixture tests
├── ui/                      # Compile-fail tests
│   ├── invalid_scope.rs
│   └── missing_param.rs
```

### Test File Template

Each integration test file follows this template:

```rust
#![cfg(feature = "integration")]

//! <Feature> tests for viewpoint-core.
//!
//! These tests verify <feature description>.

use std::sync::Once;
use std::time::Duration;

use viewpoint_core::{Browser, DocumentLoadState};
use viewpoint_js::js;

static TRACING_INIT: Once = Once::new();

fn init_tracing() {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with_test_writer()
            .try_init()
            .ok();
    });
}

// =============================================================================
// Feature Area Tests
// =============================================================================

#[tokio::test]
async fn test_feature_scenario() {
    init_tracing();
    
    let browser = Browser::launch()
        .headless(true)
        .launch()
        .await
        .expect("Failed to launch browser");
    
    // Test implementation using js! macro for any JavaScript
    
    browser.close().await.expect("Failed to close browser");
}
```

## Spec-to-Test Mapping

### Dialog Tests (dialogs spec)

| Spec Scenario | Test Function |
|---------------|---------------|
| Alert dialog event | `test_dialog_alert_event` |
| Confirm dialog event | `test_dialog_confirm_event` |
| Prompt dialog event | `test_dialog_prompt_event` |
| Beforeunload dialog event | `test_dialog_beforeunload` |
| Get dialog message | `test_dialog_message` |
| Get dialog type | `test_dialog_type` |
| Accept alert | `test_dialog_accept_alert` |
| Accept confirm | `test_dialog_accept_confirm` |
| Dismiss confirm | `test_dialog_dismiss_confirm` |
| Accept prompt with text | `test_dialog_prompt_with_text` |
| Dismiss prompt | `test_dialog_dismiss_prompt` |
| Auto-dismiss alert | `test_dialog_auto_dismiss` |
| Auto-dismiss does not block | `test_dialog_auto_dismiss_multiple` |

### Download Tests (downloads spec)

| Spec Scenario | Test Function |
|---------------|---------------|
| Download event on link click | `test_download_event` |
| Get suggested filename | `test_download_suggested_filename` |
| Get download URL | `test_download_url` |
| Get download path | `test_download_path` |
| Path waits for completion | `test_download_path_waits` |
| Save to custom path | `test_download_save_as` |
| Cancel in-progress download | `test_download_cancel` |
| Get failure reason | `test_download_failure` |
| Wait for download with action | `test_wait_for_download` |

### PDF Tests (page-operations spec)

| Spec Scenario | Test Function |
|---------------|---------------|
| Generate default PDF | `test_pdf_default` |
| Custom paper size | `test_pdf_paper_size` |
| Landscape PDF | `test_pdf_landscape` |
| PDF with margins | `test_pdf_margins` |
| Header and footer | `test_pdf_header_footer` |
| Page ranges | `test_pdf_page_ranges` |
| Background graphics | `test_pdf_background` |
| Save to file | `test_pdf_save_to_file` |

### Emulation Tests (media-emulation + device-emulation specs)

| Spec Scenario | Test Function |
|---------------|---------------|
| Emulate print media | `test_media_print` |
| Emulate screen media | `test_media_screen` |
| Dark mode | `test_color_scheme_dark` |
| Light mode | `test_color_scheme_light` |
| Reduced motion | `test_reduced_motion` |
| Forced colors | `test_forced_colors` |
| Combined emulation | `test_media_combined` |
| Clear emulation | `test_media_clear` |
| Use iPhone descriptor | `test_device_iphone` |
| Use Android descriptor | `test_device_pixel` |
| Set viewport size | `test_viewport_size` |
| Device scale factor | `test_device_scale_factor` |
| Touch emulation | `test_touch_emulation` |
| Mobile mode | `test_mobile_mode` |
| Set locale | `test_locale` |
| Set timezone | `test_timezone` |
| Vision deficiency | `test_vision_deficiency` |

### Macro Tests (test-runner spec)

| Spec Scenario | Test Function |
|---------------|---------------|
| Basic test with page fixture | `test_macro_page_fixture` |
| Multiple fixtures | `test_macro_multiple_fixtures` |
| Macro with configuration | `test_macro_configuration` |
| Test-scoped fixtures | `test_macro_test_scope` |
| Module-scoped browser | `test_macro_browser_scope` |
| Shared context | `test_macro_context_scope` |
| Missing source function error | UI test: `invalid_scope.rs` |

## Test Helpers

### Common HTML Test Pages

For dialog tests:
```rust
const DIALOG_TEST_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <button id="alert" onclick="alert('Hello')">Alert</button>
    <button id="confirm" onclick="window.result = confirm('Sure?')">Confirm</button>
    <button id="prompt" onclick="window.result = prompt('Name?', 'default')">Prompt</button>
</body>
</html>
"#;
```

For download tests (requires a server or data URL with download attribute):
```rust
const DOWNLOAD_TEST_HTML: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <a id="download" href="data:text/plain,Hello" download="test.txt">Download</a>
</body>
</html>
"#;
```

### Timeout Testing Pattern

```rust
#[tokio::test]
async fn test_timeout_expires() {
    let harness = TestHarness::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .await
        .expect("should create harness");
    
    let page = harness.page();
    page.set_content("<div id='never'>Never visible</div>").await.unwrap();
    
    // This should timeout
    let result = expect(&page.locator("#never"))
        .to_be_visible()  // Element exists but test should timeout
        .await;
    
    assert!(result.is_err());
    // Verify error message mentions timeout
}
```

## Error Path Testing Strategy

Each test file includes error scenario tests:

1. **Invalid input** - Empty strings, invalid selectors
2. **Missing elements** - Element not found errors
3. **Timeout errors** - Operations that exceed timeout
4. **State errors** - Operations on closed pages/contexts
5. **Network errors** - Failed requests (where applicable)

## File Size Management

If any test file approaches 400 lines, split by feature area:
- `dialog_tests.rs` → `dialog_alert_tests.rs`, `dialog_prompt_tests.rs`
- `emulation_tests.rs` → `media_emulation_tests.rs`, `device_emulation_tests.rs`

## Verification Checklist

Before marking complete:

1. [ ] Run `cargo test --workspace` - unit tests pass
2. [ ] Run `cargo test --workspace --features integration` - all tests pass
3. [ ] Run `cargo clippy --workspace` - zero warnings
4. [ ] Each spec scenario has at least one test
5. [ ] Each test uses `js!` macro for JavaScript (no raw strings)
6. [ ] Each test file has `#![cfg(feature = "integration")]`
7. [ ] No test file exceeds 500 lines
