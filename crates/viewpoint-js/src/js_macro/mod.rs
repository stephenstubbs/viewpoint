//! The js! proc-macro implementation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::interpolation::generate_interpolated_code;
use crate::parser::validate_js;
use crate::scanner;

/// Implementation of the js! macro.
pub fn js_impl(input: TokenStream) -> TokenStream {
    // Convert to proc_macro2 and get the string representation
    // This goes through Rust's tokenizer, which may mangle some JS syntax
    let input2: TokenStream2 = input.into();
    let js_source = tokens_to_js_string(&input2);

    js_impl_from_source(&js_source)
}

/// Process JavaScript source and generate output code.
fn js_impl_from_source(js_source: &str) -> TokenStream {
    // Use the scanner to parse for interpolations
    let (segments, has_interpolation) = scanner::scan_js_source(js_source);

    // Create validation source with interpolations replaced by null
    let validation_source = scanner::create_validation_source(js_source);

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
                // They still need space before them if previous token was an identifier
                if ch == '#' || ch == '@' {
                    if prev_needs_space {
                        result.push(' ');
                    }
                    result.push(ch);
                    // After # or @, we don't need a space before the following {
                    prev_needs_space = false;
                } else {
                    // Some punctuation needs spacing
                    if needs_space_before_punct(ch) && prev_needs_space {
                        result.push(' ');
                    }
                    result.push(ch);
                    prev_needs_space = needs_space_after_punct(ch);
                }
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

#[cfg(test)]
mod tests;
