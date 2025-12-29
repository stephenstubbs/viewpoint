use super::*;

// Note: Full tests require a CdpConnection which needs a real browser.
// These are basic structure tests.

#[test]
fn test_context_route_handler_debug() {
    // Just verify Debug is implemented
    let _fmt = format!(
        "{:?}",
        ContextRouteHandler {
            pattern: Box::new(UrlPattern::glob("*")),
            handler: Arc::new(|_| Box::pin(async { Ok(()) })),
        }
    );
}
