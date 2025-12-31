//! ARIA accessibility snapshot functionality.
//!
//! This module provides the ability to capture and compare ARIA accessibility
//! snapshots for accessibility testing.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::LocatorError;

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

    /// Convert to a YAML-like string for comparison.
    ///
    /// This format is similar to Playwright's aria snapshot format.
    pub fn to_yaml(&self) -> String {
        let mut output = String::new();
        self.write_yaml(&mut output, 0);
        output
    }

    fn write_yaml(&self, output: &mut String, indent: usize) {
        let prefix = "  ".repeat(indent);

        // Write role and name on the same line
        if let Some(ref role) = self.role {
            output.push_str(&prefix);
            output.push_str("- ");
            output.push_str(role);

            if let Some(ref name) = self.name {
                output.push_str(" \"");
                output.push_str(&name.replace('"', "\\\""));
                output.push('"');
            }

            // Add relevant attributes
            if let Some(disabled) = self.disabled {
                if disabled {
                    output.push_str(" [disabled]");
                }
            }
            if let Some(ref checked) = self.checked {
                match checked {
                    AriaCheckedState::True => output.push_str(" [checked]"),
                    AriaCheckedState::Mixed => output.push_str(" [mixed]"),
                    AriaCheckedState::False => {}
                }
            }
            if let Some(selected) = self.selected {
                if selected {
                    output.push_str(" [selected]");
                }
            }
            if let Some(expanded) = self.expanded {
                if expanded {
                    output.push_str(" [expanded]");
                }
            }
            if let Some(level) = self.level {
                output.push_str(&format!(" [level={level}]"));
            }

            // Add frame boundary marker
            if self.is_frame == Some(true) {
                output.push_str(" [frame-boundary]");
                // Include frame URL if available
                if let Some(ref url) = self.frame_url {
                    output.push_str(&format!(" [frame-url=\"{}\"]", url.replace('"', "\\\"")));
                }
                // Include frame name if available
                if let Some(ref name) = self.frame_name {
                    if !name.is_empty() {
                        output.push_str(&format!(" [frame-name=\"{}\"]", name.replace('"', "\\\"")));
                    }
                }
            }

            // Add node reference if present
            if let Some(ref node_ref) = self.node_ref {
                output.push_str(&format!(" [ref={}]", node_ref));
            }

            output.push('\n');

            // Write children
            for child in &self.children {
                child.write_yaml(output, indent + 1);
            }
        }
    }

    /// Parse from YAML-like string.
    ///
    /// This supports a simplified YAML-like format for snapshot comparison.
    pub fn from_yaml(yaml: &str) -> Result<Self, LocatorError> {
        let mut root = AriaSnapshot::new();
        root.role = Some("root".to_string());

        let mut stack: Vec<(usize, AriaSnapshot)> = vec![(0, root)];

        for line in yaml.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Calculate indent
            let indent = line.chars().take_while(|c| *c == ' ').count() / 2;
            let trimmed = line.trim();

            if !trimmed.starts_with('-') {
                continue;
            }

            let content = trimmed[1..].trim();

            // Parse role and name
            let (role, name, attrs) = parse_aria_line(content)?;

            let mut node = AriaSnapshot::with_role(role);
            if let Some(n) = name {
                node.name = Some(n);
            }

            // Apply attributes
            for attr in attrs {
                match attr.as_str() {
                    "disabled" => node.disabled = Some(true),
                    "checked" => node.checked = Some(AriaCheckedState::True),
                    "mixed" => node.checked = Some(AriaCheckedState::Mixed),
                    "selected" => node.selected = Some(true),
                    "expanded" => node.expanded = Some(true),
                    "frame-boundary" => node.is_frame = Some(true),
                    s if s.starts_with("level=") => {
                        if let Ok(level) = s[6..].parse() {
                            node.level = Some(level);
                        }
                    }
                    s if s.starts_with("frame-url=\"") && s.ends_with('"') => {
                        // Parse frame-url="value"
                        let url = &s[11..s.len() - 1];
                        node.frame_url = Some(url.replace("\\\"", "\""));
                    }
                    s if s.starts_with("frame-name=\"") && s.ends_with('"') => {
                        // Parse frame-name="value"
                        let name = &s[12..s.len() - 1];
                        node.frame_name = Some(name.replace("\\\"", "\""));
                    }
                    s if s.starts_with("ref=") => {
                        // Parse ref=e12345
                        node.node_ref = Some(s[4..].to_string());
                    }
                    _ => {}
                }
            }

            // Find parent and add as child
            while stack.len() > 1 && stack.last().is_some_and(|(i, _)| *i >= indent) {
                let (_, child) = stack.pop().unwrap();
                if let Some((_, parent)) = stack.last_mut() {
                    parent.children.push(child);
                }
            }

            stack.push((indent, node));
        }

        // Pop remaining items
        while stack.len() > 1 {
            let (_, child) = stack.pop().unwrap();
            if let Some((_, parent)) = stack.last_mut() {
                parent.children.push(child);
            }
        }

        Ok(stack.pop().map(|(_, s)| s).unwrap_or_default())
    }

    /// Check if this snapshot matches another, allowing regex patterns.
    ///
    /// The `expected` snapshot can contain regex patterns in name fields
    /// when enclosed in `/pattern/` syntax.
    pub fn matches(&self, expected: &AriaSnapshot) -> bool {
        // Check role
        if expected.role.is_some() && self.role != expected.role {
            return false;
        }

        // Check name (supports regex)
        if let Some(ref expected_name) = expected.name {
            match &self.name {
                Some(actual_name) => {
                    if !matches_name(expected_name, actual_name) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        // Check other attributes
        if expected.disabled.is_some() && self.disabled != expected.disabled {
            return false;
        }
        if expected.checked.is_some() && self.checked != expected.checked {
            return false;
        }
        if expected.selected.is_some() && self.selected != expected.selected {
            return false;
        }
        if expected.expanded.is_some() && self.expanded != expected.expanded {
            return false;
        }
        if expected.level.is_some() && self.level != expected.level {
            return false;
        }

        // Check children (order matters)
        if expected.children.len() > self.children.len() {
            return false;
        }

        for (i, expected_child) in expected.children.iter().enumerate() {
            if !self
                .children
                .get(i)
                .is_some_and(|c| c.matches(expected_child))
            {
                return false;
            }
        }

        true
    }

    /// Generate a diff between this snapshot and expected.
    pub fn diff(&self, expected: &AriaSnapshot) -> String {
        let actual_yaml = self.to_yaml();
        let expected_yaml = expected.to_yaml();

        if actual_yaml == expected_yaml {
            return String::new();
        }

        let mut diff = String::new();
        diff.push_str("Expected:\n");
        for line in expected_yaml.lines() {
            diff.push_str("  ");
            diff.push_str(line);
            diff.push('\n');
        }
        diff.push_str("\nActual:\n");
        for line in actual_yaml.lines() {
            diff.push_str("  ");
            diff.push_str(line);
            diff.push('\n');
        }

        diff
    }
}

impl fmt::Display for AriaSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_yaml())
    }
}

