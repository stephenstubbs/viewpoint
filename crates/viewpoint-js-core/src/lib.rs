//! Core types for the `viewpoint-js` macro.
//!
//! This crate provides the [`ToJsValue`] trait used for interpolating Rust values
//! into JavaScript code via the `js!` macro, as well as utilities for escaping
//! strings for use in JavaScript.
//!
//! # Example
//!
//! ```rust
//! use viewpoint_js_core::{ToJsValue, escape_js_string, escape_for_css_attr};
//!
//! // ToJsValue for interpolation
//! assert_eq!(42.to_js_value(), "42");
//! assert_eq!(true.to_js_value(), "true");
//! assert_eq!("hello".to_js_value(), r#""hello""#);
//!
//! // Direct string escaping
//! assert_eq!(escape_js_string("hello"), r#""hello""#);
//!
//! // CSS attribute selector escaping (for use inside JS strings)
//! assert_eq!(escape_for_css_attr("my-id"), r#"\"my-id\""#);
//! ```

/// Escape a string for use in a JavaScript string literal (double-quoted).
///
/// Returns a double-quoted JavaScript string with proper escaping.
///
/// This handles:
/// - Backslashes
/// - Double quotes
/// - Newlines, carriage returns, tabs
/// - Unicode characters that need escaping
///
/// # Example
///
/// ```rust
/// use viewpoint_js_core::escape_js_string;
///
/// assert_eq!(escape_js_string("hello"), r#""hello""#);
/// assert_eq!(escape_js_string("it's fine"), r#""it's fine""#);
/// assert_eq!(escape_js_string(r#"say "hi""#), r#""say \"hi\"""#);
/// ```
pub fn escape_js_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 2);
    result.push('"');
    escape_js_string_contents_into(s, &mut result);
    result.push('"');
    result
}

/// Escape a string for use in a CSS attribute selector within JavaScript.
///
/// This is for building selectors like `document.querySelector('[data-id="value"]')`.
/// The returned string includes escaped double quotes that work inside a JS string.
///
/// # Example
///
/// ```rust
/// use viewpoint_js_core::escape_for_css_attr;
///
/// // For: document.querySelector('[data-testid="submit-button"]')
/// let attr_value = escape_for_css_attr("submit-button");
/// assert_eq!(attr_value, r#"\"submit-button\""#);
///
/// // Use in a format string:
/// let selector = format!(r#"document.querySelector('[data-testid={}]')"#, attr_value);
/// assert_eq!(selector, r#"document.querySelector('[data-testid=\"submit-button\"]')"#);
/// ```
pub fn escape_for_css_attr(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    // We need escaped double quotes that work inside a JS string
    // The outer JS string uses single quotes, and CSS attr uses double quotes
    result.push_str(r#"\""#);
    // Escape the content for both JS string and CSS attribute
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c => result.push(c),
        }
    }
    result.push_str(r#"\""#);
    result
}

/// Escape a string for use in a single-quoted JavaScript string literal.
///
/// Returns a single-quoted JavaScript string with proper escaping.
/// This is useful when you need to embed a string in JavaScript that uses
/// single quotes (e.g., in template literals or when double quotes are used
/// for HTML attributes).
///
/// # Example
///
/// ```rust
/// use viewpoint_js_core::escape_js_string_single;
///
/// assert_eq!(escape_js_string_single("hello"), "'hello'");
/// assert_eq!(escape_js_string_single("it's fine"), r"'it\'s fine'");
/// assert_eq!(escape_js_string_single(r#"say "hi""#), r#"'say "hi"'"#);
/// ```
pub fn escape_js_string_single(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 2);
    result.push('\'');
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '\'' => result.push_str("\\'"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            '\u{2028}' => result.push_str("\\u2028"),
            '\u{2029}' => result.push_str("\\u2029"),
            c if c.is_control() => {
                use std::fmt::Write;
                let _ = write!(result, "\\u{:04x}", c as u32);
            }
            c => result.push(c),
        }
    }
    result.push('\'');
    result
}

/// Escape string contents for JavaScript without adding surrounding quotes.
///
/// This escapes for double-quoted JS strings. Use `escape_js_contents_single`
/// for single-quoted strings.
///
/// # Example
///
/// ```rust
/// use viewpoint_js_core::escape_js_contents;
///
/// assert_eq!(escape_js_contents("hello"), "hello");
/// assert_eq!(escape_js_contents("line1\nline2"), r"line1\nline2");
/// assert_eq!(escape_js_contents(r#"say "hi""#), r#"say \"hi\""#);
/// ```
pub fn escape_js_contents(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    escape_js_string_contents_into(s, &mut result);
    result
}

