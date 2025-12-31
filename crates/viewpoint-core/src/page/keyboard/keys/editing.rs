//! Editing key definitions (backspace, delete, enter, tab, escape, insert).

use super::definition::KeyDefinition;

/// Get editing key definitions.
pub fn get_editing_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "Backspace" => KeyDefinition {
            code: "Backspace",
            key: "Backspace",
            key_code: 8,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "Delete" => KeyDefinition {
            code: "Delete",
            key: "Delete",
            key_code: 46,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "Enter" => KeyDefinition {
            code: "Enter",
            key: "Enter",
            key_code: 13,
            text: Some("\r"),
            is_keypad: false,
            location: 0,
        },
        "NumpadEnter" => KeyDefinition {
            code: "NumpadEnter",
            key: "Enter",
            key_code: 13,
            text: Some("\r"),
            is_keypad: true,
            location: 3,
        },
        "Tab" => KeyDefinition {
            code: "Tab",
            key: "Tab",
            key_code: 9,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "Escape" => KeyDefinition {
            code: "Escape",
            key: "Escape",
            key_code: 27,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "Insert" => KeyDefinition {
            code: "Insert",
            key: "Insert",
            key_code: 45,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "Space" | " " => KeyDefinition {
            code: "Space",
            key: " ",
            key_code: 32,
            text: Some(" "),
            is_keypad: false,
            location: 0,
        },
        _ => return None,
    })
}
