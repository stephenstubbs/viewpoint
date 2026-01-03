//! Page accessor methods.
//!
//! This module contains getter methods for accessing page properties.

use std::sync::Arc;
use viewpoint_cdp::CdpConnection;

use super::Page;

impl Page {
    /// Get the target ID.
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Get the session ID.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the main frame ID.
    pub fn frame_id(&self) -> &str {
        &self.frame_id
    }

    /// Get the context index.
    ///
    /// This is used for generating scoped element refs in the format
    /// `c{contextIndex}p{pageIndex}e{counter}`.
    pub fn context_index(&self) -> usize {
        self.context_index
    }

    /// Get the page index.
    ///
    /// This is used for generating scoped element refs in the format
    /// `c{contextIndex}p{pageIndex}e{counter}`.
    pub fn index(&self) -> usize {
        self.page_index
    }

    /// Get a reference to the CDP connection.
    pub fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }
}
