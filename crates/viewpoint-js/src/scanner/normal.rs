//! Normal state handler implementations for the JavaScript scanner.
//!
//! This module contains handlers for the Normal scanning state, including
//! slash handling (comments/regex), interpolation detection, and keyword analysis.

use super::{ScanResult, ScanState, Scanner};

impl Scanner<'_> {
    /// Handle Normal state.
    pub(super) fn handle_normal(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
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
    pub(super) fn handle_normal_validation(
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

    /// Handle a slash character, determining if it starts a comment or regex.
    pub(super) fn handle_slash(&mut self, c: char, literal: &mut String) -> Option<ScanResult> {
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

    /// Handle slash for state updates only (used in validation mode).
    pub(super) fn handle_slash_state_only(&mut self) {
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

    /// Try to parse a value interpolation `#{expr}`.
    pub(super) fn try_value_interpolation(
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

    /// Try to parse a raw interpolation `@{expr}`.
    pub(super) fn try_raw_interpolation(
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

    /// Try to parse a value interpolation for validation mode.
    pub(super) fn try_value_interpolation_validation(
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

    /// Try to parse a raw interpolation for validation mode.
    pub(super) fn try_raw_interpolation_validation(
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

    /// Check if a keyword allows a regex to follow.
    pub(super) fn check_keyword_allows_regex(&self, end_idx: usize) -> bool {
        let start = self.source[..end_idx]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '$')
            .map_or(0, |i| i + 1);

        let end = self
            .source
            .char_indices()
            .find(|(i, _)| *i > end_idx)
            .map_or(self.source.len(), |(i, _)| i);

        let ident = &self.source[start..end];

        matches!(
            ident,
            "return"
                | "case"
                | "throw"
                | "in"
                | "of"
                | "typeof"
                | "void"
                | "delete"
                | "new"
                | "instanceof"
                | "else"
                | "do"
                | "await"
                | "yield"
        )
    }
}
