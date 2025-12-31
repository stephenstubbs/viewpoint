//! Wait system for Playwright-compatible load states and auto-waiting.

mod load_state;
mod navigation_waiter;
mod waiter;

pub use load_state::DocumentLoadState;
pub use navigation_waiter::NavigationWaiter;
pub use waiter::{LoadStateWaiter, NavigationResponseData};
