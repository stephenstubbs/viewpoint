//! Page error types and event handling.
//!
//! This module provides types for capturing uncaught JavaScript exceptions
//! and errors from the page.

use viewpoint_cdp::protocol::runtime::{ExceptionDetails, ExceptionThrownEvent};

/// An uncaught exception from the page.
///
/// Page errors are emitted when JavaScript code throws an uncaught exception.
/// These correspond to the 'pageerror' event in Playwright.
///
/// # Example
///
/// ```ignore
/// page.on_pageerror(|error| async move {
///     println!("Page error: {}", error.message());
///     Ok(())
/// }).await;
/// ```
#[derive(Debug, Clone)]
pub struct PageError {
    /// Exception details.
    exception_details: ExceptionDetails,
    /// Timestamp when the error occurred.
    timestamp: f64,
}

impl PageError {
    /// Create a new page error from a CDP event.
    pub(crate) fn from_event(event: ExceptionThrownEvent) -> Self {
        Self {
            exception_details: event.exception_details,
            timestamp: event.timestamp,
        }
    }

    /// Get the error message.
    pub fn message(&self) -> String {
        // Try to get the exception message first
        if let Some(ref exception) = self.exception_details.exception {
            if let Some(ref description) = exception.description {
                return description.clone();
            }
            if let Some(ref value) = exception.value {
                if let Some(s) = value.as_str() {
                    return s.to_string();
                }
                return value.to_string();
            }
        }
        
        // Fall back to the exception text
        self.exception_details.text.clone()
    }

    /// Get the full stack trace as a string.
    pub fn stack(&self) -> Option<String> {
        self.exception_details.exception.as_ref().and_then(|exc| {
            exc.description.clone()
        })
    }

    /// Get the error name (e.g., "`TypeError`", "`ReferenceError`").
    pub fn name(&self) -> Option<String> {
        self.exception_details.exception.as_ref().and_then(|exc| {
            exc.class_name.clone()
        })
    }

    /// Get the URL where the error occurred.
    pub fn url(&self) -> Option<&str> {
        self.exception_details.url.as_deref()
    }

    /// Get the line number where the error occurred.
    pub fn line_number(&self) -> i64 {
        self.exception_details.line_number
    }

    /// Get the column number where the error occurred.
    pub fn column_number(&self) -> i64 {
        self.exception_details.column_number
    }

    /// Get the timestamp when the error occurred.
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }
}

impl std::fmt::Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name() {
            write!(f, "{}: {}", name, self.message())
        } else {
            write!(f, "{}", self.message())
        }
    }
}

impl std::error::Error for PageError {}

/// An error from any page in a browser context.
///
/// Web errors are emitted at the context level when any page has an uncaught
/// exception. They include a reference to the page where the error occurred.
///
/// # Example
///
/// ```ignore
/// context.on_weberror(|error| async move {
///     println!("Error on page {}: {}", error.page_url(), error.message());
///     Ok(())
/// }).await;
/// ```
#[derive(Debug, Clone)]
pub struct WebError {
    /// The underlying page error.
    error: PageError,
    /// Target ID of the page where the error occurred.
    target_id: String,
    /// Session ID of the page.
    session_id: String,
}

impl WebError {
    /// Create a new web error.
    pub(crate) fn new(error: PageError, target_id: String, session_id: String) -> Self {
        Self {
            error,
            target_id,
            session_id,
        }
    }

    /// Get the error message.
    pub fn message(&self) -> String {
        self.error.message()
    }

    /// Get the full stack trace as a string.
    pub fn stack(&self) -> Option<String> {
        self.error.stack()
    }

    /// Get the error name.
    pub fn name(&self) -> Option<String> {
        self.error.name()
    }

    /// Get the target ID of the page where the error occurred.
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Get the session ID of the page.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the underlying page error.
    pub fn page_error(&self) -> &PageError {
        &self.error
    }
}

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for WebError {}
