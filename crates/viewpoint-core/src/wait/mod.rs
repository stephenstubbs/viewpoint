//! # Wait System
//!
//! This module provides Playwright-compatible load states and auto-waiting
//! functionality for reliable browser automation.
//!
//! ## Document Load States
//!
//! The [`DocumentLoadState`] enum represents different stages of page loading:
//!
//! - [`DocumentLoadState::Commit`] - Navigation has started (response headers received)
//! - [`DocumentLoadState::DomContentLoaded`] - DOM is ready, but resources may still be loading
//! - [`DocumentLoadState::Load`] - Page is fully loaded including resources
//! - [`DocumentLoadState::NetworkIdle`] - No network activity for 500ms
//!
//! ## Usage in Navigation
//!
//! ```no_run
//! use viewpoint_core::{Browser, DocumentLoadState};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Wait for DOM to be ready (fastest)
//! page.goto("https://example.com")
//!     .wait_until(DocumentLoadState::DomContentLoaded)
//!     .goto()
//!     .await?;
//!
//! // Wait for full load (default)
//! page.goto("https://example.com")
//!     .wait_until(DocumentLoadState::Load)
//!     .goto()
//!     .await?;
//!
//! // Wait for network to be idle (slowest, most reliable for SPAs)
//! page.goto("https://example.com")
//!     .wait_until(DocumentLoadState::NetworkIdle)
//!     .goto()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Choosing the Right Load State
//!
//! | State | When to Use |
//! |-------|-------------|
//! | `Commit` | When you only need the response headers |
//! | `DomContentLoaded` | When DOM interaction is needed, but not full resources |
//! | `Load` | General use, waits for images and stylesheets |
//! | `NetworkIdle` | For SPAs or pages with async data fetching |
//!
//! ## Auto-Waiting in Locators
//!
//! The [`Locator`](crate::page::Locator) API automatically waits for elements
//! to be actionable before performing actions. This includes waiting for:
//!
//! - Element to be attached to DOM
//! - Element to be visible
//! - Element to be stable (not animating)
//! - Element to be enabled (for form elements)
//! - Element to receive events (not obscured)
//!
//! ```no_run
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // This automatically waits for the button to be clickable
//! page.locator("button").click().await?;
//!
//! // This waits for the input to be visible and enabled
//! page.locator("input").fill("text").await?;
//! # Ok(())
//! # }
//! ```

mod load_state;
mod navigation_waiter;
mod waiter;

pub use load_state::DocumentLoadState;
pub use navigation_waiter::NavigationWaiter;
pub use waiter::{LoadStateWaiter, NavigationResponseData};
