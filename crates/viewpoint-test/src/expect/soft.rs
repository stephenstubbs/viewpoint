//! Soft assertions that collect failures without stopping test execution.
//!
//! Soft assertions allow you to make multiple assertions and collect all failures,
//! rather than failing on the first assertion. This is useful when you want to
//! check multiple things in a test and see all failures at once.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "integration")]
//! # tokio_test::block_on(async {
//! # use viewpoint_core::Browser;
//! use viewpoint_test::SoftAssertions;
//! # let browser = Browser::launch().headless(true).launch().await.unwrap();
//! # let context = browser.new_context().await.unwrap();
//! # let page = context.new_page().await.unwrap();
//! # page.goto("https://example.com").goto().await.unwrap();
//!
//! let soft = SoftAssertions::new();
//!
//! // These assertions don't fail immediately
//! let locator = page.locator("h1");
//! soft.expect(&locator).to_be_visible().await;
//! soft.expect(&locator).to_have_text("Example Domain").await;
//!
//! // Check if all assertions passed
//! if !soft.passed() {
//!     println!("Failures: {:?}", soft.errors());
//! }
//!
//! // Or assert all at the end (fails if any assertion failed)
//! soft.assert_all().unwrap();
//! # });
//! ```

use std::sync::{Arc, Mutex};

use viewpoint_core::{Locator, Page};

use super::soft_locator::SoftLocatorAssertions;
use super::soft_page::SoftPageAssertions;
use super::{LocatorAssertions, PageAssertions};
use crate::error::TestError;

/// Collection of soft assertion errors.
#[derive(Debug, Clone)]
pub struct SoftAssertionError {
    /// The assertion that failed.
    pub assertion: String,
    /// The error message.
    pub message: String,
    /// Optional expected value.
    pub expected: Option<String>,
    /// Optional actual value.
    pub actual: Option<String>,
}

impl SoftAssertionError {
    /// Create a new soft assertion error.
    pub fn new(assertion: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            assertion: assertion.into(),
            message: message.into(),
            expected: None,
            actual: None,
        }
    }

    /// Set the expected value.
    #[must_use]
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    /// Set the actual value.
    #[must_use]
    pub fn with_actual(mut self, actual: impl Into<String>) -> Self {
        self.actual = Some(actual.into());
        self
    }
}

impl std::fmt::Display for SoftAssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.assertion, self.message)?;
        if let Some(ref expected) = self.expected {
            write!(f, "\n  Expected: {expected}")?;
        }
        if let Some(ref actual) = self.actual {
            write!(f, "\n  Actual: {actual}")?;
        }
        Ok(())
    }
}

/// A context for collecting soft assertions.
///
/// Soft assertions allow you to make multiple assertions without stopping
/// on the first failure. All failures are collected and can be checked at the end.
#[derive(Debug, Clone)]
pub struct SoftAssertions {
    errors: Arc<Mutex<Vec<SoftAssertionError>>>,
}

impl Default for SoftAssertions {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftAssertions {
    /// Create a new soft assertion context.
    pub fn new() -> Self {
        Self {
            errors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create soft assertions for a locator.
    ///
    /// Assertions made through this will not fail immediately, but will be
    /// collected for later inspection.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "integration")]
    /// # tokio_test::block_on(async {
    /// # use viewpoint_core::Browser;
    /// use viewpoint_test::SoftAssertions;
    /// # let browser = Browser::launch().headless(true).launch().await.unwrap();
    /// # let context = browser.new_context().await.unwrap();
    /// # let page = context.new_page().await.unwrap();
    /// # page.goto("https://example.com").goto().await.unwrap();
    ///
    /// let soft = SoftAssertions::new();
    /// let locator = page.locator("h1");
    /// soft.expect(&locator).to_be_visible().await;
    /// soft.expect(&locator).to_have_text("Example Domain").await;
    /// soft.assert_all().unwrap();
    /// # });
    /// ```
    pub fn expect<'a>(&self, locator: &'a Locator<'a>) -> SoftLocatorAssertions<'a> {
        SoftLocatorAssertions {
            assertions: LocatorAssertions::new(locator),
            errors: self.errors.clone(),
        }
    }

    /// Create soft assertions for a page.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "integration")]
    /// # tokio_test::block_on(async {
    /// # use viewpoint_core::Browser;
    /// use viewpoint_test::SoftAssertions;
    /// # let browser = Browser::launch().headless(true).launch().await.unwrap();
    /// # let context = browser.new_context().await.unwrap();
    /// # let page = context.new_page().await.unwrap();
    /// # page.goto("https://example.com").goto().await.unwrap();
    ///
    /// let soft = SoftAssertions::new();
    /// soft.expect_page(&page).to_have_url("https://example.com/").await;
    /// soft.assert_all().unwrap();
    /// # });
    /// ```
    pub fn expect_page<'a>(&self, page: &'a Page) -> SoftPageAssertions<'a> {
        SoftPageAssertions {
            assertions: PageAssertions::new(page),
            errors: self.errors.clone(),
        }
    }

    /// Check if all soft assertions passed.
    pub fn passed(&self) -> bool {
        self.errors.lock().unwrap().is_empty()
    }

    /// Get all collected errors.
    pub fn errors(&self) -> Vec<SoftAssertionError> {
        self.errors.lock().unwrap().clone()
    }

    /// Get the number of failed assertions.
    pub fn failure_count(&self) -> usize {
        self.errors.lock().unwrap().len()
    }

    /// Clear all collected errors.
    pub fn clear(&self) {
        self.errors.lock().unwrap().clear();
    }

    /// Assert that all soft assertions passed.
    ///
    /// This will fail with a combined error message if any assertions failed.
    ///
    /// # Errors
    ///
    /// Returns an error containing all assertion failures if any occurred.
    pub fn assert_all(&self) -> Result<(), TestError> {
        let errors = self.errors.lock().unwrap();
        if errors.is_empty() {
            return Ok(());
        }

        let mut message = format!("{} soft assertion(s) failed:", errors.len());
        for (i, error) in errors.iter().enumerate() {
            message.push_str(&format!("\n{}. {}", i + 1, error));
        }

        Err(TestError::Assertion(crate::error::AssertionError::new(
            message,
            format!("{} assertions to pass", errors.len()),
            format!("{} assertions failed", errors.len()),
        )))
    }

    /// Add an error to the collection.
    pub fn add_error(&self, error: SoftAssertionError) {
        self.errors.lock().unwrap().push(error);
    }
}
