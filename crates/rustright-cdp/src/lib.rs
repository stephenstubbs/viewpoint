//! Low-level Chrome `DevTools` Protocol implementation over WebSocket.
//!
//! This crate provides the foundational CDP transport layer for `RustRight`,
//! including WebSocket connection management, message serialization, and
//! CDP domain types.
//!
//! # Example
//!
//! ```no_run
//! use rustright_cdp::{CdpConnection, protocol::target::GetTargetsParams};
//!
//! # async fn example() -> Result<(), rustright_cdp::CdpError> {
//! let conn = CdpConnection::connect("ws://localhost:9222/devtools/browser/...").await?;
//!
//! let result: rustright_cdp::protocol::target::GetTargetsResult =
//!     conn.send_command("Target.getTargets", Some(GetTargetsParams::default()), None).await?;
//!
//! for target in result.target_infos {
//!     println!("Target: {} - {}", target.target_type, target.url);
//! }
//! # Ok(())
//! # }
//! ```

pub mod connection;
pub mod error;
pub mod protocol;
pub mod transport;

pub use connection::CdpConnection;
pub use error::CdpError;
pub use transport::{CdpEvent, CdpMessage, CdpRequest, CdpResponse};
