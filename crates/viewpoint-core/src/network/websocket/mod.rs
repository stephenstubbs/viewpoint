//! WebSocket monitoring and testing.
//!
//! This module provides functionality for monitoring WebSocket connections,
//! including frame events for sent and received messages. Use this to test
//! real-time features without polling.
//!
//! # Strategy for Testing Dynamic WebSocket Content
//!
//! Testing WebSocket-driven dynamic content requires:
//! 1. Set up WebSocket listeners **before** navigation
//! 2. Navigate to the page that establishes WebSocket connections
//! 3. Use Viewpoint's auto-waiting locator assertions to verify DOM updates
//! 4. Optionally capture WebSocket messages for detailed verification
//!
//! ## Verifying WebSocket Data Updates in the DOM (Without Polling)
//!
//! The key insight is to use Viewpoint's built-in auto-waiting assertions
//! (`expect(locator).to_have_text()`, `to_be_visible()`, etc.) which automatically
//! wait for conditions without manual polling:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//! use viewpoint_test::expect;  // from viewpoint-test crate
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let browser = Browser::launch().headless(true).launch().await?;
//! let context = browser.new_context().await?;
//! let page = context.new_page().await?;
//!
//! // Navigate to page with WebSocket-driven live data
//! page.goto("https://example.com/live-dashboard").goto().await?;
//!
//! // Auto-waiting assertions verify DOM updates without polling!
//! // These wait up to 30 seconds for the condition to be true
//!
//! // Verify that live data container becomes visible
//! expect(page.locator(".live-data-container")).to_be_visible().await?;
//!
//! // Verify that WebSocket data rendered specific text content
//! expect(page.locator(".stock-price")).to_contain_text("$").await?;
//!
//! // Verify multiple data points updated via WebSocket
//! expect(page.locator(".connection-status")).to_have_text("Connected").await?;
//! expect(page.locator(".last-update")).not().to_be_empty().await?;
//!
//! // Verify a list populated by WebSocket messages
//! expect(page.locator(".message-list li")).to_have_count_greater_than(0).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Capturing and Verifying Specific WebSocket Messages
//!
//! For more detailed verification, capture WebSocket frames and correlate
//! with DOM state:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//! use viewpoint_test::expect;
//! use std::sync::Arc;
//! use tokio::sync::Mutex;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let browser = Browser::launch().headless(true).launch().await?;
//! let context = browser.new_context().await?;
//! let page = context.new_page().await?;
//!
//! // Capture WebSocket messages for verification
//! let received_messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
//! let messages_clone = received_messages.clone();
//!
//! // Set up WebSocket monitoring BEFORE navigation
//! page.on_websocket(move |ws| {
//!     let messages = messages_clone.clone();
//!     async move {
//!         println!("WebSocket connected: {}", ws.url());
//!         
//!         // Capture all received frames
//!         ws.on_framereceived(move |frame| {
//!             let messages = messages.clone();
//!             async move {
//!                 if frame.is_text() {
//!                     messages.lock().await.push(frame.payload().to_string());
//!                 }
//!             }
//!         }).await;
//!     }
//! }).await;
//!
//! // Navigate to the page
//! page.goto("https://example.com/realtime-chat").goto().await?;
//!
//! // Wait for WebSocket data to be reflected in the DOM
//! // The auto-waiting assertion handles timing without polling
//! expect(page.locator(".chat-messages")).to_be_visible().await?;
//! expect(page.locator(".chat-message")).to_have_count_greater_than(0).await?;
//!
//! // Verify the DOM content matches what was received via WebSocket
//! let messages = received_messages.lock().await;
//! if !messages.is_empty() {
//!     // Parse the WebSocket message (assuming JSON)
//!     let first_msg = &messages[0];
//!     if first_msg.contains("\"text\":") {
//!         // Verify the message text appears in the DOM
//!         let msg_text = page.locator(".chat-message").first().text_content().await?;
//!         assert!(msg_text.is_some(), "Message should be rendered in DOM");
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Waiting for Specific WebSocket Events Before DOM Verification
//!
//! Use synchronization primitives to coordinate between WebSocket events
//! and DOM assertions:
//!
//! ```ignore
//! use viewpoint_core::Browser;
//! use viewpoint_test::expect;
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicBool, Ordering};
//! use tokio::sync::Notify;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let browser = Browser::launch().headless(true).launch().await?;
//! let context = browser.new_context().await?;
//! let page = context.new_page().await?;
//!
//! // Use Notify to signal when specific data arrives
//! let data_ready = Arc::new(Notify::new());
//! let data_ready_clone = data_ready.clone();
//!
//! page.on_websocket(move |ws| {
//!     let notify = data_ready_clone.clone();
//!     async move {
//!         ws.on_framereceived(move |frame| {
//!             let notify = notify.clone();
//!             async move {
//!                 // Signal when we receive the expected data
//!                 if frame.payload().contains("\"status\":\"ready\"") {
//!                     notify.notify_one();
//!                 }
//!             }
//!         }).await;
//!     }
//! }).await;
//!
//! page.goto("https://example.com/app").goto().await?;
//!
//! // Wait for the specific WebSocket message (with timeout)
//! tokio::select! {
//!     _ = data_ready.notified() => {
//!         // Data arrived, now verify DOM reflects it
//!         expect(page.locator(".status-indicator")).to_have_text("Ready").await?;
//!         expect(page.locator(".data-panel")).to_be_visible().await?;
//!     }
//!     _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
//!         panic!("Timeout waiting for WebSocket data");
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Complete Example: Testing a Real-Time Stock Ticker
//!
//! ```ignore
//! use viewpoint_core::Browser;
//! use viewpoint_test::{TestHarness, expect};
//! use std::sync::Arc;
//! use tokio::sync::Mutex;
//!
//! #[tokio::test]
//! async fn test_stock_ticker_updates() -> Result<(), Box<dyn std::error::Error>> {
//!     let harness = TestHarness::new().await?;
//!     let page = harness.page();
//!
//!     // Track stock updates received via WebSocket
//!     let stock_updates: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
//!     let updates_clone = stock_updates.clone();
//!
//!     // Monitor WebSocket for stock price updates
//!     page.on_websocket(move |ws| {
//!         let updates = updates_clone.clone();
//!         async move {
//!             if ws.url().contains("stock-feed") {
//!                 ws.on_framereceived(move |frame| {
//!                     let updates = updates.clone();
//!                     async move {
//!                         updates.lock().await.push(frame.payload().to_string());
//!                     }
//!                 }).await;
//!             }
//!         }
//!     }).await;
//!
//!     // Navigate to the stock ticker page
//!     page.goto("https://example.com/stocks").goto().await?;
//!
//!     // Verify the ticker display updates (auto-waits for DOM changes)
//!     expect(page.locator(".stock-ticker")).to_be_visible().await?;
//!     expect(page.locator(".stock-price")).to_contain_text("$").await?;
//!     
//!     // Verify connection indicator shows live status
//!     expect(page.locator(".connection-status"))
//!         .to_have_text("Live")
//!         .await?;
//!
//!     // Verify at least one price update was rendered
//!     expect(page.locator(".price-change")).not().to_be_empty().await?;
//!
//!     // Confirm WebSocket messages were received
//!     let updates = stock_updates.lock().await;
//!     assert!(!updates.is_empty(), "Should have received stock updates via WebSocket");
//!
//!     Ok(())
//! }
//! ```

