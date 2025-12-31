//! Key definitions for keyboard input.
//!
//! This module contains the key definition struct and lookup functions
//! for mapping key names to their CDP properties.

mod definition;
mod digits;
mod editing;
mod function_keys;
mod letters;
mod modifiers;
mod navigation;
mod numpad;
mod special;

pub use definition::KeyDefinition;

use digits::get_digit_key;
use editing::get_editing_key;
use function_keys::get_function_key;
use letters::{get_lowercase_letter_key, get_uppercase_letter_key};
use modifiers::get_modifier_key;
use navigation::get_navigation_key;
use numpad::get_numpad_key;
use special::{get_lock_key, get_other_special_key, get_special_char_key};

/// Get the key definition for a given key name.
pub fn get_key_definition(key: &str) -> Option<KeyDefinition> {
    // Try each category in order
    get_modifier_key(key)
        .or_else(|| get_function_key(key))
        .or_else(|| get_navigation_key(key))
        .or_else(|| get_editing_key(key))
        .or_else(|| get_digit_key(key))
        .or_else(|| get_lowercase_letter_key(key))
        .or_else(|| get_uppercase_letter_key(key))
        .or_else(|| get_numpad_key(key))
        .or_else(|| get_special_char_key(key))
        .or_else(|| get_lock_key(key))
        .or_else(|| get_other_special_key(key))
}
