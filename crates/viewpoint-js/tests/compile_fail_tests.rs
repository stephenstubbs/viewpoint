//! Compile-fail tests using trybuild.
//!
//! These tests verify that invalid JavaScript produces helpful compile-time errors.

#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
