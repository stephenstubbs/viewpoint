//! Document load state types.

/// Document load states for navigation waiting.
///
/// These states are compatible with Playwright's load states and occur
/// in the following order: Commit → `DomContentLoaded` → Load → `NetworkIdle`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DocumentLoadState {
    /// Navigation response received (first byte).
    /// This is the earliest state and completes when the server starts sending a response.
    Commit,

    /// DOM fully parsed (`DOMContentLoaded` event fired).
    /// The HTML document has been completely loaded and parsed.
    DomContentLoaded,

    /// Full page load complete (load event fired).
    /// All resources including images, stylesheets, and scripts have loaded.
    /// This is the default wait state.
    #[default]
    Load,

    /// No network requests for 500ms.
    /// Useful for pages with dynamic content that loads after the initial page load.
    NetworkIdle,
}

impl DocumentLoadState {
    /// Check if this state has been reached given the current state.
    ///
    /// Returns `true` if `current` is at or past `self` in the load order.
    pub fn is_reached(&self, current: Self) -> bool {
        current >= *self
    }

    /// Get the CDP event name associated with this state.
    pub fn cdp_event_name(&self) -> Option<&'static str> {
        match self {
            // Commit is handled via network response, NetworkIdle requires custom tracking
            Self::Commit | Self::NetworkIdle => None,
            Self::DomContentLoaded => Some("Page.domContentEventFired"),
            Self::Load => Some("Page.loadEventFired"),
        }
    }
}

#[cfg(test)]
mod tests;
