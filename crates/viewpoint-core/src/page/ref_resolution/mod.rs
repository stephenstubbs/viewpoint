//! Node reference resolution for ARIA snapshots.
//!
//! This module provides functionality to resolve element references from
//! ARIA snapshots back to DOM elements for interaction.
//!
//! # Reference Format
//!
//! Element references follow the format `e{backendNodeId}` where `backendNodeId`
//! is the CDP backend node identifier. This format:
//!
//! - Is short and readable
//! - Uses the `e` prefix to distinguish from frame refs (`frame-0`)
//! - Maps directly to CDP `backendNodeId` for efficient resolution
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

/// Parse a ref string to extract the backend node ID.
///
/// Refs are formatted as `e{backendNodeId}`, e.g., `e12345`.
///
/// # Errors
///
/// Returns `LocatorError::EvaluationError` if the ref format is invalid.
pub fn parse_ref(ref_str: &str) -> Result<BackendNodeId, LocatorError> {
    if !ref_str.starts_with('e') {
        return Err(LocatorError::EvaluationError(format!(
            "Invalid ref format: expected 'e{{backendNodeId}}', got '{ref_str}'"
        )));
    }

    ref_str[1..]
        .parse::<BackendNodeId>()
        .map_err(|e| LocatorError::EvaluationError(format!("Invalid backend node ID in ref: {e}")))
}

/// Format a backend node ID as a ref string.
pub fn format_ref(backend_node_id: BackendNodeId) -> String {
    format!("e{backend_node_id}")
}

impl Page {
    /// Get an element handle from a snapshot ref.
    ///
    /// This resolves the ref (format: `e{backendNodeId}`) to an `ElementHandle`
    /// that can be used for low-level DOM operations.
    ///
    /// # Arguments
    ///
    /// * `ref_str` - The element ref from an ARIA snapshot (e.g., `e12345`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.aria_snapshot().await?;
    /// // Assume we found a button with ref "e12345"
    /// let handle = page.element_from_ref("e12345").await?;
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
    /// - The element no longer exists (stale ref)
    #[instrument(level = "debug", skip(self), fields(target_id = %self.target_id, ref_str = %ref_str))]
    pub async fn element_from_ref(&self, ref_str: &str) -> Result<ElementHandle<'_>, LocatorError> {
        if self.is_closed() {
            return Err(LocatorError::PageClosed);
        }

        let backend_node_id = parse_ref(ref_str)?;
        debug!(
            backend_node_id = backend_node_id,
            "Resolving ref to element"
        );

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
                    "Failed to resolve ref '{ref_str}': element may no longer exist. Error: {e}"
                ))
            })?;

        let object_id = result.object.object_id.ok_or_else(|| {
            LocatorError::NotFound(format!(
                "Failed to get object ID for ref '{ref_str}': element may be detached"
            ))
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
    /// * `ref_str` - The element ref from an ARIA snapshot (e.g., `e12345`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let snapshot = page.aria_snapshot().await?;
    /// // Assume we found a button with ref "e12345"
    /// let locator = page.locator_from_ref("e12345");
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

        // Parse the ref to validate format
        let backend_node_id = parse_ref(ref_str)
            .expect("Invalid ref format. Refs must be in format 'e{backendNodeId}'");

        // Create a locator with a backend node ID selector
        Locator::new(self, Selector::BackendNodeId(backend_node_id))
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
}
