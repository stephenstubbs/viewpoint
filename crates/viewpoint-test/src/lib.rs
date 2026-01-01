//! # Viewpoint Test - Browser Testing Framework
//!
//! Test framework for `Viewpoint` browser automation, providing assertion APIs,
//! test harness setup, and convenient test macros for browser-based E2E tests.
//!
//! ## Features
//!
//! - **Test Harness**: Automatic browser, context, and page setup/teardown
//! - **Locator Assertions**: Wait-based assertions for elements (`expect(locator)`)
//! - **Page Assertions**: Assertions for page state (`expect_page(page)`)
//! - **Soft Assertions**: Collect multiple failures without stopping the test
//! - **Fixture Scoping**: Reuse browser/context across tests for performance
//! - **Test Macro**: Convenient `#[viewpoint::test]` attribute for test setup
//!
//! ## Quick Start
//!
//! The easiest way to write tests is using the `TestHarness`:
//!
//! ```no_run
//! use viewpoint_test::{TestHarness, expect};
//!
//! #[tokio::test]
//! async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
//!     let harness = TestHarness::new().await?;
//!     let page = harness.page();
//!
//!     page.goto("https://example.com").goto().await?;
//!
//!     // Assert element is visible
//!     expect(page.locator("h1")).to_be_visible().await?;
//!
//!     // Assert element has text
//!     expect(page.locator("h1")).to_have_text("Example Domain").await?;
//!
//!     Ok(()) // harness drops and cleans up automatically
//! }
//! ```
//!
//! ## Using the Test Macro
//!
//! For even more convenience, use the `#[viewpoint::test]` attribute:
//!
//! ```text
//! use viewpoint_test::test;
//! use viewpoint_core::Page;
//!
//! #[viewpoint_test::test]
//! async fn my_test(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
//!     page.goto("https://example.com").goto().await?;
//!     // page is automatically set up and cleaned up
//!     Ok(())
//! }
//! ```
//!
//! ## Locator Assertions
//!
//! The [`expect()`] function creates assertions for locators that automatically wait:
//!
//! ```ignore
//! use viewpoint_test::{TestHarness, expect};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let harness = TestHarness::new().await?;
//! # let page = harness.page();
//! // Visibility assertions
//! expect(page.locator("button")).to_be_visible().await?;
//! expect(page.locator(".hidden")).to_be_hidden().await?;
//!
//! // Text content assertions
//! expect(page.locator("h1")).to_have_text("Welcome").await?;
//! expect(page.locator("p")).to_contain_text("Hello").await?;
//!
//! // Input value assertions
//! expect(page.locator("input")).to_have_value("initial value").await?;
//! expect(page.locator("input")).to_be_empty().await?;
//!
//! // State assertions
//! expect(page.locator("button")).to_be_enabled().await?;
//! expect(page.locator("input")).to_be_disabled().await?;
//! expect(page.locator("input[type=checkbox]")).to_be_checked().await?;
//!
//! // Attribute assertions
//! expect(page.locator("a")).to_have_attribute("href", "/about").await?;
//!
//! // CSS assertions
//! expect(page.locator("div")).to_have_css("display", "flex").await?;
//!
//! // Count assertions
//! expect(page.locator("li")).to_have_count(5).await?;
//!
//! // Focus assertions
//! expect(page.locator("input")).to_be_focused().await?;
//!
//! // Negation with `.not()`
//! expect(page.locator("button")).not().to_be_disabled().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Page Assertions
//!
//! The [`expect_page`] function creates assertions for page state:
//!
//! ```ignore
//! use viewpoint_test::{TestHarness, expect_page};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let harness = TestHarness::new().await?;
//! # let page = harness.page();
//! // URL assertions
//! expect_page(page).to_have_url("https://example.com/").await?;
//! expect_page(page).to_have_url_matching(r"example\.com").await?;
//!
//! // Title assertions
//! expect_page(page).to_have_title("Example Domain").await?;
//! expect_page(page).to_have_title_matching(r"Example.*").await?;
//!
//! // Negation
//! expect_page(page).not().to_have_url("https://wrong.com/").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Soft Assertions
//!
//! Soft assertions collect failures without stopping the test, useful for checking
//! multiple conditions:
//!
//! ```ignore
//! use viewpoint_test::{TestHarness, SoftAssertions};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let harness = TestHarness::new().await?;
//! # let page = harness.page();
//! let soft = SoftAssertions::new();
//!
//! // These won't fail immediately
//! soft.expect(page.locator("h1")).to_have_text("Welcome").await;
//! soft.expect(page.locator("nav")).to_be_visible().await;
//! soft.expect(page.locator("footer")).to_be_visible().await;
//!
//! // Assert all at once - fails if any assertion failed
//! soft.assert_all()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Fixture Scoping
//!
//! The harness supports different scoping levels for performance optimization:
//!
//! ```no_run
//! use viewpoint_test::TestHarness;
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Test-scoped (default): new browser per test
//! let harness = TestHarness::new().await?;
//!
//! // Module-scoped: reuse browser, fresh context/page per test
//! # let shared_browser = Browser::launch().headless(true).launch().await?;
//! let harness = TestHarness::from_browser(&shared_browser).await?;
//!
//! // Context-scoped: reuse context, fresh page per test
//! # let context = shared_browser.new_context().await?;
//! let harness = TestHarness::from_context(&context).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Test Configuration
//!
//! Configure tests with [`TestConfig`]:
//!
//! ```no_run
//! use viewpoint_test::{TestHarness, TestConfig};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = TestConfig::builder()
//!     .headless(true)
//!     .timeout(Duration::from_secs(60))
//!     .build();
//!
//! let harness = TestHarness::with_config(config).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! Assertions return [`AssertionError`] on failure with detailed messages:
//!
//! ```ignore
//! use viewpoint_test::{TestHarness, expect, AssertionError};
//!
//! # async fn example() -> Result<(), AssertionError> {
//! # let harness = TestHarness::new().await.unwrap();
//! # let page = harness.page();
//! // This will fail with a descriptive message if the element doesn't have the text
//! expect(page.locator("h1"))
//!     .to_have_text("Expected Title")
//!     .await?;
//! # Ok(())
//! # }
//! ```

mod config;
mod error;
pub mod expect;
mod harness;

pub use config::{TestConfig, TestConfigBuilder};
pub use error::{AssertionError, TestError};
pub use expect::{
    Expectable, LocatorAssertions, PageAssertions, SoftAssertionError, SoftAssertions,
    SoftLocatorAssertions, SoftPageAssertions, expect, expect_page,
};
pub use harness::TestHarness;

// Re-export the test macro for convenience
pub use viewpoint_test_macros::test;

// Re-export core types for convenience
pub use viewpoint_core::{Browser, BrowserContext, CoreError, DocumentLoadState, Page};
