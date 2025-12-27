//! Count assertions for locators.
//!
//! This module contains assertions for checking the count of elements
//! matched by a locator.

use std::time::Duration;

use viewpoint_core::Locator;

use crate::error::AssertionError;

/// Count assertion methods for locators.
///
/// These methods are implemented separately and called via `LocatorAssertions`.
pub struct CountAssertions<'a> {
    locator: &'a Locator<'a>,
    timeout: Duration,
    is_negated: bool,
}

impl<'a> CountAssertions<'a> {
    /// Create a new `CountAssertions`.
    pub fn new(locator: &'a Locator<'a>, timeout: Duration, is_negated: bool) -> Self {
        Self {
            locator,
            timeout,
            is_negated,
        }
    }

    /// Assert that the element has the specified count.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count(&self, expected: usize) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self
                .locator
                .count()
                .await
                .map_err(|e| AssertionError::new("Failed to count elements", expected.to_string(), e.to_string()))?;

            let matches = actual == expected;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element count should not be {expected}")
                    } else {
                        format!("Element count should be {expected}")
                    },
                    expected.to_string(),
                    actual.to_string(),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element count is greater than a value.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_greater_than(&self, n: usize) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self
                .locator
                .count()
                .await
                .map_err(|e| AssertionError::new("Failed to count elements", format!("> {n}"), e.to_string()))?;

            let matches = actual > n;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element count should not be greater than {n}")
                    } else {
                        format!("Element count should be greater than {n}")
                    },
                    format!("> {n}"),
                    actual.to_string(),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element count is less than a value.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_less_than(&self, n: usize) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self
                .locator
                .count()
                .await
                .map_err(|e| AssertionError::new("Failed to count elements", format!("< {n}"), e.to_string()))?;

            let matches = actual < n;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element count should not be less than {n}")
                    } else {
                        format!("Element count should be less than {n}")
                    },
                    format!("< {n}"),
                    actual.to_string(),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element count is at least a value (greater than or equal).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_at_least(&self, n: usize) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self
                .locator
                .count()
                .await
                .map_err(|e| AssertionError::new("Failed to count elements", format!(">= {n}"), e.to_string()))?;

            let matches = actual >= n;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element count should not be at least {n}")
                    } else {
                        format!("Element count should be at least {n}")
                    },
                    format!(">= {n}"),
                    actual.to_string(),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element count is at most a value (less than or equal).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_at_most(&self, n: usize) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self
                .locator
                .count()
                .await
                .map_err(|e| AssertionError::new("Failed to count elements", format!("<= {n}"), e.to_string()))?;

            let matches = actual <= n;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element count should not be at most {n}")
                    } else {
                        format!("Element count should be at most {n}")
                    },
                    format!("<= {n}"),
                    actual.to_string(),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
