//! Context timeout configuration.

use std::time::Duration;

use super::BrowserContext;

impl BrowserContext {
    /// Set the default timeout for actions.
    ///
    /// This timeout is used for actions like clicking, typing, etc.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Get the default timeout for actions.
    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }

    /// Set the default navigation timeout.
    ///
    /// This timeout is used for navigation operations like goto, reload, etc.
    pub fn set_default_navigation_timeout(&mut self, timeout: Duration) {
        self.default_navigation_timeout = timeout;
    }

    /// Get the default navigation timeout.
    pub fn default_navigation_timeout(&self) -> Duration {
        self.default_navigation_timeout
    }
}
