//! Compile-time validated JavaScript macro for Viewpoint.
//!
//! This crate provides a `js!` macro that validates JavaScript syntax at compile time,
//! similar to how `serde_json::json!` validates JSON. This catches JavaScript syntax
//! errors early, before they reach the browser.
//!
//! # Features
//!
//! - **Compile-time validation**: JavaScript syntax errors are caught during compilation
//! - **Value interpolation**: Embed Rust expressions using `#{expr}` syntax (quoted/escaped)
//! - **Raw interpolation**: Inject pre-built JavaScript using `@{expr}` syntax (unquoted)
//! - **Zero runtime overhead**: Static strings when no interpolation is used
//! - **Clear error messages**: Points to the exact location of syntax errors
//! - **Full JavaScript syntax**: Single-quoted strings, template literals, regex, XPath, and more
//!
//! # Usage
//!
//! ```no_run
//! use viewpoint_js::js;
//! use viewpoint_js_core::ToJsValue; // Needed for value interpolation
//!
//! // Simple expression - produces &'static str
//! let code = js!{ 1 + 2 };
//!
//! // Arrow function
//! let code = js!{ () => window.innerWidth };
//!
//! // With value interpolation (requires ToJsValue in scope)
//! let selector = ".my-class";
//! let code = js!{ document.querySelector(#{selector}) };
//!
//! // With raw interpolation (inject JS expression as-is)
//! let selector_expr = "document.querySelectorAll('.item')";
//! let code = js!{ Array.from(@{selector_expr}) };
//!
//! // Multi-line function
//! let code = js!{
//!     (() => {
//!         const items = document.querySelectorAll("li");
//!         return items.length;
//!     })()
//! };
//! ```
//!
//! # Value Interpolation (`#{expr}`)
//!
//! Use `#{expr}` to embed Rust values into JavaScript. Values are automatically
//! converted to JavaScript representations via the [`ToJsValue`] trait:
//!
//! - Strings are quoted and escaped
//! - Numbers are inserted as-is
//! - Booleans become `true` or `false`
//! - `Option::None` becomes `null`
//!
//! # Raw Interpolation (`@{expr}`)
//!
//! Use `@{expr}` to inject pre-built JavaScript expressions directly without
//! quoting or escaping. The expression must return something that implements
//! `AsRef<str>`. This is useful for:
//!
//! - Injecting dynamically-built selector expressions
//! - Composing JavaScript from multiple parts
//! - Including pre-validated JavaScript fragments
//!
//! [`ToJsValue`]: viewpoint_js_core::ToJsValue

use proc_macro::TokenStream;

mod interpolation;
mod js_macro;
mod parser;
mod scanner;

/// A macro that validates JavaScript syntax at compile time.
///
/// This macro accepts JavaScript code and validates its syntax during compilation.
/// If the JavaScript is invalid, a compile-time error is produced with details
/// about the syntax error.
///
/// # Output Type
///
/// - Without interpolation: Returns `&'static str`
/// - With interpolation: Returns `String`
///
/// # Examples
///
/// ## Simple Expression
///
/// ```no_run
/// use viewpoint_js::js;
///
/// let code: &str = js!{ 1 + 2 };
/// assert_eq!(code, "1 + 2");
/// ```
///
/// ## Arrow Function
///
/// ```no_run
/// use viewpoint_js::js;
///
/// let code = js!{ () => window.innerWidth };
/// ```
///
/// ## With Interpolation
///
/// ```no_run
/// use viewpoint_js::js;
/// use viewpoint_js_core::ToJsValue;
///
/// let selector = ".my-class";
/// let code: String = js!{ document.querySelector(#{selector}) };
/// ```
///
/// ## Invalid JavaScript (Compile Error)
///
/// ```compile_fail
/// use viewpoint_js::js;
///
/// // This will produce a compile-time error because the JavaScript is invalid
/// let code = js!{ function( };
/// ```
#[proc_macro]
pub fn js(input: TokenStream) -> TokenStream {
    js_macro::js_impl(input)
}
