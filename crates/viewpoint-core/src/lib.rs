//! Core domain types for `Viewpoint` browser automation.
//!
//! This crate provides the high-level API for browser automation,
//! including Browser, `BrowserContext`, Page, and navigation types.

pub mod api;
pub mod browser;
pub mod context;
pub mod devices;
pub mod error;
pub mod network;
pub mod page;
pub mod wait;

pub use browser::{Browser, BrowserBuilder, NewContextBuilder};
pub use context::{
    BrowserContext, ClearCookiesBuilder, ColorScheme, ContextEventManager, ContextOptions,
    ContextOptionsBuilder, Cookie, ForcedColors, Geolocation, HandlerId, HttpCredentials,
    IndexedDbDatabase, IndexedDbEntry, IndexedDbIndex, IndexedDbObjectStore,
    LocalStorageEntry, PageInfo, Permission, ReducedMotion, SameSite, SetGeolocationBuilder,
    StorageOrigin, StorageState, StorageStateBuilder, StorageStateOptions, StorageStateSource,
    Tracing, TracingOptions, ViewportSize as ContextViewportSize,
};
pub use error::CoreError;
pub use network::{
    AbortError, ContinueBuilder, FetchedResponse, FulfillBuilder, HeaderEntry, NetworkEvent,
    NetworkEventListener, RemoteAddress, Request, RequestEvent, RequestFailedEvent,
    RequestFinishedEvent, RequestSizes, RequestTiming, ResourceType, Response, ResponseEvent,
    Route, RouteHandler, RouteHandlerRegistry, SecurityDetails, UrlMatcher, UrlPattern,
    WaitForRequestBuilder, WaitForResponseBuilder,
    // WebSocket monitoring
    WebSocket, WebSocketFrame, WebSocketManager,
};
pub use page::{
    AriaCheckedState, AriaRole, AriaSnapshot, FilterBuilder, Locator, LocatorOptions, Page, RoleLocatorBuilder, Selector, TextOptions,
    // Screenshot & PDF
    Animations, ClipRegion, ScreenshotBuilder, ScreenshotFormat,
    Margins, PaperFormat, PdfBuilder,
    // JavaScript evaluation
    JsHandle, Polling, WaitForFunctionBuilder,
    // Content manipulation
    ScriptTagBuilder, ScriptType, SetContentBuilder, StyleTagBuilder,
    // Viewport
    ViewportSize,
    // Navigation
    GotoBuilder, NavigationResponse,
    // Input devices
    DragAndDropBuilder, Keyboard, Mouse, MouseButton, Touchscreen,
    // Frame support
    Frame, FrameElementLocator, FrameLocator, FrameRoleLocatorBuilder,
    // Dialog, Download, FileChooser
    Dialog, DialogType, Download, DownloadState, FileChooser, FilePayload,
    // Locator handlers
    LocatorHandlerHandle, LocatorHandlerOptions,
    // Clock mocking
    Clock, TimeValue,
    // Media and Vision Emulation
    EmulateMediaBuilder, MediaType, VisionDeficiency,
    // Console and Error events
    ConsoleMessage, ConsoleMessageLocation, ConsoleMessageType, JsArg,
    PageErrorInfo, WebError,
    // Video recording
    Video, VideoOptions,
    // Element handles and bounding boxes
    BoundingBox, BoxModel, ElementHandle,
};
pub use wait::DocumentLoadState;
