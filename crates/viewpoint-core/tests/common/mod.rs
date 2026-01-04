//! Common test utilities and setup for integration tests.

use std::sync::Once;
use std::time::Duration;

use viewpoint_core::{AriaSnapshot, Browser};

static TRACING_INIT: Once = Once::new();

/// Initialize tracing for tests.
/// This is safe to call multiple times - it will only initialize once.
pub fn init_tracing() {
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

/// Launch a headless browser for testing.
pub async fn launch_browser() -> Browser {
    init_tracing();
    Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser")
}

/// Launch a browser with a context and page ready.
pub async fn launch_with_page() -> (
    Browser,
    viewpoint_core::BrowserContext,
    viewpoint_core::Page,
) {
    let browser = launch_browser().await;
    let context = browser
        .new_context()
        .await
        .expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    (browser, context, page)
}

/// Helper to find a ref for an element by role and optional name.
pub fn find_ref_by_role(snapshot: &AriaSnapshot, role: &str, name: Option<&str>) -> Option<String> {
    if snapshot.role.as_deref() == Some(role) {
        if name.is_none() || snapshot.name.as_deref() == name {
            return snapshot.node_ref.clone();
        }
    }
    for child in &snapshot.children {
        if let Some(r) = find_ref_by_role(child, role, name) {
            return Some(r);
        }
    }
    None
}

/// Helper to find a button ref from an ARIA snapshot.
pub fn find_button_ref(snapshot: &AriaSnapshot) -> Option<String> {
    find_ref_by_role(snapshot, "button", None)
}

/// Helper to find a textbox ref from an ARIA snapshot.
pub fn find_textbox_ref(snapshot: &AriaSnapshot) -> Option<String> {
    find_ref_by_role(snapshot, "textbox", None)
}

/// Collect all refs for elements with a given role.
pub fn collect_refs_by_role(snapshot: &AriaSnapshot, role: &str, refs: &mut Vec<String>) {
    if snapshot.role.as_deref() == Some(role) {
        if let Some(ref r) = snapshot.node_ref {
            refs.push(r.clone());
        }
    }
    for child in &snapshot.children {
        collect_refs_by_role(child, role, refs);
    }
}
