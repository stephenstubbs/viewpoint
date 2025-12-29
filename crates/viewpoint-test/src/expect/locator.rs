//! Locator assertions for testing element state.

use std::time::Duration;

use viewpoint_core::Locator;

use super::count::CountAssertions;
use super::state::StateAssertions;
use super::text::TextAssertions;
use crate::error::AssertionError;

/// Default timeout for assertions.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Assertions for locators (elements).
pub struct LocatorAssertions<'a> {
    locator: &'a Locator<'a>,
    timeout: Duration,
    is_negated: bool,
}

impl<'a> LocatorAssertions<'a> {
    /// Create a new `LocatorAssertions` for the given locator.
    pub fn new(locator: &'a Locator<'a>) -> Self {
        Self {
            locator,
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

    /// Assert that the element is visible.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_visible(&self) -> Result<(), AssertionError> {
        StateAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_be_visible()
            .await
    }

    /// Assert that the element is hidden.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_hidden(&self) -> Result<(), AssertionError> {
        StateAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_be_hidden()
            .await
    }

    /// Assert that the element has the exact text content.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_text(&self, expected: &str) -> Result<(), AssertionError> {
        TextAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_have_text(expected)
            .await
    }

    /// Assert that the element contains the specified text.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_contain_text(&self, expected: &str) -> Result<(), AssertionError> {
        TextAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_contain_text(expected)
            .await
    }

    /// Assert that the element has the specified attribute value.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_attribute(&self, name: &str, value: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self.get_attribute(name).await?;
            let matches = actual.as_deref() == Some(value);
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element should not have attribute {name}=\"{value}\"")
                    } else {
                        format!("Element should have attribute {name}=\"{value}\"")
                    },
                    if self.is_negated {
                        format!("not {name}=\"{value}\"")
                    } else {
                        format!("{name}=\"{value}\"")
                    },
                    match actual {
                        Some(v) => format!("{name}=\"{v}\""),
                        None => format!("{name} not present"),
                    },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element has the specified class.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_class(&self, class_name: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let class_attr = self.get_attribute("class").await?;
            let classes = class_attr.as_deref().unwrap_or("");
            let has_class = classes.split_whitespace().any(|c| c == class_name);
            let expected_match = !self.is_negated;

            if has_class == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element should not have class \"{class_name}\"")
                    } else {
                        format!("Element should have class \"{class_name}\"")
                    },
                    if self.is_negated {
                        format!("not containing class \"{class_name}\"")
                    } else {
                        format!("class \"{class_name}\"")
                    },
                    format!("classes: \"{classes}\""),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is enabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_enabled(&self) -> Result<(), AssertionError> {
        StateAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_be_enabled()
            .await
    }

    /// Assert that the element is disabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_disabled(&self) -> Result<(), AssertionError> {
        StateAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_be_disabled()
            .await
    }

    /// Assert that the element is checked (for checkboxes/radios).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_checked(&self) -> Result<(), AssertionError> {
        StateAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_be_checked()
            .await
    }

    /// Assert that the element has the specified value (for input/textarea/select).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_value(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self.get_input_value().await?;
            let matches = actual == expected;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not have value"
                    } else {
                        "Element should have value"
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

    /// Assert that a multi-select element has the specified values selected.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_values(&self, expected: &[&str]) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self.get_selected_values().await?;
            let expected_set: std::collections::HashSet<&str> = expected.iter().copied().collect();
            let actual_set: std::collections::HashSet<&str> =
                actual.iter().map(std::string::String::as_str).collect();
            let matches = expected_set == actual_set;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not have values"
                    } else {
                        "Element should have values"
                    },
                    if self.is_negated {
                        format!("not {expected:?}")
                    } else {
                        format!("{expected:?}")
                    },
                    format!("{actual:?}"),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element has the specified id.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_id(&self, expected: &str) -> Result<(), AssertionError> {
        self.to_have_attribute("id", expected).await
    }

    /// Assert that the element has the specified count.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count(&self, expected: usize) -> Result<(), AssertionError> {
        CountAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_have_count(expected)
            .await
    }

    /// Assert that the element count is greater than a value.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_greater_than(&self, n: usize) -> Result<(), AssertionError> {
        CountAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_have_count_greater_than(n)
            .await
    }

    /// Assert that the element count is less than a value.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_less_than(&self, n: usize) -> Result<(), AssertionError> {
        CountAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_have_count_less_than(n)
            .await
    }

    /// Assert that the element count is at least a value (greater than or equal).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_at_least(&self, n: usize) -> Result<(), AssertionError> {
        CountAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_have_count_at_least(n)
            .await
    }

    /// Assert that the element count is at most a value (less than or equal).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be counted.
    pub async fn to_have_count_at_most(&self, n: usize) -> Result<(), AssertionError> {
        CountAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_have_count_at_most(n)
            .await
    }

    /// Assert that all elements have the specified texts (in order).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be queried.
    pub async fn to_have_texts(&self, expected: &[&str]) -> Result<(), AssertionError> {
        TextAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_have_texts(expected)
            .await
    }

    /// Assert that the element's ARIA snapshot matches the expected structure.
    ///
    /// This method compares the accessibility tree of the element against an expected
    /// snapshot. The expected snapshot can contain regex patterns in name fields
    /// when enclosed in `/pattern/` syntax.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_match_aria_snapshot(
        &self,
        expected: &viewpoint_core::AriaSnapshot,
    ) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self.locator.aria_snapshot().await.map_err(|e| {
                AssertionError::new("Failed to get ARIA snapshot", "snapshot", e.to_string())
            })?;

            let matches = actual.matches(expected);
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                let diff = actual.diff(expected);
                return Err(AssertionError::new(
                    if self.is_negated {
                        "ARIA snapshot should not match"
                    } else {
                        "ARIA snapshot should match"
                    },
                    expected.to_yaml(),
                    format!("{}\n\nDiff:\n{}", actual.to_yaml(), diff),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element's ARIA snapshot matches the expected YAML string.
    ///
    /// This is a convenience method that parses the YAML string and delegates to
    /// `to_match_aria_snapshot`.
    ///
    /// # Errors
    ///
    /// Returns an error if the YAML parsing fails, the assertion fails, or the
    /// element cannot be queried.
    pub async fn to_match_aria_snapshot_yaml(
        &self,
        expected_yaml: &str,
    ) -> Result<(), AssertionError> {
        let expected = viewpoint_core::AriaSnapshot::from_yaml(expected_yaml).map_err(|e| {
            AssertionError::new(
                "Failed to parse expected ARIA snapshot",
                expected_yaml,
                e.to_string(),
            )
        })?;
        self.to_match_aria_snapshot(&expected).await
    }

    /// Assert that all elements contain the specified texts (in order).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the elements cannot be queried.
    pub async fn to_contain_texts(&self, expected: &[&str]) -> Result<(), AssertionError> {
        TextAssertions::new(self.locator, self.timeout, self.is_negated)
            .to_contain_texts(expected)
            .await
    }

    /// Assert that the element has all specified classes.
    ///
    /// Unlike `to_have_class()` which checks for a single class, this method
    /// verifies that the element has ALL specified classes.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_classes(&self, expected_classes: &[&str]) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let class_attr = self.get_attribute("class").await?;
            let actual_classes: std::collections::HashSet<&str> = class_attr
                .as_deref()
                .unwrap_or("")
                .split_whitespace()
                .collect();

            let has_all = expected_classes.iter().all(|c| actual_classes.contains(c));
            let expected_match = !self.is_negated;

            if has_all == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element should not have classes {expected_classes:?}")
                    } else {
                        format!("Element should have classes {expected_classes:?}")
                    },
                    format!("{expected_classes:?}"),
                    format!("{:?}", actual_classes.into_iter().collect::<Vec<_>>()),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    // =========================================================================
    // Internal helpers (delegated to locator_helpers module)
    // =========================================================================

    async fn get_input_value(&self) -> Result<String, AssertionError> {
        super::locator_helpers::get_input_value(self.locator).await
    }

    async fn get_selected_values(&self) -> Result<Vec<String>, AssertionError> {
        super::locator_helpers::get_selected_values(self.locator).await
    }

    async fn get_attribute(&self, name: &str) -> Result<Option<String>, AssertionError> {
        super::locator_helpers::get_attribute(self.locator, name).await
    }
}
