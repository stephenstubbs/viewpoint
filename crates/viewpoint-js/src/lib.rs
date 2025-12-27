//! Compile-time validated JavaScript macro for Viewpoint.
//!
//! This crate provides a `js!` macro that validates JavaScript syntax at compile time,
//! similar to how `serde_json::json!` validates JSON. This catches JavaScript syntax
//! errors early, before they reach the browser.
//!
//! # Features
//!
//! - **Compile-time validation**: JavaScript syntax errors are caught during compilation
//! - **Rust variable interpolation**: Embed Rust expressions using `#{expr}` syntax
//! - **Zero runtime overhead**: Static strings when no interpolation is used
//! - **Clear error messages**: Points to the exact location of syntax errors
//!
//! # Usage
//!
//! ```rust,ignore
//! use viewpoint_js::js;
//! use viewpoint_js_core::ToJsValue; // Needed for interpolation
//!
//! // Simple expression - produces &'static str
//! let code = js!{ 1 + 2 };
//!
//! // Arrow function
//! let code = js!{ () => window.innerWidth };
//!
//! // With Rust variable interpolation (requires ToJsValue in scope)
//! let selector = ".my-class";
//! let code = js!{ document.querySelector(#{selector}) };
//!
//! // Multi-line function
//! let code = js!{
//!     (() => {
//!         const items = document.querySelectorAll('li');
//!         return items.length;
//!     })()
//! };
//! ```
//!
//! # Interpolation
//!
//! Use `#{expr}` to embed Rust expressions into JavaScript. Values are automatically
//! converted to JavaScript representations via the [`ToJsValue`] trait:
//!
//! - Strings are quoted and escaped
//! - Numbers are inserted as-is
//! - Booleans become `true` or `false`
//! - `Option::None` becomes `null`
//!
//! [`ToJsValue`]: viewpoint_js_core::ToJsValue

use proc_macro::TokenStream;

mod interpolation;
mod js_macro;
mod parser;

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
/// ```rust,ignore
/// use viewpoint_js::js;
///
/// let code: &str = js!{ 1 + 2 };
/// assert_eq!(code, "1 + 2");
/// ```
///
/// ## Arrow Function
///
/// ```rust,ignore
/// use viewpoint_js::js;
///
/// let code = js!{ () => window.innerWidth };
/// ```
///
/// ## With Interpolation
///
/// ```rust,ignore
/// use viewpoint_js::js;
/// use viewpoint_js_core::ToJsValue;
///
/// let selector = ".my-class";
/// let code: String = js!{ document.querySelector(#{selector}) };
/// ```
///
/// ## Invalid JavaScript (Compile Error)
///
/// ```rust,ignore,compile_fail
/// use viewpoint_js::js;
///
/// // This will produce a compile-time error because the JavaScript is invalid
/// let code = js!{ function( };
/// ```
#[proc_macro]
pub fn js(input: TokenStream) -> TokenStream {
    js_macro::js_impl(input)
}
