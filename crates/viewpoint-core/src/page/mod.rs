//! # Page Management and Interaction
//!
//! The `Page` type represents a browser tab and provides methods for navigation,
//! content interaction, and capturing screenshots or PDFs.
//!
//! ## Features
//!
//! - **Navigation**: Navigate to URLs, go back/forward, reload
//! - **Element Interaction**: Locate and interact with elements via [`Locator`]
//! - **JavaScript Evaluation**: Execute JavaScript in the page context
//! - **Screenshots**: Capture viewport or full page screenshots
//! - **PDF Generation**: Generate PDFs from page content
//! - **Input Devices**: Control keyboard, mouse, and touchscreen
//! - **Event Handling**: Handle dialogs, downloads, console messages
//! - **Network Interception**: Route, modify, and mock network requests
//! - **Clock Mocking**: Control time in the page with [`Clock`]
//! - **Frames**: Access and interact with iframes via [`Frame`] and [`FrameLocator`]
//! - **Video Recording**: Record page interactions
//!
//! ## Quick Start
//!
//! ```no_run
//! use viewpoint_core::{Browser, DocumentLoadState};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! let browser = Browser::launch().headless(true).launch().await?;
//! let context = browser.new_context().await?;
//! let page = context.new_page().await?;
//!
//! // Navigate to a URL
//! page.goto("https://example.com")
//!     .wait_until(DocumentLoadState::DomContentLoaded)
//!     .goto()
//!     .await?;
//!
//! // Get page title
//! let title = page.title().await?;
//! println!("Page title: {}", title);
//!
//! // Get current URL
//! let url = page.url().await?;
//! println!("Current URL: {}", url);
//! # Ok(())
//! # }
//! ```
//!
//! ## Element Interaction with Locators
//!
//! ```no_run
//! use viewpoint_core::{Browser, AriaRole};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Click a button
//! page.locator("button#submit").click().await?;
//!
//! // Fill an input
//! page.locator("input[name='email']").fill("user@example.com").await?;
//!
//! // Get text content
//! let text = page.locator("h1").text_content().await?;
//!
//! // Use semantic locators
//! page.get_by_role(AriaRole::Button)
//!     .with_name("Submit")
//!     .build()
//!     .click()
//!     .await?;
//!
//! page.get_by_label("Username").fill("john").await?;
//! page.get_by_placeholder("Search...").fill("query").await?;
//! page.get_by_test_id("submit-btn").click().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Screenshots and PDF
//!
//! ```no_run
//! use viewpoint_core::Browser;
//! use viewpoint_core::page::PaperFormat;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Viewport screenshot
//! page.screenshot()
//!     .path("screenshot.png")
//!     .capture()
//!     .await?;
//!
//! // Full page screenshot
//! page.screenshot()
//!     .full_page(true)
//!     .path("full-page.png")
//!     .capture()
//!     .await?;
//!
//! // Generate PDF
//! page.pdf()
//!     .format(PaperFormat::A4)
//!     .path("document.pdf")
//!     .generate()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Input Devices
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Keyboard
//! page.keyboard().press("Tab").await?;
//! page.keyboard().type_text("Hello World").await?;
//! page.keyboard().press("Control+a").await?;
//!
//! // Mouse
//! page.mouse().click(100.0, 200.0).await?;
//! page.mouse().move_to(300.0, 400.0).await?;
//!
//! // Touchscreen
//! page.touchscreen().tap(100.0, 200.0).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Event Handling
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Handle dialogs
//! page.on_dialog(|dialog| async move {
//!     println!("Dialog: {}", dialog.message());
//!     dialog.accept(None).await
//! }).await;
//!
//! // Handle downloads
//! page.on_download(|download| async move {
//!     download.save_as("file.zip").await
//! }).await;
//!
//! // Handle console messages
//! page.on_console(|msg| async move {
//!     println!("[{}] {}", msg.message_type(), msg.text());
//!     Ok(())
//! }).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Frames
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Access iframe by selector
//! let frame = page.frame_locator("iframe#content");
//! frame.locator("button").click().await?;
//!
//! // Access iframe by name
//! let frame = page.frame("content-frame").await;
//! if let Some(f) = frame {
//!     f.locator("input").fill("text").await?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Clock Mocking
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Install clock mocking
//! page.clock().install().await?;
//!
//! // Set to specific time
//! page.clock().set_fixed_time("2024-01-01T12:00:00Z").await?;
//!
//! // Advance time
//! page.clock().run_for(60000).await?; // 60 seconds
//! # Ok(())
//! # }
//! ```