// Allow dead code for websocket monitoring scaffolding (spec: network-events)

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::{RwLock, broadcast};
use tracing::{debug, trace};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::{
    WebSocketClosedEvent, WebSocketCreatedEvent, WebSocketFrame as CdpWebSocketFrame,
    WebSocketFrameReceivedEvent, WebSocketFrameSentEvent,
};

/// A WebSocket connection being monitored.
///
/// This struct represents an active WebSocket connection and provides
/// methods to register handlers for frame events.
#[derive(Clone)]
pub struct WebSocket {
    /// The request ID identifying this WebSocket.
    request_id: String,
    /// The WebSocket URL.
    url: String,
    /// Whether the WebSocket is closed.
    is_closed: Arc<AtomicBool>,
    /// Frame sent event broadcaster.
    frame_sent_tx: broadcast::Sender<WebSocketFrame>,
    /// Frame received event broadcaster.
    frame_received_tx: broadcast::Sender<WebSocketFrame>,
    /// Close event broadcaster.
    close_tx: broadcast::Sender<()>,
}

impl std::fmt::Debug for WebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocket")
            .field("request_id", &self.request_id)
            .field("url", &self.url)
            .field("is_closed", &self.is_closed.load(Ordering::SeqCst))
            .finish()
    }
}

impl WebSocket {
    /// Create a new WebSocket instance.
    pub(crate) fn new(request_id: String, url: String) -> Self {
        let (frame_sent_tx, _) = broadcast::channel(256);
        let (frame_received_tx, _) = broadcast::channel(256);
        let (close_tx, _) = broadcast::channel(16);

        Self {
            request_id,
            url,
            is_closed: Arc::new(AtomicBool::new(false)),
            frame_sent_tx,
            frame_received_tx,
            close_tx,
        }
    }

