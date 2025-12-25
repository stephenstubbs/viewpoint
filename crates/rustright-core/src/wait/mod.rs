//! Wait system for Playwright-compatible load states and auto-waiting.

mod load_state;
mod waiter;

pub use load_state::DocumentLoadState;
pub use waiter::LoadStateWaiter;
