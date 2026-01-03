//! Frame locator for interacting with iframe content.
//!
//! `FrameLocator` provides a way to locate and interact with elements inside
//! iframes without needing to directly access Frame objects.

// Allow dead code for frame locator methods (spec: frames)

use std::time::Duration;

use super::locator::{AriaRole, LocatorOptions, Selector};
use crate::Page;
use viewpoint_js::js;
use viewpoint_js_core::escape_js_string_single;

/// Default timeout for frame locator operations.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A locator for finding and interacting with iframe content.
///
/// `FrameLocator` represents a view into an iframe on the page. It provides
/// methods to locate elements within the iframe using the same patterns
/// as page-level locators.
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
/// # page.goto("about:blank").goto().await.unwrap();
///
/// // Locate elements inside an iframe
/// page.frame_locator("#my-iframe")
///     .locator("button")
///     .click()
///     .await.ok();
///
/// // Use semantic locators inside frames
/// page.frame_locator("#payment-frame")
///     .get_by_role(AriaRole::Button)
///     .with_name("Submit")
///     .build()
///     .click()
///     .await.ok();
///
/// // Nested frames
/// page.frame_locator("#outer")
///     .frame_locator("#inner")
///     .locator("input")
///     .fill("text")
///     .await.ok();
/// # });
/// ```
#[derive(Debug, Clone)]
pub struct FrameLocator<'a> {
    /// Reference to the page.
    page: &'a Page,
    /// Selector for the iframe element.
    frame_selector: String,
    /// Parent frame locators (for nested frames).
    parent_selectors: Vec<String>,
    /// Timeout for operations.
    timeout: Duration,
}

impl<'a> FrameLocator<'a> {
    /// Create a new frame locator.
    pub(crate) fn new(page: &'a Page, selector: impl Into<String>) -> Self {
        Self {
            page,
            frame_selector: selector.into(),
            parent_selectors: Vec::new(),
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Create a nested frame locator with parent context.
    fn with_parent(
        page: &'a Page,
        frame_selector: String,
        mut parent_selectors: Vec<String>,
        parent_selector: String,
    ) -> Self {
        parent_selectors.push(parent_selector);
        Self {
            page,
            frame_selector,
            parent_selectors,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Set a custom timeout for this frame locator.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Get the page this frame locator belongs to.
    pub fn page(&self) -> &'a Page {
        self.page
    }

    /// Create a locator for elements within this frame.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// let button = page.frame_locator("#iframe").locator("button.submit");
    /// button.click().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn locator(&self, selector: impl Into<String>) -> FrameElementLocator<'a> {
        FrameElementLocator::new(self.clone(), Selector::Css(selector.into()))
    }

    /// Create a locator for elements containing the specified text within this frame.
    pub fn get_by_text(&self, text: impl Into<String>) -> FrameElementLocator<'a> {
        FrameElementLocator::new(
            self.clone(),
            Selector::Text {
                text: text.into(),
                exact: false,
            },
        )
    }

    /// Create a locator for elements with exact text content within this frame.
    pub fn get_by_text_exact(&self, text: impl Into<String>) -> FrameElementLocator<'a> {
        FrameElementLocator::new(
            self.clone(),
            Selector::Text {
                text: text.into(),
                exact: true,
            },
        )
    }

    /// Create a locator for elements with the specified ARIA role within this frame.
    pub fn get_by_role(&self, role: AriaRole) -> FrameRoleLocatorBuilder<'a> {
        FrameRoleLocatorBuilder::new(self.clone(), role)
    }

    /// Create a locator for elements with the specified test ID within this frame.
    pub fn get_by_test_id(&self, test_id: impl Into<String>) -> FrameElementLocator<'a> {
        FrameElementLocator::new(self.clone(), Selector::TestId(test_id.into()))
    }

    /// Create a locator for form controls by their associated label text within this frame.
    pub fn get_by_label(&self, label: impl Into<String>) -> FrameElementLocator<'a> {
        FrameElementLocator::new(self.clone(), Selector::Label(label.into()))
    }

    /// Create a locator for inputs by their placeholder text within this frame.
    pub fn get_by_placeholder(&self, placeholder: impl Into<String>) -> FrameElementLocator<'a> {
        FrameElementLocator::new(self.clone(), Selector::Placeholder(placeholder.into()))
    }

    /// Create a frame locator for a nested iframe within this frame.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Access element in nested frame
    /// page.frame_locator("#outer-frame")
    ///     .frame_locator("#inner-frame")
    ///     .locator("button")
    ///     .click()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn frame_locator(&self, selector: impl Into<String>) -> FrameLocator<'a> {
        FrameLocator::with_parent(
            self.page,
            selector.into(),
            self.parent_selectors.clone(),
            self.frame_selector.clone(),
        )
    }

    /// Get the frame selector.
    pub fn selector(&self) -> &str {
        &self.frame_selector
    }

    /// Get the parent selectors (for nested frames).
    pub fn parent_selectors(&self) -> &[String] {
        &self.parent_selectors
    }

    /// Build the JavaScript expression to access the frame's content document.
    ///
    /// Note: This function builds JavaScript dynamically at runtime because it processes
    /// a variable number of parent frame selectors. It uses `viewpoint_js_core` escaping
    /// utilities to ensure proper string escaping.
    pub(crate) fn to_js_frame_access(&self) -> String {
        let mut js = String::new();

        // Start from the top-level document
        js.push_str("(function() {\n");
        js.push_str("  let doc = document;\n");

        // Navigate through parent frames
        for parent_selector in &self.parent_selectors {
            let escaped_selector = escape_js_string_single(parent_selector);
            js.push_str("  const parent = doc.querySelector(");
            js.push_str(&escaped_selector);
            js.push_str(");\n");
            js.push_str("  if (!parent || !parent.contentDocument) return null;\n");
            js.push_str("  doc = parent.contentDocument;\n");
        }

        // Access the final frame
        let escaped_frame_selector = escape_js_string_single(&self.frame_selector);
        js.push_str("  const frame = doc.querySelector(");
        js.push_str(&escaped_frame_selector);
        js.push_str(");\n");
        js.push_str("  if (!frame || !frame.contentDocument) return null;\n");
        js.push_str("  return frame.contentDocument;\n");
        js.push_str("})()");
        js
    }
}

