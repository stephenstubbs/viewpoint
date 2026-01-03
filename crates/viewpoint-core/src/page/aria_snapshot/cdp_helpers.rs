//! CDP helper methods for aria snapshot capture.
//!
//! These are internal helpers for interacting with the Chrome DevTools Protocol
//! to resolve element references and manage remote objects.

use std::collections::HashMap;

use futures::stream::{FuturesUnordered, StreamExt};
use tracing::{debug, trace};
use viewpoint_cdp::protocol::dom::{BackendNodeId, DescribeNodeParams, DescribeNodeResult};
use viewpoint_js::js;

use super::super::Page;
use crate::error::PageError;

impl Page {
    /// Batch-fetch all array element object IDs using `Runtime.getProperties`.
    ///
    /// This replaces N individual `get_array_element()` calls with a single CDP call,
    /// significantly reducing round-trips for large arrays.
    pub(super) async fn get_all_array_elements(
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
            .connection()
            .send_command(
                "Runtime.getProperties",
                Some(serde_json::json!({
                    "objectId": array_object_id,
                    "ownProperties": true,
                    "generatePreview": false
                })),
                Some(self.session_id()),
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

        trace!(
            element_count = elements.len(),
            "Batch-fetched array elements"
        );

        Ok(elements)
    }

    /// Resolve node IDs in parallel with a concurrency limit.
    ///
    /// Uses chunked processing with `FuturesUnordered` to limit concurrency
    /// and avoid overwhelming the browser's CDP connection.
    pub(super) async fn resolve_node_ids_parallel(
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

        let js_fn = js! {
            (function() { return this[#{property}]; })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let function_declaration = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result: CallResult = self
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": function_declaration,
                    "returnByValue": true
                })),
                Some(self.session_id()),
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

        let js_fn = js! {
            (function() { return this[#{property}]; })
        };
        // Strip outer parentheses for CDP functionDeclaration
        let function_declaration = js_fn.trim_start_matches('(').trim_end_matches(')');

        let result: CallResult = self
            .connection()
            .send_command(
                "Runtime.callFunctionOn",
                Some(serde_json::json!({
                    "objectId": object_id,
                    "functionDeclaration": function_declaration,
                    "returnByValue": false
                })),
                Some(self.session_id()),
            )
            .await?;

        Ok(result.result.object_id)
    }

    /// Get the backendNodeId for an element by its object ID.
    pub(super) async fn describe_node(&self, object_id: &str) -> Result<BackendNodeId, PageError> {
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

    /// Release a RemoteObject by its object ID.
    pub(super) async fn release_object(&self, object_id: &str) -> Result<(), PageError> {
        let _: serde_json::Value = self
            .connection()
            .send_command(
                "Runtime.releaseObject",
                Some(serde_json::json!({
                    "objectId": object_id
                })),
                Some(self.session_id()),
            )
            .await?;

        Ok(())
    }
}
