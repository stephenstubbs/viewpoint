//! JavaScript parsing using `swc_ecma_parser`.

use swc_common::{FileName, SourceMap, Spanned, errors::Handler, sync::Lrc};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};

use std::io::Write;
use std::sync::{Arc, Mutex};

/// Collected error message from JavaScript parsing.
#[derive(Debug, Clone)]
pub struct JsParseError {
    pub message: String,
    pub line: Option<usize>,
    pub col: Option<usize>,
}

impl std::fmt::Display for JsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(line), Some(col)) = (self.line, self.col) {
            write!(f, "line {}, col {}: {}", line, col, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

/// Result of validating JavaScript.
pub type ParseResult = Result<(), JsParseError>;

/// A writer that discards all output.
struct NullWriter;

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Validate JavaScript syntax.
///
/// This function parses the given JavaScript source code and returns
/// an error if the syntax is invalid.
///
/// # Arguments
///
/// * `source` - The JavaScript source code to validate.
///
/// # Returns
///
/// * `Ok(())` if the JavaScript is valid
/// * `Err(JsParseError)` if there's a syntax error
pub fn validate_js(source: &str) -> ParseResult {
    use swc_ecma_ast::EsVersion;
    use swc_ecma_parser::EsSyntax;

    let cm: Lrc<SourceMap> = Lrc::default();
    let errors: Arc<Mutex<Vec<JsParseError>>> = Arc::new(Mutex::new(Vec::new()));
    let errors_clone = Arc::clone(&errors);
    let cm_clone = cm.clone();

    let fm = cm.new_source_file(FileName::Custom("js!".into()).into(), source.to_string());

    // Use a writer that discards output - we capture errors directly
    let _handler = Handler::with_emitter_writer(Box::new(NullWriter), Some(cm.clone()));

    let lexer = Lexer::new(
        Syntax::Es(EsSyntax::default()),
        EsVersion::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    // Capture lexer errors
    for e in parser.take_errors() {
        let span = e.span();
        let loc = cm_clone.lookup_char_pos(span.lo);
        let mut errs = errors_clone.lock().unwrap();
        errs.push(JsParseError {
            message: format!("{e:?}"),
            line: Some(loc.line),
            col: Some(loc.col_display + 1),
        });
    }

    // Check if we already have errors from lexing
    {
        let errs = errors.lock().unwrap();
        if let Some(err) = errs.first() {
            return Err(err.clone());
        }
    }

    // Try to parse as a script (allows both expressions and statements)
    match parser.parse_script() {
        Ok(_) => {
            // Check for any remaining errors
            for e in parser.take_errors() {
                let span = e.span();
                let loc = cm_clone.lookup_char_pos(span.lo);
                let mut errs = errors_clone.lock().unwrap();
                errs.push(JsParseError {
                    message: format!("{e:?}"),
                    line: Some(loc.line),
                    col: Some(loc.col_display + 1),
                });
            }

            let errs = errors.lock().unwrap();
            if let Some(err) = errs.first() {
                return Err(err.clone());
            }

            Ok(())
        }
        Err(e) => {
            let span = e.span();
            let loc = cm_clone.lookup_char_pos(span.lo);
            Err(JsParseError {
                message: format!("{e:?}"),
                line: Some(loc.line),
                col: Some(loc.col_display + 1),
            })
        }
    }
}

#[cfg(test)]
mod tests;
