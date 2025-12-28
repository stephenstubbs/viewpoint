//! CDP protocol domain types.

pub mod browser;
pub mod dom;
pub mod dom_snapshot;
pub mod emulation;
pub mod fetch;
pub mod input;
pub mod network;
mod network_cookies;
mod network_websocket;
pub mod page;
mod page_dialog;
mod page_download;
mod page_screencast;
pub mod runtime;
pub mod storage;
pub mod target_domain;
pub mod tracing;

// Re-export cookie and websocket types from network module
pub use network_cookies::*;
pub use network_websocket::*;

// Re-export page dialog, download, and screencast types
pub use page_dialog::*;
pub use page_download::*;
pub use page_screencast::*;
