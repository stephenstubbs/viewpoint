//! Dialog handling for browser dialogs.
//!
//! This module provides functionality for handling JavaScript dialogs
//! (alert, confirm, prompt, beforeunload).
//!
//! # Intercepting and Responding to Dialogs
//!
//! Browser dialogs block page execution until handled. Use `page.on_dialog()`
//! to intercept dialogs and respond with accept or dismiss:
//!
//! ```ignore
//! use viewpoint_core::{Browser, Dialog, DialogType};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let browser = Browser::launch().headless(true).launch().await?;
//! let context = browser.new_context().await?;
//! let page = context.new_page().await?;
//!
//! // Handle all dialogs by accepting them
//! page.on_dialog(|dialog| async move {
//!     match dialog.type_() {
//!         DialogType::Alert => {
//!             println!("Alert: {}", dialog.message());
//!             dialog.accept().await
//!         }
//!         DialogType::Confirm => {
//!             println!("Confirm: {}", dialog.message());
//!             dialog.accept().await  // Click "OK"
//!         }
//!         DialogType::Prompt => {
//!             println!("Prompt: {}", dialog.message());
//!             // Respond with custom text
//!             dialog.accept_with_text("my response").await
//!         }
//!         DialogType::Beforeunload => {
//!             dialog.dismiss().await  // Stay on page
//!         }
//!     }
//! }).await;
//!
//! page.goto("https://example.com").goto().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Dismissing Dialogs
//!
//! To dismiss (cancel) a dialog instead of accepting it:
//!
//! ```ignore
//! page.on_dialog(|dialog| async move {
//!     dialog.dismiss().await  // Click "Cancel" or dismiss
//! }).await;
//! ```
//!
//! # Responding to Prompt Dialogs with Custom Values
//!
//! ```ignore
//! page.on_dialog(|dialog| async move {
//!     if matches!(dialog.type_(), DialogType::Prompt) {
//!         // Enter custom text in the prompt
//!         dialog.accept_with_text("custom value").await
//!     } else {
//!         dialog.accept().await
//!     }
//! }).await;
//! ```

use std::sync::Arc;

use tracing::{debug, instrument};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::{DialogType, HandleJavaScriptDialogParams};

use crate::error::PageError;

/// A browser dialog (alert, confirm, prompt, or beforeunload).
///
/// Dialogs are emitted via the `page.on_dialog()` callback. You must either
/// `accept()` or `dismiss()` the dialog - otherwise the page will freeze
/// waiting for user input.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
///
/// page.on_dialog(|dialog| async move {
///     println!("Dialog message: {}", dialog.message());
///     dialog.accept().await
/// });
/// # });
/// ```
#[derive(Debug)]
pub struct Dialog {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Dialog type.
    dialog_type: DialogType,
    /// Dialog message.
    message: String,
    /// Default prompt value.
    default_value: String,
    /// Whether the dialog has been handled.
    handled: bool,
}

impl Dialog {
    /// Create a new Dialog.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        session_id: String,
        dialog_type: DialogType,
        message: String,
        default_value: Option<String>,
    ) -> Self {
        Self {
            connection,
            session_id,
            dialog_type,
            message,
            default_value: default_value.unwrap_or_default(),
            handled: false,
        }
    }

    /// Get the dialog type.
    ///
    /// Returns one of: `alert`, `confirm`, `prompt`, or `beforeunload`.
    pub fn type_(&self) -> DialogType {
        self.dialog_type
    }

    /// Get the dialog message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the default prompt value.
    ///
    /// Only applicable for `prompt` dialogs.
    pub fn default_value(&self) -> &str {
        &self.default_value
    }

    /// Accept the dialog.
    ///
    /// For `alert` dialogs, this closes the dialog.
    /// For `confirm` dialogs, this returns `true` to the JavaScript.
    /// For `prompt` dialogs, this returns the default value to the JavaScript.
    /// For `beforeunload` dialogs, this allows navigation to proceed.
    ///
    /// # Errors
    ///
    /// Returns an error if the dialog has already been handled or CDP fails.
    #[instrument(level = "debug", skip(self), fields(dialog_type = %self.dialog_type))]
    pub async fn accept(self) -> Result<(), PageError> {
        if self.handled {
            return Err(PageError::EvaluationFailed(
                "Dialog has already been handled".to_string(),
            ));
        }

        debug!("Accepting dialog");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.handleJavaScriptDialog",
                Some(HandleJavaScriptDialogParams {
                    accept: true,
                    prompt_text: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        // Dialog is consumed here - Drop won't run on success
        // Use forget to prevent Drop from warning about unhandled dialog
        std::mem::forget(self);
        Ok(())
    }

    /// Accept the dialog with the specified text.
    ///
    /// This is primarily useful for `prompt` dialogs where you want to
    /// provide a custom response.
    ///
    /// # Errors
    ///
    /// Returns an error if the dialog has already been handled or CDP fails.
    #[instrument(level = "debug", skip(self, text), fields(dialog_type = %self.dialog_type))]
    pub async fn accept_with_text(self, text: impl Into<String>) -> Result<(), PageError> {
        if self.handled {
            return Err(PageError::EvaluationFailed(
                "Dialog has already been handled".to_string(),
            ));
        }

        debug!("Accepting dialog with text");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.handleJavaScriptDialog",
                Some(HandleJavaScriptDialogParams {
                    accept: true,
                    prompt_text: Some(text.into()),
                }),
                Some(&self.session_id),
            )
            .await?;

        // Dialog is consumed here - Drop won't run on success
        std::mem::forget(self);
        Ok(())
    }

    /// Dismiss the dialog.
    ///
    /// For `alert` dialogs, this closes the dialog.
    /// For `confirm` dialogs, this returns `false` to the JavaScript.
    /// For `prompt` dialogs, this returns `null` to the JavaScript.
    /// For `beforeunload` dialogs, this cancels navigation.
    ///
    /// # Errors
    ///
    /// Returns an error if the dialog has already been handled or CDP fails.
    #[instrument(level = "debug", skip(self), fields(dialog_type = %self.dialog_type))]
    pub async fn dismiss(self) -> Result<(), PageError> {
        if self.handled {
            return Err(PageError::EvaluationFailed(
                "Dialog has already been handled".to_string(),
            ));
        }

        debug!("Dismissing dialog");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.handleJavaScriptDialog",
                Some(HandleJavaScriptDialogParams {
                    accept: false,
                    prompt_text: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        // Dialog is consumed here - Drop won't run on success
        std::mem::forget(self);
        Ok(())
    }
}

impl Drop for Dialog {
    fn drop(&mut self) {
        // If dialog wasn't handled, we could log a warning
        // but we can't auto-dismiss here since we can't do async in Drop
        if !self.handled {
            tracing::warn!(
                "Dialog of type {} was dropped without being handled. This may cause the page to freeze.",
                self.dialog_type
            );
        }
    }
}
