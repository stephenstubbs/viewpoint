//! Test framework for `Viewpoint` browser automation.
//!
//! This crate provides the primary testing API via `TestHarness`, along with
//! assertions and configuration for browser-based E2E tests.
//!
//! # Primary API: `TestHarness`
//!
//! The `TestHarness` provides explicit test setup with browser, context, and page access.
//! Cleanup happens automatically via `Drop`.
//!
//! ```no_run
//! use viewpoint_test::TestHarness;
//!
//! #[tokio::test]
//! async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
//!     let harness = TestHarness::new().await?;
//!     let page = harness.page();
//!
//!     page.goto("https://example.com").goto().await?;
//!
//!     Ok(()) // harness drops and cleans up
//! }
//! ```
//!
//! # Fixture Scoping
//!
//! The harness supports different scoping levels:
//!
//! - `TestHarness::new()` - Test-scoped: new browser per test (default)
//! - `TestHarness::from_browser()` - Module-scoped: reuse browser, fresh context/page
//! - `TestHarness::from_context()` - Shared context: reuse context, fresh page only

mod config;
mod error;
pub mod expect;
mod harness;

pub use config::{TestConfig, TestConfigBuilder};
pub use error::{AssertionError, TestError};
pub use expect::{
    expect, expect_page, Expectable, LocatorAssertions, PageAssertions,
    SoftAssertionError, SoftAssertions, SoftLocatorAssertions, SoftPageAssertions,
};
pub use harness::TestHarness;

// Re-export the test macro for convenience
pub use viewpoint_test_macros::test;

// Re-export core types for convenience
pub use viewpoint_core::{Browser, BrowserContext, CoreError, DocumentLoadState, Page};
