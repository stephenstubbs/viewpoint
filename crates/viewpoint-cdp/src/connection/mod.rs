//! CDP WebSocket connection management.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::{broadcast, mpsc, oneshot, Mutex};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::error::CdpError;
use crate::transport::{CdpEvent, CdpMessage, CdpRequest, CdpResponse};

/// Default timeout for CDP commands.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Buffer size for the event broadcast channel.
const EVENT_CHANNEL_SIZE: usize = 256;

/// A CDP connection to a browser.
#[derive(Debug)]
pub struct CdpConnection {
    /// Sender for outgoing messages.
    tx: mpsc::Sender<CdpRequest>,
    /// Receiver for incoming events.
    event_rx: broadcast::Sender<CdpEvent>,
    /// Pending responses waiting for completion.
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<CdpResponse>>>>,
    /// Atomic counter for message IDs.
    message_id: AtomicU64,
    /// Handle to the background read task.
    _read_handle: tokio::task::JoinHandle<()>,
    /// Handle to the background write task.
    _write_handle: tokio::task::JoinHandle<()>,
}

impl CdpConnection {
    /// Connect to a CDP WebSocket endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error if the WebSocket connection fails.
    #[instrument(level = "info", skip(ws_url), fields(ws_url = %ws_url))]
    pub async fn connect(ws_url: &str) -> Result<Self, CdpError> {
        info!("Connecting to CDP WebSocket endpoint");
        let (ws_stream, response) = tokio_tungstenite::connect_async(ws_url).await?;
        info!(status = %response.status(), "WebSocket connection established");
        
        let (write, read) = ws_stream.split();

        // Channels for internal communication
        let (tx, rx) = mpsc::channel::<CdpRequest>(64);
        let (event_tx, _) = broadcast::channel::<CdpEvent>(EVENT_CHANNEL_SIZE);
        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<CdpResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Spawn the write task
        let write_handle = tokio::spawn(Self::write_loop(rx, write));
        debug!("Spawned CDP write loop");

        // Spawn the read task
        let read_pending = pending.clone();
        let read_event_tx = event_tx.clone();
        let read_handle = tokio::spawn(Self::read_loop(read, read_pending, read_event_tx));
        debug!("Spawned CDP read loop");

        info!("CDP connection ready");
        Ok(Self {
            tx,
            event_rx: event_tx,
            pending,
            message_id: AtomicU64::new(1),
            _read_handle: read_handle,
            _write_handle: write_handle,
        })
    }

    /// Background task that writes CDP requests to the WebSocket.
    async fn write_loop<S>(mut rx: mpsc::Receiver<CdpRequest>, mut sink: S)
    where
        S: futures_util::Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
    {
        debug!("CDP write loop started");
        while let Some(request) = rx.recv().await {
            let method = request.method.clone();
            let id = request.id;
            
            let json = match serde_json::to_string(&request) {
                Ok(j) => j,
                Err(e) => {
                    error!(error = %e, method = %method, "Failed to serialize CDP request");
                    continue;
                }
            };

            trace!(id = id, method = %method, json_len = json.len(), "Sending CDP request");

            if sink.send(Message::Text(json.into())).await.is_err() {
                warn!("WebSocket sink closed, ending write loop");
                break;
            }
            
            debug!(id = id, method = %method, "CDP request sent");
        }
        debug!("CDP write loop ended");
    }

