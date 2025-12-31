//! ARIA accessibility snapshot functionality.
//!
//! This module provides the ability to capture and compare ARIA accessibility
//! snapshots for accessibility testing.

mod matching;
mod serialization;

use std::fmt;

use serde::{Deserialize, Serialize};

/// An ARIA accessibility snapshot of an element or subtree.
///
/// The snapshot represents the accessible structure as it would be
/// exposed to assistive technologies.
///
/// # Node References
///
/// Each element in the snapshot has a unique `node_ref` identifier (format: `e{backendNodeId}`)
/// that can be used to interact with the element:
///
/// - `node_ref`: Unique reference for each element (e.g., `e12345`)
///
/// Use `Page::element_from_ref()` or `Page::locator_from_ref()` to interact with
/// elements discovered in the snapshot.
///
/// # Frame Boundary Support
///
/// For MCP (Model Context Protocol) servers and multi-frame accessibility testing,
/// this struct supports frame boundaries:
///
/// - `is_frame`: Indicates this node represents an iframe/frame boundary
/// - `frame_url`: The src URL of the iframe
/// - `frame_name`: The name attribute of the iframe
/// - `iframe_refs`: Collected at root level, lists all iframe ref IDs for enumeration
///
/// Frame boundaries are rendered in YAML as `[frame-boundary]` markers.
///
/// # Example with Node References
///
/// ```no_run
/// use viewpoint_core::Page;
///
/// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
/// // Capture snapshot with refs
/// let snapshot = page.aria_snapshot().await?;
///
/// // Each element has a unique ref
/// if let Some(ref node_ref) = snapshot.node_ref {
///     // Use the ref to interact with the element
///     let locator = page.locator_from_ref(node_ref);
///     locator.click().await?;
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Example with Frame Boundaries
///
/// ```
/// use viewpoint_core::AriaSnapshot;
///
/// let mut snapshot = AriaSnapshot::with_role("iframe");
/// snapshot.is_frame = Some(true);
/// snapshot.frame_url = Some("https://example.com/widget".to_string());
/// snapshot.frame_name = Some("payment-frame".to_string());
///
/// let yaml = snapshot.to_yaml();
/// assert!(yaml.contains("[frame-boundary]"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct AriaSnapshot {
    /// The ARIA role of the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The accessible name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The accessible description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the element is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Whether the element is expanded (for expandable elements).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    /// Whether the element is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// Whether the element is checked (for checkboxes/radios).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<AriaCheckedState>,
    /// Whether the element is pressed (for toggle buttons).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
    /// The level (for headings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    /// The value (for sliders, progress bars, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_now: Option<f64>,
    /// The minimum value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_min: Option<f64>,
    /// The maximum value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_max: Option<f64>,
    /// The value text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_text: Option<String>,
    /// Whether this node represents a frame boundary (iframe/frame element).
    ///
    /// When true, this node marks an iframe that may contain content from
    /// a separate frame context. Use `frame_url` and `frame_name` to identify
    /// the frame for separate content retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frame: Option<bool>,
    /// The URL of the iframe (from src attribute).
    ///
    /// Only present when `is_frame` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_url: Option<String>,
    /// The name attribute of the iframe.
    ///
    /// Only present when `is_frame` is true. Can be used to identify
    /// the frame for navigation or content retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_name: Option<String>,
    /// Collected iframe reference IDs at the root level.
    ///
    /// This field is only populated at the root of a snapshot tree.
    /// It contains unique identifiers for all iframes found during traversal,
    /// enabling MCP servers to enumerate frames for separate content retrieval.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub iframe_refs: Vec<String>,
    /// Unique reference identifier for this element.
    ///
    /// The ref is used to identify and interact with elements discovered in the
    /// accessibility snapshot. It follows the format `e{backendNodeId}` where
    /// `backendNodeId` is the CDP backend node identifier.
    ///
    /// # Example
    ///
    /// ```
    /// use viewpoint_core::AriaSnapshot;
    ///
    /// let mut snapshot = AriaSnapshot::with_role("button");
    /// snapshot.node_ref = Some("e12345".to_string());
    ///
    /// let yaml = snapshot.to_yaml();
    /// assert!(yaml.contains("[ref=e12345]"));
    /// ```
    ///
    /// Refs can be used with `Page::element_from_ref()` or `Page::locator_from_ref()`
    /// to interact with the element.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub node_ref: Option<String>,
    /// Temporary element index used during snapshot capture.
    ///
    /// This field is used internally to map snapshot nodes to their corresponding
    /// DOM elements during the ref resolution process. It is not serialized to YAML
    /// and should not be used by external code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) element_index: Option<usize>,
    /// Child elements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<AriaSnapshot>,
}

/// ARIA checked state (supports tri-state checkboxes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AriaCheckedState {
    /// Not checked.
    False,
    /// Checked.
    True,
    /// Mixed (indeterminate).
    Mixed,
}

impl AriaSnapshot {
    /// Create a new empty ARIA snapshot.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an ARIA snapshot with a role.
    pub fn with_role(role: impl Into<String>) -> Self {
        Self {
            role: Some(role.into()),
            ..Self::default()
        }
    }

    /// Set the accessible name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add a child element.
    #[must_use]
    pub fn child(mut self, child: AriaSnapshot) -> Self {
        self.children.push(child);
        self
    }
}

impl fmt::Display for AriaSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_yaml())
    }
}

// Re-export the JavaScript code from the separate module
pub use super::aria_js::{aria_snapshot_js, aria_snapshot_with_refs_js};

#[cfg(test)]
mod tests;
