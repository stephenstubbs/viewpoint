//! Assertion API for browser automation tests.
//!
//! The `expect` function creates assertion builders for locators and pages,
//! enabling fluent async assertions.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "integration")]
//! # tokio_test::block_on(async {
//! # use viewpoint_core::Browser;
//! use viewpoint_test::{expect, expect_page};
//! # let browser = Browser::launch().headless(true).launch().await.unwrap();
//! # let context = browser.new_context().await.unwrap();
//! # let page = context.new_page().await.unwrap();
//! # page.goto("https://example.com").goto().await.unwrap();
//!
//! // Assert element is visible
//! let locator = page.locator("h1");
//! expect(&locator).to_be_visible().await.unwrap();
//!
//! // Assert text content
//! expect(&locator).to_have_text("Example Domain").await.unwrap();
//!
//! // Assert page URL
//! expect_page(&page).to_have_url("https://example.com/").await.unwrap();
//! # });
//! ```

mod count;
mod locator;
mod locator_helpers;
mod page;
mod soft;
mod soft_locator;
mod soft_page;
mod state;
mod text;

#[cfg(test)]
mod tests;

pub use locator::LocatorAssertions;
pub use page::PageAssertions;
pub use soft::{SoftAssertionError, SoftAssertions};
pub use soft_locator::SoftLocatorAssertions;
pub use soft_page::SoftPageAssertions;

use viewpoint_core::{Locator, Page};

/// Create assertions for a locator.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// use viewpoint_test::expect;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
/// # page.goto("https://example.com").goto().await.unwrap();
///
/// let locator = page.locator("h1");
/// expect(&locator).to_be_visible().await.unwrap();
/// expect(&locator).to_have_text("Example Domain").await.unwrap();
/// # });
/// ```
pub fn expect<'a>(locator: &'a Locator<'a>) -> LocatorAssertions<'a> {
    LocatorAssertions::new(locator)
}

/// Create assertions for a page.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// use viewpoint_test::expect_page;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
/// # page.goto("https://example.com").goto().await.unwrap();
///
/// expect_page(&page).to_have_url("https://example.com/").await.unwrap();
/// expect_page(&page).to_have_title("Example Domain").await.unwrap();
/// # });
/// ```
pub fn expect_page(page: &Page) -> PageAssertions<'_> {
    PageAssertions::new(page)
}

/// Trait for creating assertions from different types.
///
/// This enables a unified `expect()` function that works with both
/// locators and pages.
pub trait Expectable<'a> {
    /// The assertion builder type for this value.
    type Assertions;

    /// Create an assertion builder for this value.
    fn assertions(&'a self) -> Self::Assertions;
}

impl<'a> Expectable<'a> for Locator<'a> {
    type Assertions = LocatorAssertions<'a>;

    fn assertions(&'a self) -> Self::Assertions {
        LocatorAssertions::new(self)
    }
}

impl<'a> Expectable<'a> for Page {
    type Assertions = PageAssertions<'a>;

    fn assertions(&'a self) -> Self::Assertions {
        PageAssertions::new(self)
    }
}
