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

mod aria_snapshot;
pub use aria_snapshot::SnapshotOptions;
pub mod binding;
mod ref_resolution;
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
pub mod locator;
mod locator_factory;
pub mod locator_handler;
mod mouse;
mod mouse_drag;
mod navigation;
pub mod page_error;
mod pdf;
pub mod popup;
mod routing_impl;
mod screenshot;
mod screenshot_element;
mod scripts;
mod touchscreen;
pub mod video;
mod video_io;

use std::sync::Arc;
use std::time::Duration;

use tracing::{debug, info, instrument, trace, warn};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::page::{NavigateParams, NavigateResult};
use viewpoint_cdp::protocol::target_domain::CloseTargetParams;
use viewpoint_js::js;

use crate::error::{NavigationError, PageError};
use crate::network::{RouteHandlerRegistry, WebSocketManager};
use crate::wait::{DocumentLoadState, LoadStateWaiter};

pub use clock::{Clock, TimeValue};
pub use console::{ConsoleMessage, ConsoleMessageLocation, ConsoleMessageType, JsArg};
pub use content::{ScriptTagBuilder, ScriptType, SetContentBuilder, StyleTagBuilder};
pub use dialog::Dialog;
pub use download::{Download, DownloadState};
pub use emulation::{EmulateMediaBuilder, MediaType, VisionDeficiency};
pub use evaluate::{JsHandle, Polling, WaitForFunctionBuilder};
pub use events::PageEventManager;
pub use file_chooser::{FileChooser, FilePayload};
pub use frame::Frame;
pub(crate) use frame::ExecutionContextRegistry;
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
}

// Manual Debug implementation since some fields don't implement Debug
impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("target_id", &self.target_id)
            .field("session_id", &self.session_id)
            .field("frame_id", &self.frame_id)
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

    /// Navigate to a URL with the given options.
    #[instrument(level = "info", skip(self), fields(target_id = %self.target_id, url = %url, wait_until = ?wait_until, timeout_ms = timeout.as_millis()))]
    pub(crate) async fn navigate_internal(
        &self,
        url: &str,
        wait_until: DocumentLoadState,
        timeout: Duration,
        referer: Option<&str>,
    ) -> Result<NavigationResponse, NavigationError> {
        if self.closed {
            warn!("Attempted navigation on closed page");
            return Err(NavigationError::Cancelled);
        }

        info!("Starting navigation");

        // Create a load state waiter
        let event_rx = self.connection.subscribe_events();
        let mut waiter =
            LoadStateWaiter::new(event_rx, self.session_id.clone(), self.frame_id.clone());
        trace!("Created load state waiter");

        // Send the navigation command
        debug!("Sending Page.navigate command");
        let result: NavigateResult = self
            .connection
            .send_command(
                "Page.navigate",
                Some(NavigateParams {
                    url: url.to_string(),
                    referrer: referer.map(ToString::to_string),
                    transition_type: None,
                    frame_id: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        debug!(frame_id = %result.frame_id, loader_id = ?result.loader_id, "Page.navigate completed");

        // Check for navigation errors
        // Note: Chrome reports HTTP error status codes (4xx, 5xx) as errors with
        // "net::ERR_HTTP_RESPONSE_CODE_FAILURE" or "net::ERR_INVALID_AUTH_CREDENTIALS".
        // Following Playwright's behavior, we treat these as successful navigations
        // that return a response with the appropriate status code.
        if let Some(ref error_text) = result.error_text {
            let is_http_error = error_text == "net::ERR_HTTP_RESPONSE_CODE_FAILURE"
                || error_text == "net::ERR_INVALID_AUTH_CREDENTIALS";

            if !is_http_error {
                warn!(error = %error_text, "Navigation failed with error");
                return Err(NavigationError::NetworkError(error_text.clone()));
            }
            debug!(error = %error_text, "HTTP error response - continuing to capture status");
        }

        // Mark commit as received
        trace!("Setting commit received");
        waiter.set_commit_received().await;

        // Wait for the target load state
        debug!(wait_until = ?wait_until, "Waiting for load state");
        waiter
            .wait_for_load_state_with_timeout(wait_until, timeout)
            .await?;

        // Get response data captured during navigation
        let response_data = waiter.response_data().await;

        info!(frame_id = %result.frame_id, "Navigation completed successfully");

        // Use the final URL from response data if available (handles redirects)
        let final_url = response_data.url.unwrap_or_else(|| url.to_string());

        // Build the response with captured data
        if let Some(status) = response_data.status {
            Ok(NavigationResponse::with_response(
                final_url,
                result.frame_id,
                status,
                response_data.headers,
            ))
        } else {
            Ok(NavigationResponse::new(final_url, result.frame_id))
        }
    }

    /// Close this page.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    #[instrument(level = "info", skip(self), fields(target_id = %self.target_id))]
    pub async fn close(&mut self) -> Result<(), PageError> {
        if self.closed {
            debug!("Page already closed");
            return Ok(());
        }

        info!("Closing page");

        // Clean up route handlers
        self.route_registry.unroute_all().await;
        debug!("Route handlers cleaned up");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Target.closeTarget",
                Some(CloseTargetParams {
                    target_id: self.target_id.clone(),
                }),
                None,
            )
            .await?;

        self.closed = true;
        info!("Page closed");
        Ok(())
    }

    /// Get the target ID.
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Get the session ID.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get the main frame ID.
    pub fn frame_id(&self) -> &str {
        &self.frame_id
    }

    /// Check if this page has been closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get a reference to the CDP connection.
    pub fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
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

    /// Get the current page URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the evaluation fails.
    pub async fn url(&self) -> Result<String, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js! { window.location.href }.to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get URL".to_string()))
    }

    /// Get the current page title.
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the evaluation fails.
    pub async fn title(&self) -> Result<String, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: "document.title".to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get title".to_string()))
    }
}

// Additional Page methods are defined in:
// - scripts.rs: add_init_script, add_init_script_path
// - locator_handler.rs: add_locator_handler, add_locator_handler_with_options, remove_locator_handler
// - video.rs: video, start_video_recording, stop_video_recording
// - binding.rs: expose_function, remove_exposed_function
// - input_devices.rs: keyboard, mouse, touchscreen, clock, drag_and_drop
// - locator_factory.rs: locator, get_by_*, set_test_id_attribute
// - DragAndDropBuilder is defined in mouse.rs
// - RoleLocatorBuilder is defined in locator/mod.rs
