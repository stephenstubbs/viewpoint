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