mod accessors;
mod aria_snapshot;
pub use aria_snapshot::SnapshotOptions;
pub mod binding;
pub mod clock;
mod clock_script;
pub mod console;
mod constructors;
mod content;
pub mod dialog;
pub mod download;
pub mod emulation;
mod evaluate;
pub mod events;
pub mod file_chooser;
pub mod frame;
pub mod frame_locator;
mod frame_locator_actions;
mod frame_page_methods;
mod input_devices;
pub mod keyboard;
mod lifecycle;
pub mod locator;
mod locator_factory;
pub mod locator_handler;
mod mouse;
mod mouse_drag;
mod navigation;
pub mod page_error;
mod page_info;
mod pdf;
pub mod popup;
mod ref_resolution;
mod routing_impl;
mod screenshot;
mod screenshot_element;
mod scripts;
mod touchscreen;
pub mod video;
mod video_io;

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use viewpoint_cdp::CdpConnection;



use crate::error::NavigationError;
use crate::network::{RouteHandlerRegistry, WebSocketManager};

pub use clock::{Clock, TimeValue};
pub use console::{ConsoleMessage, ConsoleMessageLocation, ConsoleMessageType, JsArg};
pub use content::{ScriptTagBuilder, ScriptType, SetContentBuilder, StyleTagBuilder};
pub use dialog::Dialog;
pub use download::{Download, DownloadState};
pub use emulation::{EmulateMediaBuilder, MediaType, VisionDeficiency};
pub use evaluate::{JsHandle, Polling, WaitForFunctionBuilder};
pub use events::PageEventManager;
pub use file_chooser::{FileChooser, FilePayload};
pub(crate) use frame::ExecutionContextRegistry;
pub use frame::Frame;
pub use frame_locator::{FrameElementLocator, FrameLocator, FrameRoleLocatorBuilder};
pub use keyboard::Keyboard;
pub use locator::{
    AriaCheckedState, AriaRole, AriaSnapshot, BoundingBox, BoxModel, ElementHandle, FilterBuilder,
    Locator, LocatorOptions, RoleLocatorBuilder, Selector, TapBuilder, TextOptions,
};
pub use locator_handler::{LocatorHandlerHandle, LocatorHandlerManager, LocatorHandlerOptions};
pub use mouse::Mouse;
pub use mouse_drag::DragAndDropBuilder;
pub use navigation::{GotoBuilder, NavigationResponse};
pub use page_error::{PageError as PageErrorInfo, WebError};
pub use pdf::{Margins, PaperFormat, PdfBuilder};
pub use screenshot::{Animations, ClipRegion, ScreenshotBuilder, ScreenshotFormat};
pub use touchscreen::Touchscreen;
pub use video::{Video, VideoOptions};
pub use viewpoint_cdp::protocol::DialogType;
pub use viewpoint_cdp::protocol::emulation::ViewportSize;
pub use viewpoint_cdp::protocol::input::MouseButton;

/// Default navigation timeout.
const DEFAULT_NAVIGATION_TIMEOUT: Duration = Duration::from_secs(30);

/// Default test ID attribute name.
pub const DEFAULT_TEST_ID_ATTRIBUTE: &str = "data-testid";

