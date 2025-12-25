//! Core domain types for `RustRight` browser automation.
//!
//! This crate provides the high-level API for browser automation,
//! including Browser, `BrowserContext`, Page, and navigation types.

pub mod browser;
pub mod context;
pub mod error;
pub mod page;
pub mod wait;

pub use browser::{Browser, BrowserBuilder};
pub use context::BrowserContext;
pub use error::CoreError;
pub use page::Page;
pub use wait::DocumentLoadState;
