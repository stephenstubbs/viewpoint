//! Assertion API for browser automation tests.
//!
//! The `expect` function creates assertion builders for locators and pages,
//! enabling fluent async assertions.
//!
//! # Example
//!
//! ```ignore
//! use rustright_test::expect;
//!
//! // Assert element is visible
//! expect(&locator).to_be_visible().await?;
//!
//! // Assert text content
//! expect(&locator).to_have_text("Hello").await?;
//!
//! // Assert page URL
//! expect(&page).to_have_url("https://example.com").await?;
//! ```

mod locator;
mod page;

pub use locator::LocatorAssertions;
pub use page::PageAssertions;

use rustright_core::{Locator, Page};

/// Create assertions for a locator.
///
/// # Example
///
/// ```ignore
/// use rustright_test::expect;
///
/// expect(&locator).to_be_visible().await?;
/// expect(&locator).to_have_text("Hello").await?;
/// ```
pub fn expect<'a>(locator: &'a Locator<'a>) -> LocatorAssertions<'a> {
    LocatorAssertions::new(locator)
}

/// Create assertions for a page.
///
/// # Example
///
/// ```ignore
/// use rustright_test::expect_page;
///
/// expect_page(&page).to_have_url("https://example.com").await?;
/// expect_page(&page).to_have_title("Example").await?;
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
