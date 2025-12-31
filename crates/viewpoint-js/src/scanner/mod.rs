//! JavaScript-aware scanner for the js! macro.
//!
//! This module provides a scanner that understands JavaScript syntax well enough
//! to correctly identify Rust interpolation markers (`#{expr}` and `@{expr}`)
//! while distinguishing them from similar patterns inside JavaScript strings,
//! template literals, regex literals, and comments.

mod handlers;
mod state;
#[cfg(test)]
mod tests;

pub use state::{ScanResult, ScanState};

use crate::interpolation::Segment;

/// Scan JavaScript source code and extract segments (literal JS and interpolations).
///
/// This scanner correctly handles:
/// - Single-quoted strings (`'...'`)
/// - Double-quoted strings (`"..."`)
/// - Template literals (`` `...` ``) including `${...}` expressions
/// - Regex literals (`/.../flags`)
/// - Line comments (`//...`)
/// - Block comments (`/*...*/`)
/// - Rust interpolation markers (`#{expr}` and `@{expr}`) only in code context
///
/// # Arguments
/// * `source` - The JavaScript source code to scan
///
/// # Returns
/// A tuple of (segments, has_interpolation)
pub fn scan_js_source(source: &str) -> (Vec<Segment>, bool) {
    let mut scanner = Scanner::new(source);
    scanner.scan()
}

/// Create validation source by replacing interpolations with `null`.
///
/// This preserves the overall structure of the JavaScript while replacing
/// Rust interpolation markers with valid JavaScript placeholders.
pub fn create_validation_source(source: &str) -> String {
    let mut scanner = Scanner::new(source);
    scanner.create_validation_source()
}

