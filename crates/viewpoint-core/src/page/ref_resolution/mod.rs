//! Node reference resolution for ARIA snapshots.
//!
//! This module provides functionality to resolve element references from
//! ARIA snapshots back to DOM elements for interaction.
//!
//! # Reference Format
//!
//! Element references follow the format `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}` where:
//!
//! - `c{contextIndex}` - Which browser context this ref belongs to
//! - `p{pageIndex}` - Which page/tab within the context this ref belongs to
//! - `f{frameIndex}` - Which frame within the page (0 = main frame, 1+ = child frames)
//! - `e{counter}` - Simple incrementing counter per snapshot
//!
//! For example: `c0p0f0e1`, `c0p0f0e2`, `c0p0f1e1`, `c1p0f0e1`
//!
//! This format:
//! - Prevents ref collisions across contexts, pages, and frames
//! - Is short and readable
//! - Enables validation of refs against the correct context and page
//!
//! # MCP Server Usage
//!
//! This feature is designed for MCP (Model Context Protocol) servers that need to:
//!
//! 1. Present an accessibility tree to AI/users
//! 2. Allow interaction with any element in that tree
//!
//! Without refs, users would need to re-query elements by role/name, which is fragile
//! when multiple elements share the same accessible properties.
//!
//! # Example: Click a Button by Ref
//!
//! ```no_run
//! use viewpoint_core::Page;
//!
//! # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
//! // Capture snapshot with refs
//! let snapshot = page.aria_snapshot().await?;
//!
//! // Find a button's ref in the snapshot
//! if let Some(ref node_ref) = snapshot.node_ref.as_ref() {
//!     // Resolve ref to element handle (for low-level operations)
//!     let handle = page.element_from_ref(node_ref).await?;
//!
//!     // Or get a locator for auto-waiting behavior (preferred)
//!     let locator = page.locator_from_ref(node_ref);
//!     locator.click().await?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Example: Find and Interact with Snapshot Elements
//!
//! ```no_run
//! use viewpoint_core::{Page, AriaSnapshot};
//!
//! # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
//! // Capture snapshot
//! let snapshot = page.aria_snapshot().await?;
//!
//! // Helper to find a button by name
//! fn find_button_ref(snapshot: &AriaSnapshot, name: &str) -> Option<String> {
//!     if snapshot.role.as_deref() == Some("button")
//!         && snapshot.name.as_deref() == Some(name)
//!     {
//!         return snapshot.node_ref.clone();
//!     }
//!     for child in &snapshot.children {
//!         if let Some(r) = find_button_ref(child, name) {
//!             return Some(r);
//!         }
//!     }
//!     None
//! }
//!
//! // Find "Submit" button and click it
//! if let Some(submit_ref) = find_button_ref(&snapshot, "Submit") {
//!     page.locator_from_ref(&submit_ref).click().await?;
//! }
//! # Ok(())
//! # }
//! ```

use tracing::{debug, instrument};
use viewpoint_cdp::protocol::dom::{
    BackendNodeId, DescribeNodeParams, DescribeNodeResult, ResolveNodeParams, ResolveNodeResult,
};

use super::Page;
use super::locator::ElementHandle;
use crate::error::{LocatorError, PageError};

/// Parsed element reference with context, page, frame, and element indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedRef {
    /// Context index.
    pub context_index: usize,
    /// Page index within the context.
    pub page_index: usize,
    /// Frame index within the page (0 = main frame, 1+ = child frames).
    pub frame_index: usize,
    /// Element counter within the snapshot.
    pub element_counter: usize,
}

impl ParsedRef {
    /// Create a new parsed ref.
    pub fn new(context_index: usize, page_index: usize, frame_index: usize, element_counter: usize) -> Self {
        Self {
            context_index,
            page_index,
            frame_index,
            element_counter,
        }
    }
}