/// Parse an aria line into role, optional name, and attributes.
fn parse_aria_line(content: &str) -> Result<(String, Option<String>, Vec<String>), LocatorError> {
    let mut parts = content.splitn(2, ' ');
    let role = parts.next().unwrap_or("").to_string();

    if role.is_empty() {
        return Err(LocatorError::EvaluationError(
            "Empty role in aria snapshot".to_string(),
        ));
    }

    let rest = parts.next().unwrap_or("");
    let mut name = None;
    let mut attrs = Vec::new();

    // Parse name (quoted string)
    if let Some(start) = rest.find('"') {
        if let Some(end) = rest[start + 1..].find('"') {
            name = Some(rest[start + 1..start + 1 + end].replace("\\\"", "\""));
        }
    }

    // Parse attributes [attr] or [attr=value]
    for part in rest.split('[') {
        if let Some(end) = part.find(']') {
            attrs.push(part[..end].to_string());
        }
    }

    Ok((role, name, attrs))
}

/// Check if a name matches (supports regex patterns).
fn matches_name(pattern: &str, actual: &str) -> bool {
    // Check for regex pattern /.../ or /.../i
    if pattern.starts_with('/') {
        let flags_end = pattern.rfind('/');
        if let Some(end) = flags_end {
            if end > 0 {
                let regex_str = &pattern[1..end];
                let flags = &pattern[end + 1..];
                let case_insensitive = flags.contains('i');

                let regex_result = if case_insensitive {
                    regex::RegexBuilder::new(regex_str)
                        .case_insensitive(true)
                        .build()
                } else {
                    regex::Regex::new(regex_str)
                };

                if let Ok(re) = regex_result {
                    return re.is_match(actual);
                }
            }
        }
    }

    // Exact match
    pattern == actual
}

// Re-export the JavaScript code from the separate module
pub use super::aria_js::{aria_snapshot_js, aria_snapshot_with_refs_js};

#[cfg(test)]
mod tests;
