//! Special character and other key definitions.

use super::definition::KeyDefinition;

/// Get special character key definitions.
pub fn get_special_char_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "Backquote" | "`" => KeyDefinition {
            code: "Backquote",
            key: "`",
            key_code: 192,
            text: Some("`"),
            is_keypad: false,
            location: 0,
        },
        "Minus" | "-" => KeyDefinition {
            code: "Minus",
            key: "-",
            key_code: 189,
            text: Some("-"),
            is_keypad: false,
            location: 0,
        },
        "Equal" | "=" => KeyDefinition {
            code: "Equal",
            key: "=",
            key_code: 187,
            text: Some("="),
            is_keypad: false,
            location: 0,
        },
        "BracketLeft" | "[" => KeyDefinition {
            code: "BracketLeft",
            key: "[",
            key_code: 219,
            text: Some("["),
            is_keypad: false,
            location: 0,
        },
        "BracketRight" | "]" => KeyDefinition {
            code: "BracketRight",
            key: "]",
            key_code: 221,
            text: Some("]"),
            is_keypad: false,
            location: 0,
        },
        "Backslash" | "\\" => KeyDefinition {
            code: "Backslash",
            key: "\\",
            key_code: 220,
            text: Some("\\"),
            is_keypad: false,
            location: 0,
        },
        "Semicolon" | ";" => KeyDefinition {
            code: "Semicolon",
            key: ";",
            key_code: 186,
            text: Some(";"),
            is_keypad: false,
            location: 0,
        },
        "Quote" | "'" => KeyDefinition {
            code: "Quote",
            key: "'",
            key_code: 222,
            text: Some("'"),
            is_keypad: false,
            location: 0,
        },
        "Comma" | "," => KeyDefinition {
            code: "Comma",
            key: ",",
            key_code: 188,
            text: Some(","),
            is_keypad: false,
            location: 0,
        },
        "Period" | "." => KeyDefinition {
            code: "Period",
            key: ".",
            key_code: 190,
            text: Some("."),
            is_keypad: false,
            location: 0,
        },
        "Slash" | "/" => KeyDefinition {
            code: "Slash",
            key: "/",
            key_code: 191,
            text: Some("/"),
            is_keypad: false,
            location: 0,
        },
        _ => return None,
    })
}

/// Get lock key definitions.
pub fn get_lock_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "CapsLock" => KeyDefinition {
            code: "CapsLock",
            key: "CapsLock",
            key_code: 20,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "NumLock" => KeyDefinition {
            code: "NumLock",
            key: "NumLock",
            key_code: 144,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "ScrollLock" => KeyDefinition {
            code: "ScrollLock",
            key: "ScrollLock",
            key_code: 145,
            text: None,
            is_keypad: false,
            location: 0,
        },
        _ => return None,
    })
}

/// Get other special key definitions.
pub fn get_other_special_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "Pause" => KeyDefinition {
            code: "Pause",
            key: "Pause",
            key_code: 19,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "PrintScreen" => KeyDefinition {
            code: "PrintScreen",
            key: "PrintScreen",
            key_code: 44,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "ContextMenu" => KeyDefinition {
            code: "ContextMenu",
            key: "ContextMenu",
            key_code: 93,
            text: None,
            is_keypad: false,
            location: 0,
        },
        _ => return None,
    })
}
