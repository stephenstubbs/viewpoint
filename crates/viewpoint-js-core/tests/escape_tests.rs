//! Tests for escape functions and ToJsValue trait.

use viewpoint_js_core::{
    ToJsValue, escape_for_css_attr, escape_js_contents, escape_js_contents_single,
    escape_js_string, escape_js_string_single,
};

// ==========================================================================
// escape_js_string tests
// ==========================================================================

#[test]
fn test_escape_simple_string() {
    assert_eq!(escape_js_string("hello"), r#""hello""#);
}

#[test]
fn test_escape_empty_string() {
    assert_eq!(escape_js_string(""), r#""""#);
}

#[test]
fn test_escape_quotes() {
    assert_eq!(escape_js_string(r#"hello "world""#), r#""hello \"world\"""#);
}

#[test]
fn test_escape_single_quotes_unchanged() {
    // Single quotes don't need escaping in double-quoted JS strings
    assert_eq!(escape_js_string("it's fine"), r#""it's fine""#);
}

#[test]
fn test_escape_backslash() {
    assert_eq!(escape_js_string(r"path\to\file"), r#""path\\to\\file""#);
}

#[test]
fn test_escape_newlines() {
    assert_eq!(escape_js_string("line1\nline2"), r#""line1\nline2""#);
}

#[test]
fn test_escape_carriage_return() {
    assert_eq!(escape_js_string("line1\rline2"), r#""line1\rline2""#);
}

#[test]
fn test_escape_tab() {
    assert_eq!(escape_js_string("col1\tcol2"), r#""col1\tcol2""#);
}

#[test]
fn test_escape_mixed_special_chars() {
    assert_eq!(
        escape_js_string("line1\nline2\ttab\\slash\"quote"),
        r#""line1\nline2\ttab\\slash\"quote""#
    );
}

// ==========================================================================
// escape_for_css_attr tests
// ==========================================================================

#[test]
fn test_css_attr_simple() {
    assert_eq!(escape_for_css_attr("submit-button"), r#"\"submit-button\""#);
}

#[test]
fn test_css_attr_empty() {
    assert_eq!(escape_for_css_attr(""), r#"\"\""#);
}

#[test]
fn test_css_attr_with_spaces() {
    assert_eq!(escape_for_css_attr("my button"), r#"\"my button\""#);
}

#[test]
fn test_css_attr_with_quotes() {
    // If the value contains quotes, they need to be escaped
    assert_eq!(escape_for_css_attr(r#"say "hi""#), r#"\"say \"hi\"\""#);
}

#[test]
fn test_css_attr_with_backslash() {
    assert_eq!(escape_for_css_attr(r"path\to"), r#"\"path\\to\""#);
}

#[test]
fn test_css_attr_in_queryselector() {
    // This is the main use case: building a querySelector call
    let id = "submit-button";
    let attr_value = escape_for_css_attr(id);
    let selector = format!(r"document.querySelector('[data-testid={attr_value}]')");
    assert_eq!(
        selector,
        r#"document.querySelector('[data-testid=\"submit-button\"]')"#
    );
}

#[test]
fn test_css_attr_in_queryselectorall() {
    let id = "my-test-id";
    let attr_value = escape_for_css_attr(id);
    let selector = format!(r"document.querySelectorAll('[data-testid={attr_value}]')");
    assert_eq!(
        selector,
        r#"document.querySelectorAll('[data-testid=\"my-test-id\"]')"#
    );
}

#[test]
fn test_css_attr_custom_attribute() {
    let id = "test-value";
    let attr_value = escape_for_css_attr(id);
    let selector = format!(r"document.querySelectorAll('[data-cy={attr_value}]')");
    assert_eq!(
        selector,
        r#"document.querySelectorAll('[data-cy=\"test-value\"]')"#
    );
}

// ==========================================================================
// escape_js_contents tests
// ==========================================================================

#[test]
fn test_js_contents_simple() {
    assert_eq!(escape_js_contents("hello"), "hello");
}

#[test]
fn test_js_contents_empty() {
    assert_eq!(escape_js_contents(""), "");
}

#[test]
fn test_js_contents_with_quotes() {
    assert_eq!(escape_js_contents(r#"say "hi""#), r#"say \"hi\""#);
}

#[test]
fn test_js_contents_with_newlines() {
    assert_eq!(escape_js_contents("line1\nline2"), r"line1\nline2");
}

#[test]
fn test_js_contents_with_backslash() {
    assert_eq!(escape_js_contents(r"path\to\file"), r"path\\to\\file");
}

// ==========================================================================
// ToJsValue tests
// ==========================================================================

#[test]
fn test_integers() {
    assert_eq!(42_i32.to_js_value(), "42");
    assert_eq!((-42_i32).to_js_value(), "-42");
    assert_eq!(0_i32.to_js_value(), "0");
}

#[test]
fn test_unsigned_integers() {
    assert_eq!(42_u32.to_js_value(), "42");
    assert_eq!(0_u64.to_js_value(), "0");
    assert_eq!(255_u8.to_js_value(), "255");
}

#[test]
fn test_floats() {
    assert_eq!(3.15_f64.to_js_value(), "3.15");
    assert_eq!(f64::NAN.to_js_value(), "NaN");
    assert_eq!(f64::INFINITY.to_js_value(), "Infinity");
    assert_eq!(f64::NEG_INFINITY.to_js_value(), "-Infinity");
}

#[test]
fn test_f32_special_values() {
    assert_eq!(f32::NAN.to_js_value(), "NaN");
    assert_eq!(f32::INFINITY.to_js_value(), "Infinity");
    assert_eq!(f32::NEG_INFINITY.to_js_value(), "-Infinity");
}

#[test]
fn test_bool() {
    assert_eq!(true.to_js_value(), "true");
    assert_eq!(false.to_js_value(), "false");
}

#[test]
fn test_string() {
    assert_eq!("hello".to_js_value(), r#""hello""#);
    assert_eq!("hello \"world\"".to_js_value(), r#""hello \"world\"""#);
}

#[test]
fn test_string_owned() {
    let s = String::from("hello");
    assert_eq!(s.to_js_value(), r#""hello""#);
}

#[test]
fn test_option_some() {
    let opt: Option<i32> = Some(42);
    assert_eq!(opt.to_js_value(), "42");
}

#[test]
fn test_option_some_string() {
    let opt: Option<&str> = Some("hello");
    assert_eq!(opt.to_js_value(), r#""hello""#);
}

#[test]
fn test_option_none() {
    let opt: Option<i32> = None;
    assert_eq!(opt.to_js_value(), "null");
}

#[test]
fn test_reference() {
    let n = 42;
    let r = &n;
    assert_eq!(r.to_js_value(), "42");
}

#[test]
fn test_mut_reference() {
    let mut n = 42;
    let r = &mut n;
    assert_eq!(r.to_js_value(), "42");
}

#[test]
fn test_boxed() {
    let b = Box::new(42);
    assert_eq!(b.to_js_value(), "42");
}

#[test]
fn test_boxed_string() {
    let b: Box<str> = "hello".into();
    assert_eq!(b.to_js_value(), r#""hello""#);
}

// ==========================================================================
// escape_js_string_single tests
// ==========================================================================

#[test]
fn test_single_quote_simple() {
    assert_eq!(escape_js_string_single("hello"), "'hello'");
}

#[test]
fn test_single_quote_empty() {
    assert_eq!(escape_js_string_single(""), "''");
}

#[test]
fn test_single_quote_with_single_quotes() {
    assert_eq!(escape_js_string_single("it's fine"), r"'it\'s fine'");
}

#[test]
fn test_single_quote_double_quotes_unchanged() {
    // Double quotes don't need escaping in single-quoted JS strings
    assert_eq!(escape_js_string_single(r#"say "hi""#), r#"'say "hi"'"#);
}

#[test]
fn test_single_quote_backslash() {
    assert_eq!(
        escape_js_string_single(r"path\to\file"),
        r"'path\\to\\file'"
    );
}

#[test]
fn test_single_quote_newlines() {
    assert_eq!(escape_js_string_single("line1\nline2"), r"'line1\nline2'");
}

#[test]
fn test_single_quote_mixed() {
    assert_eq!(escape_js_string_single("it's a\ntest"), r"'it\'s a\ntest'");
}

// ==========================================================================
// escape_js_contents_single tests
// ==========================================================================

#[test]
fn test_contents_single_simple() {
    assert_eq!(escape_js_contents_single("hello"), "hello");
}

#[test]
fn test_contents_single_empty() {
    assert_eq!(escape_js_contents_single(""), "");
}

#[test]
fn test_contents_single_with_single_quotes() {
    assert_eq!(escape_js_contents_single("it's"), r"it\'s");
}

#[test]
fn test_contents_single_double_quotes_unchanged() {
    // Double quotes don't need escaping in single-quoted JS strings
    assert_eq!(escape_js_contents_single(r#"say "hi""#), r#"say "hi""#);
}

#[test]
fn test_contents_single_css_selector() {
    // This is the main use case: CSS selectors with attribute values
    assert_eq!(
        escape_js_contents_single("input[type='text']"),
        r"input[type=\'text\']"
    );
}

#[test]
fn test_contents_single_complex_css() {
    assert_eq!(
        escape_js_contents_single("button, input[type='submit'], input[type='button']"),
        r"button, input[type=\'submit\'], input[type=\'button\']"
    );
}

#[test]
fn test_contents_single_backslash() {
    assert_eq!(escape_js_contents_single(r"path\to"), r"path\\to");
}

#[test]
fn test_contents_single_newlines() {
    assert_eq!(escape_js_contents_single("line1\nline2"), r"line1\nline2");
}
