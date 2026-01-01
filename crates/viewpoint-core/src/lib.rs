//! # Viewpoint Core - Browser Automation Library
//!
//! Core domain types for `Viewpoint` browser automation, providing a Playwright-inspired
//! API for controlling Chromium-based browsers via the Chrome DevTools Protocol (CDP).
//!
//! This crate provides the high-level API for browser automation,
//! including [`Browser`], [`BrowserContext`], [`Page`], and navigation types.
//!
//! ## Features
//!
//! - **Browser Control**: Launch or connect to Chromium browsers
//! - **Page Navigation**: Navigate pages and wait for load states
//! - **Element Interaction**: Click, type, and interact with page elements via [`Locator`]
//! - **Network Interception**: Route, modify, and mock network requests
//! - **Device Emulation**: Emulate mobile devices, geolocation, and media features
//! - **Input Devices**: Keyboard, mouse, and touchscreen control
//! - **Screenshots & PDF**: Capture screenshots and generate PDFs
//! - **Clock Mocking**: Control time in tests with [`Clock`]
//! - **Event Handling**: Dialogs, downloads, file choosers, console messages
//! - **Tracing**: Record traces for debugging
//! - **Video Recording**: Record page interactions as video
//!
//! ## Quick Start
//!
//! ```no_run
//! use viewpoint_core::{Browser, DocumentLoadState};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! // Launch a browser and create a new context and page
//! let browser = Browser::launch()
//!     .headless(true)
//!     .launch()
//!     .await?;
//!
//! let context = browser.new_context().await?;
//! let page = context.new_page().await?;
//!
//! // Navigate to a page
//! page.goto("https://example.com").goto().await?;
//!
//! // Interact with elements
//! page.locator("button#submit").click().await?;
//!
//! // Fill a form
//! page.locator("input[name='email']").fill("user@example.com").await?;
//!
//! // Get text content
//! let text = page.locator("h1").text_content().await?;
//! println!("Page title: {:?}", text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Browser Connection Methods
//!
//! There are three ways to get a [`Browser`] instance:
//!
//! ```no_run
//! use viewpoint_core::Browser;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! // 1. Launch a new browser process
//! let browser = Browser::launch()
//!     .headless(true)
//!     .launch()
//!     .await?;
//!
//! // 2. Connect via WebSocket URL (for pre-configured connections)
//! let browser = Browser::connect("ws://localhost:9222/devtools/browser/...").await?;
//!
//! // 3. Connect via HTTP endpoint (auto-discovers WebSocket URL)
//! let browser = Browser::connect_over_cdp("http://localhost:9222")
//!     .timeout(Duration::from_secs(10))
//!     .connect()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Element Locators
//!
//! The [`Locator`] API provides auto-waiting and retry logic for robust element interaction:
//!
//! ```no_run
//! use viewpoint_core::{Browser, AriaRole};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // CSS selector
//! page.locator("button.primary").click().await?;
//!
//! // Text selector
//! page.get_by_text("Submit").click().await?;
//!
//! // Role selector (accessibility)
//! page.get_by_role(AriaRole::Button)
//!     .with_name("Submit")
//!     .build()
//!     .click()
//!     .await?;
//!
//! // Test ID selector (recommended for stable tests)
//! page.get_by_test_id("submit-button").click().await?;
//!
//! // Label selector (for form fields)
//! page.get_by_label("Email address").fill("test@example.com").await?;
//!
//! // Placeholder selector
//! page.get_by_placeholder("Enter your name").fill("John Doe").await?;
//!
//! // Chained locators
//! page.locator(".form")
//!     .locator("input")
//!     .first()
//!     .fill("value")
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Network Interception
//!
//! Intercept and modify network requests using [`Route`]:
//!
//! ```ignore
//! use viewpoint_core::{Browser, Route};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Block images
//! page.route("**/*.{png,jpg,jpeg,gif}", |route| {
//!     async move { route.abort().await }
//! }).await?;
//!
//! // Mock an API response
//! page.route("**/api/users", |route| {
//!     async move {
//!         route.fulfill()
//!             .status(200)
//!             .content_type("application/json")
//!             .body(r#"{"users": []}"#)
//!             .fulfill()
//!             .await
//!     }
//! }).await?;
//!
//! // Modify requests
//! page.route("**/api/**", |route| {
//!     async move {
//!         route.continue_route()
//!             .header("X-Custom-Header", "value")
//!             .continue_route()
//!             .await
//!     }
//! }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Device Emulation
//!
//! Emulate mobile devices and other capabilities:
//!
//! ```no_run
//! use viewpoint_core::{Browser, Permission, ViewportSize};
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! // Create a context with mobile viewport and geolocation
//! let context = browser.new_context_builder()
//!     .viewport(390, 844)  // iPhone 14 size
//!     .device_scale_factor(3.0)
//!     .is_mobile(true)
//!     .has_touch(true)
//!     .geolocation(37.7749, -122.4194)  // San Francisco
//!     .permissions(vec![Permission::Geolocation])
//!     .build()
//!     .await?;
//!
//! let page = context.new_page().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Screenshots and PDF
//!
//! Capture screenshots and generate PDFs:
//!
//! ```no_run
//! use viewpoint_core::Browser;
//! use viewpoint_core::page::PaperFormat;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Screenshot the viewport
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
//! // Generate PDF (headless only)
//! page.pdf()
//!     .format(PaperFormat::A4)
//!     .path("document.pdf")
//!     .generate()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Event Handling
//!
//! Handle browser events like dialogs and downloads:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//!
//! # async fn example() -> Result<(), viewpoint_core::CoreError> {
//! # let browser = Browser::launch().headless(true).launch().await?;
//! # let context = browser.new_context().await?;
//! # let page = context.new_page().await?;
//! // Handle dialogs (alerts, confirms, prompts)
//! page.on_dialog(|dialog| async move {
//!     println!("Dialog message: {}", dialog.message());
//!     dialog.accept(None).await
//! }).await;
//!
//! // Handle downloads
//! page.on_download(|download| async move {
//!     download.save_as("downloads/file.zip").await
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
//! ## Module Organization
//!
//! - [`browser`] - Browser launching and connection management
//! - [`context`] - Browser context (similar to incognito window) management
//! - [`page`] - Page navigation, content, and interaction
//! - [`network`] - Network interception, routing, and HAR recording
//! - [`wait`] - Wait system and load states
//! - [`devices`] - Predefined device descriptors
//! - [`error`] - Error types
//! - [`api`] - API request context for HTTP requests