    /// Get the WebSocket URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Check if the WebSocket is closed.
    pub fn is_closed(&self) -> bool {
        self.is_closed.load(Ordering::SeqCst)
    }

    /// Get the request ID for this WebSocket.
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Register a handler for frame sent events.
    ///
    /// The handler will be called whenever a frame is sent over this WebSocket.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::WebSocket;
    ///
    /// # async fn example(websocket: WebSocket) -> Result<(), viewpoint_core::CoreError> {
    /// websocket.on_framesent(|frame| async move {
    ///     println!("Sent: {:?}", frame.payload());
    /// }).await;
    /// # Ok(())
    /// # }
    pub async fn on_framesent<F, Fut>(&self, handler: F)
    where
        F: Fn(WebSocketFrame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut rx = self.frame_sent_tx.subscribe();
        tokio::spawn(async move {
            while let Ok(frame) = rx.recv().await {
                handler(frame).await;
            }
        });
    }

    /// Register a handler for frame received events.
    ///
    /// The handler will be called whenever a frame is received on this WebSocket.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::WebSocket;
    ///
    /// # async fn example(websocket: WebSocket) -> Result<(), viewpoint_core::CoreError> {
    /// websocket.on_framereceived(|frame| async move {
    ///     println!("Received: {:?}", frame.payload());
    /// }).await;
    /// # Ok(())
    /// # }
    pub async fn on_framereceived<F, Fut>(&self, handler: F)
    where
        F: Fn(WebSocketFrame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut rx = self.frame_received_tx.subscribe();
        tokio::spawn(async move {
            while let Ok(frame) = rx.recv().await {
                handler(frame).await;
            }
        });
    }

    /// Register a handler for WebSocket close events.
    ///
    /// The handler will be called when this WebSocket connection is closed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::WebSocket;
    ///
    /// # async fn example(websocket: WebSocket) -> Result<(), viewpoint_core::CoreError> {
    /// websocket.on_close(|| async {
    ///     println!("WebSocket closed");
    /// }).await;
    /// # Ok(())
    /// # }
    pub async fn on_close<F, Fut>(&self, handler: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut rx = self.close_tx.subscribe();
        tokio::spawn(async move {
            if rx.recv().await.is_ok() {
                handler().await;
            }
        });
    }

    /// Emit a frame sent event (internal use).
    pub(crate) fn emit_frame_sent(&self, frame: WebSocketFrame) {
        let _ = self.frame_sent_tx.send(frame);
    }

    /// Emit a frame received event (internal use).
    pub(crate) fn emit_frame_received(&self, frame: WebSocketFrame) {
        let _ = self.frame_received_tx.send(frame);
    }

    /// Mark the WebSocket as closed and emit close event (internal use).
    pub(crate) fn mark_closed(&self) {
        self.is_closed.store(true, Ordering::SeqCst);
        let _ = self.close_tx.send(());
    }
}

/// A WebSocket message frame.
#[derive(Debug, Clone)]
pub struct WebSocketFrame {
    /// The frame opcode (1 for text, 2 for binary).
    opcode: u8,
    /// The frame payload data.
    payload_data: String,
}

impl WebSocketFrame {
    /// Create a new WebSocket frame.
    pub(crate) fn new(opcode: u8, payload_data: String) -> Self {
        Self {
            opcode,
            payload_data,
        }
    }

    /// Create a WebSocket frame from CDP frame data.
    pub(crate) fn from_cdp(cdp_frame: &CdpWebSocketFrame) -> Self {
        Self {
            opcode: cdp_frame.opcode as u8,
            payload_data: cdp_frame.payload_data.clone(),
        }
    }

    /// Get the frame opcode.
    ///
    /// Common opcodes:
    /// - 1: Text frame
    /// - 2: Binary frame
    /// - 8: Close frame
    /// - 9: Ping frame
    /// - 10: Pong frame
    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    /// Get the frame payload data.
    pub fn payload(&self) -> &str {
        &self.payload_data
    }

    /// Check if this is a text frame.
    pub fn is_text(&self) -> bool {
        self.opcode == 1
    }

    /// Check if this is a binary frame.
    pub fn is_binary(&self) -> bool {
        self.opcode == 2
    }
}

