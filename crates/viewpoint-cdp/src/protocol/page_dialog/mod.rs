//! Page dialog types.
//!
//! Types for JavaScript dialogs (alert, confirm, prompt, beforeunload).

use serde::{Deserialize, Serialize};

/// Type of JavaScript dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DialogType {
    /// Alert dialog.
    Alert,
    /// Confirm dialog.
    Confirm,
    /// Prompt dialog.
    Prompt,
    /// Beforeunload dialog.
    Beforeunload,
}

impl std::fmt::Display for DialogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alert => write!(f, "alert"),
            Self::Confirm => write!(f, "confirm"),
            Self::Prompt => write!(f, "prompt"),
            Self::Beforeunload => write!(f, "beforeunload"),
        }
    }
}

/// Event: Page.javascriptDialogOpening
///
/// Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to open.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavascriptDialogOpeningEvent {
    /// Frame url.
    pub url: String,
    /// Message that will be displayed by the dialog.
    pub message: String,
    /// Dialog type.
    #[serde(rename = "type")]
    pub dialog_type: DialogType,
    /// True iff browser is capable showing or acting on the given dialog.
    /// When browser has no dialog handler for given target, calling alert while
    /// Page domain is engaged will stall the page execution. Execution can be
    /// resumed via calling Page.handleJavaScriptDialog.
    pub has_browser_handler: bool,
    /// Default dialog prompt.
    pub default_prompt: Option<String>,
}

/// Event: Page.javascriptDialogClosed
///
/// Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been closed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavascriptDialogClosedEvent {
    /// Whether dialog was confirmed.
    pub result: bool,
    /// User input in case of prompt.
    pub user_input: String,
}

/// Parameters for Page.handleJavaScriptDialog.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleJavaScriptDialogParams {
    /// Whether to accept or dismiss the dialog.
    pub accept: bool,
    /// The text to enter into the dialog prompt before accepting.
    /// Used only if this is a prompt dialog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_text: Option<String>,
}