/// JavaScript-aware scanner.
pub(crate) struct Scanner<'a> {
    source: &'a str,
    pub(super) chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    pub(super) state: ScanState,
    /// Stack for tracking nested template literal expressions
    pub(super) template_depth_stack: Vec<usize>,
    /// Current brace depth (for tracking `${...}` in templates)
    pub(super) brace_depth: usize,
    /// Context for regex vs division disambiguation
    pub(super) last_token_allows_regex: bool,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            state: ScanState::Normal,
            template_depth_stack: Vec::new(),
            brace_depth: 0,
            last_token_allows_regex: true,
        }
    }

    /// Scan and return segments and whether interpolation was found.
    fn scan(&mut self) -> (Vec<Segment>, bool) {
        let mut segments = Vec::new();
        let mut has_interpolation = false;
        let mut current_literal = String::new();

        while let Some((idx, c)) = self.chars.next() {
            if let Some(result) = self.process_char(c, idx, &mut current_literal) {
                match result {
                    ScanResult::ValueInterpolation(tokens) => {
                        if !current_literal.is_empty() {
                            segments.push(Segment::Literal(std::mem::take(&mut current_literal)));
                        }
                        segments.push(Segment::ValueInterpolation(tokens));
                        has_interpolation = true;
                    }
                    ScanResult::RawInterpolation(tokens) => {
                        if !current_literal.is_empty() {
                            segments.push(Segment::Literal(std::mem::take(&mut current_literal)));
                        }
                        segments.push(Segment::RawInterpolation(tokens));
                        has_interpolation = true;
                    }
                    ScanResult::Continue => {}
                }
            }
        }

        if !current_literal.is_empty() {
            segments.push(Segment::Literal(current_literal));
        }

        (segments, has_interpolation)
    }

    /// Create validation source with interpolations replaced by `null`.
    fn create_validation_source(&mut self) -> String {
        let mut result = String::new();

        while let Some((idx, c)) = self.chars.next() {
            if let Some(scan_result) = self.process_char_for_validation(c, idx, &mut result) {
                match scan_result {
                    ScanResult::ValueInterpolation(_) | ScanResult::RawInterpolation(_) => {
                        result.push_str("null");
                    }
                    ScanResult::Continue => {}
                }
            }
        }

        result
    }

    /// Process a character and update state.
    fn process_char(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
        match self.state {
            ScanState::Normal => self.handle_normal(c, idx, literal),
            ScanState::DoubleString => {
                literal.push(c);
                self.handle_double_string(c);
                None
            }
            ScanState::SingleString => {
                literal.push(c);
                self.handle_single_string(c);
                None
            }
            ScanState::TemplateString => self.handle_template_string(c, idx, literal),
            ScanState::TemplateExpr => self.handle_template_expr(c, idx, literal),
            ScanState::Regex => {
                literal.push(c);
                self.handle_regex(c);
                None
            }
            ScanState::RegexCharClass => {
                literal.push(c);
                self.handle_regex_char_class(c);
                None
            }
            ScanState::LineComment => {
                literal.push(c);
                self.handle_line_comment(c);
                None
            }
            ScanState::BlockComment => {
                literal.push(c);
                self.handle_block_comment(c, literal);
                None
            }
            ScanState::EscapeDouble => {
                literal.push(c);
                self.state = ScanState::DoubleString;
                None
            }
            ScanState::EscapeSingle => {
                literal.push(c);
                self.state = ScanState::SingleString;
                None
            }
            ScanState::EscapeTemplate => {
                literal.push(c);
                self.state = ScanState::TemplateString;
                None
            }
            ScanState::EscapeRegex => {
                literal.push(c);
                self.state = ScanState::Regex;
                None
            }
            ScanState::EscapeRegexCharClass => {
                literal.push(c);
                self.state = ScanState::RegexCharClass;
                None
            }
        }
    }

    /// Process a character for validation source.
    fn process_char_for_validation(
        &mut self,
        c: char,
        idx: usize,
        result: &mut String,
    ) -> Option<ScanResult> {
        match self.state {
            ScanState::Normal => self.handle_normal_validation(c, idx, result),
            ScanState::DoubleString => {
                result.push(c);
                self.handle_double_string(c);
                None
            }
            ScanState::SingleString => {
                result.push(c);
                self.handle_single_string(c);
                None
            }
            ScanState::TemplateString => self.handle_template_string_validation(c, idx, result),
            ScanState::TemplateExpr => self.handle_template_expr_validation(c, idx, result),
            ScanState::Regex => {
                result.push(c);
                self.handle_regex(c);
                None
            }
            ScanState::RegexCharClass => {
                result.push(c);
                self.handle_regex_char_class(c);
                None
            }
            ScanState::LineComment => {
                result.push(c);
                self.handle_line_comment(c);
                None
            }
            ScanState::BlockComment => {
                result.push(c);
                self.handle_block_comment(c, result);
                None
            }
            ScanState::EscapeDouble => {
                result.push(c);
                self.state = ScanState::DoubleString;
                None
            }
            ScanState::EscapeSingle => {
                result.push(c);
                self.state = ScanState::SingleString;
                None
            }
            ScanState::EscapeTemplate => {
                result.push(c);
                self.state = ScanState::TemplateString;
                None
            }
            ScanState::EscapeRegex => {
                result.push(c);
                self.state = ScanState::Regex;
                None
            }
            ScanState::EscapeRegexCharClass => {
                result.push(c);
                self.state = ScanState::RegexCharClass;
                None
            }
        }
    }

    /// Handle Normal state.
    fn handle_normal(&mut self, c: char, idx: usize, literal: &mut String) -> Option<ScanResult> {
        match c {
            '"' => {
                literal.push(c);
                self.state = ScanState::DoubleString;
                self.last_token_allows_regex = false;
                None
            }
            '\'' => {
                literal.push(c);
                self.state = ScanState::SingleString;
                self.last_token_allows_regex = false;
                None
            }
            '`' => {
                literal.push(c);
                self.state = ScanState::TemplateString;
                self.last_token_allows_regex = false;
                None
            }
            '/' => self.handle_slash(c, literal),
            '#' => self.try_value_interpolation(c, idx, literal),
            '@' => self.try_raw_interpolation(c, idx, literal),
            '{' => {
                literal.push(c);
                self.brace_depth += 1;
                self.last_token_allows_regex = true;
                None
            }
            '}' => {
                literal.push(c);
                self.brace_depth = self.brace_depth.saturating_sub(1);
                self.last_token_allows_regex = false;
                None
            }
            '(' | '[' | ',' | ';' | ':' | '?' | '!' | '=' | '+' | '-' | '*' | '%' | '&' | '|'
            | '^' | '<' | '>' | '~' => {
                literal.push(c);
                self.last_token_allows_regex = true;
                None
            }
            ')' | ']' => {
                literal.push(c);
                self.last_token_allows_regex = false;
                None
            }
            _ if c.is_alphanumeric() || c == '_' || c == '$' => {
                literal.push(c);
                self.last_token_allows_regex = self.check_keyword_allows_regex(idx);
                None
            }
            _ => {
                literal.push(c);
                None
            }
        }
    }

    /// Handle Normal state for validation.
    fn handle_normal_validation(
        &mut self,
        c: char,
        idx: usize,
        result: &mut String,
    ) -> Option<ScanResult> {
        match c {
            '"' => {
                result.push(c);
                self.state = ScanState::DoubleString;
                self.last_token_allows_regex = false;
                None
            }
            '\'' => {
                result.push(c);
                self.state = ScanState::SingleString;
                self.last_token_allows_regex = false;
                None
            }
            '`' => {
                result.push(c);
                self.state = ScanState::TemplateString;
                self.last_token_allows_regex = false;
                None
            }
            '/' => {
                result.push(c);
                self.handle_slash_state_only();
                None
            }
            '#' => self.try_value_interpolation_validation(c, idx, result),
            '@' => self.try_raw_interpolation_validation(c, idx, result),
            '{' => {
                result.push(c);
                self.brace_depth += 1;
                self.last_token_allows_regex = true;
                None
            }
            '}' => {
                result.push(c);
                self.brace_depth = self.brace_depth.saturating_sub(1);
                self.last_token_allows_regex = false;
                None
            }
            '(' | '[' | ',' | ';' | ':' | '?' | '!' | '=' | '+' | '-' | '*' | '%' | '&' | '|'
            | '^' | '<' | '>' | '~' => {
                result.push(c);
                self.last_token_allows_regex = true;
                None
            }
            ')' | ']' => {
                result.push(c);
                self.last_token_allows_regex = false;
                None
            }
            _ if c.is_alphanumeric() || c == '_' || c == '$' => {
                result.push(c);
                self.last_token_allows_regex = self.check_keyword_allows_regex(idx);
                None
            }
            _ => {
                result.push(c);
                None
            }
        }
    }

    fn handle_slash(&mut self, c: char, literal: &mut String) -> Option<ScanResult> {
        if let Some(&(_, next)) = self.chars.peek() {
            if next == '/' {
                literal.push(c);
                self.state = ScanState::LineComment;
            } else if next == '*' {
                literal.push(c);
                self.state = ScanState::BlockComment;
            } else if self.last_token_allows_regex {
                literal.push(c);
                self.state = ScanState::Regex;
            } else {
                literal.push(c);
                self.last_token_allows_regex = true;
            }
        } else {
            literal.push(c);
        }
        None
    }

    fn handle_slash_state_only(&mut self) {
        if let Some(&(_, next)) = self.chars.peek() {
            if next == '/' {
                self.state = ScanState::LineComment;
            } else if next == '*' {
                self.state = ScanState::BlockComment;
            } else if self.last_token_allows_regex {
                self.state = ScanState::Regex;
            } else {
                self.last_token_allows_regex = true;
            }
        }
    }

    fn try_value_interpolation(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            self.chars.next();
            if let Some(tokens) = self.parse_interpolation_expr(idx + 2) {
                self.last_token_allows_regex = false;
                return Some(ScanResult::ValueInterpolation(tokens));
            }
            literal.push('#');
            literal.push('{');
        } else {
            literal.push(c);
        }
        None
    }

    fn try_raw_interpolation(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            self.chars.next();
            if let Some(tokens) = self.parse_interpolation_expr(idx + 2) {
                self.last_token_allows_regex = false;
                return Some(ScanResult::RawInterpolation(tokens));
            }
            literal.push('@');
            literal.push('{');
        } else {
            literal.push(c);
        }
        None
    }

    fn try_value_interpolation_validation(
        &mut self,
        c: char,
        idx: usize,
        result: &mut String,
    ) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            self.chars.next();
            if let Some(tokens) = self.parse_interpolation_expr(idx + 2) {
                self.last_token_allows_regex = false;
                return Some(ScanResult::ValueInterpolation(tokens));
            }
            result.push('#');
            result.push('{');
        } else {
            result.push(c);
        }
        None
    }

    fn try_raw_interpolation_validation(
        &mut self,
        c: char,
        idx: usize,
        result: &mut String,
    ) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            self.chars.next();
            if let Some(tokens) = self.parse_interpolation_expr(idx + 2) {
                self.last_token_allows_regex = false;
                return Some(ScanResult::RawInterpolation(tokens));
            }
            result.push('@');
            result.push('{');
        } else {
            result.push(c);
        }
        None
    }

    fn check_keyword_allows_regex(&self, end_idx: usize) -> bool {
        let start = self.source[..end_idx]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '$')
            .map(|i| i + 1)
            .unwrap_or(0);

        let end = self
            .source
            .char_indices()
            .find(|(i, _)| *i > end_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.source.len());

        let ident = &self.source[start..end];

        matches!(
            ident,
            "return" | "case" | "throw" | "in" | "of" | "typeof" | "void" | "delete" | "new"
                | "instanceof" | "else" | "do" | "await" | "yield"
        )
    }
}