/// Type alias for the WebSocket event handler function.
pub type WebSocketEventHandler =
    Box<dyn Fn(WebSocket) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Manager for WebSocket events on a page.
pub struct WebSocketManager {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Active WebSocket connections indexed by request ID.
    websockets: Arc<RwLock<HashMap<String, WebSocket>>>,
    /// WebSocket created event handler.
    handler: Arc<RwLock<Option<WebSocketEventHandler>>>,
    /// Whether the manager is listening for events.
    is_listening: AtomicBool,
}

impl WebSocketManager {
    /// Create a new WebSocket manager for a page.
    pub fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        Self {
            connection,
            session_id,
            websockets: Arc::new(RwLock::new(HashMap::new())),
            handler: Arc::new(RwLock::new(None)),
            is_listening: AtomicBool::new(false),
        }
    }

    /// Set a handler for WebSocket created events.
    pub async fn set_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(WebSocket) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed_handler: WebSocketEventHandler = Box::new(move |ws| Box::pin(handler(ws)));
        let mut h = self.handler.write().await;
        *h = Some(boxed_handler);

        // Start listening for events if not already
        self.start_listening().await;
    }

    /// Remove the WebSocket handler.
    pub async fn remove_handler(&self) {
        let mut h = self.handler.write().await;
        *h = None;
    }

    /// Start listening for WebSocket CDP events.
    async fn start_listening(&self) {
        if self.is_listening.swap(true, Ordering::SeqCst) {
            // Already listening
            return;
        }

        let mut events = self.connection.subscribe_events();
        let session_id = self.session_id.clone();
        let websockets = self.websockets.clone();
        let handler = self.handler.clone();

        tokio::spawn(async move {
            debug!("WebSocket manager started listening for events");

            while let Ok(event) = events.recv().await {
                // Filter events for this session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                match event.method.as_str() {
                    "Network.webSocketCreated" => {
                        if let Some(params) = &event.params {
                            if let Ok(created) =
                                serde_json::from_value::<WebSocketCreatedEvent>(params.clone())
                            {
                                trace!(
                                    "WebSocket created: {} -> {}",
                                    created.request_id, created.url
                                );

                                let ws = WebSocket::new(created.request_id.clone(), created.url);

                                // Store the WebSocket
                                {
                                    let mut sockets = websockets.write().await;
                                    sockets.insert(created.request_id, ws.clone());
                                }

                                // Call the handler
                                let h = handler.read().await;
                                if let Some(ref handler_fn) = *h {
                                    handler_fn(ws).await;
                                }
                            }
                        }
                    }
                    "Network.webSocketClosed" => {
                        if let Some(params) = &event.params {
                            if let Ok(closed) =
                                serde_json::from_value::<WebSocketClosedEvent>(params.clone())
                            {
                                trace!("WebSocket closed: {}", closed.request_id);

                                let sockets = websockets.read().await;
                                if let Some(ws) = sockets.get(&closed.request_id) {
                                    ws.mark_closed();
                                }
                            }
                        }
                    }
                    "Network.webSocketFrameSent" => {
                        if let Some(params) = &event.params {
                            if let Ok(frame_event) =
                                serde_json::from_value::<WebSocketFrameSentEvent>(params.clone())
                            {
                                trace!("WebSocket frame sent: {}", frame_event.request_id);

                                let sockets = websockets.read().await;
                                if let Some(ws) = sockets.get(&frame_event.request_id) {
                                    let frame = WebSocketFrame::from_cdp(&frame_event.response);
                                    ws.emit_frame_sent(frame);
                                }
                            }
                        }
                    }
                    "Network.webSocketFrameReceived" => {
                        if let Some(params) = &event.params {
                            if let Ok(frame_event) = serde_json::from_value::<
                                WebSocketFrameReceivedEvent,
                            >(params.clone())
                            {
                                trace!("WebSocket frame received: {}", frame_event.request_id);

                                let sockets = websockets.read().await;
                                if let Some(ws) = sockets.get(&frame_event.request_id) {
                                    let frame = WebSocketFrame::from_cdp(&frame_event.response);
                                    ws.emit_frame_received(frame);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            debug!("WebSocket manager stopped listening");
        });
    }

    /// Get a WebSocket by request ID.
    pub async fn get(&self, request_id: &str) -> Option<WebSocket> {
        let sockets = self.websockets.read().await;
        sockets.get(request_id).cloned()
    }

    /// Get all active `WebSockets`.
    pub async fn all(&self) -> Vec<WebSocket> {
        let sockets = self.websockets.read().await;
        sockets.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests;
