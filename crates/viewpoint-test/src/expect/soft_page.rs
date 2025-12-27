//! Soft page assertion methods.
//!
//! This module contains all the assertion methods for `SoftPageAssertions`.

use std::sync::{Arc, Mutex};

use super::page::PageAssertions;
use super::soft::SoftAssertionError;

/// Soft assertions for pages.
///
/// These assertions collect failures instead of failing immediately.
pub struct SoftPageAssertions<'a> {
    pub(super) assertions: PageAssertions<'a>,
    pub(super) errors: Arc<Mutex<Vec<SoftAssertionError>>>,
}

impl SoftPageAssertions<'_> {
    /// Assert page URL (soft).
    pub async fn to_have_url(&self, expected: impl AsRef<str>) {
        let expected_str = expected.as_ref().to_string();
        match self.assertions.to_have_url(&expected_str).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_url", e.to_string())
                        .with_expected(&expected_str)
                );
            }
        }
    }

    /// Assert page title (soft).
    pub async fn to_have_title(&self, expected: impl AsRef<str>) {
        let expected_str = expected.as_ref().to_string();
        match self.assertions.to_have_title(&expected_str).await {
            Ok(()) => {}
            Err(e) => {
                self.errors.lock().unwrap().push(
                    SoftAssertionError::new("to_have_title", e.to_string())
                        .with_expected(&expected_str)
                );
            }
        }
    }
}
