//! State assertions for locators.
//!
//! This module contains assertions for checking element state
//! such as visibility, enabled/disabled status, and checked state.

use std::time::Duration;

use viewpoint_core::Locator;

use crate::error::AssertionError;

/// State assertion methods for locators.
///
/// These methods check the state of elements (visible, enabled, checked, etc.)
pub struct StateAssertions<'a> {
    locator: &'a Locator<'a>,
    timeout: Duration,
    is_negated: bool,
}

impl<'a> StateAssertions<'a> {
    /// Create a new `StateAssertions`.
    pub fn new(locator: &'a Locator<'a>, timeout: Duration, is_negated: bool) -> Self {
        Self {
            locator,
            timeout,
            is_negated,
        }
    }

    /// Assert that the element is visible.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_visible(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_visible = self.locator.is_visible().await.map_err(|e| {
                AssertionError::new("Failed to check visibility", "visible", e.to_string())
            })?;

            let expected = !self.is_negated;
            if is_visible == expected {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be visible"
                    } else {
                        "Element should be visible"
                    },
                    if expected { "visible" } else { "hidden" },
                    if is_visible { "visible" } else { "hidden" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is hidden.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_hidden(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_visible = self.locator.is_visible().await.map_err(|e| {
                AssertionError::new("Failed to check visibility", "hidden", e.to_string())
            })?;

            let expected_hidden = !self.is_negated;
            let is_hidden = !is_visible;

            if is_hidden == expected_hidden {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be hidden"
                    } else {
                        "Element should be hidden"
                    },
                    if expected_hidden { "hidden" } else { "visible" },
                    if is_hidden { "hidden" } else { "visible" },
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
        let start = std::time::Instant::now();

        loop {
            let is_enabled = super::locator_helpers::is_enabled(self.locator).await?;
            let expected_enabled = !self.is_negated;

            if is_enabled == expected_enabled {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be enabled"
                    } else {
                        "Element should be enabled"
                    },
                    if expected_enabled {
                        "enabled"
                    } else {
                        "disabled"
                    },
                    if is_enabled { "enabled" } else { "disabled" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is disabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_disabled(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_enabled = super::locator_helpers::is_enabled(self.locator).await?;
            let expected_disabled = !self.is_negated;
            let is_disabled = !is_enabled;

            if is_disabled == expected_disabled {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be disabled"
                    } else {
                        "Element should be disabled"
                    },
                    if expected_disabled {
                        "disabled"
                    } else {
                        "enabled"
                    },
                    if is_disabled { "disabled" } else { "enabled" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is checked (for checkboxes/radios).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_checked(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_checked = self.locator.is_checked().await.map_err(|e| {
                AssertionError::new("Failed to check checked state", "checked", e.to_string())
            })?;

            let expected_checked = !self.is_negated;

            if is_checked == expected_checked {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be checked"
                    } else {
                        "Element should be checked"
                    },
                    if expected_checked {
                        "checked"
                    } else {
                        "unchecked"
                    },
                    if is_checked { "checked" } else { "unchecked" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
