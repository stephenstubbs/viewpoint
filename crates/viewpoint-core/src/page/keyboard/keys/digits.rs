//! Digit key definitions (0-9).

use super::definition::KeyDefinition;

/// Get digit key definitions.
pub fn get_digit_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "Digit0" | "0" => KeyDefinition {
            code: "Digit0",
            key: "0",
            key_code: 48,
            text: Some("0"),
            is_keypad: false,
            location: 0,
        },
        "Digit1" | "1" => KeyDefinition {
            code: "Digit1",
            key: "1",
            key_code: 49,
            text: Some("1"),
            is_keypad: false,
            location: 0,
        },
        "Digit2" | "2" => KeyDefinition {
            code: "Digit2",
            key: "2",
            key_code: 50,
            text: Some("2"),
            is_keypad: false,
            location: 0,
        },
        "Digit3" | "3" => KeyDefinition {
            code: "Digit3",
            key: "3",
            key_code: 51,
            text: Some("3"),
            is_keypad: false,
            location: 0,
        },
        "Digit4" | "4" => KeyDefinition {
            code: "Digit4",
            key: "4",
            key_code: 52,
            text: Some("4"),
            is_keypad: false,
            location: 0,
        },
        "Digit5" | "5" => KeyDefinition {
            code: "Digit5",
            key: "5",
            key_code: 53,
            text: Some("5"),
            is_keypad: false,
            location: 0,
        },
        "Digit6" | "6" => KeyDefinition {
            code: "Digit6",
            key: "6",
            key_code: 54,
            text: Some("6"),
            is_keypad: false,
            location: 0,
        },
        "Digit7" | "7" => KeyDefinition {
            code: "Digit7",
            key: "7",
            key_code: 55,
            text: Some("7"),
            is_keypad: false,
            location: 0,
        },
        "Digit8" | "8" => KeyDefinition {
            code: "Digit8",
            key: "8",
            key_code: 56,
            text: Some("8"),
            is_keypad: false,
            location: 0,
        },
        "Digit9" | "9" => KeyDefinition {
            code: "Digit9",
            key: "9",
            key_code: 57,
            text: Some("9"),
            is_keypad: false,
            location: 0,
        },
        _ => return None,
    })
}