    /// Background task that reads CDP messages from the WebSocket.
    async fn read_loop<S>(
        mut stream: S,
        pending: Arc<Mutex<HashMap<u64, oneshot::Sender<CdpResponse>>>>,
        event_tx: broadcast::Sender<CdpEvent>,
    ) where
        S: futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
            + Unpin,
    {
        debug!("CDP read loop started");
        while let Some(msg) = stream.next().await {
            let msg = match msg {
                Ok(Message::Text(text)) => text,
                Ok(Message::Close(frame)) => {
                    info!(?frame, "WebSocket closed by remote");
                    break;
                }
                Err(e) => {
                    warn!(error = %e, "WebSocket error, ending read loop");
                    break;
                }
                Ok(_) => continue,
            };

            trace!(json_len = msg.len(), "Received CDP message");

            // Parse the incoming message
            let cdp_msg: CdpMessage = match serde_json::from_str(&msg) {
                Ok(m) => m,
                Err(e) => {
                    error!(error = %e, "Failed to parse CDP message");
                    continue;
                }
            };

            match cdp_msg {
                CdpMessage::Response(resp) => {
                    let id = resp.id;
                    let has_error = resp.error.is_some();
                    debug!(id = id, has_error = has_error, "Received CDP response");
                    
                    let mut pending = pending.lock().await;
                    if let Some(sender) = pending.remove(&id) {
                        let _ = sender.send(resp);
                    } else {
                        warn!(id = id, "Received response for unknown request ID");
                    }
                }
                CdpMessage::Event(ref event) => {
                    trace!(method = %event.method, session_id = ?event.session_id, "Received CDP event");
                    // Broadcast to all subscribers; ignore if no receivers.
                    let _ = event_tx.send(event.clone());
                }
            }
        }
        debug!("CDP read loop ended");
    }

    /// Send a CDP command and wait for the response.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The command cannot be sent
    /// - The response times out
    /// - The browser returns a protocol error
    pub async fn send_command<P, R>(
        &self,
        method: &str,
        params: Option<P>,
        session_id: Option<&str>,
    ) -> Result<R, CdpError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        self.send_command_with_timeout(method, params, session_id, DEFAULT_TIMEOUT)
            .await
    }

    /// Send a CDP command with a custom timeout.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The command cannot be sent
    /// - The response times out
    /// - The browser returns a protocol error
    #[instrument(level = "debug", skip(self, params), fields(method = %method, session_id = ?session_id))]
    pub async fn send_command_with_timeout<P, R>(
        &self,
        method: &str,
        params: Option<P>,
        session_id: Option<&str>,
        timeout_duration: Duration,
    ) -> Result<R, CdpError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let id = self.message_id.fetch_add(1, Ordering::Relaxed);
        debug!(id = id, timeout_ms = timeout_duration.as_millis(), "Preparing CDP command");

        let params_value = params
            .map(|p| serde_json::to_value(p))
            .transpose()?;

        let request = CdpRequest {
            id,
            method: method.to_string(),
            params: params_value,
            session_id: session_id.map(ToString::to_string),
        };

        // Create a oneshot channel for the response
        let (resp_tx, resp_rx) = oneshot::channel();

        // Register the pending response
        {
            let mut pending = self.pending.lock().await;
            pending.insert(id, resp_tx);
            trace!(id = id, pending_count = pending.len(), "Registered pending response");
        }

        // Send the request
        self.tx
            .send(request)
            .await
            .map_err(|_| CdpError::ConnectionLost)?;
        
        trace!(id = id, "Request queued for sending");

        // Wait for the response with timeout
        let response = timeout(timeout_duration, resp_rx)
            .await
            .map_err(|_| {
                warn!(id = id, method = %method, "CDP command timed out");
                CdpError::Timeout(timeout_duration)
            })?
            .map_err(|_| CdpError::ConnectionLost)?;

        // Check for protocol errors
        if let Some(ref error) = response.error {
            warn!(id = id, method = %method, code = error.code, error_msg = %error.message, "CDP protocol error");
            return Err(CdpError::Protocol {
                code: error.code,
                message: error.message.clone(),
            });
        }

        debug!(id = id, "CDP command completed successfully");

        // Parse the result
        let result = response.result.unwrap_or(Value::Null);
        serde_json::from_value(result).map_err(CdpError::from)
    }

    /// Subscribe to CDP events.
    ///
    /// Returns a receiver that will receive all CDP events from the browser.
    pub fn subscribe_events(&self) -> broadcast::Receiver<CdpEvent> {
        debug!("New CDP event subscription created");
        self.event_rx.subscribe()
    }
}

#[cfg(test)]
mod tests;
