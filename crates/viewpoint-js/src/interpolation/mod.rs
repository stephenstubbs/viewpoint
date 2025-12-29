//! Interpolation detection and processing for the js! macro.

use proc_macro2::TokenStream;
use quote::quote;

/// Represents a segment of the JavaScript source, either literal or interpolated.
#[derive(Debug, Clone)]
pub enum Segment {
    /// Literal JavaScript text
    Literal(String),
    /// A Rust expression to interpolate as a JS value (quoted/escaped via ToJsValue)
    /// Syntax: `#{expr}`
    ValueInterpolation(TokenStream),
    /// A Rust expression to interpolate raw (injected directly without quoting)
    /// Syntax: `@{expr}`
    RawInterpolation(TokenStream),
}

/// Parse the JavaScript source for interpolation markers.
///
/// Supports two interpolation syntaxes:
/// - `#{expr}` - Value interpolation (converted via ToJsValue, quoted/escaped)
/// - `@{expr}` - Raw interpolation (injected directly without modification)
///
/// Returns a list of segments and whether any interpolation was found.
pub fn parse_interpolations(source: &str) -> (Vec<Segment>, bool) {
    let mut segments = Vec::new();
    let mut has_interpolation = false;
    let mut chars = source.chars().peekable();
    let mut current_literal = String::new();

    while let Some(c) = chars.next() {
        // Check for value interpolation: #{expr}
        if c == '#' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'

            // Save current literal if any
            if !current_literal.is_empty() {
                segments.push(Segment::Literal(current_literal.clone()));
                current_literal.clear();
            }

            // Find matching closing brace and parse expression
            if let Some((_expr_str, tokens)) = parse_interpolation_expr(&mut chars) {
                segments.push(Segment::ValueInterpolation(tokens));
                has_interpolation = true;
            } else {
                // If parsing fails, treat as literal
                current_literal.push_str("#{");
            }
        }
        // Check for raw interpolation: @{expr}
        else if c == '@' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'

            // Save current literal if any
            if !current_literal.is_empty() {
                segments.push(Segment::Literal(current_literal.clone()));
                current_literal.clear();
            }

            // Find matching closing brace and parse expression
            if let Some((_expr_str, tokens)) = parse_interpolation_expr(&mut chars) {
                segments.push(Segment::RawInterpolation(tokens));
                has_interpolation = true;
            } else {
                // If parsing fails, treat as literal
                current_literal.push_str("@{");
            }
        } else {
            current_literal.push(c);
        }
    }

    // Push remaining literal
    if !current_literal.is_empty() {
        segments.push(Segment::Literal(current_literal));
    }

    (segments, has_interpolation)
}

/// Parse an interpolation expression from the character iterator.
/// Expects the opening '{' to have already been consumed.
/// Returns the expression string and parsed tokens, or None if parsing fails.
fn parse_interpolation_expr(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Option<(String, TokenStream)> {
    let mut depth = 1;
    let mut expr_str = String::new();

    for c in chars.by_ref() {
        if c == '{' {
            depth += 1;
            expr_str.push(c);
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                break;
            }
            expr_str.push(c);
        } else {
            expr_str.push(c);
        }
    }

    // Parse the expression
    match expr_str.parse::<TokenStream>() {
        Ok(tokens) => Some((expr_str, tokens)),
        Err(_) => None,
    }
}

/// Generate code for interpolated JavaScript.
///
/// If no interpolation is present, returns a static string literal.
/// If interpolation is present, returns code that builds the string at runtime.
///
/// - `ValueInterpolation` (`#{expr}`) calls `ToJsValue::to_js_value()` for proper escaping
/// - `RawInterpolation` (`@{expr}`) injects the expression directly (must implement `AsRef<str>`)
pub fn generate_interpolated_code(segments: &[Segment], has_interpolation: bool) -> TokenStream {
    if has_interpolation {
        // Has interpolation - build format string and args
        let mut format_str = String::new();
        let mut args: Vec<TokenStream> = Vec::new();

        for segment in segments {
            match segment {
                Segment::Literal(lit) => {
                    // Escape braces for format!
                    format_str.push_str(&lit.replace('{', "{{").replace('}', "}}"));
                }
                Segment::ValueInterpolation(expr) => {
                    format_str.push_str("{}");
                    // Call to_js_value() on the expression via the core crate
                    args.push(quote! { ::viewpoint_js_core::ToJsValue::to_js_value(&(#expr)) });
                }
                Segment::RawInterpolation(expr) => {
                    format_str.push_str("{}");
                    // Inject raw - the expression must return something that implements AsRef<str>
                    args.push(quote! { (#expr).as_ref() as &str });
                }
            }
        }

        quote! {
            {
                format!(#format_str, #(#args),*)
            }
        }
    } else {
        // No interpolation - return static string
        let js_str = segments
            .iter()
            .filter_map(|s| match s {
                Segment::Literal(lit) => Some(lit.as_str()),
                Segment::ValueInterpolation(_) | Segment::RawInterpolation(_) => None,
            })
            .collect::<String>();

        quote! { #js_str }
    }
}

#[cfg(test)]
mod tests;
