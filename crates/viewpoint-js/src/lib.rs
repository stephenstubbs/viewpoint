//! # Viewpoint JS - Compile-Time JavaScript Validation
//!
//! This crate provides the `js!` macro for compile-time JavaScript validation,
//! catching syntax errors before they reach the browser. Similar to how
//! `serde_json::json!` validates JSON at compile time.
//!
//! ## Features
//!
//! - **Compile-time validation**: JavaScript syntax errors are caught during compilation
//! - **Value interpolation**: Embed Rust values using `#{expr}` syntax (quoted/escaped)
//! - **Raw interpolation**: Inject pre-built JavaScript using `@{expr}` syntax (unquoted)
//! - **Zero runtime overhead**: Static strings when no interpolation is used
//! - **Clear error messages**: Points to the exact location of syntax errors
//! - **Full JavaScript syntax**: Single-quoted strings, template literals, regex, XPath, and more
//!
//! ## Quick Start
//!
//! ```no_run
//! use viewpoint_js::js;
//! use viewpoint_js_core::ToJsValue;
//!
//! // Simple expression - produces &'static str
//! let code = js!{ 1 + 2 };
//! assert_eq!(code, "1 + 2");
//!
//! // Arrow function
//! let code = js!{ () => window.innerWidth };
//!
//! // With value interpolation (requires ToJsValue in scope)
//! let selector = ".my-class";
//! let code = js!{ document.querySelector(#{selector}) };
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
//! ## Value Interpolation (`#{expr}`)
//!
//! Use `#{expr}` to embed Rust values into JavaScript. Values are automatically
//! converted to JavaScript representations via the [`ToJsValue`] trait:
//!
//! - Strings are quoted and escaped
//! - Numbers are inserted as-is
//! - Booleans become `true` or `false`
//! - `Option::None` becomes `null`
//!
//! ```no_run
//! use viewpoint_js::js;
//! use viewpoint_js_core::ToJsValue;
//!
//! let name = "John";
//! let age = 25;
//! let active = true;
//! let optional: Option<i32> = None;
//!
//! // Strings are quoted: document.querySelector("John")
//! let code = js!{ document.querySelector(#{name}) };
//!
//! // Numbers as-is: console.log(25)
//! let code = js!{ console.log(#{age}) };
//!
//! // Booleans: element.disabled = true
//! let code = js!{ element.disabled = #{active} };
//!
//! // None becomes null: setConfig(null)
//! let code = js!{ setConfig(#{optional}) };
//! ```
//!
//! ## Raw Interpolation (`@{expr}`)
//!
//! Use `@{expr}` to inject pre-built JavaScript expressions directly without
//! quoting or escaping. The expression must return something that implements
//! `AsRef<str>`. This is useful for:
//!
//! - Injecting dynamically-built selector expressions
//! - Composing JavaScript from multiple parts
//! - Including pre-validated JavaScript fragments
//!
//! ```no_run
//! use viewpoint_js::js;
//!
//! // Build a selector dynamically
//! let selector_expr = "'.item-' + index";
//! let code = js!{ document.querySelector(@{selector_expr}) };
//! // Produces: document.querySelector('.item-' + index)
//!
//! // Compose JavaScript fragments
//! let function_call = "myFunction()";
//! let code = js!{ const result = @{function_call} };
//! // Produces: const result = myFunction()
//! ```
//!
//! ## Output Type
//!
//! - **Without interpolation**: Returns `&'static str` (zero runtime cost)
//! - **With interpolation**: Returns `String` (runtime string building)
//!
//! ```no_run
//! use viewpoint_js::js;
//! use viewpoint_js_core::ToJsValue;
//!
//! // Static string, no allocation
//! let code: &'static str = js!{ 1 + 2 };
//!
//! // Dynamic string due to interpolation
//! let x = 5;
//! let code: String = js!{ 1 + #{x} };
//! ```
//!
//! ## Compile-Time Error Detection
//!
//! Invalid JavaScript produces clear compile-time errors:
//!
//! ```text
//! // This will produce a compile-time error because the JavaScript is invalid
//! use viewpoint_js::js;
//! let code = js!{ function( };
//! // Error: unexpected end of input
//! ```
//!
//! ## Supported JavaScript Syntax
//!
//! The macro supports a wide range of JavaScript syntax:
//!
//! ```text
//! use viewpoint_js::js;
//!
//! // Single-quoted strings
//! let code = js!{ document.querySelector('div') };
//!
//! // Template literals
//! let code = js!{ `Hello ${name}` };
//!
//! // Arrow functions
//! let code = js!{ (x) => x * 2 };
//!
//! // Object literals
//! let code = js!{ { key: "value", nested: { x: 1 } } };
//!
//! // Array literals
//! let code = js!{ [1, 2, 3].map(x => x * 2) };
//!
//! // Regular expressions
//! let code = js!{ /pattern/gi };
//!
//! // XPath expressions
//! let code = js!{ document.evaluate("//div", document) };
//!
//! // Async/await
//! let code = js!{ async () => await fetch('/api') };
//!
//! // Classes
//! let code = js!{ class Foo extends Bar { constructor() { super(); } } };
//! ```
//!
//! ## Integration with Viewpoint Core
//!
//! The `js!` macro is designed for use with Viewpoint's JavaScript evaluation:
//!
//! ```ignore
//! use viewpoint_core::Page;
//! use viewpoint_js::js;
//! use viewpoint_js_core::ToJsValue;
//!
//! # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
//! // Evaluate simple expression
//! let width: i32 = page.evaluate(js!{ window.innerWidth }).await?;
//!
//! // Evaluate with interpolation
//! let selector = "button.primary";
//! let result: serde_json::Value = page.evaluate(
//!     &js!{ document.querySelector(#{selector})?.textContent }
//! ).await?;
//!
//! // Multi-line script
//! let items: Vec<String> = page.evaluate(js!{
//!     Array.from(document.querySelectorAll("li"))
//!         .map(el => el.textContent)
//! }).await?;
//! # Ok(())
//! # }
//! ```
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
/// ```text
/// // This will produce a compile-time error because the JavaScript is invalid
/// use viewpoint_js::js;
/// let code = js!{ function( };
/// // Error: unexpected end of input
/// ```
#[proc_macro]
pub fn js(input: TokenStream) -> TokenStream {
    js_macro::js_impl(input)
}
