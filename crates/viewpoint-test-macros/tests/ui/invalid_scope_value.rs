// Test: invalid scope value should produce compile error

use viewpoint_test_macros::test;

// This should fail to compile: "invalid" is not a valid scope
#[test(scope = "invalid")]
async fn test_invalid_scope_value(page: viewpoint_core::Page) {
    let _ = page;
}

fn main() {}
