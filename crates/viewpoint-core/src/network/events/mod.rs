//! Network event handling.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast;
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::network::{
    LoadingFailedEvent, LoadingFinishedEvent, RequestWillBeSentEvent, ResponseReceivedEvent,
};

use super::request::Request;
use super::response::Response;
use super::types::{ResourceType, UrlMatcher};
use crate::error::NetworkError;

/// Event emitted when a request is made.
#[derive(Debug, Clone)]
pub struct RequestEvent {
    /// The request.
    pub request: Request,
}

/// Event emitted when a response is received.
#[derive(Debug, Clone)]
pub struct ResponseEvent {
    /// The response.
    pub response: Response,
}

/// Event emitted when a request finishes.
#[derive(Debug, Clone)]
pub struct RequestFinishedEvent {
    /// The request that finished.
    pub request: Request,
}

/// Event emitted when a request fails.
#[derive(Debug, Clone)]
pub struct RequestFailedEvent {
    /// The failed request.
    pub request: Request,
    /// The error message.
    pub error: String,
}

/// Network event types.
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// Request made.
    Request(RequestEvent),
    /// Response received.
    Response(ResponseEvent),
    /// Request finished.
    RequestFinished(RequestFinishedEvent),
    /// Request failed.
    RequestFailed(RequestFailedEvent),
}

/// Network event listener for a page.
#[derive(Debug)]
pub struct NetworkEventListener {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Event sender.
    event_tx: broadcast::Sender<NetworkEvent>,
}

