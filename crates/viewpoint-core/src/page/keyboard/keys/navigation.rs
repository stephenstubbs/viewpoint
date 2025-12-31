//! Navigation key definitions (arrows, home, end, page up/down).

use super::definition::KeyDefinition;

/// Get navigation key definitions.
pub fn get_navigation_key(key: &str) -> Option<KeyDefinition> {
    Some(match key {
        "ArrowDown" => KeyDefinition {
            code: "ArrowDown",
            key: "ArrowDown",
            key_code: 40,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "ArrowLeft" => KeyDefinition {
            code: "ArrowLeft",
            key: "ArrowLeft",
            key_code: 37,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "ArrowRight" => KeyDefinition {
            code: "ArrowRight",
            key: "ArrowRight",
            key_code: 39,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "ArrowUp" => KeyDefinition {
            code: "ArrowUp",
            key: "ArrowUp",
            key_code: 38,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "End" => KeyDefinition {
            code: "End",
            key: "End",
            key_code: 35,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "Home" => KeyDefinition {
            code: "Home",
            key: "Home",
            key_code: 36,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "PageDown" => KeyDefinition {
            code: "PageDown",
            key: "PageDown",
            key_code: 34,
            text: None,
            is_keypad: false,
            location: 0,
        },
        "PageUp" => KeyDefinition {
            code: "PageUp",
            key: "PageUp",
            key_code: 33,
            text: None,
            is_keypad: false,
            location: 0,
        },
        _ => return None,
    })
}