/// Parse a ref string to extract context, page, frame, and element indices.
///
/// Refs are formatted as `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}`, e.g., `c0p0f0e1`.
///
/// # Errors
///
/// Returns `LocatorError::EvaluationError` if the ref format is invalid.
pub fn parse_ref(ref_str: &str) -> Result<ParsedRef, LocatorError> {
    if !ref_str.starts_with('c') {
        return Err(LocatorError::EvaluationError(format!(
            "Invalid ref format: expected 'c{{ctx}}p{{page}}f{{frame}}e{{counter}}', got '{ref_str}'"
        )));
    }

    parse_ref_format(ref_str)
}

/// Parse the ref format: c{contextIndex}p{pageIndex}f{frameIndex}e{counter}
fn parse_ref_format(ref_str: &str) -> Result<ParsedRef, LocatorError> {
    // Format: c0p0f0e1
    let without_c = ref_str.strip_prefix('c').ok_or_else(|| {
        LocatorError::EvaluationError(format!("Invalid ref format: expected 'c' prefix in '{ref_str}'"))
    })?;

    let (context_part, rest) = without_c.split_once('p').ok_or_else(|| {
        LocatorError::EvaluationError(format!("Invalid ref format: expected 'p' separator in '{ref_str}'"))
    })?;

    let (page_part, rest) = rest.split_once('f').ok_or_else(|| {
        LocatorError::EvaluationError(format!("Invalid ref format: expected 'f' separator in '{ref_str}'"))
    })?;

    let (frame_part, element_part) = rest.split_once('e').ok_or_else(|| {
        LocatorError::EvaluationError(format!("Invalid ref format: expected 'e' separator in '{ref_str}'"))
    })?;

    let context_index = context_part.parse::<usize>().map_err(|e| {
        LocatorError::EvaluationError(format!("Invalid context index in ref '{ref_str}': {e}"))
    })?;

    let page_index = page_part.parse::<usize>().map_err(|e| {
        LocatorError::EvaluationError(format!("Invalid page index in ref '{ref_str}': {e}"))
    })?;

    let frame_index = frame_part.parse::<usize>().map_err(|e| {
        LocatorError::EvaluationError(format!("Invalid frame index in ref '{ref_str}': {e}"))
    })?;

    let element_counter = element_part.parse::<usize>().map_err(|e| {
        LocatorError::EvaluationError(format!("Invalid element counter in ref '{ref_str}': {e}"))
    })?;

    Ok(ParsedRef::new(context_index, page_index, frame_index, element_counter))
}

/// Format a ref string from context, page, frame, and element indices.
///
/// Produces the format `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}`.
pub fn format_ref(context_index: usize, page_index: usize, frame_index: usize, element_counter: usize) -> String {
    format!("c{context_index}p{page_index}f{frame_index}e{element_counter}")
}

impl Page {
    /// Get an element handle from a snapshot ref.
    ///
    /// This resolves the ref (format: `c{contextIndex}p{pageIndex}f{frameIndex}e{counter}`)
    /// to an `ElementHandle` that can be used for low-level DOM operations.
    ///
    /// # Arguments
    ///
    /// * `ref_str` - The element ref from an ARIA snapshot (e.g., `c0p0f0e1`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.aria_snapshot().await?;
    /// // Assume we found a button with ref "c0p0f0e1"
    /// let handle = page.element_from_ref("c0p0f0e1").await?;
    /// let text: String = handle.evaluate("this.textContent").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - The ref format is invalid
    /// - The ref is from a different context or page
    /// - The element no longer exists (stale ref)
    #[instrument(level = "debug", skip(self), fields(target_id = %self.target_id, ref_str = %ref_str))]
    pub async fn element_from_ref(&self, ref_str: &str) -> Result<ElementHandle<'_>, LocatorError> {
        if self.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        let parsed = parse_ref(ref_str)?;
        
        // Validate context index
        if parsed.context_index != self.context_index {
            return Err(LocatorError::EvaluationError(format!(
                "Context index mismatch: ref '{ref_str}' is for context {}, but this page is in context {}",
                parsed.context_index, self.context_index
            )));
        }
        
        // Validate page index
        if parsed.page_index != self.page_index {
            return Err(LocatorError::EvaluationError(format!(
                "Page index mismatch: ref '{ref_str}' is for page {}, but this is page {}",
                parsed.page_index, self.page_index
            )));
        }

