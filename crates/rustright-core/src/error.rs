//! Core error types.

use std::time::Duration;
use thiserror::Error;

/// Errors that can occur in the core domain.
#[derive(Error, Debug)]
pub enum CoreError {
    /// CDP communication error.
    #[error("CDP error: {0}")]
    Cdp(#[from] rustright_cdp::CdpError),

    /// Browser error.
    #[error("browser error: {0}")]
    Browser(#[from] BrowserError),

    /// Context error.
    #[error("context error: {0}")]
    Context(#[from] ContextError),

    /// Page error.
    #[error("page error: {0}")]
    Page(#[from] PageError),

    /// Wait error.
    #[error("wait error: {0}")]
    Wait(#[from] WaitError),

    /// Navigation error.
    #[error("navigation error: {0}")]
    Navigation(#[from] NavigationError),
}

/// Errors related to browser operations.
#[derive(Error, Debug)]
pub enum BrowserError {
    /// Chromium executable not found.
    #[error("Chromium not found. Set CHROMIUM_PATH environment variable or install Chromium.")]
    ChromiumNotFound,

    /// Failed to launch browser process.
    #[error("failed to launch browser: {0}")]
    LaunchFailed(String),

    /// Browser launch timed out.
    #[error("browser launch timeout after {0:?}")]
    LaunchTimeout(Duration),

    /// Failed to connect to browser.
    #[error("failed to connect to browser: {0}")]
    ConnectionFailed(String),

    /// Browser is already closed.
    #[error("browser is closed")]
    Closed,

    /// CDP error during browser operation.
    #[error("CDP error: {0}")]
    Cdp(#[from] rustright_cdp::CdpError),
}

/// Errors related to browser context operations.
#[derive(Error, Debug)]
pub enum ContextError {
    /// Context is already closed.
    #[error("context is closed")]
    Closed,

    /// Failed to create context.
    #[error("failed to create context: {0}")]
    CreateFailed(String),

    /// CDP error during context operation.
    #[error("CDP error: {0}")]
    Cdp(#[from] rustright_cdp::CdpError),
}

/// Errors related to page operations.
#[derive(Error, Debug)]
pub enum PageError {
    /// Page is already closed.
    #[error("page is closed")]
    Closed,

    /// Failed to create page.
    #[error("failed to create page: {0}")]
    CreateFailed(String),

    /// CDP error during page operation.
    #[error("CDP error: {0}")]
    Cdp(#[from] rustright_cdp::CdpError),
}

/// Errors related to wait operations.
#[derive(Error, Debug)]
pub enum WaitError {
    /// Wait operation timed out.
    #[error("timeout after {0:?}")]
    Timeout(Duration),

    /// Wait operation was cancelled.
    #[error("wait cancelled")]
    Cancelled,

    /// Page was closed during wait.
    #[error("page closed during wait")]
    PageClosed,
}

/// Errors related to navigation operations.
#[derive(Error, Debug)]
pub enum NavigationError {
    /// Navigation timed out.
    #[error("navigation timeout after {0:?}")]
    Timeout(Duration),

    /// Network error during navigation.
    #[error("network error: {0}")]
    NetworkError(String),

    /// SSL certificate error.
    #[error("SSL error: {0}")]
    SslError(String),

    /// Navigation was cancelled.
    #[error("navigation cancelled")]
    Cancelled,

    /// CDP error during navigation.
    #[error("CDP error: {0}")]
    Cdp(#[from] rustright_cdp::CdpError),

    /// Wait error during navigation.
    #[error("wait error: {0}")]
    Wait(#[from] WaitError),
}
