//! Locator system for element selection.
//!
//! Locators are lazy handles that store selection criteria but don't query the DOM
//! until an action is performed. This enables auto-waiting and chainable refinement.
//!
//! # Example
//!
//! ```ignore
//! // CSS selector
//! let button = page.locator("button.submit");
//!
//! // Text locator
//! let heading = page.get_by_text("Welcome");
//!
//! // Role locator
//! let submit = page.get_by_role(AriaRole::Button).with_name("Submit");
//!
//! // Chained locators
//! let item = page.locator(".list").locator(".item").first();
//! ```

mod actions;
mod selector;

use std::time::Duration;

pub use selector::{AriaRole, Selector, TextOptions};

use crate::Page;

/// Default timeout for locator operations.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A locator for finding elements on a page.
///
/// Locators are lightweight handles that store selection criteria. They don't
/// query the DOM until an action is performed, enabling auto-waiting.
#[derive(Debug, Clone)]
pub struct Locator<'a> {
    /// Reference to the page.
    page: &'a Page,
    /// The selector for finding elements.
    selector: Selector,
    /// Locator options.
    options: LocatorOptions,
}

/// Options for locator behavior.
#[derive(Debug, Clone)]
pub struct LocatorOptions {
    /// Timeout for operations.
    pub timeout: Duration,
}

impl Default for LocatorOptions {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl<'a> Locator<'a> {
    /// Create a new locator.
    pub(crate) fn new(page: &'a Page, selector: Selector) -> Self {
        Self {
            page,
            selector,
            options: LocatorOptions::default(),
        }
    }

    /// Create a new locator with custom options.
    #[allow(dead_code)] // Available for future use
    pub(crate) fn with_options(page: &'a Page, selector: Selector, options: LocatorOptions) -> Self {
        Self {
            page,
            selector,
            options,
        }
    }

    /// Get the page this locator belongs to.
    pub fn page(&self) -> &'a Page {
        self.page
    }

    /// Get the selector.
    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    /// Get the options.
    pub fn options(&self) -> &LocatorOptions {
        &self.options
    }

    /// Set a custom timeout for this locator.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.options.timeout = timeout;
        self
    }

    /// Create a child locator that further filters elements.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let items = page.locator(".list").locator(".item");
    /// ```
    #[must_use]
    pub fn locator(&self, selector: impl Into<String>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::Chained(
                Box::new(self.selector.clone()),
                Box::new(Selector::Css(selector.into())),
            ),
            options: self.options.clone(),
        }
    }

    /// Select the first matching element.
    #[must_use]
    pub fn first(&self) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::Nth {
                base: Box::new(self.selector.clone()),
                index: 0,
            },
            options: self.options.clone(),
        }
    }

    /// Select the last matching element.
    #[must_use]
    pub fn last(&self) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::Nth {
                base: Box::new(self.selector.clone()),
                index: -1,
            },
            options: self.options.clone(),
        }
    }

    /// Select the nth matching element (0-indexed).
    #[must_use]
    pub fn nth(&self, index: i32) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::Nth {
                base: Box::new(self.selector.clone()),
                index,
            },
            options: self.options.clone(),
        }
    }

    /// Convert the selector to a JavaScript expression for CDP evaluation.
    #[allow(dead_code)] // Available for future use
    pub(crate) fn to_js_selector(&self) -> String {
        self.selector.to_js_expression()
    }
}
