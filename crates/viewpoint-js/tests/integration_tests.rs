//! Integration tests for the js! macro.

use viewpoint_js::js;

// ToJsValue is used implicitly by the macro for interpolation
#[allow(unused_imports)]
use viewpoint_js_core::ToJsValue;

#[test]
fn test_simple_expression() {
    let code: &str = js! { 1 + 2 };
    assert!(code.contains('1'));
    assert!(code.contains('+'));
    assert!(code.contains('2'));
}

#[test]
fn test_arrow_function() {
    let code: &str = js! { () => window.innerWidth };
    assert!(code.contains("=>"));
    assert!(code.contains("window"));
    assert!(code.contains("innerWidth"));
}

#[test]
fn test_arrow_with_param() {
    let code: &str = js! { x => x * 2 };
    assert!(code.contains("=>"));
    assert!(code.contains('x'));
}

#[test]
fn test_iife() {
    let code: &str = js! {
        (() => {
            const x = 1;
            return x;
        })()
    };
    assert!(code.contains("const"));
    assert!(code.contains("return"));
}

#[test]
fn test_document_query() {
    let code: &str = js! { document.querySelector(selector) };
    assert!(code.contains("document"));
    assert!(code.contains("querySelector"));
}

#[test]
fn test_interpolation_string() {
    let id = "my-element";
    let code: String = js! { document.getElementById(#{id}) };
    assert!(code.contains("document.getElementById"));
    assert!(code.contains("my-element"));
    // String should be quoted
    assert!(code.contains('"'));
}

#[test]
fn test_interpolation_number() {
    let count = 42;
    let code: String = js! { Array(#{count}).fill(0) };
    assert!(code.contains("Array"));
    assert!(code.contains("42"));
    assert!(code.contains("fill"));
}

#[test]
fn test_interpolation_bool() {
    let flag = true;
    let code: String = js! { console.log(#{flag}) };
    assert!(code.contains("console.log"));
    assert!(code.contains("true"));
}

#[test]
fn test_interpolation_none() {
    let value: Option<i32> = None;
    let code: String = js! { console.log(#{value}) };
    assert!(code.contains("null"));
}

#[test]
fn test_interpolation_some() {
    let value: Option<i32> = Some(42);
    let code: String = js! { console.log(#{value}) };
    assert!(code.contains("42"));
}

#[test]
fn test_multiple_interpolations() {
    let x = 1;
    let y = 2;
    let code: String = js! { [#{x}, #{y}] };
    assert!(code.contains('1'));
    assert!(code.contains('2'));
    assert!(code.contains('['));
    assert!(code.contains(']'));
}

#[test]
fn test_string_escaping() {
    let text = "hello \"world\"";
    let code: String = js! { console.log(#{text}) };
    // The string should be properly escaped
    assert!(code.contains("\\\""));
}

#[test]
fn test_newline_escaping() {
    let text = "line1\nline2";
    let code: String = js! { console.log(#{text}) };
    // Newline should be escaped
    assert!(code.contains("\\n"));
}

#[test]
fn test_static_output_type() {
    // Without interpolation, should be &'static str
    let code: &'static str = js! { window.location.href };
    assert!(code.contains("window"));
}

#[test]
fn test_dynamic_output_type() {
    // With interpolation, should be String
    let x = 1;
    let code: String = js! { console.log(#{x}) };
    assert!(code.contains("console"));
}

#[test]
fn test_complex_expression() {
    let selector = ".item";
    let code: String = js! {
        (() => {
            const items = document.querySelectorAll(#{selector});
            return items.length;
        })()
    };
    assert!(code.contains("querySelectorAll"));
    assert!(code.contains(".item"));
    assert!(code.contains("length"));
}
