//! Error types for the test framework.

use thiserror::Error;

/// Errors that can occur during test execution.
#[derive(Debug, Error)]
pub enum TestError {
    /// Error from the core browser automation library.
    #[error("Core error: {0}")]
    Core(#[from] rustright_core::CoreError),

    /// Error during test harness setup.
    #[error("Harness setup failed: {0}")]
    Setup(String),

    /// Error during test harness cleanup.
    #[error("Harness cleanup failed: {0}")]
    Cleanup(String),

    /// Assertion failed.
    #[error("Assertion failed: {0}")]
    Assertion(#[from] AssertionError),

    /// Timeout exceeded.
    #[error("Timeout exceeded after {0:?}")]
    Timeout(std::time::Duration),
}

/// Error type for failed assertions.
#[derive(Debug, Error)]
#[error("{message}\n  Expected: {expected}\n  Actual: {actual}")]
pub struct AssertionError {
    /// Description of what was being asserted.
    pub message: String,
    /// The expected value.
    pub expected: String,
    /// The actual value.
    pub actual: String,
}

impl AssertionError {
    /// Create a new assertion error.
    pub fn new(message: impl Into<String>, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}
