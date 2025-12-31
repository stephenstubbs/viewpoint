//! Tracing implementation for recording test execution traces.
//!
//! Traces capture screenshots, DOM snapshots, and network activity
//! for debugging test failures. Traces are compatible with Playwright's
//! Trace Viewer.

// Allow dead code for tracing scaffolding (spec: tracing)

mod action_handle;
mod capture;
mod network;
mod sources;
mod tracing_manager;
mod types;
mod writer;

// Re-export public types
pub use action_handle::ActionHandle;
pub use tracing_manager::Tracing;
pub use types::{ActionEntry, TracingOptions};

// Internal re-exports
pub(crate) use types::TracingState;