/// Escape string contents for a single-quoted JavaScript string.
///
/// This is useful when building JS strings that use single quotes,
/// such as `querySelectorAll('...')`. Single quotes are escaped,
/// double quotes are left as-is.
///
/// # Example
///
/// ```rust
/// use viewpoint_js_core::escape_js_contents_single;
///
/// assert_eq!(escape_js_contents_single("hello"), "hello");
/// assert_eq!(escape_js_contents_single("it's"), r"it\'s");
/// assert_eq!(escape_js_contents_single(r#"say "hi""#), r#"say "hi""#);
/// assert_eq!(escape_js_contents_single("input[type='text']"), r"input[type=\'text\']");
/// ```
pub fn escape_js_contents_single(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '\'' => result.push_str("\\'"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            '\u{2028}' => result.push_str("\\u2028"),
            '\u{2029}' => result.push_str("\\u2029"),
            c if c.is_control() => {
                use std::fmt::Write;
                let _ = write!(result, "\\u{:04x}", c as u32);
            }
            c => result.push(c),
        }
    }
    result
}

/// Internal helper to escape JS string contents into a buffer.
fn escape_js_string_contents_into(s: &str, result: &mut String) {
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            // Line separator and paragraph separator need escaping
            '\u{2028}' => result.push_str("\\u2028"),
            '\u{2029}' => result.push_str("\\u2029"),
            c if c.is_control() => {
                // Escape control characters as \uXXXX
                use std::fmt::Write;
                let _ = write!(result, "\\u{:04x}", c as u32);
            }
            c => result.push(c),
        }
    }
}

/// A trait for converting Rust types to JavaScript value representations.
///
/// Types that implement this trait can be used in `js!` macro interpolation.
///
/// # Examples
///
/// ```rust
/// use viewpoint_js_core::ToJsValue;
///
/// assert_eq!(42.to_js_value(), "42");
/// assert_eq!(true.to_js_value(), "true");
/// assert_eq!("hello".to_js_value(), r#""hello""#);
/// ```
pub trait ToJsValue {
    /// Convert this value to a JavaScript representation.
    ///
    /// The returned string should be valid JavaScript that represents
    /// this value. For example:
    /// - Integers: `"42"`
    /// - Strings: `"\"hello\""`
    /// - Booleans: `"true"` or `"false"`
    /// - null: `"null"`
    fn to_js_value(&self) -> String;
}

// Implement for integers
macro_rules! impl_to_js_value_int {
    ($($t:ty),*) => {
        $(
            impl ToJsValue for $t {
                fn to_js_value(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_to_js_value_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

// Implement for floats
impl ToJsValue for f32 {
    fn to_js_value(&self) -> String {
        if self.is_nan() {
            "NaN".to_string()
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                "Infinity".to_string()
            } else {
                "-Infinity".to_string()
            }
        } else {
            self.to_string()
        }
    }
}

impl ToJsValue for f64 {
    fn to_js_value(&self) -> String {
        if self.is_nan() {
            "NaN".to_string()
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                "Infinity".to_string()
            } else {
                "-Infinity".to_string()
            }
        } else {
            self.to_string()
        }
    }
}

// Implement for bool
impl ToJsValue for bool {
    fn to_js_value(&self) -> String {
        if *self { "true" } else { "false" }.to_string()
    }
}

// Implement for strings
impl ToJsValue for str {
    fn to_js_value(&self) -> String {
        escape_js_string(self)
    }
}

impl ToJsValue for String {
    fn to_js_value(&self) -> String {
        escape_js_string(self)
    }
}

// Implement for Option<T>
impl<T: ToJsValue> ToJsValue for Option<T> {
    fn to_js_value(&self) -> String {
        match self {
            Some(v) => v.to_js_value(),
            None => "null".to_string(),
        }
    }
}

// Implement for references
impl<T: ToJsValue + ?Sized> ToJsValue for &T {
    fn to_js_value(&self) -> String {
        (*self).to_js_value()
    }
}

impl<T: ToJsValue + ?Sized> ToJsValue for &mut T {
    fn to_js_value(&self) -> String {
        (**self).to_js_value()
    }
}

impl<T: ToJsValue + ?Sized> ToJsValue for Box<T> {
    fn to_js_value(&self) -> String {
        (**self).to_js_value()
    }
}

// Implement for serde_json::Value when the feature is enabled
#[cfg(feature = "json")]
impl ToJsValue for serde_json::Value {
    fn to_js_value(&self) -> String {
        match self {
            serde_json::Value::Null => "null".to_string(),
            serde_json::Value::Bool(b) => b.to_js_value(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::String(s) => escape_js_string(s),
            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                // For complex types, serialize to JSON (which is valid JS)
                self.to_string()
            }
        }
    }
}
