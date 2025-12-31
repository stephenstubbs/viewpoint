//! State handler implementations for the JavaScript scanner.
//!
//! This module contains the state machine handlers for different JavaScript
//! syntactic contexts (strings, templates, regex, comments).

use super::{ScanResult, ScanState, Scanner};
use proc_macro2::TokenStream;

impl Scanner<'_> {
    /// Handle double-quoted string state.
    pub(super) fn handle_double_string(&mut self, c: char) {
        match c {
            '"' => self.state = ScanState::Normal,
            '\\' => self.state = ScanState::EscapeDouble,
            _ => {}
        }
    }

    /// Handle single-quoted string state.
    pub(super) fn handle_single_string(&mut self, c: char) {
        match c {
            '\'' => self.state = ScanState::Normal,
            '\\' => self.state = ScanState::EscapeSingle,
            _ => {}
        }
    }

    /// Handle template string state.
    pub(super) fn handle_template_string(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
        match c {
            '`' => {
                literal.push(c);
                self.state = ScanState::Normal;
                None
            }
            '\\' => {
                literal.push(c);
                self.state = ScanState::EscapeTemplate;
                None
            }
            '$' => self.handle_template_dollar(c, literal),
            '#' => self.try_template_value_interpolation(c, idx, literal),
            '@' => self.try_template_raw_interpolation(c, idx, literal),
            _ => {
                literal.push(c);
                None
            }
        }
    }

    fn handle_template_dollar(&mut self, c: char, literal: &mut String) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            literal.push(c);
            self.chars.next();
            literal.push('{');
            self.template_depth_stack.push(self.brace_depth);
            self.brace_depth = 1;
            self.state = ScanState::TemplateExpr;
        } else {
            literal.push(c);
        }
        None
    }

    fn try_template_value_interpolation(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            self.chars.next();
            if let Some(tokens) = self.parse_interpolation_expr(idx + 2) {
                return Some(ScanResult::ValueInterpolation(tokens));
            }
            literal.push('#');
            literal.push('{');
        } else {
            literal.push(c);
        }
        None
    }

    fn try_template_raw_interpolation(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            self.chars.next();
            if let Some(tokens) = self.parse_interpolation_expr(idx + 2) {
                return Some(ScanResult::RawInterpolation(tokens));
            }
            literal.push('@');
            literal.push('{');
        } else {
            literal.push(c);
        }
        None
    }

    /// Handle template string state for validation.
    pub(super) fn handle_template_string_validation(
        &mut self,
        c: char,
        idx: usize,
        result: &mut String,
    ) -> Option<ScanResult> {
        match c {
            '`' => {
                result.push(c);
                self.state = ScanState::Normal;
                None
            }
            '\\' => {
                result.push(c);
                self.state = ScanState::EscapeTemplate;
                None
            }
            '$' => self.handle_template_dollar_validation(c, result),
            '#' => self.try_template_value_interpolation(c, idx, result),
            '@' => self.try_template_raw_interpolation(c, idx, result),
            _ => {
                result.push(c);
                None
            }
        }
    }

    fn handle_template_dollar_validation(
        &mut self,
        c: char,
        result: &mut String,
    ) -> Option<ScanResult> {
        if let Some(&(_, '{')) = self.chars.peek() {
            result.push(c);
            self.chars.next();
            result.push('{');
            self.template_depth_stack.push(self.brace_depth);
            self.brace_depth = 1;
            self.state = ScanState::TemplateExpr;
        } else {
            result.push(c);
        }
        None
    }

    /// Handle template expression state (inside `${...}`).
    pub(super) fn handle_template_expr(
        &mut self,
        c: char,
        idx: usize,
        literal: &mut String,
    ) -> Option<ScanResult> {
        match c {
            '`' => {
                literal.push(c);
                self.template_depth_stack.push(self.brace_depth);
                self.brace_depth = 0;
                self.state = ScanState::TemplateString;
                None
            }
            '{' => {
                literal.push(c);
                self.brace_depth += 1;
                None
            }
            '}' => {
                self.brace_depth -= 1;
                literal.push(c);
                if self.brace_depth == 0 {
                    self.brace_depth = self.template_depth_stack.pop().unwrap_or(0);
                    self.state = ScanState::TemplateString;
                }
                None
            }
            '"' => {
                literal.push(c);
                self.state = ScanState::DoubleString;
                None
            }
            '\'' => {
                literal.push(c);
                self.state = ScanState::SingleString;
                None
            }
            '#' => self.try_template_value_interpolation(c, idx, literal),
            '@' => self.try_template_raw_interpolation(c, idx, literal),
            _ => {
                literal.push(c);
                None
            }
        }
    }

    /// Handle template expression state for validation.
    pub(super) fn handle_template_expr_validation(
        &mut self,
        c: char,
        idx: usize,
        result: &mut String,
    ) -> Option<ScanResult> {
        match c {
            '`' => {
                result.push(c);
                self.template_depth_stack.push(self.brace_depth);
                self.brace_depth = 0;
                self.state = ScanState::TemplateString;
                None
            }
            '{' => {
                result.push(c);
                self.brace_depth += 1;
                None
            }
            '}' => {
                self.brace_depth -= 1;
                result.push(c);
                if self.brace_depth == 0 {
                    self.brace_depth = self.template_depth_stack.pop().unwrap_or(0);
                    self.state = ScanState::TemplateString;
                }
                None
            }
            '"' => {
                result.push(c);
                self.state = ScanState::DoubleString;
                None
            }
            '\'' => {
                result.push(c);
                self.state = ScanState::SingleString;
                None
            }
            '#' => self.try_template_value_interpolation(c, idx, result),
            '@' => self.try_template_raw_interpolation(c, idx, result),
            _ => {
                result.push(c);
                None
            }
        }
    }

    /// Handle regex state.
    pub(super) fn handle_regex(&mut self, c: char) {
        match c {
            '/' => {
                self.state = ScanState::Normal;
                self.last_token_allows_regex = false;
            }
            '\\' => self.state = ScanState::EscapeRegex,
            '[' => self.state = ScanState::RegexCharClass,
            _ => {}
        }
    }

    /// Handle regex character class state.
    pub(super) fn handle_regex_char_class(&mut self, c: char) {
        match c {
            ']' => self.state = ScanState::Regex,
            '\\' => self.state = ScanState::EscapeRegexCharClass,
            _ => {}
        }
    }

    /// Handle line comment state.
    pub(super) fn handle_line_comment(&mut self, c: char) {
        if c == '\n' {
            self.state = ScanState::Normal;
        }
    }

    /// Handle block comment state.
    pub(super) fn handle_block_comment(&mut self, c: char, literal: &mut String) {
        if c == '*' {
            if let Some(&(_, '/')) = self.chars.peek() {
                self.chars.next();
                literal.push('/');
                self.state = ScanState::Normal;
            }
        }
    }

    /// Parse a Rust interpolation expression.
    pub(super) fn parse_interpolation_expr(&mut self, _start_idx: usize) -> Option<TokenStream> {
        let mut depth = 1;
        let mut expr_str = String::new();

        while let Some((_, c)) = self.chars.next() {
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

        expr_str.parse::<TokenStream>().ok()
    }
}
