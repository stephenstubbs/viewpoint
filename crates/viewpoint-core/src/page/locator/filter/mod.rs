//! Filter and role builders for locators.
//!
//! This module provides builders for filtering locators and creating role-based locators.

use crate::Page;

use super::{AriaRole, Locator, LocatorOptions, Selector};

/// Builder for filtering locators by various criteria.
///
/// Created via [`Locator::filter`].
#[derive(Debug)]
pub struct FilterBuilder<'a> {
    page: &'a Page,
    base_selector: Selector,
    options: LocatorOptions,
}

impl<'a> FilterBuilder<'a> {
    pub(super) fn new(page: &'a Page, base_selector: Selector, options: LocatorOptions) -> Self {
        Self {
            page,
            base_selector,
            options,
        }
    }

    /// Filter to elements that contain the specified text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let items = page.locator("li").filter().has_text("Product").build();
    /// ```
    #[must_use]
    pub fn has_text(self, text: impl Into<String>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::FilterText {
                base: Box::new(self.base_selector),
                text: text.into(),
                exact: false,
                has_not: false,
            },
            options: self.options,
        }
    }

    /// Filter to elements that contain the exact text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let items = page.locator("li").filter().has_text_exact("Product A").build();
    /// ```
    #[must_use]
    pub fn has_text_exact(self, text: impl Into<String>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::FilterText {
                base: Box::new(self.base_selector),
                text: text.into(),
                exact: true,
                has_not: false,
            },
            options: self.options,
        }
    }

    /// Filter to elements that do NOT contain the specified text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let items = page.locator("li").filter().has_not_text("Sold Out").build();
    /// ```
    #[must_use]
    pub fn has_not_text(self, text: impl Into<String>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::FilterText {
                base: Box::new(self.base_selector),
                text: text.into(),
                exact: false,
                has_not: true,
            },
            options: self.options,
        }
    }

    /// Filter to elements that do NOT contain the exact text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let items = page.locator("li").filter().has_not_text_exact("Out of Stock").build();
    /// ```
    #[must_use]
    pub fn has_not_text_exact(self, text: impl Into<String>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::FilterText {
                base: Box::new(self.base_selector),
                text: text.into(),
                exact: true,
                has_not: true,
            },
            options: self.options,
        }
    }

    /// Filter to elements that have a descendant matching the given locator.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let rows = page.locator("tr").filter().has(page.locator(".active")).build();
    /// ```
    #[must_use]
    pub fn has(self, child: Locator<'a>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::FilterHas {
                base: Box::new(self.base_selector),
                child: Box::new(child.selector),
                has_not: false,
            },
            options: self.options,
        }
    }

    /// Filter to elements that do NOT have a descendant matching the given locator.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let rows = page.locator("tr").filter().has_not(page.locator(".disabled")).build();
    /// ```
    #[must_use]
    pub fn has_not(self, child: Locator<'a>) -> Locator<'a> {
        Locator {
            page: self.page,
            selector: Selector::FilterHas {
                base: Box::new(self.base_selector),
                child: Box::new(child.selector),
                has_not: true,
            },
            options: self.options,
        }
    }
}

/// Builder for role-based locators.
///
/// Created via [`Page::get_by_role`].
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// use viewpoint_core::AriaRole;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
///
/// let button = page.get_by_role(AriaRole::Button)
///     .with_name("Submit")
///     .build();
/// # });
/// ```
#[derive(Debug)]
pub struct RoleLocatorBuilder<'a> {
    page: &'a Page,
    role: AriaRole,
    name: Option<String>,
}

impl<'a> RoleLocatorBuilder<'a> {
    /// Create a new role locator builder.
    pub(crate) fn new(page: &'a Page, role: AriaRole) -> Self {
        Self {
            page,
            role,
            name: None,
        }
    }

    /// Filter by accessible name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Build the locator.
    pub fn build(self) -> Locator<'a> {
        Locator::new(
            self.page,
            Selector::Role {
                role: self.role,
                name: self.name,
            },
        )
    }
}

impl<'a> From<RoleLocatorBuilder<'a>> for Locator<'a> {
    fn from(builder: RoleLocatorBuilder<'a>) -> Self {
        builder.build()
    }
}
