//! Tests for the JavaScript scanner.

use super::*;
use crate::interpolation::Segment;

// Helper to check if segments contain a literal with given text
fn has_literal(segments: &[Segment], text: &str) -> bool {
    segments
        .iter()
        .any(|s| matches!(s, Segment::Literal(s) if s == text))
}

fn literal_contains(segments: &[Segment], substr: &str) -> bool {
    segments
        .iter()
        .any(|s| matches!(s, Segment::Literal(s) if s.contains(substr)))
}

#[test]
fn test_simple_code() {
    let (segments, has_interp) = scan_js_source("1 + 2");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    assert!(has_literal(&segments, "1 + 2"));
}

#[test]
fn test_double_quoted_string() {
    let (segments, has_interp) = scan_js_source(r#"console.log("hello")"#);
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    assert!(literal_contains(&segments, r#""hello""#));
}

#[test]
fn test_single_quoted_string() {
    let (segments, has_interp) = scan_js_source("console.log('hello')");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    assert!(literal_contains(&segments, "'hello'"));
}

#[test]
fn test_template_literal() {
    let (segments, has_interp) = scan_js_source("`hello ${name}`");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    assert!(literal_contains(&segments, "`hello ${name}`"));
}

#[test]
fn test_value_interpolation_in_normal() {
    let (segments, has_interp) = scan_js_source("document.getElementById(#{id})");
    assert!(has_interp);
    assert_eq!(segments.len(), 3);
    assert!(matches!(&segments[0], Segment::Literal(s) if s == "document.getElementById("));
    assert!(matches!(&segments[1], Segment::ValueInterpolation(_)));
    assert!(matches!(&segments[2], Segment::Literal(s) if s == ")"));
}

#[test]
fn test_raw_interpolation_in_normal() {
    let (segments, has_interp) = scan_js_source("Array.from(@{expr})");
    assert!(has_interp);
    assert_eq!(segments.len(), 3);
    assert!(matches!(&segments[0], Segment::Literal(s) if s == "Array.from("));
    assert!(matches!(&segments[1], Segment::RawInterpolation(_)));
    assert!(matches!(&segments[2], Segment::Literal(s) if s == ")"));
}

#[test]
fn test_interpolation_inside_template_literal() {
    let (segments, has_interp) = scan_js_source("`value: #{x}`");
    assert!(has_interp);
    // Should have: "`value: " + interpolation + "`"
    assert_eq!(segments.len(), 3);
    assert!(matches!(&segments[0], Segment::Literal(s) if s == "`value: "));
    assert!(matches!(&segments[1], Segment::ValueInterpolation(_)));
    assert!(matches!(&segments[2], Segment::Literal(s) if s == "`"));
}

#[test]
fn test_js_template_interpolation_preserved() {
    let (segments, has_interp) = scan_js_source("`Hello, ${userName}!`");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    // JS template interpolation should be preserved as-is
    assert!(literal_contains(&segments, "${userName}"));
}

#[test]
fn test_regex_literal() {
    let (segments, has_interp) = scan_js_source("/^test/.test(str)");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    assert!(literal_contains(&segments, "/^test/"));
}

#[test]
fn test_regex_with_flags() {
    let (segments, has_interp) = scan_js_source("/pattern/gi.test(str)");
    assert!(!has_interp);
    assert!(literal_contains(&segments, "/pattern/"));
}

#[test]
fn test_regex_with_char_class() {
    let (segments, has_interp) = scan_js_source("/[a-z/]/");
    assert!(!has_interp);
    // The `/` inside `[...]` should not end the regex
    assert!(literal_contains(&segments, "[a-z/]"));
}

#[test]
fn test_division_not_regex() {
    let (segments, has_interp) = scan_js_source("a / b / c");
    assert!(!has_interp);
    assert!(has_literal(&segments, "a / b / c"));
}

#[test]
fn test_regex_after_return() {
    let (segments, has_interp) = scan_js_source("return /pattern/");
    assert!(!has_interp);
    assert!(literal_contains(&segments, "/pattern/"));
}

#[test]
fn test_regex_after_equals() {
    let (segments, has_interp) = scan_js_source("x = /pattern/");
    assert!(!has_interp);
    assert!(literal_contains(&segments, "/pattern/"));
}

#[test]
fn test_line_comment() {
    let (segments, has_interp) = scan_js_source("x // comment\ny");
    assert!(!has_interp);
    assert!(literal_contains(&segments, "// comment"));
}

#[test]
fn test_block_comment() {
    let (segments, has_interp) = scan_js_source("x /* comment */ y");
    assert!(!has_interp);
    assert!(literal_contains(&segments, "/* comment */"));
}

#[test]
fn test_escaped_quote_in_double_string() {
    let (segments, has_interp) = scan_js_source(r#""say \"hello\"""#);
    assert!(!has_interp);
    assert!(literal_contains(&segments, r#"\"hello\""#));
}

#[test]
fn test_escaped_quote_in_single_string() {
    let (segments, has_interp) = scan_js_source(r"'it\'s'");
    assert!(!has_interp);
    assert!(literal_contains(&segments, r"\'s"));
}

#[test]
fn test_escaped_backtick_in_template() {
    let (segments, has_interp) = scan_js_source(r"`code: \`x\``");
    assert!(!has_interp);
    assert!(literal_contains(&segments, r"\`x\`"));
}

#[test]
fn test_nested_template_literals() {
    let (segments, has_interp) = scan_js_source("`outer ${`inner`} end`");
    assert!(!has_interp);
    assert!(literal_contains(&segments, "`inner`"));
}

#[test]
fn test_xpath_single_quotes() {
    let source = r#"document.evaluate("//div[@class='x']", doc)"#;
    let (segments, has_interp) = scan_js_source(source);
    assert!(!has_interp);
    assert!(literal_contains(&segments, "@class='x'"));
}

#[test]
fn test_css_selector_mixed_quotes() {
    let source = r#"document.querySelector('[data-id="test"]')"#;
    let (segments, has_interp) = scan_js_source(source);
    assert!(!has_interp);
    assert!(literal_contains(&segments, r#"[data-id="test"]"#));
}

#[test]
fn test_no_interpolation_inside_double_string() {
    // #{x} inside a double-quoted string should NOT be treated as interpolation
    let (segments, has_interp) = scan_js_source(r#""color: #{foo}""#);
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    assert!(literal_contains(&segments, "#{foo}"));
}

#[test]
fn test_no_interpolation_inside_single_string() {
    let (segments, has_interp) = scan_js_source("'#{foo}'");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    assert!(literal_contains(&segments, "#{foo}"));
}

#[test]
fn test_validation_source_simple() {
    let result = create_validation_source("document.getElementById(#{id})");
    assert_eq!(result, "document.getElementById(null)");
}

#[test]
fn test_validation_source_multiple() {
    let result = create_validation_source("[#{x}, #{y}]");
    assert_eq!(result, "[null, null]");
}

#[test]
fn test_validation_source_raw() {
    let result = create_validation_source("Array.from(@{expr})");
    assert_eq!(result, "Array.from(null)");
}

#[test]
fn test_validation_source_preserves_template() {
    let result = create_validation_source("`Hello ${name}`");
    assert_eq!(result, "`Hello ${name}`");
}

#[test]
fn test_validation_source_mixed() {
    let result = create_validation_source("@{expr}.setAttribute('id', #{value})");
    assert_eq!(result, "null.setAttribute('id', null)");
}

#[test]
fn test_hash_without_brace() {
    let (segments, has_interp) = scan_js_source("x # y");
    assert!(!has_interp);
    assert!(has_literal(&segments, "x # y"));
}

#[test]
fn test_at_without_brace() {
    let (segments, has_interp) = scan_js_source("x @ y");
    assert!(!has_interp);
    assert!(has_literal(&segments, "x @ y"));
}

#[test]
fn test_nested_braces_in_interpolation() {
    let (segments, has_interp) = scan_js_source("#{vec![1, 2]}");
    assert!(has_interp);
    assert!(
        segments
            .iter()
            .any(|s| matches!(s, Segment::ValueInterpolation(_)))
    );
}

#[test]
fn test_complex_xpath() {
    let source = r#"document.evaluate("//button[contains(@class, 'submit')]", document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null)"#;
    let (segments, has_interp) = scan_js_source(source);
    assert!(!has_interp);
    assert!(literal_contains(&segments, "contains(@class, 'submit')"));
}
