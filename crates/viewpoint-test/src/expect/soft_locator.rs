//! Soft locator assertion methods.
//!
//! This module contains all the assertion methods for `SoftLocatorAssertions`.

use std::sync::{Arc, Mutex};

use viewpoint_core::AriaSnapshot;

use super::locator::LocatorAssertions;
use super::soft::SoftAssertionError;

/// Soft assertions for locators.
///
/// These assertions collect failures instead of failing immediately.
pub struct SoftLocatorAssertions<'a> {
    pub(super) assertions: LocatorAssertions<'a>,
    pub(super) errors: Arc<Mutex<Vec<SoftAssertionError>>>,
}

/// Helper macro to reduce boilerplate in soft assertion methods.
macro_rules! soft_assert {
    ($self:expr, $method:ident, $assertion_name:expr) => {
        match $self.assertions.$method().await {
            Ok(()) => {}
            Err(e) => {
                $self.errors.lock().unwrap().push(
                    SoftAssertionError::new($assertion_name, e.to_string())
                );
            }
        }
    };
    ($self:expr, $method:ident, $assertion_name:expr, expected: $expected:expr) => {
        match $self.assertions.$method(&$expected).await {
            Ok(()) => {}
            Err(e) => {
                $self.errors.lock().unwrap().push(
                    SoftAssertionError::new($assertion_name, e.to_string())
                        .with_expected($expected.to_string())
                );
            }
        }
    };
}

impl SoftLocatorAssertions<'_> {
    /// Assert element is visible (soft).
    pub async fn to_be_visible(&self) {
        soft_assert!(self, to_be_visible, "to_be_visible");
    }

    /// Assert element is hidden (soft).
    pub async fn to_be_hidden(&self) {
        soft_assert!(self, to_be_hidden, "to_be_hidden");
    }

    /// Assert element is enabled (soft).
    pub async fn to_be_enabled(&self) {
        soft_assert!(self, to_be_enabled, "to_be_enabled");
    }

    /// Assert element is disabled (soft).
    pub async fn to_be_disabled(&self) {
        soft_assert!(self, to_be_disabled, "to_be_disabled");
    }

    /// Assert element is checked (soft).
    pub async fn to_be_checked(&self) {
        soft_assert!(self, to_be_checked, "to_be_checked");
    }

    /// Assert element has text (soft).
    pub async fn to_have_text(&self, expected: impl AsRef<str>) {
        let expected_str = expected.as_ref().to_string();
        soft_assert!(self, to_have_text, "to_have_text", expected: expected_str);
    }

    /// Assert element contains text (soft).
    pub async fn to_contain_text(&self, expected: impl AsRef<str>) {
        let expected_str = expected.as_ref().to_string();
        soft_assert!(self, to_contain_text, "to_contain_text", expected: expected_str);
    }

    /// Assert element has value (soft).
    pub async fn to_have_value(&self, expected: impl AsRef<str>) {
        let expected_str = expected.as_ref().to_string();
        soft_assert!(self, to_have_value, "to_have_value", expected: expected_str);
    }

    /// Assert element has attribute (soft).
    pub async fn to_have_attribute(&self, name: impl AsRef<str>, value: impl AsRef<str>) {
        let name_str = name.as_ref().to_string();
        let value_str = value.as_ref().to_string();
        match self.assertions.to_have_attribute(&name_str, &value_str).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new(
                        format!("to_have_attribute({name_str})"),
                        e.to_string(),
                    )
                    .with_expected(&value_str)
                );
            }
        }
    }

    /// Assert element has class (soft).
    pub async fn to_have_class(&self, class_name: impl AsRef<str>) {
        let class_str = class_name.as_ref().to_string();
        soft_assert!(self, to_have_class, "to_have_class", expected: class_str);
    }

    /// Assert element has id (soft).
    pub async fn to_have_id(&self, expected: impl AsRef<str>) {
        let expected_str = expected.as_ref().to_string();
        soft_assert!(self, to_have_id, "to_have_id", expected: expected_str);
    }

    /// Assert element count (soft).
    pub async fn to_have_count(&self, expected: usize) {
        match self.assertions.to_have_count(expected).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_count", e.to_string())
                        .with_expected(expected.to_string())
                );
            }
        }
    }

    /// Assert element count is greater than a value (soft).
    pub async fn to_have_count_greater_than(&self, n: usize) {
        match self.assertions.to_have_count_greater_than(n).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_count_greater_than", e.to_string())
                        .with_expected(format!("> {n}"))
                );
            }
        }
    }

    /// Assert element count is less than a value (soft).
    pub async fn to_have_count_less_than(&self, n: usize) {
        match self.assertions.to_have_count_less_than(n).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_count_less_than", e.to_string())
                        .with_expected(format!("< {n}"))
                );
            }
        }
    }

    /// Assert element count is at least a value (soft).
    pub async fn to_have_count_at_least(&self, n: usize) {
        match self.assertions.to_have_count_at_least(n).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_count_at_least", e.to_string())
                        .with_expected(format!(">= {n}"))
                );
            }
        }
    }

    /// Assert element count is at most a value (soft).
    pub async fn to_have_count_at_most(&self, n: usize) {
        match self.assertions.to_have_count_at_most(n).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_count_at_most", e.to_string())
                        .with_expected(format!("<= {n}"))
                );
            }
        }
    }

    /// Assert element's ARIA snapshot matches expected structure (soft).
    pub async fn to_match_aria_snapshot(&self, expected: &AriaSnapshot) {
        match self.assertions.to_match_aria_snapshot(expected).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_match_aria_snapshot", e.to_string())
                        .with_expected(expected.to_yaml())
                );
            }
        }
    }

    /// Assert element's ARIA snapshot matches expected YAML string (soft).
    pub async fn to_match_aria_snapshot_yaml(&self, expected_yaml: &str) {
        match self.assertions.to_match_aria_snapshot_yaml(expected_yaml).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_match_aria_snapshot_yaml", e.to_string())
                        .with_expected(expected_yaml.to_string())
                );
            }
        }
    }

    /// Assert elements have texts (soft).
    pub async fn to_have_texts(&self, expected: &[&str]) {
        match self.assertions.to_have_texts(expected).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_texts", e.to_string())
                        .with_expected(format!("{expected:?}"))
                );
            }
        }
    }

    /// Assert elements contain texts (soft).
    pub async fn to_contain_texts(&self, expected: &[&str]) {
        match self.assertions.to_contain_texts(expected).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_contain_texts", e.to_string())
                        .with_expected(format!("{expected:?}"))
                );
            }
        }
    }

    /// Assert element has all specified classes (soft).
    pub async fn to_have_classes(&self, expected_classes: &[&str]) {
        match self.assertions.to_have_classes(expected_classes).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_classes", e.to_string())
                        .with_expected(format!("{expected_classes:?}"))
                );
            }
        }
    }

    /// Assert multi-select element has specified values (soft).
    pub async fn to_have_values(&self, expected: &[&str]) {
        match self.assertions.to_have_values(expected).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_values", e.to_string())
                        .with_expected(format!("{expected:?}"))
                );
            }
        }
    }
}
