//! Keyboard state tracking and helper functions.

use std::collections::HashSet;

use viewpoint_cdp::protocol::input::modifiers;

/// Check if a key is an uppercase letter that requires Shift.
pub(super) fn is_uppercase_letter(key: &str) -> bool {
    key.len() == 1 && key.chars().next().is_some_and(|c| c.is_ascii_uppercase())
}

/// Check if a key is a modifier key.
pub(super) fn is_modifier_key(key: &str) -> bool {
    matches!(
        key,
        "Alt"
            | "AltLeft"
            | "AltRight"
            | "Control"
            | "ControlLeft"
            | "ControlRight"
            | "Meta"
            | "MetaLeft"
            | "MetaRight"
            | "Shift"
            | "ShiftLeft"
            | "ShiftRight"
    )
}

/// Keyboard state tracking.
#[derive(Debug)]
pub(super) struct KeyboardState {
    /// Currently pressed modifier keys.
    pub(super) modifiers: i32,
    /// Set of currently held keys.
    pressed_keys: HashSet<String>,
}

impl KeyboardState {
    pub(super) fn new() -> Self {
        Self {
            modifiers: 0,
            pressed_keys: HashSet::new(),
        }
    }

    pub(super) fn key_down(&mut self, key: &str) -> bool {
        let is_repeat = self.pressed_keys.contains(key);
        self.pressed_keys.insert(key.to_string());

        // Update modifiers
        match key {
            "Alt" | "AltLeft" | "AltRight" => self.modifiers |= modifiers::ALT,
            "Control" | "ControlLeft" | "ControlRight" => self.modifiers |= modifiers::CTRL,
            "Meta" | "MetaLeft" | "MetaRight" => self.modifiers |= modifiers::META,
            "Shift" | "ShiftLeft" | "ShiftRight" => self.modifiers |= modifiers::SHIFT,
            _ => {}
        }

        is_repeat
    }

    pub(super) fn key_up(&mut self, key: &str) {
        self.pressed_keys.remove(key);

        // Update modifiers
        match key {
            "Alt" | "AltLeft" | "AltRight" => self.modifiers &= !modifiers::ALT,
            "Control" | "ControlLeft" | "ControlRight" => self.modifiers &= !modifiers::CTRL,
            "Meta" | "MetaLeft" | "MetaRight" => self.modifiers &= !modifiers::META,
            "Shift" | "ShiftLeft" | "ShiftRight" => self.modifiers &= !modifiers::SHIFT,
            _ => {}
        }
    }
}
