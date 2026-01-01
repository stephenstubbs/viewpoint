//! Frame ARIA accessibility snapshot operations.

use std::collections::HashMap;

use futures::stream::{FuturesUnordered, StreamExt};
use tracing::{debug, instrument, trace};
use viewpoint_cdp::protocol::dom::{BackendNodeId, DescribeNodeParams, DescribeNodeResult};
use viewpoint_cdp::protocol::runtime::EvaluateParams;
use viewpoint_js::js;

use super::Frame;
use crate::error::PageError;
use crate::page::aria_snapshot::{apply_refs_to_snapshot, SnapshotOptions};
use crate::page::locator::aria_js::aria_snapshot_with_refs_js;

impl Frame {
    /// Capture an ARIA accessibility snapshot of this frame's document.
    ///
    /// The snapshot represents the accessible structure of the frame's content
    /// as it would be exposed to assistive technologies. This is useful for
    /// accessibility testing and MCP (Model Context Protocol) integrations.
    ///
    /// # Node References
    ///
    /// The snapshot includes `node_ref` on each element (format: `e{backendNodeId}`).
    /// These refs can be used with `Page::element_from_ref()` or `Page::locator_from_ref()`
    /// to interact with elements discovered in the snapshot.
    ///
    /// # Frame Boundaries
    ///
    /// Any iframes within this frame are marked as frame boundaries in the snapshot
    /// with `is_frame: true`. Their content is NOT traversed (for security reasons).
    /// To capture multi-frame accessibility trees, use `Page::aria_snapshot_with_frames()`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The frame is detached
    /// - JavaScript evaluation fails
    /// - The snapshot cannot be parsed
    #[instrument(level = "debug", skip(self), fields(frame_id = %self.id))]
    pub async fn aria_snapshot(&self) -> Result<crate::page::locator::AriaSnapshot, PageError> {
        self.aria_snapshot_with_options(SnapshotOptions::default())
            .await
    }

    /// Capture an ARIA accessibility snapshot with custom options.
    ///
    /// See [`aria_snapshot`](Self::aria_snapshot) for details.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::SnapshotOptions;
    ///
    /// # async fn example(frame: &viewpoint_core::Frame) -> Result<(), viewpoint_core::CoreError> {
    /// // Skip ref resolution for maximum performance
    /// let options = SnapshotOptions::default().include_refs(false);
    /// let snapshot = frame.aria_snapshot_with_options(options).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self, options), fields(frame_id = %self.id))]
    pub async fn aria_snapshot_with_options(
        &self,
        options: SnapshotOptions,
    ) -> Result<crate::page::locator::AriaSnapshot, PageError> {
        if self.is_detached() {
            return Err(PageError::EvaluationFailed("Frame is detached".to_string()));
        }

        // Capture snapshot with element collection for ref resolution
        self.capture_snapshot_with_refs(options).await
    }

    /// Internal method to capture a snapshot with refs resolved.
    ///
    /// This uses a two-phase approach:
    /// 1. JS traversal collects the snapshot and element references
    /// 2. CDP calls resolve each element to its backendNodeId (in parallel)
    ///
    /// # Performance Optimizations
    ///
    /// - Uses `Runtime.getProperties` to batch-fetch all array element object IDs
    /// - Uses `FuturesUnordered` to resolve node IDs in parallel
    /// - Configurable concurrency limit to avoid overwhelming the browser
    #[instrument(level = "debug", skip(self, options), fields(frame_id = %self.id))]
    pub(super) async fn capture_snapshot_with_refs(
        &self,
        options: SnapshotOptions,
    ) -> Result<crate::page::locator::AriaSnapshot, PageError> {
        let snapshot_fn = aria_snapshot_with_refs_js();

        // Evaluate the JS function to get snapshot and element array
        // We return by value for the snapshot, but need remote objects for elements
        let js_code = js! {
            (function() {
                const getSnapshotWithRefs = @{snapshot_fn};
                return getSnapshotWithRefs(document.body);
            })()
        };

        // Get the execution context ID for this frame's main world
        let context_id = self.main_world_context_id();
        trace!(context_id = ?context_id, "Using execution context for aria_snapshot()");

        // First, evaluate to get the result as a RemoteObject (not by value)
        // so we can access the elements array
        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: js_code,
                    object_group: Some("viewpoint-snapshot".to_string()),
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id,
                    return_by_value: Some(false), // Get RemoteObject, not value
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        if let Some(exception) = result.exception_details {
            return Err(PageError::EvaluationFailed(exception.text));
        }

        let result_object_id = result.result.object_id.ok_or_else(|| {
            PageError::EvaluationFailed("No object ID from snapshot evaluation".to_string())
        })?;

        // Get the snapshot property (by value)
        let snapshot_value = self
            .get_property_value(&result_object_id, "snapshot")
            .await?;

        // Parse the snapshot
        let mut snapshot: crate::page::locator::AriaSnapshot =
            serde_json::from_value(snapshot_value).map_err(|e| {
                PageError::EvaluationFailed(format!("Failed to parse aria snapshot: {e}"))
            })?;

        // Only resolve refs if requested
        if options.get_include_refs() {
            // Get the elements array as a RemoteObject
            let elements_result = self
                .get_property_object(&result_object_id, "elements")
                .await?;

            if let Some(elements_object_id) = elements_result {
                // Batch-fetch all array element object IDs using Runtime.getProperties
                let element_object_ids = self.get_all_array_elements(&elements_object_id).await?;
                let element_count = element_object_ids.len();

                debug!(
                    element_count = element_count,
                    max_concurrency = options.get_max_concurrency(),
                    "Resolving element refs in parallel"
                );

                // Resolve all node IDs in parallel with concurrency limit
                let ref_map = self
                    .resolve_node_ids_parallel(element_object_ids, options.get_max_concurrency())
                    .await;

                debug!(
                    resolved_count = ref_map.len(),
                    total_count = element_count,
                    "Completed parallel ref resolution"
                );

                // Apply refs to the snapshot tree
                apply_refs_to_snapshot(&mut snapshot, &ref_map);

                // Release the elements array to free memory
                let _ = self.release_object(&elements_object_id).await;
            }
        }

        // Release the result object
        let _ = self.release_object(&result_object_id).await;

        Ok(snapshot)
    }

