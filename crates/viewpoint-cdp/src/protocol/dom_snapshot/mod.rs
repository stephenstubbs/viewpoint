//! `DOMSnapshot` domain types.
//!
//! This domain facilitates obtaining document snapshots with DOM, layout, and style information.

use serde::{Deserialize, Serialize};

// ============================================================================
// DOMSnapshot.captureSnapshot
// ============================================================================

/// Parameters for DOMSnapshot.captureSnapshot.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotParams {
    /// Whitelist of computed styles to return.
    pub computed_styles: Vec<String>,
    /// Whether to include DOM rectangles (for layout/CSS painting).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_dom_rects: Option<bool>,
    /// Whether to include blended background colors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blended_background_colors: Option<bool>,
    /// Whether to include text color opacities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_text_color_opacities: Option<bool>,
    /// Whether to include paint orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paint_order: Option<bool>,
}

impl CaptureSnapshotParams {
    /// Create new capture snapshot params with default styles.
    pub fn new() -> Self {
        Self {
            computed_styles: vec![
                "display".to_string(),
                "visibility".to_string(),
                "opacity".to_string(),
            ],
            include_dom_rects: Some(true),
            ..Default::default()
        }
    }

    /// Create params with custom computed styles.
    pub fn with_styles(styles: Vec<String>) -> Self {
        Self {
            computed_styles: styles,
            ..Default::default()
        }
    }

    /// Set whether to include DOM rectangles.
    #[must_use]
    pub fn include_dom_rects(mut self, include: bool) -> Self {
        self.include_dom_rects = Some(include);
        self
    }

    /// Set whether to include blended background colors.
    #[must_use]
    pub fn include_blended_background_colors(mut self, include: bool) -> Self {
        self.include_blended_background_colors = Some(include);
        self
    }
}

/// Result of DOMSnapshot.captureSnapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSnapshotResult {
    /// The document snapshots for all documents in the page.
    pub documents: Vec<DocumentSnapshot>,
    /// Shared strings.
    pub strings: Vec<String>,
}

/// A document snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSnapshot {
    /// Document URL index into the `strings` array.
    pub document_url: i32,
    /// Document title index into the `strings` array.
    pub title: i32,
    /// Base URL index into the `strings` array.
    pub base_url: i32,
    /// Content language index into the `strings` array.
    pub content_language: i32,
    /// Encoding name index into the `strings` array.
    pub encoding_name: i32,
    /// Public ID index into the `strings` array.
    pub public_id: i32,
    /// System ID index into the `strings` array.
    pub system_id: i32,
    /// Frame ID index into the `strings` array.
    pub frame_id: i32,
    /// Node tree snapshot.
    pub nodes: NodeTreeSnapshot,
    /// Layout tree snapshot.
    pub layout: LayoutTreeSnapshot,
    /// Text box snapshot.
    pub text_boxes: TextBoxSnapshot,
    /// Scroll offset X.
    #[serde(default)]
    pub scroll_offset_x: Option<f64>,
    /// Scroll offset Y.
    #[serde(default)]
    pub scroll_offset_y: Option<f64>,
    /// Document content width.
    #[serde(default)]
    pub content_width: Option<f64>,
    /// Document content height.
    #[serde(default)]
    pub content_height: Option<f64>,
}

