use super::*;

// =========================================================================
// Selector Display Tests
// =========================================================================

#[test]
fn test_selector_display_css() {
    let selector = Selector::Css("button.primary".to_string());
    assert_eq!(format!("{selector}"), "css=button.primary");
}

#[test]
fn test_selector_display_text_exact() {
    let selector = Selector::Text {
        text: "Click me".to_string(),
        exact: true,
    };
    assert_eq!(format!("{selector}"), "text=Click me");
}

#[test]
fn test_selector_display_text_partial() {
    let selector = Selector::Text {
        text: "Click".to_string(),
        exact: false,
    };
    assert_eq!(format!("{selector}"), "text*=Click");
}

#[test]
fn test_selector_display_role() {
    let selector = Selector::Role {
        role: AriaRole::Button,
        name: None,
    };
    assert_eq!(format!("{selector}"), "role=button");
}

#[test]
fn test_selector_display_role_with_name() {
    let selector = Selector::Role {
        role: AriaRole::Button,
        name: Some("Submit".to_string()),
    };
    assert_eq!(format!("{selector}"), "role=button[name=Submit]");
}

#[test]
fn test_selector_display_testid() {
    let selector = Selector::TestId("login-btn".to_string());
    assert_eq!(format!("{selector}"), "testid=login-btn");
}

#[test]
fn test_selector_display_chained() {
    let parent = Box::new(Selector::Css("ul".to_string()));
    let child = Box::new(Selector::Css("li".to_string()));
    let selector = Selector::Chained(parent, child);
    assert_eq!(format!("{selector}"), "css=ul >> css=li");
}

// =========================================================================
// Selector to JS Expression Tests
// =========================================================================

#[test]
fn test_css_selector_js() {
    let selector = Selector::Css("button.submit".to_string());
    let js = selector.to_js_expression();
    assert!(js.contains("querySelectorAll"));
    assert!(js.contains("button.submit"));
}

#[test]
fn test_text_selector_exact_js() {
    let selector = Selector::Text {
        text: "Hello".to_string(),
        exact: true,
    };
    let js = selector.to_js_expression();
    assert!(js.contains("textContent"));
    assert!(js.contains("=== 'Hello'"));
}

#[test]
fn test_testid_selector_js() {
    let selector = Selector::TestId("my-button".to_string());
    let js = selector.to_js_expression();
    assert!(js.contains("data-testid"));
    assert!(js.contains("my-button"));
}

// =========================================================================
// JS String Escaping Tests
// =========================================================================

#[test]
fn test_js_string_escaping() {
    let result = js_string_literal("it's a \"test\"\nwith newlines");
    assert_eq!(result, "'it\\'s a \"test\"\\nwith newlines'");
}

#[test]
fn test_css_attr_value_simple() {
    let result = css_attr_value("simple");
    assert_eq!(result, r#"\"simple\""#);
}

#[test]
fn test_text_options_default() {
    let options = TextOptions::default();
    assert!(!options.exact);
}

#[test]
fn test_deeply_nested_selector() {
    // ul >> li >> a
    let a = Selector::Css("a".to_string());
    let li = Selector::Css("li".to_string());
    let ul = Selector::Css("ul".to_string());

    let li_a = Selector::Chained(Box::new(li), Box::new(a));
    let ul_li_a = Selector::Chained(Box::new(ul), Box::new(li_a));

    let display = format!("{ul_li_a}");
    assert_eq!(display, "css=ul >> css=li >> css=a");
}