    /// Batch-fetch all array element object IDs using `Runtime.getProperties`.
    ///
    /// This replaces N individual `get_array_element()` calls with a single CDP call,
    /// significantly reducing round-trips for large arrays.
    async fn get_all_array_elements(
        &self,
        array_object_id: &str,
    ) -> Result<Vec<(usize, String)>, PageError> {
        #[derive(Debug, serde::Deserialize)]
        struct PropertyDescriptor {
            name: String,
            value: Option<viewpoint_cdp::protocol::runtime::RemoteObject>,
        }

        #[derive(Debug, serde::Deserialize)]
        struct GetPropertiesResult {
            result: Vec<PropertyDescriptor>,
        }

        let result: GetPropertiesResult = self
            .connection
            .send_command(
                "Runtime.getProperties",
                Some(serde_json::json!({
                    "objectId": array_object_id,
                    "ownProperties": true,
                    "generatePreview": false
                })),
                Some(&self.session_id),
            )
            .await?;

        // Filter to numeric indices and extract object IDs
        let mut elements: Vec<(usize, String)> = Vec::new();

        for prop in result.result {
            // Parse numeric indices (array elements)
            if let Ok(index) = prop.name.parse::<usize>() {
                if let Some(value) = prop.value {
                    if let Some(object_id) = value.object_id {
                        elements.push((index, object_id));
                    }
                }
            }
        }

        // Sort by index to maintain order
        elements.sort_by_key(|(index, _)| *index);

        trace!(element_count = elements.len(), "Batch-fetched array elements");

        Ok(elements)
    }

    /// Resolve node IDs in parallel with a concurrency limit.
    ///
    /// Uses chunked processing with `FuturesUnordered` to limit concurrency
    /// and avoid overwhelming the browser's CDP connection.
    async fn resolve_node_ids_parallel(
        &self,
        element_object_ids: Vec<(usize, String)>,
        max_concurrency: usize,
    ) -> HashMap<usize, BackendNodeId> {
        let mut ref_map = HashMap::new();

        // Process in chunks to limit concurrency
        for chunk in element_object_ids.chunks(max_concurrency) {
            let futures: FuturesUnordered<_> = chunk
                .iter()
                .map(|(index, object_id)| {
                    let index = *index;
                    let object_id = object_id.clone();
                    async move {
                        match self.describe_node(&object_id).await {
                            Ok(backend_node_id) => {
                                trace!(
                                    index = index,
                                    backend_node_id = backend_node_id,
                                    "Resolved element ref"
                                );
                                Some((index, backend_node_id))
                            }
                            Err(e) => {
                                debug!(index = index, error = %e, "Failed to get backendNodeId for element");
                                None
                            }
                        }
                    }
                })
                .collect();

            // Collect all results from this chunk
            let results: Vec<_> = futures.collect().await;
            for result in results.into_iter().flatten() {
                ref_map.insert(result.0, result.1);
            }
        }

        ref_map
    }

    /// Get a property value from a RemoteObject by name.
    pub(super) async fn get_property_value(
        &self,
        object_id: &str,
        property: &str,
    ) -> Result<serde_json::Value, PageError> {
        #[derive(Debug, serde::Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
        }

        let result: CallResult = self
            .connection
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": format!("function() {{ return this.{}; }}", property),
                    "returnByValue": true
                })),
                Some(&self.session_id),
            )
            .await?;

        Ok(result.result.value.unwrap_or(serde_json::Value::Null))
    }

    /// Get a property as a RemoteObject from a RemoteObject by name.
    pub(super) async fn get_property_object(
        &self,
        object_id: &str,
        property: &str,
    ) -> Result<Option<String>, PageError> {
        #[derive(Debug, serde::Deserialize)]
        struct CallResult {
            result: viewpoint_cdp::protocol::runtime::RemoteObject,
        }

        let result: CallResult = self
            .connection
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": format!("function() {{ return this.{}; }}", property),
                    "returnByValue": false
                })),
                Some(&self.session_id),
            )
            .await?;

        Ok(result.result.object_id)
    }

    /// Get the backendNodeId for an element by its object ID.
    pub(super) async fn describe_node(&self, object_id: &str) -> Result<BackendNodeId, PageError> {
        let result: DescribeNodeResult = self
            .connection
            .send_command(
                "DOM.describeNode",
                Some(DescribeNodeParams {
                    node_id: None,
                    backend_node_id: None,
                    object_id: Some(object_id.to_string()),
                    depth: Some(0),
                    pierce: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        Ok(result.node.backend_node_id)
    }

    /// Release a RemoteObject by its object ID.
    pub(super) async fn release_object(&self, object_id: &str) -> Result<(), PageError> {
        let _: serde_json::Value = self
            .connection
            .send_command(
                "Runtime.releaseObject",
                Some(serde_json::json!({
                    "objectId": object_id
                })),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }
}
