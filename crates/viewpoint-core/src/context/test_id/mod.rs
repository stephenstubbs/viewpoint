//! Test ID attribute configuration for BrowserContext.
//!
//! This module provides methods to configure the test ID attribute used by locators.

use super::{BrowserContext, DEFAULT_TEST_ID_ATTRIBUTE};

impl BrowserContext {
    /// Set the custom test ID attribute for this context.
    ///
    /// By default, `get_by_test_id()` uses the `data-testid` attribute.
    /// Call this method to use a different attribute name.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::BrowserContext;
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), viewpoint_core::CoreError> {
    /// // Use data-test instead of data-testid
    /// context.set_test_id_attribute("data-test").await;
    ///
    /// // Now get_by_test_id looks for data-test attribute
    /// let page = context.new_page().await?;
    /// let button = page.get_by_test_id("submit"); // looks for [data-test="submit"]
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_test_id_attribute(&self, name: impl Into<String>) {
        let mut attr = self.test_id_attribute.write().await;
        *attr = name.into();
    }

    /// Get the current test ID attribute name.
    ///
    /// Returns the attribute name used by `get_by_test_id()` (default: `data-testid`).
    pub async fn test_id_attribute(&self) -> String {
        self.test_id_attribute.read().await.clone()
    }

    /// Get the test ID attribute synchronously (for internal use).
    pub(crate) fn test_id_attribute_blocking(&self) -> String {
        // Use try_read to avoid blocking; fall back to default if lock is held
        self.test_id_attribute
            .try_read().map_or_else(|_| DEFAULT_TEST_ID_ATTRIBUTE.to_string(), |guard| guard.clone())
    }
}
