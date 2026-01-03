use super::*;

#[test]
fn test_aria_role_as_str() {
    assert_eq!(AriaRole::Button.as_str(), "button");
    assert_eq!(AriaRole::Checkbox.as_str(), "checkbox");
    assert_eq!(AriaRole::AlertDialog.as_str(), "alertdialog");
    assert_eq!(AriaRole::MenuItemCheckbox.as_str(), "menuitemcheckbox");
    assert_eq!(AriaRole::ProgressBar.as_str(), "progressbar");
}

#[test]
fn test_aria_role_all_variants_have_lowercase_str() {
    let roles = [
        AriaRole::Alert,
        AriaRole::Button,
        AriaRole::Checkbox,
        AriaRole::Dialog,
        AriaRole::Link,
        AriaRole::Menu,
        AriaRole::Tab,
        AriaRole::TextBox,
        AriaRole::TreeItem,
    ];
    for role in roles {
        let s = role.as_str();
        assert_eq!(
            s,
            s.to_lowercase(),
            "AriaRole::{role:?} should be lowercase"
        );
    }
}

#[test]
fn test_aria_role_equality() {
    assert_eq!(AriaRole::Button, AriaRole::Button);
    assert_ne!(AriaRole::Button, AriaRole::Link);
}

#[test]
fn test_implicit_role_button() {
    let selector = implicit_role_selector(AriaRole::Button);
    assert!(selector.contains("button"));
    assert!(selector.contains("input[type='submit']"));
}

#[test]
fn test_implicit_role_textbox() {
    let selector = implicit_role_selector(AriaRole::TextBox);
    assert!(selector.contains("input[type='text']"));
    assert!(selector.contains("textarea"));
}

#[test]
fn test_implicit_role_no_mapping() {
    // Roles without implicit mapping return empty string
    let selector = implicit_role_selector(AriaRole::Alert);
    assert!(selector.is_empty());
}