/// A browser page (tab).
pub struct Page {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Target ID.
    target_id: String,
    /// Session ID for this page.
    session_id: String,
    /// Main frame ID.
    frame_id: String,
    /// Context index for element ref generation.
    /// Used to generate scoped element refs in the format `c{contextIndex}p{pageIndex}e{counter}`.
    context_index: usize,
    /// Page index within the context for element ref generation.
    /// Used to generate scoped element refs in the format `c{contextIndex}p{pageIndex}e{counter}`.
    page_index: usize,
    /// Whether the page has been closed.
    closed: bool,
    /// Route handler registry.
    route_registry: Arc<RouteHandlerRegistry>,
    /// Keyboard controller.
    keyboard: Keyboard,
    /// Mouse controller.
    mouse: Mouse,
    /// Touchscreen controller.
    touchscreen: Touchscreen,
    /// Event manager for dialogs, downloads, and file choosers.
    event_manager: Arc<PageEventManager>,
    /// Locator handler manager.
    locator_handler_manager: Arc<LocatorHandlerManager>,
    /// Video recording controller (if recording is enabled).
    video_controller: Option<Arc<Video>>,
    /// Opener target ID (for popup pages).
    opener_target_id: Option<String>,
    /// Popup event manager.
    popup_manager: Arc<popup::PopupManager>,
    /// WebSocket event manager.
    websocket_manager: Arc<WebSocketManager>,
    /// Exposed function binding manager.
    binding_manager: Arc<binding::BindingManager>,
    /// Custom test ID attribute (defaults to "data-testid").
    test_id_attribute: String,
    /// Execution context registry for tracking frame contexts.
    context_registry: Arc<ExecutionContextRegistry>,
    /// Ref map for element ref resolution.
    /// Maps ref strings (e.g., `c0p0e1`) to their backendNodeIds.
    /// Updated on each `aria_snapshot()` call.
    ref_map: std::sync::Arc<
        parking_lot::RwLock<
            std::collections::HashMap<String, viewpoint_cdp::protocol::dom::BackendNodeId>,
        >,
    >,
    /// Reference to context's pages list for removal on close.
    /// This is used to remove the page from the context's tracking list when closed,
    /// preventing stale sessions from accumulating.
    /// Stores a Vec<Page> to enable returning functional Page objects from context.pages().
    context_pages: Option<Arc<RwLock<Vec<Page>>>>,
}

// Manual Debug implementation since some fields don't implement Debug
impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("target_id", &self.target_id)
            .field("session_id", &self.session_id)
            .field("frame_id", &self.frame_id)
            .field("context_index", &self.context_index)
            .field("page_index", &self.page_index)
            .field("closed", &self.closed)
            .finish_non_exhaustive()
    }
}

impl Page {
    /// Navigate to a URL.
    ///
    /// Returns a builder for configuring navigation options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    /// use viewpoint_core::DocumentLoadState;
    /// use std::time::Duration;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple navigation
    /// page.goto("https://example.com").goto().await?;
    ///
    /// // Navigation with options
    /// page.goto("https://example.com")
    ///     .wait_until(DocumentLoadState::DomContentLoaded)
    ///     .timeout(Duration::from_secs(10))
    ///     .goto()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn goto(&self, url: impl Into<String>) -> GotoBuilder<'_> {
        GotoBuilder::new(self, url.into())
    }

    /// Navigate to a URL and wait for the specified load state.
    ///
    /// This is a convenience method that calls `goto(url).goto().await`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - Navigation fails
    /// - The wait times out
    pub async fn goto_url(&self, url: &str) -> Result<NavigationResponse, NavigationError> {
        self.goto(url).goto().await
    }

    // =========================================================================
    // Screenshot & PDF Methods
    // =========================================================================

    /// Create a screenshot builder for capturing page screenshots.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Capture viewport screenshot
    /// let bytes = page.screenshot().capture().await?;
    ///
    /// // Capture full page screenshot
    /// page.screenshot()
    ///     .full_page(true)
    ///     .path("screenshot.png")
    ///     .capture()
    ///     .await?;
    ///
    /// // Capture JPEG with quality
    /// page.screenshot()
    ///     .jpeg(Some(80))
    ///     .path("screenshot.jpg")
    ///     .capture()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn screenshot(&self) -> screenshot::ScreenshotBuilder<'_> {
        screenshot::ScreenshotBuilder::new(self)
    }

    /// Create a PDF builder for generating PDFs from the page.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::PaperFormat;
    ///
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Generate PDF with default settings
    /// let bytes = page.pdf().generate().await?;
    ///
    /// // Generate A4 landscape PDF
    /// page.pdf()
    ///     .format(PaperFormat::A4)
    ///     .landscape(true)
    ///     .path("document.pdf")
    ///     .generate()
    ///     .await?;
    ///
    /// // Generate PDF with custom margins
    /// page.pdf()
    ///     .margin(1.0) // 1 inch margins
    ///     .print_background(true)
    ///     .generate()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn pdf(&self) -> pdf::PdfBuilder<'_> {
        pdf::PdfBuilder::new(self)
    }
}
