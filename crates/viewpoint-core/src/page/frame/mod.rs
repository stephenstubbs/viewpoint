//! Frame management and navigation.
//!
//! Frames represent separate browsing contexts within a page, typically
//! created by `<iframe>` elements. Each frame has its own DOM, JavaScript
//! context, and URL.

mod aria;
mod content;
mod core;
mod execution_context;
mod navigation;
mod tree;

pub(crate) use execution_context::ExecutionContextRegistry;
pub use core::Frame;
