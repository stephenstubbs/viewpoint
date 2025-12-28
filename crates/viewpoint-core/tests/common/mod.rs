//! Common test utilities and setup for integration tests.


use std::sync::Once;
use std::time::Duration;

use viewpoint_core::Browser;

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
pub async fn launch_with_page() -> (Browser, viewpoint_core::BrowserContext, viewpoint_core::Page) {
    let browser = launch_browser().await;
    let context = browser.new_context().await.expect("Failed to create context");
    let page = context.new_page().await.expect("Failed to create page");
    (browser, context, page)
}
