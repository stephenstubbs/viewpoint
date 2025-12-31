//! DOM domain types.
//!
//! The DOM domain exposes DOM read/write operations.

use serde::{Deserialize, Serialize};

/// Unique DOM node identifier.
pub type NodeId = i32;

/// Unique DOM node identifier used to reference a node that may not have been pushed to the front-end.
pub type BackendNodeId = i32;

/// Backend node with a friendly name.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendNode {
    /// Node's nodeType.
    pub node_type: i32,
    /// Node's nodeName.
    pub node_name: String,
    /// Backend node id.
    pub backend_node_id: BackendNodeId,
}

/// Parameters for DOM.setFileInputFiles.
///
/// Sets files for the given file input element.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFileInputFilesParams {
    /// Array of file paths to set.
    pub files: Vec<String>,
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}

/// Parameters for DOM.getDocument.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDocumentParams {
    /// The maximum depth at which children should be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Whether or not iframes and shadow roots should be traversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}

/// DOM Node.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Node identifier that is passed into the rest of the DOM messages.
    pub node_id: NodeId,
    /// Node's nodeType.
    pub node_type: i32,
    /// Node's nodeName.
    pub node_name: String,
    /// Node's local name.
    pub local_name: String,
    /// Node's nodeValue.
    pub node_value: String,
    /// Child count for Container nodes.
    pub child_node_count: Option<i32>,
    /// Child nodes of this node when requested.
    pub children: Option<Vec<Node>>,
    /// Attributes of the Element node in the form of flat array.
    pub attributes: Option<Vec<String>>,
    /// Document URL.
    pub document_url: Option<String>,
    /// Base URL.
    pub base_url: Option<String>,
    /// Content document for frame owner elements.
    pub content_document: Option<Box<Node>>,
    /// Shadow root list for given element host.
    pub shadow_roots: Option<Vec<Node>>,
    /// Frame ID for frame owner elements.
    pub frame_id: Option<String>,
}

/// Result of DOM.getDocument.
#[derive(Debug, Clone, Deserialize)]
pub struct GetDocumentResult {
    /// Resulting node.
    pub root: Node,
}

/// Parameters for DOM.querySelector.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorParams {
    /// Id of the node to query upon.
    pub node_id: NodeId,
    /// Selector string.
    pub selector: String,
}

/// Result of DOM.querySelector.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySelectorResult {
    /// Query selector result.
    pub node_id: NodeId,
}

/// Parameters for DOM.resolveNode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveNodeParams {
    /// Id of the node to resolve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<NodeId>,
    /// Backend identifier of the node to resolve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_node_id: Option<BackendNodeId>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_group: Option<String>,
    /// Execution context in which to resolve the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<i64>,
}

/// Result of DOM.resolveNode.
#[derive(Debug, Clone, Deserialize)]
pub struct ResolveNodeResult {
    /// JavaScript object wrapper for given node.
    pub object: crate::protocol::runtime::RemoteObject,
}

/// Parameters for DOM.describeNode.
///
/// Describes node given its id. Does not require domain to be enabled.
/// Does not start tracking any objects, can be used for automation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeNodeParams {
    /// Identifier of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<NodeId>,
    /// Identifier of the backend node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_node_id: Option<BackendNodeId>,
    /// JavaScript object id of the node wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// The maximum depth at which children should be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Whether or not iframes and shadow roots should be traversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pierce: Option<bool>,
}

/// Result of DOM.describeNode.
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeNodeResult {
    /// Node description.
    pub node: NodeDescription,
}

/// Node description from DOM.describeNode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDescription {
    /// Node identifier that is passed into the rest of the DOM messages.
    pub node_id: NodeId,
    /// The BackendNodeId for this node.
    pub backend_node_id: BackendNodeId,
    /// Node's nodeType.
    pub node_type: i32,
    /// Node's nodeName.
    pub node_name: String,
    /// Node's local name.
    pub local_name: String,
    /// Node's nodeValue.
    pub node_value: String,
    /// Child count for Container nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_node_count: Option<i32>,
    /// Child nodes of this node when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
    /// Attributes of the Element node in the form of flat array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// Frame ID for frame owner elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<String>,
}