impl NetworkEventListener {
    /// Create a new network event listener.
    pub fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        let (event_tx, _) = broadcast::channel(256);
        Self {
            connection,
            session_id,
            event_tx,
        }
    }

    /// Subscribe to network events.
    pub fn subscribe(&self) -> broadcast::Receiver<NetworkEvent> {
        self.event_tx.subscribe()
    }

    /// Start listening for network events.
    ///
    /// This spawns a background task that processes CDP events.
    pub fn start(&self) {
        let mut cdp_events = self.connection.subscribe_events();
        let session_id = self.session_id.clone();
        let event_tx = self.event_tx.clone();
        let connection = self.connection.clone();

        tokio::spawn(async move {
            // Track pending requests for building responses
            let mut pending_requests: HashMap<String, Request> = HashMap::new();

            while let Ok(event) = cdp_events.recv().await {
                // Filter events for this session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                // Process network events
                match event.method.as_str() {
                    "Network.requestWillBeSent" => {
                        if let Some(params) = &event.params {
                            if let Ok(req_event) =
                                serde_json::from_value::<RequestWillBeSentEvent>(params.clone())
                            {
                                // Check if this is a redirect (redirect_response is present)
                                let previous_request = if req_event.redirect_response.is_some() {
                                    // This is a redirect - get the previous request with the same ID
                                    pending_requests.remove(&req_event.request_id)
                                } else {
                                    None
                                };

                                let request =
                                    parse_request_will_be_sent(&req_event, previous_request);
                                pending_requests
                                    .insert(req_event.request_id.clone(), request.clone());
                                let _ =
                                    event_tx.send(NetworkEvent::Request(RequestEvent { request }));
                            }
                        }
                    }
                    "Network.responseReceived" => {
                        if let Some(params) = &event.params {
                            if let Ok(resp_event) =
                                serde_json::from_value::<ResponseReceivedEvent>(params.clone())
                            {
                                // Get the associated request
                                if let Some(request) =
                                    pending_requests.get(&resp_event.request_id).cloned()
                                {
                                    let response = Response::new(
                                        resp_event.response,
                                        request,
                                        connection.clone(),
                                        session_id.clone(),
                                        resp_event.request_id.clone(),
                                    );
                                    let _ = event_tx
                                        .send(NetworkEvent::Response(ResponseEvent { response }));
                                }
                            }
                        }
                    }
                    "Network.loadingFinished" => {
                        if let Some(params) = &event.params {
                            if let Ok(finished_event) =
                                serde_json::from_value::<LoadingFinishedEvent>(params.clone())
                            {
                                if let Some(request) =
                                    pending_requests.remove(&finished_event.request_id)
                                {
                                    let _ = event_tx.send(NetworkEvent::RequestFinished(
                                        RequestFinishedEvent { request },
                                    ));
                                }
                            }
                        }
                    }
                    "Network.loadingFailed" => {
                        if let Some(params) = &event.params {
                            if let Ok(failed_event) =
                                serde_json::from_value::<LoadingFailedEvent>(params.clone())
                            {
                                if let Some(request) =
                                    pending_requests.remove(&failed_event.request_id)
                                {
                                    let _ = event_tx.send(NetworkEvent::RequestFailed(
                                        RequestFailedEvent {
                                            request,
                                            error: failed_event.error_text,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

/// Parse a `RequestWillBeSentEvent` into a Request.
/// Parse a `RequestWillBeSentEvent` into a Request.
///
/// If `previous_request` is provided, it will be set as the `redirected_from` source.
fn parse_request_will_be_sent(
    event: &RequestWillBeSentEvent,
    previous_request: Option<Request>,
) -> Request {
    let resource_type = event
        .resource_type
        .as_ref()
        .map_or(ResourceType::Other, |t| parse_resource_type(t));

    Request {
        url: event.request.url.clone(),
        method: event.request.method.clone(),
        headers: event.request.headers.clone(),
        post_data: event.request.post_data.clone(),
        resource_type,
        frame_id: event.frame_id.clone().unwrap_or_default(),
        is_navigation: event.initiator.initiator_type == "navigation",
        connection: None,
        session_id: None,
        request_id: Some(event.request_id.clone()),
        redirected_from: previous_request.map(Box::new),
        redirected_to: None,
        timing: None,
        failure_text: None,
    }
}

/// Parse a resource type string into `ResourceType` enum.
fn parse_resource_type(s: &str) -> ResourceType {
    match s.to_lowercase().as_str() {
        "document" => ResourceType::Document,
        "stylesheet" => ResourceType::Stylesheet,
        "image" => ResourceType::Image,
        "media" => ResourceType::Media,
        "font" => ResourceType::Font,
        "script" => ResourceType::Script,
        "texttrack" => ResourceType::TextTrack,
        "xhr" => ResourceType::Xhr,
        "fetch" => ResourceType::Fetch,
        "eventsource" => ResourceType::EventSource,
        "websocket" => ResourceType::WebSocket,
        "manifest" => ResourceType::Manifest,
        "ping" => ResourceType::Ping,
        "other" | _ => ResourceType::Other,
    }
}

/// Builder for waiting for a request.
#[derive(Debug)]
pub struct WaitForRequestBuilder<'a, M> {
    /// Connection.
    connection: &'a Arc<CdpConnection>,
    /// Session ID.
    session_id: &'a str,
    /// Pattern to match.
    pattern: M,
    /// Timeout duration.
    timeout: Duration,
}

impl<'a, M: UrlMatcher + Clone + 'static> WaitForRequestBuilder<'a, M> {
    /// Create a new wait for request builder.
    pub fn new(connection: &'a Arc<CdpConnection>, session_id: &'a str, pattern: M) -> Self {
        Self {
            connection,
            session_id,
            pattern,
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the timeout duration.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Wait for a matching request.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out before a matching request is received,
    /// or if the event stream is aborted.
    pub async fn wait(self) -> Result<Request, NetworkError> {
        let mut events = self.connection.subscribe_events();
        let session_id = self.session_id.to_string();
        let pattern = self.pattern;
        let timeout = self.timeout;

        tokio::time::timeout(timeout, async move {
            while let Ok(event) = events.recv().await {
                // Filter for this session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                if event.method == "Network.requestWillBeSent" {
                    if let Some(params) = &event.params {
                        if let Ok(req_event) =
                            serde_json::from_value::<RequestWillBeSentEvent>(params.clone())
                        {
                            if pattern.matches(&req_event.request.url) {
                                return Ok(parse_request_will_be_sent(&req_event, None));
                            }
                        }
                    }
                }
            }
            Err(NetworkError::Aborted)
        })
        .await
        .map_err(|_| NetworkError::Timeout(timeout))?
    }
}

/// Builder for waiting for a response.
#[derive(Debug)]
pub struct WaitForResponseBuilder<'a, M> {
    /// Connection.
    connection: &'a Arc<CdpConnection>,
    /// Session ID.
    session_id: &'a str,
    /// Pattern to match.
    pattern: M,
    /// Timeout duration.
    timeout: Duration,
}

impl<'a, M: UrlMatcher + Clone + 'static> WaitForResponseBuilder<'a, M> {
    /// Create a new wait for response builder.
    pub fn new(connection: &'a Arc<CdpConnection>, session_id: &'a str, pattern: M) -> Self {
        Self {
            connection,
            session_id,
            pattern,
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the timeout duration.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Wait for a matching response.
    ///
    /// # Errors
    ///
    /// Returns an error if the wait times out before a matching response is received,
    /// or if the event stream is aborted.
    pub async fn wait(self) -> Result<Response, NetworkError> {
        let mut events = self.connection.subscribe_events();
        let session_id = self.session_id.to_string();
        let pattern = self.pattern;
        let timeout = self.timeout;
        let connection = self.connection.clone();

        tokio::time::timeout(timeout, async move {
            let mut pending_requests: HashMap<String, Request> = HashMap::new();

            while let Ok(event) = events.recv().await {
                // Filter for this session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                match event.method.as_str() {
                    "Network.requestWillBeSent" => {
                        // Track requests so we can associate them with responses
                        if let Some(params) = &event.params {
                            if let Ok(req_event) =
                                serde_json::from_value::<RequestWillBeSentEvent>(params.clone())
                            {
                                let request = parse_request_will_be_sent(&req_event, None);
                                pending_requests.insert(req_event.request_id.clone(), request);
                            }
                        }
                    }
                    "Network.responseReceived" => {
                        if let Some(params) = &event.params {
                            if let Ok(resp_event) =
                                serde_json::from_value::<ResponseReceivedEvent>(params.clone())
                            {
                                if pattern.matches(&resp_event.response.url) {
                                    // Get the associated request or create a minimal one
                                    let request = pending_requests
                                        .get(&resp_event.request_id)
                                        .cloned()
                                        .unwrap_or_else(|| Request {
                                            url: resp_event.response.url.clone(),
                                            method: "GET".to_string(),
                                            headers: HashMap::new(),
                                            post_data: None,
                                            resource_type: ResourceType::Other,
                                            frame_id: resp_event
                                                .frame_id
                                                .clone()
                                                .unwrap_or_default(),
                                            is_navigation: false,
                                            connection: None,
                                            session_id: None,
                                            request_id: Some(resp_event.request_id.clone()),
                                            redirected_from: None,
                                            redirected_to: None,
                                            timing: None,
                                            failure_text: None,
                                        });

                                    return Ok(Response::new(
                                        resp_event.response,
                                        request,
                                        connection.clone(),
                                        session_id.clone(),
                                        resp_event.request_id.clone(),
                                    ));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(NetworkError::Aborted)
        })
        .await
        .map_err(|_| NetworkError::Timeout(timeout))?
    }
}

#[cfg(test)]
mod tests;
