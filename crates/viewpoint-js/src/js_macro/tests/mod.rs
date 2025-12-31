use super::*;
use crate::scanner::create_validation_source;
use proc_macro2::TokenStream as TokenStream2;

fn parse_tokens(s: &str) -> TokenStream2 {
    s.parse().unwrap()
}

#[test]
fn test_tokens_to_js_simple() {
    let tokens = parse_tokens("1 + 2");
    let js = tokens_to_js_string(&tokens);
    // Token spacing may not be preserved exactly
    assert!(js.contains('1'));
    assert!(js.contains('+'));
    assert!(js.contains('2'));
}

#[test]
fn test_tokens_to_js_arrow() {
    let tokens = parse_tokens("() => window.innerWidth");
    let js = tokens_to_js_string(&tokens);
    assert!(js.contains("=>"));
    assert!(js.contains("window"));
}

#[test]
fn test_tokens_to_js_function_call() {
    // Use a simpler case - Rust tokenizer has issues with '.foo' as a string
    let tokens = parse_tokens("document.querySelector(selector)");
    let js = tokens_to_js_string(&tokens);
    assert!(js.contains("document"));
    assert!(js.contains("querySelector"));
}

#[test]
fn test_create_validation_source_no_interp() {
    let source = "1 + 2";
    let result = create_validation_source(source);
    assert_eq!(result, "1 + 2");
}

#[test]
fn test_create_validation_source_with_interp() {
    let source = "document.getElementById(#{id})";
    let result = create_validation_source(source);
    assert_eq!(result, "document.getElementById(null)");
}

#[test]
fn test_create_validation_source_multiple_interp() {
    let source = "[#{x}, #{y}]";
    let result = create_validation_source(source);
    assert_eq!(result, "[null, null]");
}

#[test]
fn test_create_validation_source_raw_interp() {
    let source = "Array.from(@{selector_expr})";
    let result = create_validation_source(source);
    assert_eq!(result, "Array.from(null)");
}

#[test]
fn test_create_validation_source_mixed_interp() {
    let source = "@{expr}.setAttribute('id', #{value})";
    let result = create_validation_source(source);
    assert_eq!(result, "null.setAttribute('id', null)");
}

#[test]
fn test_create_validation_source_at_without_brace() {
    let source = "x @ y";
    let result = create_validation_source(source);
    assert_eq!(result, "x @ y");
}
