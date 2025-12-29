//! Page event handler methods.
//!
//! This module contains the `impl Page` block with methods for setting up
//! event handlers (on_dialog, on_console, etc.) and waiting for events.

use std::time::Duration;

use super::super::console::ConsoleMessage;
use super::super::dialog::Dialog;
use super::super::download::Download;
use super::super::file_chooser::FileChooser;
use super::super::frame::Frame;
use super::super::page_error::PageError as PageErrorInfo;
use super::super::Page;
use crate::error::{LocatorError, PageError};

/// Default timeout for navigation and event waiting.
const DEFAULT_NAVIGATION_TIMEOUT: Duration = Duration::from_secs(30);

impl Page {
    // =========================================================================
    // Dialog Handling Methods
    // =========================================================================

    /// Set a handler for browser dialogs (alert, confirm, prompt, beforeunload).
    ///
    /// The handler will be called whenever a dialog appears. If no handler is
    /// set, dialogs are automatically dismissed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use viewpoint_core::DialogType;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Accept all dialogs
    /// page.on_dialog(|dialog| async move {
    ///     println!("Dialog: {:?} - {}", dialog.type_(), dialog.message());
    ///     dialog.accept().await
    /// }).await;
    ///
    /// // Handle prompt with custom text
    /// page.on_dialog(|dialog| async move {
    ///     if matches!(dialog.type_(), DialogType::Prompt) {
    ///         dialog.accept_with_text("my answer").await
    ///     } else {
    ///         dialog.accept().await
    ///     }
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_dialog<F, Fut>(&self, handler: F)
    where
        F: Fn(Dialog) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), PageError>> + Send + 'static,
    {
        self.event_manager.set_dialog_handler(handler).await;
    }

    /// Remove the dialog handler.
    ///
    /// After calling this, dialogs will be automatically dismissed.
    pub async fn off_dialog(&self) {
        self.event_manager.remove_dialog_handler().await;
    }

    // =========================================================================
    // Console Event Methods
    // =========================================================================

    /// Set a handler for console messages (console.log, console.error, etc.).
    ///
    /// The handler will be called whenever JavaScript code logs to the console.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use viewpoint_core::page::console::ConsoleMessageType;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_console(|message| async move {
    ///     println!("[{:?}] {}", message.type_(), message.text());
    /// }).await;
    ///
    /// // Filter by message type
    /// page.on_console(|message| async move {
    ///     if matches!(message.type_(), ConsoleMessageType::Error) {
    ///         eprintln!("Console error: {}", message.text());
    ///     }
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_console<F, Fut>(&self, handler: F)
    where
        F: Fn(ConsoleMessage) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.event_manager.set_console_handler(handler).await;
    }

    /// Remove the console message handler.
    pub async fn off_console(&self) {
        self.event_manager.remove_console_handler().await;
    }

    // =========================================================================
    // Page Error Event Methods
    // =========================================================================

    /// Set a handler for page errors (uncaught exceptions).
    ///
    /// The handler will be called whenever an uncaught JavaScript exception occurs.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_pageerror(|error| async move {
    ///     eprintln!("Page error: {}", error.message());
    ///     if let Some(stack) = error.stack() {
    ///         eprintln!("Stack trace:\n{}", stack);
    ///     }
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_pageerror<F, Fut>(&self, handler: F)
    where
        F: Fn(PageErrorInfo) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.event_manager.set_pageerror_handler(handler).await;
    }

    /// Remove the page error handler.
    pub async fn off_pageerror(&self) {
        self.event_manager.remove_pageerror_handler().await;
    }

    // =========================================================================
    // Frame Event Methods
    // =========================================================================

    /// Set a handler for frame attached events.
    ///
    /// The handler will be called whenever a new frame is attached to the page,
    /// typically when an `<iframe>` is added to the DOM.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_frameattached(|frame| async move {
    ///     println!("Frame attached: {}", frame.url());
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_frameattached<F, Fut>(&self, handler: F)
    where
        F: Fn(Frame) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.event_manager.set_frameattached_handler(handler).await;
    }

    /// Remove the frame attached handler.
    pub async fn off_frameattached(&self) {
        self.event_manager.remove_frameattached_handler().await;
    }

    /// Set a handler for frame navigated events.
    ///
    /// The handler will be called whenever a frame navigates to a new URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_framenavigated(|frame| async move {
    ///     println!("Frame navigated to: {}", frame.url());
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_framenavigated<F, Fut>(&self, handler: F)
    where
        F: Fn(Frame) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.event_manager.set_framenavigated_handler(handler).await;
    }

    /// Remove the frame navigated handler.
    pub async fn off_framenavigated(&self) {
        self.event_manager.remove_framenavigated_handler().await;
    }

    /// Set a handler for frame detached events.
    ///
    /// The handler will be called whenever a frame is detached from the page,
    /// typically when an `<iframe>` is removed from the DOM.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_framedetached(|frame| async move {
    ///     println!("Frame detached: {}", frame.id());
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_framedetached<F, Fut>(&self, handler: F)
    where
        F: Fn(Frame) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.event_manager.set_framedetached_handler(handler).await;
    }

    /// Remove the frame detached handler.
    pub async fn off_framedetached(&self) {
        self.event_manager.remove_framedetached_handler().await;
    }

    // =========================================================================
    // Expect Methods (Wait for events triggered by actions)
    // =========================================================================

    /// Wait for a console message triggered by an action.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// let message = page.expect_console(|| async {
    ///     page.locator("#log-button").click().await?;
    ///     Ok(())
    /// }).await?;
    ///
    /// println!("Console message: {}", message.text());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn expect_console<F, Fut>(&self, action: F) -> Result<ConsoleMessage, PageError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), LocatorError>>,
    {
        let timeout = DEFAULT_NAVIGATION_TIMEOUT;
        let console_future = self.event_manager.wait_for_console(timeout);
        
        // Perform the action
        action().await.map_err(|e| PageError::EvaluationFailed(e.to_string()))?;
        
        // Wait for the console message
        console_future.await
    }

    /// Wait for a page error triggered by an action.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// let error = page.expect_pageerror(|| async {
    ///     page.locator("#trigger-error").click().await?;
    ///     Ok(())
    /// }).await?;
    ///
    /// println!("Page error: {}", error.message());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn expect_pageerror<F, Fut>(&self, action: F) -> Result<PageErrorInfo, PageError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), LocatorError>>,
    {
        let timeout = DEFAULT_NAVIGATION_TIMEOUT;
        let pageerror_future = self.event_manager.wait_for_pageerror(timeout);
        
        // Perform the action
        action().await.map_err(|e| PageError::EvaluationFailed(e.to_string()))?;
        
        // Wait for the page error
        pageerror_future.await
    }

    // =========================================================================
    // Download Handling Methods
    // =========================================================================

    /// Set a handler for file downloads.
    ///
    /// The handler will be called whenever a download starts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_download(|mut download| async move {
    ///     let path = download.path().await.unwrap();
    ///     println!("Downloaded: {}", path.display());
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_download<F, Fut>(&self, handler: F)
    where
        F: Fn(Download) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        // Enable downloads first
        let _ = self.event_manager.set_download_behavior(true).await;
        self.event_manager.set_download_handler(handler).await;
    }

    /// Wait for a download triggered by an action.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// let mut download = page.expect_download(|| async {
    ///     page.locator("a.download").click().await?;
    ///     Ok(())
    /// }).await?;
    ///
    /// download.save_as("./my-file.pdf").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn expect_download<F, Fut>(&self, action: F) -> Result<Download, PageError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), LocatorError>>,
    {
        // Enable downloads first
        self.event_manager.set_download_behavior(true).await?;

        // Start waiting and then perform action
        let timeout = DEFAULT_NAVIGATION_TIMEOUT;
        let download_future = self.event_manager.wait_for_download(timeout);
        
        // Perform the action
        action().await.map_err(|e| PageError::EvaluationFailed(e.to_string()))?;
        
        // Wait for the download
        download_future.await
    }

    // =========================================================================
    // File Chooser Handling Methods
    // =========================================================================

    /// Set whether to intercept file chooser dialogs.
    ///
    /// When enabled, file chooser dialogs will be intercepted and the
    /// `on_filechooser` handler will be called instead of showing the
    /// native file picker.
    pub async fn set_intercept_file_chooser(&self, enabled: bool) -> Result<(), PageError> {
        self.event_manager.set_intercept_file_chooser(enabled).await
    }

    /// Set a handler for file chooser dialogs.
    ///
    /// You must call `set_intercept_file_chooser(true)` before using this.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.set_intercept_file_chooser(true).await?;
    /// page.on_filechooser(|chooser| async move {
    ///     chooser.set_files(&["./upload.txt"]).await.unwrap();
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_filechooser<F, Fut>(&self, handler: F)
    where
        F: Fn(FileChooser) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.event_manager.set_file_chooser_handler(handler).await;
    }

    /// Wait for a file chooser triggered by an action.
    ///
    /// You must call `set_intercept_file_chooser(true)` before using this.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.set_intercept_file_chooser(true).await?;
    /// let chooser = page.expect_file_chooser(|| async {
    ///     page.locator("input[type=file]").click().await?;
    ///     Ok(())
    /// }).await?;
    ///
    /// chooser.set_files(&["./upload.txt"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn expect_file_chooser<F, Fut>(&self, action: F) -> Result<FileChooser, PageError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), LocatorError>>,
    {
        let timeout = DEFAULT_NAVIGATION_TIMEOUT;
        let chooser_future = self.event_manager.wait_for_file_chooser(timeout);
        
        // Perform the action
        action().await.map_err(|e| PageError::EvaluationFailed(e.to_string()))?;
        
        // Wait for the file chooser
        chooser_future.await
    }
}
