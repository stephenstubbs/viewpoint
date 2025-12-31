//! Builder types for locator actions.
//!
//! These builders provide a fluent API for configuring locator operations
//! like click, type, hover, and tap with various options.

mod check;
mod click;
mod dblclick;
mod fill;
mod hover;
mod press;
mod select_option;
mod tap;
mod type_builder;

pub use check::CheckBuilder;
pub use click::ClickBuilder;
pub use dblclick::DblclickBuilder;
pub use fill::FillBuilder;
pub use hover::HoverBuilder;
pub use press::PressBuilder;
pub use select_option::SelectOptionBuilder;
pub use tap::TapBuilder;
pub use type_builder::TypeBuilder;
