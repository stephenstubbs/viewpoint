//! Page domain types.
//!
//! The Page domain provides actions and events related to the inspected page.

mod events;
mod params;
mod results;
mod types;

// Re-export all types for backwards compatibility
pub use events::*;
pub use params::*;
pub use results::*;
pub use types::*;
