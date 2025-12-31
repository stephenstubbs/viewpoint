//! Locator system for element selection.
//!
//! Locators are lazy handles that store selection criteria but don't query the DOM
//! until an action is performed. This enables auto-waiting and chainable refinement.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "integration")]
//! # tokio_test::block_on(async {
//! # use viewpoint_core::Browser;
//! use viewpoint_core::AriaRole;
//! # let browser = Browser::launch().headless(true).launch().await.unwrap();
//! # let context = browser.new_context().await.unwrap();
//! # let page = context.new_page().await.unwrap();
//! # page.goto("about:blank").goto().await.unwrap();
//!
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
//! # });
//! ```

mod actions;
pub mod aria;
pub(crate) mod aria_js;
mod aria_role;
mod builders;
mod debug;
mod element;
mod evaluation;
mod files;
mod filter;
mod helpers;
mod queries;
mod select;
pub(crate) mod selector;

use std::time::Duration;

pub use aria::{AriaCheckedState, AriaSnapshot};
pub use builders::{
    CheckBuilder, ClickBuilder, DblclickBuilder, FillBuilder, HoverBuilder, PressBuilder,
    SelectOptionBuilder, TapBuilder, TypeBuilder,
};
pub use element::{BoundingBox, BoxModel, ElementHandle};
pub use filter::{FilterBuilder, RoleLocatorBuilder};
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
    pub(crate) fn with_options(
        page: &'a Page,
        selector: Selector,
        options: LocatorOptions,
    ) -> Self {
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
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let items = page.locator(".list").locator(".item");
    /// # }
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
    pub(crate) fn to_js_selector(&self) -> String {
        self.selector.to_js_expression()
    }

    /// Create a locator that matches elements that match both this locator and `other`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Page, AriaRole};
    ///
    /// # fn example(page: &Page) {
    /// // Find visible buttons with specific text
    /// let button = page.get_by_role(AriaRole::Button).build()
    ///     .and(page.get_by_text("Submit"));
    /// # }
    /// ```
    #[must_use]
    pub fn and(&self, other: Locator<'a>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::And(Box::new(self.selector.clone()), Box::new(other.selector)),
            options: self.options.clone(),
        }
    }

    /// Create a locator that matches elements that match either this locator or `other`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Page, AriaRole};
    ///
    /// # fn example(page: &Page) {
    /// // Find buttons or links
    /// let clickable = page.get_by_role(AriaRole::Button).build()
    ///     .or(page.get_by_role(AriaRole::Link).build());
    /// # }
    /// ```
    #[must_use]
    pub fn or(&self, other: Locator<'a>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::Or(Box::new(self.selector.clone()), Box::new(other.selector)),
            options: self.options.clone(),
        }
    }

    /// Create a filter builder to narrow down the elements matched by this locator.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// // Filter list items by text
    /// let item = page.locator("li").filter().has_text("Product");
    ///
    /// // Filter by having a child element
    /// let rows = page.locator("tr").filter().has(page.locator(".active"));
    /// # }
    /// ```
    pub fn filter(&self) -> FilterBuilder<'a> {
        FilterBuilder::new(self.page, self.selector.clone(), self.options.clone())
    }

    /// Get an ARIA accessibility snapshot of this element.
    ///
    /// The snapshot captures the accessible tree structure as it would be
    /// exposed to assistive technologies. This is useful for accessibility testing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.locator("form").aria_snapshot().await?;
    /// println!("{}", snapshot); // YAML-like output
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the element is not found or snapshot capture fails.
    pub async fn aria_snapshot(&self) -> Result<AriaSnapshot, crate::error::LocatorError> {
        use crate::error::LocatorError;
        use viewpoint_js::js;

        if self.page.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        // Get the element and evaluate ARIA snapshot
        // Note: to_js_expression() returns code that evaluates to NodeList/array,
        // so we need to get the first element from it
        let js_selector = self.selector.to_js_expression();
        let snapshot_fn = aria::aria_snapshot_js();
        let js_code = js! {
            (function() {
                const elements = @{js_selector};
                const element = elements && elements[0] ? elements[0] : elements;
                if (!element) {
                    return { error: "Element not found" };
                }
                const getSnapshot = @{snapshot_fn};
                return getSnapshot(element);
            })()
        };

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .page
            .connection()
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js_code,
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(self.page.session_id()),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(LocatorError::EvaluationError(exception.text));
        }

        let value = result.result.value.ok_or_else(|| {
            LocatorError::EvaluationError("No result from aria snapshot".to_string())
        })?;

        // Check for error
        if let Some(error) = value.get("error").and_then(|e| e.as_str()) {
            return Err(LocatorError::NotFound(error.to_string()));
        }

        // Parse the snapshot
        let snapshot: AriaSnapshot = serde_json::from_value(value).map_err(|e| {
            LocatorError::EvaluationError(format!("Failed to parse aria snapshot: {e}"))
        })?;

        Ok(snapshot)
    }
}

// FilterBuilder and RoleLocatorBuilder are in filter.rs
