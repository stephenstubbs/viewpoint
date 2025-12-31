//! Function key definitions (F1-F12).

use super::definition::KeyDefinition;

/// Get function key definitions.
pub fn get_function_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "F1" => KeyDefinition {
            code: "F1",
            key: "F1",
            key_code: 112,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F2" => KeyDefinition {
            code: "F2",
            key: "F2",
            key_code: 113,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F3" => KeyDefinition {
            code: "F3",
            key: "F3",
            key_code: 114,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F4" => KeyDefinition {
            code: "F4",
            key: "F4",
            key_code: 115,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F5" => KeyDefinition {
            code: "F5",
            key: "F5",
            key_code: 116,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F6" => KeyDefinition {
            code: "F6",
            key: "F6",
            key_code: 117,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F7" => KeyDefinition {
            code: "F7",
            key: "F7",
            key_code: 118,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F8" => KeyDefinition {
            code: "F8",
            key: "F8",
            key_code: 119,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F9" => KeyDefinition {
            code: "F9",
            key: "F9",
            key_code: 120,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F10" => KeyDefinition {
            code: "F10",
            key: "F10",
            key_code: 121,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F11" => KeyDefinition {
            code: "F11",
            key: "F11",
            key_code: 122,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "F12" => KeyDefinition {
            code: "F12",
            key: "F12",
            key_code: 123,
            text: None,
            is_keypad: false,
            location: 0,
        },
        _ => return None,
    })
}
