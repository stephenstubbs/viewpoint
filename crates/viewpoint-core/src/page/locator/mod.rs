//! Locator system for element selection.
//!
//! Locators are lazy handles that store selection criteria but don't query the DOM
//! until an action is performed. This enables auto-waiting and chainable refinement.
//!
//! # Basic Locator Usage
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
//!
//! # Form Filling with Multiple Locators
//!
//! Fill forms using multiple locator strategies for resilient tests:
//!
//! ```no_run
//! use viewpoint_core::{Browser, AriaRole};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Fill a complete form using different locator strategies
//! // Use label locators for form fields (most resilient)
//! page.get_by_label("First Name").fill("John").await?;
//! page.get_by_label("Last Name").fill("Doe").await?;
//! page.get_by_label("Email Address").fill("john.doe@example.com").await?;
//!
//! // Use placeholder for fields without labels
//! page.get_by_placeholder("Enter phone number").fill("+1-555-0123").await?;
//!
//! // Use role locators for dropdowns and buttons
//! page.locator("select#country")
//!     .select_option()
//!     .label("United States")
//!     .await?;
//!
//! // Use test-id for dynamic/generated elements
//! page.get_by_test_id("address-line-1").fill("123 Main St").await?;
//! page.get_by_test_id("address-line-2").fill("Apt 4B").await?;
//!
//! // Combine locators for complex forms
//! let form = page.locator("form#registration");
//! form.locator("input[name='zipcode']").fill("10001").await?;
//!
//! // Check terms checkbox using role
//! page.get_by_role(AriaRole::Checkbox)
//!     .with_name("I agree to the terms")
//!     .build()
//!     .check()
//!     .await?;
//!
//! // Submit using role locator
//! page.get_by_role(AriaRole::Button)
//!     .with_name("Create Account")
//!     .build()
//!     .click()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Accessibility Testing at Scale
//!
//! Capture and verify ARIA snapshots for accessibility testing across multiple pages:
//!
//! ```no_run
//! use viewpoint_core::{Browser, AriaRole};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Capture accessibility snapshot for the entire page
//! let snapshot = page.aria_snapshot().await?;
//! println!("Accessibility tree:\n{}", snapshot.to_yaml());
//!
//! // Capture snapshot for a specific component
//! let nav_snapshot = page.locator("nav").aria_snapshot().await?;
//!
//! // Verify required ARIA landmarks exist
//! let main_count = page.get_by_role(AriaRole::Main).build().count().await?;
//! assert!(main_count >= 1, "Page must have main landmark");
//!
//! let nav_count = page.get_by_role(AriaRole::Navigation).build().count().await?;
//! assert!(nav_count >= 1, "Page must have navigation landmark");
//!
//! // Check headings exist (use CSS selector for h1 specifically)
//! let h1_count = page.locator("h1").count().await?;
//! assert!(h1_count >= 1, "Page must have at least one h1");
//!
//! // Check buttons have accessible names
//! let buttons = page.get_by_role(AriaRole::Button).build();
//! let button_count = buttons.count().await?;
//! for i in 0..button_count {
//!     let button = buttons.nth(i as i32);
//!     // Verify button has either aria-label or visible text
//!     let text = button.text_content().await?;
//!     let label = button.get_attribute("aria-label").await?;
//!     assert!(text.is_some() || label.is_some(), "Button {} must have accessible name", i);
//! }
//!
//! // Check images have alt text
//! let images = page.get_by_role(AriaRole::Img).build();
//! let img_count = images.count().await?;
//! for i in 0..img_count {
//!     let alt = images.nth(i as i32).get_attribute("alt").await?;
//!     assert!(alt.is_some(), "Image {} must have alt text", i);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Multi-Page Accessibility Auditing
//!
//! Run accessibility checks across multiple pages:
//!
//! ```no_run
//! use viewpoint_core::{Browser, AriaRole};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! let browser = Browser::launch().headless(true).launch().await?;
//!
//! // Define pages to audit
//! let pages_to_audit = vec![
//!     "https://example.com/",
//!     "https://example.com/about",
//!     "https://example.com/contact",
//! ];
//!
//! // Audit each page (can parallelize with tokio::spawn)
//! for url in pages_to_audit {
//!     let mut context = browser.new_context().await?;
//!     let page = context.new_page().await?;
//!     page.goto(url).goto().await?;
//!
//!     // Capture full accessibility snapshot
//!     let snapshot = page.aria_snapshot().await?;
//!
//!     // Check for common accessibility issues:
//!     // 1. Missing page title
//!     let title = page.title().await?;
//!     assert!(!title.is_empty(), "{}: Missing page title", url);
//!
//!     // 2. Missing main landmark
//!     let main_count = page.get_by_role(AriaRole::Main).build().count().await?;
//!     assert!(main_count >= 1, "{}: Missing main landmark", url);
//!
//!     // 3. Check form inputs have labels
//!     let inputs = page.locator("input:not([type='hidden'])");
//!     let input_count = inputs.count().await?;
//!     for i in 0..input_count {
//!         let input = inputs.nth(i as i32);
//!         let label = input.get_attribute("aria-label").await?;
//!         let labelled_by = input.get_attribute("aria-labelledby").await?;
//!         let id = input.get_attribute("id").await?;
//!         // Verify input has some form of labelling
//!         assert!(
//!             label.is_some() || labelled_by.is_some() || id.is_some(),
//!             "{}: Input {} missing accessible label", url, i
//!         );
//!     }
//!
//!     context.close().await?;
//! }
//! # Ok(())
//! # }
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