/// Node tree snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeTreeSnapshot {
    /// Parent node index.
    #[serde(default)]
    pub parent_index: Option<Vec<i32>>,
    /// Node type.
    #[serde(default)]
    pub node_type: Option<Vec<i32>>,
    /// Shadow root type (null, open, closed).
    #[serde(default)]
    pub shadow_root_type: Option<RareStringData>,
    /// Node name index into the `strings` array.
    #[serde(default)]
    pub node_name: Option<Vec<i32>>,
    /// Node value index into the `strings` array.
    #[serde(default)]
    pub node_value: Option<Vec<i32>>,
    /// Backend node ID.
    #[serde(default)]
    pub backend_node_id: Option<Vec<i32>>,
    /// Attributes of Element nodes.
    #[serde(default)]
    pub attributes: Option<Vec<ArrayOfStrings>>,
    /// Text value index (for text nodes).
    #[serde(default)]
    pub text_value: Option<RareStringData>,
    /// Input value index (for input elements).
    #[serde(default)]
    pub input_value: Option<RareStringData>,
    /// Input checked state (for checkbox/radio).
    #[serde(default)]
    pub input_checked: Option<RareBooleanData>,
    /// Option selected state.
    #[serde(default)]
    pub option_selected: Option<RareBooleanData>,
    /// Document content document index.
    #[serde(default)]
    pub content_document_index: Option<RareIntegerData>,
    /// Type of a pseudo element node (before, after, backdrop).
    #[serde(default)]
    pub pseudo_type: Option<RareStringData>,
    /// Pseudo element identifier for this node (CSS `::marker`).
    #[serde(default)]
    pub pseudo_identifier: Option<RareStringData>,
    /// Whether this DOM node responds to mouse clicks.
    #[serde(default)]
    pub is_clickable: Option<RareBooleanData>,
    /// The URL of the script (if any) that generates this node.
    #[serde(default)]
    pub current_source_url: Option<RareStringData>,
    /// Origin URL of the script (if any) that generates this node.
    #[serde(default)]
    pub origin_url: Option<RareStringData>,
}

/// Layout tree snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutTreeSnapshot {
    /// Index of the corresponding node in the `NodeTreeSnapshot`.
    pub node_index: Vec<i32>,
    /// Style index array into `computedStyles` array.
    pub styles: Vec<ArrayOfStrings>,
    /// CSS box model bounds (x, y, width, height).
    pub bounds: Vec<Rectangle>,
    /// Text content of text nodes.
    pub text: Vec<i32>,
    /// Stacking contexts.
    #[serde(default)]
    pub stacking_contexts: Option<RareBooleanData>,
    /// Paint orders.
    #[serde(default)]
    pub paint_orders: Option<Vec<i32>>,
    /// Offset rects.
    #[serde(default)]
    pub offset_rects: Option<Vec<Rectangle>>,
    /// Scroll rects.
    #[serde(default)]
    pub scroll_rects: Option<Vec<Rectangle>>,
    /// Client rects.
    #[serde(default)]
    pub client_rects: Option<Vec<Rectangle>>,
    /// Blended background colors.
    #[serde(default)]
    pub blended_background_colors: Option<Vec<i32>>,
    /// Text color opacities.
    #[serde(default)]
    pub text_color_opacities: Option<Vec<f64>>,
}

/// Text box snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextBoxSnapshot {
    /// Index of the layout tree node that owns this box.
    pub layout_index: Vec<i32>,
    /// Text box bounds (x, y, width, height).
    pub bounds: Vec<Rectangle>,
    /// Start offset of text in the text value.
    pub start: Vec<i32>,
    /// Length of the text box substring in the text value.
    pub length: Vec<i32>,
}

/// Rectangle coordinates (x, y, width, height).
pub type Rectangle = Vec<f64>;

/// Array of string indices.
pub type ArrayOfStrings = Vec<i32>;

/// Data for rare string attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareStringData {
    /// Index of the data item.
    pub index: Vec<i32>,
    /// String value.
    pub value: Vec<i32>,
}

/// Data for rare boolean attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareBooleanData {
    /// Index of the data item.
    pub index: Vec<i32>,
}

/// Data for rare integer attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareIntegerData {
    /// Index of the data item.
    pub index: Vec<i32>,
    /// Integer value.
    pub value: Vec<i32>,
}

// ============================================================================
// DOMSnapshot.disable
// ============================================================================

/// Parameters for DOMSnapshot.disable (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct DisableParams {}

// ============================================================================
// DOMSnapshot.enable
// ============================================================================

/// Parameters for DOMSnapshot.enable (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct EnableParams {}
