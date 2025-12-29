//! Locator factory methods for Page.
//!
//! This module contains all the `get_by_*` and `locator` methods that create
//! locator instances for finding elements on the page.

use crate::page::locator::{AriaRole, Locator, RoleLocatorBuilder, Selector};
use crate::page::{Page, DEFAULT_TEST_ID_ATTRIBUTE};

impl Page {
    /// Create a locator for elements matching a CSS selector.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let button = page.locator("button.submit");
    /// let items = page.locator(".list > .item");
    /// # }
    /// ```
    pub fn locator(&self, selector: impl Into<String>) -> Locator<'_> {
        Locator::new(self, Selector::Css(selector.into()))
    }

    /// Create a locator for elements containing the specified text.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let heading = page.get_by_text("Welcome");
    /// let exact = page.get_by_text_exact("Welcome to our site");
    /// # }
    /// ```
    pub fn get_by_text(&self, text: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::Text {
                text: text.into(),
                exact: false,
            },
        )
    }

    /// Create a locator for elements with exact text content.
    pub fn get_by_text_exact(&self, text: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::Text {
                text: text.into(),
                exact: true,
            },
        )
    }

    /// Create a locator for elements with the specified ARIA role.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use viewpoint_core::page::locator::AriaRole;
    ///
    /// # fn example(page: &Page) {
    /// let buttons = page.get_by_role(AriaRole::Button);
    /// let submit = page.get_by_role(AriaRole::Button).with_name("Submit");
    /// # }
    /// ```
    pub fn get_by_role(&self, role: AriaRole) -> RoleLocatorBuilder<'_> {
        RoleLocatorBuilder::new(self, role)
    }

    /// Create a locator for elements with the specified test ID.
    ///
    /// By default, looks for `data-testid` attribute. Use
    /// `BrowserContext::set_test_id_attribute()` to customize which attribute
    /// is used.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let button = page.get_by_test_id("submit-button");
    /// # }
    /// ```
    pub fn get_by_test_id(&self, test_id: impl Into<String>) -> Locator<'_> {
        let id = test_id.into();
        if self.test_id_attribute == DEFAULT_TEST_ID_ATTRIBUTE {
            Locator::new(self, Selector::TestId(id))
        } else {
            Locator::new(
                self,
                Selector::TestIdCustom {
                    id,
                    attribute: self.test_id_attribute.clone(),
                },
            )
        }
    }

    /// Get the test ID attribute used by this page.
    pub fn test_id_attribute(&self) -> &str {
        &self.test_id_attribute
    }

    /// Set the test ID attribute for this page.
    ///
    /// This only affects this page. For context-wide configuration,
    /// use `BrowserContext::set_test_id_attribute()`.
    pub fn set_test_id_attribute(&mut self, attribute: impl Into<String>) {
        self.test_id_attribute = attribute.into();
    }

    /// Create a locator for form controls by their associated label text.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let email = page.get_by_label("Email address");
    /// # }
    /// ```
    pub fn get_by_label(&self, label: impl Into<String>) -> Locator<'_> {
        Locator::new(self, Selector::Label(label.into()))
    }

    /// Create a locator for inputs by their placeholder text.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let search = page.get_by_placeholder("Search...");
    /// # }
    /// ```
    pub fn get_by_placeholder(&self, placeholder: impl Into<String>) -> Locator<'_> {
        Locator::new(self, Selector::Placeholder(placeholder.into()))
    }

    /// Create a locator for images by their alt text.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let logo = page.get_by_alt_text("Company Logo");
    /// # }
    /// ```
    pub fn get_by_alt_text(&self, alt: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::AltText {
                text: alt.into(),
                exact: false,
            },
        )
    }

    /// Create a locator for images with exact alt text.
    pub fn get_by_alt_text_exact(&self, alt: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::AltText {
                text: alt.into(),
                exact: true,
            },
        )
    }

    /// Create a locator for elements by their title attribute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # fn example(page: &Page) {
    /// let tooltip = page.get_by_title("Click to expand");
    /// # }
    /// ```
    pub fn get_by_title(&self, title: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::Title {
                text: title.into(),
                exact: false,
            },
        )
    }

    /// Create a locator for elements with exact title attribute.
    pub fn get_by_title_exact(&self, title: impl Into<String>) -> Locator<'_> {
        Locator::new(
            self,
            Selector::Title {
                text: title.into(),
                exact: true,
            },
        )
    }
}