/// A locator for elements within a frame.
///
/// This combines a `FrameLocator` with an element `Selector` to locate
/// elements inside an iframe.
#[derive(Debug, Clone)]
pub struct FrameElementLocator<'a> {
    /// The frame locator.
    frame_locator: FrameLocator<'a>,
    /// The element selector within the frame.
    selector: Selector,
    /// Locator options.
    options: LocatorOptions,
}

impl<'a> FrameElementLocator<'a> {
    /// Create a new frame element locator.
    fn new(frame_locator: FrameLocator<'a>, selector: Selector) -> Self {
        Self {
            frame_locator,
            selector,
            options: LocatorOptions::default(),
        }
    }

    /// Set a custom timeout for this locator.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.options.timeout = timeout;
        self
    }

    /// Create a child locator that further filters elements.
    #[must_use]
    pub fn locator(&self, selector: impl Into<String>) -> FrameElementLocator<'a> {
        FrameElementLocator {
            frame_locator: self.frame_locator.clone(),
            selector: Selector::Chained(
                Box::new(self.selector.clone()),
                Box::new(Selector::Css(selector.into())),
            ),
            options: self.options.clone(),
        }
    }

    /// Select the first matching element.
    #[must_use]
    pub fn first(&self) -> FrameElementLocator<'a> {
        FrameElementLocator {
            frame_locator: self.frame_locator.clone(),
            selector: Selector::Nth {
                base: Box::new(self.selector.clone()),
                index: 0,
            },
            options: self.options.clone(),
        }
    }

    /// Select the last matching element.
    #[must_use]
    pub fn last(&self) -> FrameElementLocator<'a> {
        FrameElementLocator {
            frame_locator: self.frame_locator.clone(),
            selector: Selector::Nth {
                base: Box::new(self.selector.clone()),
                index: -1,
            },
            options: self.options.clone(),
        }
    }

    /// Select the nth matching element (0-indexed).
    #[must_use]
    pub fn nth(&self, index: i32) -> FrameElementLocator<'a> {
        FrameElementLocator {
            frame_locator: self.frame_locator.clone(),
            selector: Selector::Nth {
                base: Box::new(self.selector.clone()),
                index,
            },
            options: self.options.clone(),
        }
    }

    /// Get the frame locator.
    pub fn frame_locator(&self) -> &FrameLocator<'a> {
        &self.frame_locator
    }

    /// Get the selector.
    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    /// Get the locator options.
    pub(crate) fn options(&self) -> &LocatorOptions {
        &self.options
    }

    /// Build the JavaScript expression to query elements within the frame.
    fn to_js_expression(&self) -> String {
        let frame_access = self.frame_locator.to_js_frame_access();
        let element_selector = self.selector.to_js_expression();

        js! {
            (function() {
                const frameDoc = @{frame_access};
                if (!frameDoc) return { found: false, count: 0, error: "Frame not found or not accessible" };

                // Override document for the selector expression
                const originalDocument = document;
                try {
                    // Create a modified expression that uses frameDoc instead of document
                    const elements = (function() {
                        const document = frameDoc;
                        return Array.from(@{element_selector});
                    })();
                    return elements;
                } catch (e) {
                    return [];
                }
            })()
        }
    }
}

/// Builder for role-based frame locators.
#[derive(Debug)]
pub struct FrameRoleLocatorBuilder<'a> {
    frame_locator: FrameLocator<'a>,
    role: AriaRole,
    name: Option<String>,
}

impl<'a> FrameRoleLocatorBuilder<'a> {
    fn new(frame_locator: FrameLocator<'a>, role: AriaRole) -> Self {
        Self {
            frame_locator,
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
    pub fn build(self) -> FrameElementLocator<'a> {
        FrameElementLocator::new(
            self.frame_locator,
            Selector::Role {
                role: self.role,
                name: self.name,
            },
        )
    }
}

impl<'a> From<FrameRoleLocatorBuilder<'a>> for FrameElementLocator<'a> {
    fn from(builder: FrameRoleLocatorBuilder<'a>) -> Self {
        builder.build()
    }
}
