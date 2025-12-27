//! Interpolation detection and processing for the js! macro.

use proc_macro2::TokenStream;
use quote::quote;

/// Represents a segment of the JavaScript source, either literal or interpolated.
#[derive(Debug, Clone)]
pub enum Segment {
    /// Literal JavaScript text
    Literal(String),
    /// A Rust expression to interpolate
    Interpolation(TokenStream),
}

/// Parse the JavaScript source for interpolation markers `#{...}`.
///
/// Returns a list of segments and whether any interpolation was found.
pub fn parse_interpolations(source: &str) -> (Vec<Segment>, bool) {
    let mut segments = Vec::new();
    let mut has_interpolation = false;
    let mut chars = source.chars().peekable();
    let mut current_literal = String::new();

    while let Some(c) = chars.next() {
        if c == '#' {
            if chars.peek() == Some(&'{') {
                // Found interpolation start
                chars.next(); // consume '{'

                // Save current literal if any
                if !current_literal.is_empty() {
                    segments.push(Segment::Literal(current_literal.clone()));
                    current_literal.clear();
                }

                // Find matching closing brace
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
                if let Ok(tokens) = expr_str.parse::<TokenStream>() {
                    segments.push(Segment::Interpolation(tokens));
                    has_interpolation = true;
                } else {
                    // If parsing fails, treat as literal (will likely cause JS error)
                    current_literal.push_str("#{");
                    current_literal.push_str(&expr_str);
                    current_literal.push('}');
                }
            } else {
                current_literal.push(c);
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

/// Generate code for interpolated JavaScript.
///
/// If no interpolation is present, returns a static string literal.
/// If interpolation is present, returns code that builds the string at runtime.
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
                Segment::Interpolation(expr) => {
                    format_str.push_str("{}");
                    // Call to_js_value() on the expression via the core crate
                    args.push(quote! { ::viewpoint_js_core::ToJsValue::to_js_value(&(#expr)) });
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
                Segment::Interpolation(_) => None,
            })
            .collect::<String>();

        quote! { #js_str }
    }
}

#[cfg(test)]
mod tests;
