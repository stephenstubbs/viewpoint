//! Page domain parameter types.

use serde::Serialize;

use super::types::{ScreenshotFormat, Viewport};

/// Parameters for Page.navigate.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigateParams {
    /// URL to navigate the page to.
    pub url: String,
    /// Referrer URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// Intended transition type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_type: Option<String>,
    /// Frame id to navigate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<String>,
}

/// Parameters for Page.reload.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReloadParams {
    /// If true, browser cache is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
    /// Script to inject into all frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_to_evaluate_on_load: Option<String>,
}

/// Parameters for Page.setLifecycleEventsEnabled.
#[derive(Debug, Clone, Serialize)]
pub struct SetLifecycleEventsEnabledParams {
    /// Whether to enable lifecycle events.
    pub enabled: bool,
}

/// Parameters for Page.captureScreenshot.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureScreenshotParams {
    /// Image compression format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScreenshotFormat>,
    /// Compression quality from range [0..100] (jpeg/webp only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u8>,
    /// Capture the screenshot of a given region only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<Viewport>,
    /// Capture the screenshot from the surface, rather than the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_surface: Option<bool>,
    /// Capture the screenshot beyond the viewport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_beyond_viewport: Option<bool>,
    /// Optimize image encoding for speed, not for resulting size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_for_speed: Option<bool>,
}

/// Parameters for Page.printToPDF.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPdfParams {
    /// Paper orientation (default: false = portrait).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    /// Display header and footer (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_header_footer: Option<bool>,
    /// Print background graphics (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_background: Option<bool>,
    /// Scale of the webpage rendering (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Paper width in inches (default: 8.5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_width: Option<f64>,
    /// Paper height in inches (default: 11).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_height: Option<f64>,
    /// Top margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<f64>,
    /// Bottom margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<f64>,
    /// Left margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    /// Right margin in inches (default: 0.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<f64>,
    /// Paper ranges to print, e.g., '1-5, 8, 11-13'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<String>,
    /// HTML template for the print header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_template: Option<String>,
    /// HTML template for the print footer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_template: Option<String>,
    /// Whether or not to prefer page size as defined by css.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_css_page_size: Option<bool>,
    /// Return as stream (experimental).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<String>,
    /// Whether to generate tagged PDF. Default: true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_tagged_pdf: Option<bool>,
    /// Whether to generate document outline. Default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_document_outline: Option<bool>,
}

/// Parameters for Page.navigateToHistoryEntry.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigateToHistoryEntryParams {
    /// Unique id of the entry to navigate to.
    pub entry_id: i32,
}

/// Parameters for Page.addScriptToEvaluateOnNewDocument.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptToEvaluateOnNewDocumentParams {
    /// JavaScript source code to evaluate.
    pub source: String,
    /// If specified, creates an isolated world and evaluates given script in it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_name: Option<String>,
    /// Whether this script should be injected into all frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_command_line_api: Option<bool>,
    /// If true, this script will run in utility world.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_immediately: Option<bool>,
}

/// Parameters for Page.removeScriptToEvaluateOnNewDocument.
#[derive(Debug, Clone, Serialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentParams {
    /// Identifier of the script to remove.
    pub identifier: String,
}

/// Parameters for Page.bringToFront (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct BringToFrontParams {}

/// Parameters for Page.setDocumentContent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentContentParams {
    /// Frame id to set HTML for.
    pub frame_id: String,
    /// HTML content to set.
    pub html: String,
}

/// Parameters for Page.setInterceptFileChooserDialog.
#[derive(Debug, Clone, Serialize)]
pub struct SetInterceptFileChooserDialogParams {
    /// Whether to intercept file chooser dialogs.
    pub enabled: bool,
}

/// Parameters for Page.createIsolatedWorld.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIsolatedWorldParams {
    /// Id of the frame in which the isolated world should be created.
    pub frame_id: String,
    /// An optional name which is reported in the Execution Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_name: Option<String>,
    /// Whether or not universal access should be granted to the isolated world.
    /// This is a powerful option, use with caution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_univeral_access: Option<bool>,
}
