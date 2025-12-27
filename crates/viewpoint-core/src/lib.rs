//! Core domain types for `Viewpoint` browser automation.
//!
//! This crate provides the high-level API for browser automation,
//! including Browser, `BrowserContext`, Page, and navigation types.

// Clippy configuration for this crate
// These lints are allowed at crate level to reduce noise during development
// TODO: Address these in a dedicated documentation/cleanup pass
#![allow(clippy::missing_errors_doc)] // Many async functions return Result
#![allow(clippy::missing_panics_doc)] // Panics are documented where critical
#![allow(clippy::too_many_arguments)] // Some CDP functions need many params
#![allow(clippy::too_many_lines)] // Large functions will be refactored in Phase 3
#![allow(clippy::type_complexity)] // Route handler types are intentionally complex
#![allow(clippy::cast_possible_truncation)] // Intentional numeric conversions
#![allow(clippy::cast_sign_loss)] // Intentional for timestamp conversions
#![allow(clippy::cast_possible_wrap)] // Intentional for size conversions
#![allow(clippy::unused_async)] // Some async fns are scaffolding for future implementation
#![allow(clippy::unused_self)] // Some methods are designed for future use
#![allow(clippy::items_after_statements)] // Local imports for clarity in long functions
#![allow(clippy::needless_pass_by_value)] // API design choice for builder patterns
#![allow(clippy::large_enum_variant)] // NetworkEvent variants have different sizes by design
#![allow(clippy::match_same_arms)] // Explicit matching for documentation clarity
#![allow(clippy::manual_let_else)] // Style preference
#![allow(clippy::return_self_not_must_use)] // Builders don't always need must_use
#![allow(clippy::assigning_clones)] // Performance optimization can be done later
#![allow(clippy::cast_precision_loss)] // Acceptable for timestamp calculations
#![allow(clippy::doc_markdown)] // Some technical terms don't need backticks
#![allow(clippy::wildcard_in_or_patterns)] // Pattern style preference
#![allow(clippy::struct_field_names)] // Field naming is intentional
#![allow(clippy::missing_fields_in_debug)] // Debug impls are intentionally partial
#![allow(clippy::format_push_string)] // Performance can be optimized later

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
