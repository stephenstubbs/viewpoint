//! Scanner state definitions.

use proc_macro2::TokenStream;

/// State of the JavaScript scanner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanState {
    /// Normal code context
    Normal,
    /// Inside a double-quoted string "..."
    DoubleString,
    /// Inside a single-quoted string '...'
    SingleString,
    /// Inside a template literal `...`
    TemplateString,
    /// Inside a template expression ${...}
    TemplateExpr,
    /// Inside a regex literal /.../
    Regex,
    /// Inside a regex character class [...]
    RegexCharClass,
    /// Inside a line comment //...
    LineComment,
    /// Inside a block comment /*...*/
    BlockComment,
    /// After a backslash in a double-quoted string
    EscapeDouble,
    /// After a backslash in a single-quoted string
    EscapeSingle,
    /// After a backslash in a template literal
    EscapeTemplate,
    /// After a backslash in a regex
    EscapeRegex,
    /// After a backslash in a regex character class
    EscapeRegexCharClass,
}

/// Result from scanning a character that might be an interpolation.
pub enum ScanResult {
    /// Found a value interpolation `#{expr}`
    ValueInterpolation(TokenStream),
    /// Found a raw interpolation `@{expr}`
    RawInterpolation(TokenStream),
    /// Continue scanning (no interpolation found)
    Continue,
}
