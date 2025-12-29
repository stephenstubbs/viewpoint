//! Type aliases for event handlers.
//!
//! This module contains type aliases for the various event handler function types
//! used throughout the page event system.

use std::future::Future;
use std::pin::Pin;

use super::super::console::ConsoleMessage;
use super::super::dialog::Dialog;
use super::super::download::Download;
use super::super::file_chooser::FileChooser;
use super::super::frame::Frame;
use super::super::page_error::PageError as PageErrorInfo;
use crate::error::PageError;

/// Type alias for dialog handler function.
pub type DialogHandler = Box<
    dyn Fn(Dialog) -> Pin<Box<dyn Future<Output = Result<(), PageError>> + Send>> + Send + Sync,
>;

/// Type alias for download handler function.
pub type DownloadHandler =
    Box<dyn Fn(Download) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for file chooser handler function.
pub type FileChooserHandler =
    Box<dyn Fn(FileChooser) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for console message handler function.
pub type ConsoleHandler =
    Box<dyn Fn(ConsoleMessage) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for page error handler function.
pub type PageErrorHandler =
    Box<dyn Fn(PageErrorInfo) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for frame attached handler function.
pub type FrameAttachedHandler =
    Box<dyn Fn(Frame) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for frame navigated handler function.
pub type FrameNavigatedHandler =
    Box<dyn Fn(Frame) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for frame detached handler function.
pub type FrameDetachedHandler =
    Box<dyn Fn(Frame) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
