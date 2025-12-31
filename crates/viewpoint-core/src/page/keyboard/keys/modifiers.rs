//! Modifier key definitions.

use super::definition::KeyDefinition;

/// Get modifier key definitions.
pub fn get_modifier_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "Alt" | "AltLeft" => KeyDefinition {
            code: "AltLeft",
            key: "Alt",
            key_code: 18,
            text: None,
            is_keypad: false,
            location: 1,
        },
        "AltRight" => KeyDefinition {
            code: "AltRight",
            key: "Alt",
            key_code: 18,
            text: None,
            is_keypad: false,
            location: 2,
        },
        "Control" | "ControlLeft" => KeyDefinition {
            code: "ControlLeft",
            key: "Control",
            key_code: 17,
            text: None,
            is_keypad: false,
            location: 1,
        },
        "ControlRight" => KeyDefinition {
            code: "ControlRight",
            key: "Control",
            key_code: 17,
            text: None,
            is_keypad: false,
            location: 2,
        },
        "Meta" | "MetaLeft" => KeyDefinition {
            code: "MetaLeft",
            key: "Meta",
            key_code: 91,
            text: None,
            is_keypad: false,
            location: 1,
        },
        "MetaRight" => KeyDefinition {
            code: "MetaRight",
            key: "Meta",
            key_code: 92,
            text: None,
            is_keypad: false,
            location: 2,
        },
        "Shift" | "ShiftLeft" => KeyDefinition {
            code: "ShiftLeft",
            key: "Shift",
            key_code: 16,
            text: None,
            is_keypad: false,
            location: 1,
        },
        "ShiftRight" => KeyDefinition {
            code: "ShiftRight",
            key: "Shift",
            key_code: 16,
            text: None,
            is_keypad: false,
            location: 2,
        },
        _ => return None,
    })
}
