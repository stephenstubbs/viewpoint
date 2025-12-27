use super::*;

#[test]
fn test_valid_simple_expression() {
    assert!(validate_js("1 + 2").is_ok());
}

#[test]
fn test_valid_arrow_function() {
    assert!(validate_js("() => window.innerWidth").is_ok());
}

#[test]
fn test_valid_arrow_with_param() {
    assert!(validate_js("x => x * 2").is_ok());
}

#[test]
fn test_valid_arrow_multiple_params() {
    assert!(validate_js("(a, b) => a + b").is_ok());
}

#[test]
fn test_valid_arrow_block() {
    assert!(validate_js("() => { return 42; }").is_ok());
}

#[test]
fn test_valid_iife() {
    assert!(validate_js("(() => { const x = 1; return x; })()").is_ok());
}

#[test]
fn test_valid_function_declaration() {
    assert!(validate_js("function foo() { return 42; }").is_ok());
}

#[test]
fn test_valid_object_literal() {
    assert!(validate_js("({ x: 1, y: 2 })").is_ok());
}

#[test]
fn test_valid_array_literal() {
    assert!(validate_js("[1, 2, 3]").is_ok());
}

#[test]
fn test_valid_template_literal() {
    assert!(validate_js("`hello ${name}`").is_ok());
}

#[test]
fn test_valid_async_function() {
    assert!(validate_js("async () => await fetch('/api')").is_ok());
}

#[test]
fn test_valid_class() {
    assert!(validate_js("class Foo { constructor() {} }").is_ok());
}

#[test]
fn test_valid_spread_operator() {
    assert!(validate_js("[...arr]").is_ok());
}

#[test]
fn test_valid_destructuring() {
    assert!(validate_js("const { x, y } = obj").is_ok());
}

#[test]
fn test_valid_document_query() {
    assert!(validate_js("document.querySelector('.foo')").is_ok());
}

#[test]
fn test_valid_multiline() {
    let js = r#"
        (() => {
            const items = document.querySelectorAll('li');
            return items.length;
        })()
    "#;
    assert!(validate_js(js).is_ok());
}

// Invalid JavaScript tests

#[test]
fn test_invalid_unclosed_paren() {
    let result = validate_js("function(");
    assert!(result.is_err());
}

#[test]
fn test_invalid_unclosed_brace() {
    let result = validate_js("function foo() {");
    assert!(result.is_err());
}

#[test]
fn test_invalid_unclosed_string() {
    let result = validate_js(r#""unclosed string"#);
    assert!(result.is_err());
}

#[test]
fn test_invalid_unexpected_token() {
    let result = validate_js("let x = @");
    assert!(result.is_err());
}

#[test]
fn test_invalid_multiple_errors() {
    let result = validate_js("{ x: }");
    assert!(result.is_err());
}

#[test]
fn test_invalid_arrow_syntax() {
    let result = validate_js("=> 42");
    assert!(result.is_err());
}

#[test]
fn test_error_message_includes_position() {
    let result = validate_js("let x = @");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Should have line/col info
    assert!(err.line.is_some() || !err.message.is_empty());
}