pub mod api;
pub mod browser;
pub mod context;
pub mod devices;
pub mod error;
pub mod network;
pub mod page;
pub mod wait;

pub use browser::{Browser, BrowserBuilder, ConnectOverCdpBuilder, NewContextBuilder, UserDataDir};
pub use context::{
    BrowserContext, ClearCookiesBuilder, ColorScheme, ContextEventManager, ContextOptions,
    ContextOptionsBuilder, Cookie, ForcedColors, Geolocation, HandlerId, HttpCredentials,
    IndexedDbDatabase, IndexedDbEntry, IndexedDbIndex, IndexedDbObjectStore, LocalStorageEntry,
    PageInfo, Permission, ReducedMotion, SameSite, SetGeolocationBuilder, StorageOrigin,
    StorageState, StorageStateBuilder, StorageStateOptions, StorageStateSource, Tracing,
    TracingOptions, ViewportSize as ContextViewportSize,
};
pub use error::CoreError;
pub use network::{
    AbortError,
    ContinueBuilder,
    FetchedResponse,
    FulfillBuilder,
    HeaderEntry,
    NetworkEvent,
    NetworkEventListener,
    RemoteAddress,
    Request,
    RequestEvent,
    RequestFailedEvent,
    RequestFinishedEvent,
    RequestSizes,
    RequestTiming,
    ResourceType,
    Response,
    ResponseEvent,
    Route,
    RouteHandler,
    RouteHandlerRegistry,
    SecurityDetails,
    UrlMatcher,
    UrlPattern,
    WaitForRequestBuilder,
    WaitForResponseBuilder,
    // WebSocket monitoring
    WebSocket,
    WebSocketFrame,
    WebSocketManager,
};
pub use page::{
    // Screenshot & PDF
    Animations,
    AriaCheckedState,
    AriaRole,
    AriaSnapshot,
    // Element handles and bounding boxes
    BoundingBox,
    BoxModel,
    ClipRegion,
    // Clock mocking
    Clock,
    // Console and Error events
    ConsoleMessage,
    ConsoleMessageLocation,
    ConsoleMessageType,
    // Dialog, Download, FileChooser
    Dialog,
    DialogType,
    Download,
    DownloadState,
    // Input devices
    DragAndDropBuilder,
    ElementHandle,
    // Media and Vision Emulation
    EmulateMediaBuilder,
    FileChooser,
    FilePayload,
    FilterBuilder,
    // Frame support
    Frame,
    FrameElementLocator,
    FrameLocator,
    FrameRoleLocatorBuilder,
    // Navigation
    GotoBuilder,
    JsArg,
    // JavaScript evaluation
    JsHandle,
    Keyboard,
    Locator,
    // Locator handlers
    LocatorHandlerHandle,
    LocatorHandlerOptions,
    LocatorOptions,
    Margins,
    MediaType,
    Mouse,
    MouseButton,
    NavigationResponse,
    Page,
    PageErrorInfo,
    PaperFormat,
    PdfBuilder,
    Polling,
    RoleLocatorBuilder,
    ScreenshotBuilder,
    ScreenshotFormat,
    // Content manipulation
    ScriptTagBuilder,
    ScriptType,
    Selector,
    SetContentBuilder,
    StyleTagBuilder,
    TextOptions,
    TimeValue,
    Touchscreen,
    // Video recording
    Video,
    VideoOptions,
    // Viewport
    ViewportSize,
    VisionDeficiency,
    WaitForFunctionBuilder,
    WebError,
};
pub use wait::DocumentLoadState;
