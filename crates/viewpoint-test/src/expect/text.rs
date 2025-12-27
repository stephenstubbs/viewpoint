//! Text assertions for locators.
//!
//! This module contains assertions for checking text content
//! of elements matched by a locator.

use std::time::Duration;

use viewpoint_core::Locator;

use crate::error::AssertionError;

/// Text assertion methods for locators.
///
/// These methods are implemented separately and called via `LocatorAssertions`.
pub struct TextAssertions<'a> {
    locator: &'a Locator<'a>,
    timeout: Duration,
    is_negated: bool,
}

impl<'a> TextAssertions<'a> {
    /// Create a new `TextAssertions`.
    pub fn new(locator: &'a Locator<'a>, timeout: Duration, is_negated: bool) -> Self {
        Self {
            locator,
            timeout,
            is_negated,
        }
    }

    /// Assert that the element has the exact text content.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_text(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let text = self
                .locator
                .text_content()
                .await
                .map_err(|e| AssertionError::new("Failed to get text content", expected, e.to_string()))?;

            let actual = text.as_deref().unwrap_or("");
            let matches = actual.trim() == expected;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not have text"
                    } else {
                        "Element should have text"
                    },
                    if self.is_negated {
                        format!("not \"{expected}\"")
                    } else {
                        format!("\"{expected}\"")
                    },
                    format!("\"{actual}\""),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element contains the specified text.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_contain_text(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let text = self
                .locator
                .text_content()
                .await
                .map_err(|e| AssertionError::new("Failed to get text content", expected, e.to_string()))?;

            let actual = text.as_deref().unwrap_or("");
            let contains = actual.contains(expected);
            let expected_match = !self.is_negated;

            if contains == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not contain text"
                    } else {
                        "Element should contain text"
                    },
                    if self.is_negated {
                        format!("not containing \"{expected}\"")
                    } else {
                        format!("containing \"{expected}\"")
                    },
                    format!("\"{actual}\""),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that all elements have the specified texts (in order).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be queried.
    pub async fn to_have_texts(&self, expected: &[&str]) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self
                .locator
                .all_text_contents()
                .await
                .map_err(|e| AssertionError::new("Failed to get text contents", format!("{expected:?}"), e.to_string()))?;

            let actual_trimmed: Vec<&str> = actual.iter().map(|s| s.trim()).collect();
            let matches = actual_trimmed.len() == expected.len()
                && actual_trimmed.iter().zip(expected.iter()).all(|(a, e)| a == e);
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Elements should not have texts"
                    } else {
                        "Elements should have texts"
                    },
                    format!("{expected:?}"),
                    format!("{actual_trimmed:?}"),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that all elements contain the specified texts (in order).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be queried.
    pub async fn to_contain_texts(&self, expected: &[&str]) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self
                .locator
                .all_text_contents()
                .await
                .map_err(|e| AssertionError::new("Failed to get text contents", format!("{expected:?}"), e.to_string()))?;

            let matches = actual.len() == expected.len()
                && actual.iter().zip(expected.iter()).all(|(a, e)| a.contains(e));
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Elements should not contain texts"
                    } else {
                        "Elements should contain texts"
                    },
                    format!("{expected:?}"),
                    format!("{actual:?}"),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
