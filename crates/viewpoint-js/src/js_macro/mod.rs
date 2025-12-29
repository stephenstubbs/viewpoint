//! The js! proc-macro implementation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::interpolation::{generate_interpolated_code, parse_interpolations};
use crate::parser::validate_js;

/// Implementation of the js! macro.
pub fn js_impl(input: TokenStream) -> TokenStream {
    let input2: TokenStream2 = input.into();

    // Convert the token stream to a string
    let js_source = tokens_to_js_string(&input2);

    // Parse for interpolations first
    let (segments, has_interpolation) = parse_interpolations(&js_source);

    // For validation, we need to replace interpolations with placeholders
    // that are valid JavaScript
    let validation_source = create_validation_source(&js_source);

    // Validate the JavaScript syntax
    if let Err(err) = validate_js(&validation_source) {
        let msg = format!("Invalid JavaScript syntax: {err}");
        return syn::Error::new(proc_macro2::Span::call_site(), msg)
            .to_compile_error()
            .into();
    }

    // Generate the output code
    let output = generate_interpolated_code(&segments, has_interpolation);

    output.into()
}

/// Convert a token stream to a JavaScript string.
///
/// This handles the conversion of Rust tokens to their JavaScript representation.
fn tokens_to_js_string(tokens: &TokenStream2) -> String {
    let mut result = String::new();
    let mut prev_needs_space = false;

    for token in tokens.clone() {
        match token {
            proc_macro2::TokenTree::Group(group) => {
                let (open, close) = match group.delimiter() {
                    proc_macro2::Delimiter::Brace => ('{', '}'),
                    proc_macro2::Delimiter::Bracket => ('[', ']'),
                    proc_macro2::Delimiter::Parenthesis => ('(', ')'),
                    proc_macro2::Delimiter::None => (' ', ' '),
                };
                result.push(open);
                result.push_str(&tokens_to_js_string(&group.stream()));
                result.push(close);
                prev_needs_space = false;
            }
            proc_macro2::TokenTree::Ident(ident) => {
                if prev_needs_space {
                    result.push(' ');
                }
                result.push_str(&ident.to_string());
                prev_needs_space = true;
            }
            proc_macro2::TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                // Handle # and @ specially for interpolation
                if ch == '#' || ch == '@' {
                    result.push(ch);
                } else {
                    // Some punctuation needs spacing
                    if needs_space_before_punct(ch) && prev_needs_space {
                        result.push(' ');
                    }
                    result.push(ch);
                }
                prev_needs_space = needs_space_after_punct(ch);
            }
            proc_macro2::TokenTree::Literal(lit) => {
                if prev_needs_space {
                    result.push(' ');
                }
                result.push_str(&lit.to_string());
                prev_needs_space = true;
            }
        }
    }

    result
}

/// Determine if a space is needed before a punctuation character.
fn needs_space_before_punct(_ch: char) -> bool {
    // Most operators don't need spaces in JavaScript for parsing purposes
    false
}

/// Determine if a space is needed after a punctuation character.
fn needs_space_after_punct(ch: char) -> bool {
    matches!(ch, ',' | ';' | ':')
}

/// Create a version of the source suitable for validation.
///
/// Replaces interpolation markers with valid JavaScript placeholders.
/// Handles both `#{...}` (value) and `@{...}` (raw) interpolation.
fn create_validation_source(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        // Handle value interpolation: #{...} and raw interpolation: @{...}
        if (c == '#' || c == '@') && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            skip_interpolation_expr(&mut chars);
            // Replace with a placeholder that's valid JS (null is always valid)
            result.push_str("null");
        } else {
            result.push(c);
        }
    }

    result
}

/// Skip over an interpolation expression in the character iterator.
/// Expects the opening '{' to have already been consumed.
fn skip_interpolation_expr(chars: &mut std::iter::Peekable<std::str::Chars>) {
    let mut depth = 1;
    for c in chars.by_ref() {
        if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests;
