//! Compile-fail tests for viewpoint-test-macros.
//!
//! These tests verify that the macro produces helpful error messages
//! for invalid usage patterns.

#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