        debug!(
            context_index = parsed.context_index,
            page_index = parsed.page_index,
            frame_index = parsed.frame_index,
            element_counter = parsed.element_counter,
            "Resolving ref to element"
        );

        // Look up the backendNodeId from the ref map
        let backend_node_id = self.get_backend_node_id_for_ref(ref_str)?;

        // Use DOM.resolveNode to get a RemoteObject from the backend node ID
        let result: ResolveNodeResult = self
            .connection()
            .send_command(
                "DOM.resolveNode",
                Some(ResolveNodeParams {
                    node_id: None,
                    backend_node_id: Some(backend_node_id),
                    object_group: Some("viewpoint-ref".to_string()),
                    execution_context_id: None,
                }),
                Some(self.session_id()),
            )
            .await
            .map_err(|e| {
                LocatorError::NotFound(format!(
                    "Ref not found. Capture a new snapshot. Error: {e}"
                ))
            })?;

        let object_id = result.object.object_id.ok_or_else(|| {
            LocatorError::NotFound("Ref not found. Capture a new snapshot.".to_string())
        })?;

        debug!(object_id = %object_id, "Resolved ref to element handle");

        Ok(ElementHandle {
            object_id,
            page: self,
        })
    }

    /// Create a locator from a snapshot ref.
    ///
    /// This creates a `Locator` that targets the element identified by the ref.
    /// Unlike `element_from_ref`, the locator provides auto-waiting behavior
    /// and is the preferred way to interact with elements.
    ///
    /// # Arguments
    ///
    /// * `ref_str` - The element ref from an ARIA snapshot (e.g., `c0p0f0e1`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.aria_snapshot().await?;
    /// // Assume we found a button with ref "c0p0f0e1"
    /// let locator = page.locator_from_ref("c0p0f0e1");
    /// locator.click().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the ref format is invalid. Use `element_from_ref` if you need
    /// to handle invalid refs gracefully.
    pub fn locator_from_ref(&self, ref_str: &str) -> super::Locator<'_> {
        use super::locator::{Locator, Selector};

        // Parse the ref to validate format and indices
        let parsed = parse_ref(ref_str)
            .expect("Invalid ref format. Refs must be in format 'c{ctx}p{page}f{frame}e{counter}'");
        
        // Validate indices match this page
        assert!(
            parsed.context_index == self.context_index,
            "Context index mismatch: ref is for context {}, but this page is in context {}",
            parsed.context_index,
            self.context_index
        );
        
        assert!(
            parsed.page_index == self.page_index,
            "Page index mismatch: ref is for page {}, but this is page {}",
            parsed.page_index,
            self.page_index
        );

        // Create a locator with a ref selector that will lookup from the ref map
        Locator::new(self, Selector::Ref(ref_str.to_string()))
    }

    /// Get the backend node ID for an element from its object ID.
    ///
    /// This is a lower-level method used internally during snapshot capture
    /// to resolve element references.
    pub(crate) async fn get_backend_node_id(
        &self,
        object_id: &str,
    ) -> Result<BackendNodeId, PageError> {
        let result: DescribeNodeResult = self
            .connection()
            .send_command(
                "DOM.describeNode",
                Some(DescribeNodeParams {
                    node_id: None,
                    backend_node_id: None,
                    object_id: Some(object_id.to_string()),
                    depth: Some(0),
                    pierce: None,
                }),
                Some(self.session_id()),
            )
            .await?;

        Ok(result.node.backend_node_id)
    }

    /// Get the backend node ID for a ref from the ref map.
    ///
    /// This is used by `element_from_ref` and `locator_from_ref` to lookup
    /// the backendNodeId for a ref captured during `aria_snapshot()`.
    ///
    /// # Errors
    ///
    /// Returns an error if the ref is not found in the ref map.
    pub(crate) fn get_backend_node_id_for_ref(
        &self,
        ref_str: &str,
    ) -> Result<BackendNodeId, LocatorError> {
        self.ref_map
            .read()
            .get(ref_str)
            .copied()
            .ok_or_else(|| {
                LocatorError::NotFound(
                    "Ref not found. Capture a new snapshot.".to_string()
                )
            })
    }

    /// Store a ref mapping in the page's ref map.
    ///
    /// This is called during `aria_snapshot()` to populate the ref map
    /// with the element refs and their corresponding backendNodeIds.
    pub(crate) fn store_ref_mapping(&self, ref_str: String, backend_node_id: BackendNodeId) {
        self.ref_map.write().insert(ref_str, backend_node_id);
    }

    /// Clear all ref mappings.
    ///
    /// This is called at the beginning of `aria_snapshot()` to clear
    /// stale refs from a previous snapshot.
    pub(crate) fn clear_ref_map(&self) {
        self.ref_map.write().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ref_new_format() {
        let parsed = parse_ref("c0p0f0e1").unwrap();
        assert_eq!(parsed.context_index, 0);
        assert_eq!(parsed.page_index, 0);
        assert_eq!(parsed.frame_index, 0);
        assert_eq!(parsed.element_counter, 1);
    }

    #[test]
    fn test_parse_ref_new_format_larger_indices() {
        let parsed = parse_ref("c12p34f56e789").unwrap();
        assert_eq!(parsed.context_index, 12);
        assert_eq!(parsed.page_index, 34);
        assert_eq!(parsed.frame_index, 56);
        assert_eq!(parsed.element_counter, 789);
    }

    #[test]
    fn test_parse_ref_child_frame() {
        let parsed = parse_ref("c0p0f1e5").unwrap();
        assert_eq!(parsed.context_index, 0);
        assert_eq!(parsed.page_index, 0);
        assert_eq!(parsed.frame_index, 1);
        assert_eq!(parsed.element_counter, 5);
    }

    #[test]
    fn test_parse_ref_invalid_format() {
        assert!(parse_ref("invalid").is_err());
        assert!(parse_ref("x0p0f0e1").is_err());
        assert!(parse_ref("c0p0e1").is_err()); // missing frame
        assert!(parse_ref("c0f0e1").is_err()); // missing page
        assert!(parse_ref("").is_err());
    }

    #[test]
    fn test_parse_ref_legacy_format_rejected() {
        // Legacy e{id} format is no longer supported
        assert!(parse_ref("e12345").is_err());
        assert!(parse_ref("e1").is_err());
    }

    #[test]
    fn test_parse_ref_invalid_numbers() {
        assert!(parse_ref("cXp0f0e1").is_err());
        assert!(parse_ref("c0pXf0e1").is_err());
        assert!(parse_ref("c0p0fXe1").is_err());
        assert!(parse_ref("c0p0f0eX").is_err());
    }

    #[test]
    fn test_format_ref() {
        assert_eq!(format_ref(0, 0, 0, 1), "c0p0f0e1");
        assert_eq!(format_ref(1, 2, 3, 4), "c1p2f3e4");
        assert_eq!(format_ref(12, 34, 56, 789), "c12p34f56e789");
    }

    #[test]
    fn test_format_and_parse_roundtrip() {
        let original = format_ref(5, 10, 2, 100);
        let parsed = parse_ref(&original).unwrap();
        assert_eq!(parsed.context_index, 5);
        assert_eq!(parsed.page_index, 10);
        assert_eq!(parsed.frame_index, 2);
        assert_eq!(parsed.element_counter, 100);
    }

    #[test]
    fn test_parsed_ref_new() {
        let parsed = ParsedRef::new(1, 2, 3, 4);
        assert_eq!(parsed.context_index, 1);
        assert_eq!(parsed.page_index, 2);
        assert_eq!(parsed.frame_index, 3);
        assert_eq!(parsed.element_counter, 4);
    }

    #[test]
    fn test_parsed_ref_equality() {
        let a = ParsedRef::new(1, 2, 3, 4);
        let b = ParsedRef::new(1, 2, 3, 4);
        let c = ParsedRef::new(1, 2, 3, 5);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
