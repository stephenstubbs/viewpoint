// Test: scope = "browser" without browser parameter should produce compile error

use viewpoint_test_macros::test;

// This should fail to compile: scope = "browser" requires browser = "..."
#[test(scope = "browser")]
async fn test_missing_browser_source(page: viewpoint_core::Page) {
    let _ = page;
}

fn main() {}
