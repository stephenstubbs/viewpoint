//! WebSocket CDP event listener.
//!
//! Internal module that handles CDP event dispatching for WebSocket events.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, trace};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::{
    WebSocketClosedEvent, WebSocketCreatedEvent, WebSocketFrameReceivedEvent,
    WebSocketFrameSentEvent,
};

use super::{WebSocket, WebSocketEventHandler, WebSocketFrame};

/// Start listening for WebSocket CDP events.
///
/// This spawns a background task that listens for Network.webSocket* events
/// and dispatches them to the appropriate WebSocket instances.
pub(super) fn spawn_event_listener(
    connection: Arc<CdpConnection>,
    session_id: String,
    websockets: Arc<RwLock<HashMap<String, WebSocket>>>,
    handler: Arc<RwLock<Option<WebSocketEventHandler>>>,
) {
    let mut events = connection.subscribe_events();

    tokio::spawn(async move {
        debug!("WebSocket manager started listening for events");

        while let Ok(event) = events.recv().await {
            // Filter events for this session
            if event.session_id.as_deref() != Some(&session_id) {
                continue;
            }

            match event.method.as_str() {
                "Network.webSocketCreated" => {
                    handle_websocket_created(event.params.as_ref(), &websockets, &handler).await;
                }
                "Network.webSocketClosed" => {
                    handle_websocket_closed(event.params.as_ref(), &websockets).await;
                }
                "Network.webSocketFrameSent" => {
                    handle_frame_sent(event.params.as_ref(), &websockets).await;
                }
                "Network.webSocketFrameReceived" => {
                    handle_frame_received(event.params.as_ref(), &websockets).await;
                }
                _ => {}
            }
        }

        debug!("WebSocket manager stopped listening");
    });
}

async fn handle_websocket_created(
    params: Option<&serde_json::Value>,
    websockets: &RwLock<HashMap<String, WebSocket>>,
    handler: &RwLock<Option<WebSocketEventHandler>>,
) {
    let Some(params) = params else { return };
    let Ok(created) = serde_json::from_value::<WebSocketCreatedEvent>(params.clone()) else {
        return;
    };

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

async fn handle_websocket_closed(
    params: Option<&serde_json::Value>,
    websockets: &RwLock<HashMap<String, WebSocket>>,
) {
    let Some(params) = params else { return };
    let Ok(closed) = serde_json::from_value::<WebSocketClosedEvent>(params.clone()) else {
        return;
    };

    trace!("WebSocket closed: {}", closed.request_id);

    let sockets = websockets.read().await;
    if let Some(ws) = sockets.get(&closed.request_id) {
        ws.mark_closed();
    }
}

async fn handle_frame_sent(
    params: Option<&serde_json::Value>,
    websockets: &RwLock<HashMap<String, WebSocket>>,
) {
    let Some(params) = params else { return };
    let Ok(frame_event) = serde_json::from_value::<WebSocketFrameSentEvent>(params.clone()) else {
        return;
    };

    trace!("WebSocket frame sent: {}", frame_event.request_id);

    let sockets = websockets.read().await;
    if let Some(ws) = sockets.get(&frame_event.request_id) {
        let frame = WebSocketFrame::from_cdp(&frame_event.response);
        ws.emit_frame_sent(frame);
    }
}

async fn handle_frame_received(
    params: Option<&serde_json::Value>,
    websockets: &RwLock<HashMap<String, WebSocket>>,
) {
    let Some(params) = params else { return };
    let Ok(frame_event) = serde_json::from_value::<WebSocketFrameReceivedEvent>(params.clone())
    else {
        return;
    };

    trace!("WebSocket frame received: {}", frame_event.request_id);

    let sockets = websockets.read().await;
    if let Some(ws) = sockets.get(&frame_event.request_id) {
        let frame = WebSocketFrame::from_cdp(&frame_event.response);
        ws.emit_frame_received(frame);
    }
}
