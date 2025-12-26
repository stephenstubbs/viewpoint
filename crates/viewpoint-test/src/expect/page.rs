//! Page assertions for testing page state.

use std::time::Duration;

use viewpoint_core::Page;

use crate::error::AssertionError;

/// Default timeout for assertions.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Assertions for pages.
pub struct PageAssertions<'a> {
    page: &'a Page,
    timeout: Duration,
    is_negated: bool,
}

impl<'a> PageAssertions<'a> {
    /// Create a new `PageAssertions` for the given page.
    pub fn new(page: &'a Page) -> Self {
        Self {
            page,
            timeout: DEFAULT_TIMEOUT,
            is_negated: false,
        }
    }

    /// Set the timeout for this assertion.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Negate the assertion.
    ///
    /// This is an alias for the `not` method to avoid conflict with `std::ops::Not`.
    #[must_use]
    pub fn negated(mut self) -> Self {
        self.is_negated = !self.is_negated;
        self
    }

    /// Negate the assertion.
    ///
    /// Note: This method name shadows the `Not` trait's method. Use `negated()` if
    /// you need to avoid this conflict.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        self.negated()
    }

    /// Assert that the page has the specified URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the URL cannot be retrieved.
    pub async fn to_have_url(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let url = self
                .page
                .url()
                .await
                .map_err(|e| AssertionError::new("Failed to get URL", expected, e.to_string()))?;

            let matches = url == expected;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Page should not have URL"
                    } else {
                        "Page should have URL"
                    },
                    if self.is_negated {
                        format!("not \"{expected}\"")
                    } else {
                        expected.to_string()
                    },
                    url,
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the page URL contains the specified substring.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the URL cannot be retrieved.
    pub async fn to_have_url_containing(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let url = self
                .page
                .url()
                .await
                .map_err(|e| AssertionError::new("Failed to get URL", expected, e.to_string()))?;

            let contains = url.contains(expected);
            let expected_match = !self.is_negated;

            if contains == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Page URL should not contain"
                    } else {
                        "Page URL should contain"
                    },
                    if self.is_negated {
                        format!("not containing \"{expected}\"")
                    } else {
                        format!("containing \"{expected}\"")
                    },
                    url,
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the page has the specified title.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the title cannot be retrieved.
    pub async fn to_have_title(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let title = self
                .page
                .title()
                .await
                .map_err(|e| AssertionError::new("Failed to get title", expected, e.to_string()))?;

            let matches = title == expected;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Page should not have title"
                    } else {
                        "Page should have title"
                    },
                    if self.is_negated {
                        format!("not \"{expected}\"")
                    } else {
                        expected.to_string()
                    },
                    title,
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the page title contains the specified substring.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the title cannot be retrieved.
    pub async fn to_have_title_containing(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let title = self
                .page
                .title()
                .await
                .map_err(|e| AssertionError::new("Failed to get title", expected, e.to_string()))?;

            let contains = title.contains(expected);
            let expected_match = !self.is_negated;

            if contains == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Page title should not contain"
                    } else {
                        "Page title should contain"
                    },
                    if self.is_negated {
                        format!("not containing \"{expected}\"")
                    } else {
                        format!("containing \"{expected}\"")
                    },
                    title,
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
