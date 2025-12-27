//! Fetch builder for intercepting and modifying responses.
//!
//! This module provides the `FetchBuilder` and `FetchedResponse` types for
//! intercepting network responses and modifying them before they reach the page.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use viewpoint_cdp::protocol::fetch::{
    ContinueRequestParams, FulfillRequestParams, HeaderEntry, RequestPausedEvent,
};

use super::route::Route;
use crate::error::NetworkError;

/// Builder for fetching the actual response with optional request modifications.
#[derive(Debug)]
pub struct FetchBuilder<'a> {
    pub(super) route: &'a Route,
    pub(super) url: Option<String>,
    pub(super) method: Option<String>,
    pub(super) headers: Vec<HeaderEntry>,
    pub(super) post_data: Option<Vec<u8>>,
    pub(super) timeout: Duration,
}

impl<'a> FetchBuilder<'a> {
    pub(super) fn new(route: &'a Route) -> Self {
        Self {
            route,
            url: None,
            method: None,
            headers: Vec::new(),
            post_data: None,
            timeout: Duration::from_secs(30),
        }
    }

    /// Override the request URL before fetching.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Override the request method before fetching.
    #[must_use]
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Add or override a request header before fetching.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(HeaderEntry {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    /// Set multiple request headers before fetching.
    #[must_use]
    pub fn headers(
        mut self,
        headers: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        for (name, value) in headers {
            self.headers.push(HeaderEntry {
                name: name.into(),
                value: value.into(),
            });
        }
        self
    }

    /// Override the request POST data before fetching.
    #[must_use]
    pub fn post_data(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.post_data = Some(data.into());
        self
    }

    /// Set the timeout for waiting for the response.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Fetch the response.
    pub async fn send(self) -> Result<FetchedResponse<'a>, NetworkError> {
        use base64::Engine;

        // Subscribe to CDP events before sending the continue command
        let mut events = self.route.connection().subscribe_events();
        let request_id = self.route.request_id().to_string();
        let session_id = self.route.session_id().to_string();

        // Build continue params with modifications
        let post_data = self
            .post_data
            .map(|d| base64::engine::general_purpose::STANDARD.encode(&d));

        let params = ContinueRequestParams {
            request_id: self.route.request_id().to_string(),
            url: self.url,
            method: self.method,
            post_data,
            headers: if self.headers.is_empty() {
                None
            } else {
                Some(self.headers)
            },
            intercept_response: Some(true),
        };

        // Continue the request but intercept the response
        self.route
            .connection()
            .send_command::<_, serde_json::Value>(
                "Fetch.continueRequest",
                Some(params),
                Some(&session_id),
            )
            .await
            .map_err(NetworkError::from)?;

        // Wait for the response-stage Fetch.requestPaused event
        let timeout = self.timeout;
        let response_event = tokio::time::timeout(timeout, async {
            while let Ok(event) = events.recv().await {
                // Filter for our session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                // Look for Fetch.requestPaused at response stage
                if event.method == "Fetch.requestPaused" {
                    if let Some(params) = &event.params {
                        if let Ok(paused) =
                            serde_json::from_value::<RequestPausedEvent>(params.clone())
                        {
                            // Check if this is for our request and at response stage
                            if paused.request_id == request_id && paused.is_response_stage() {
                                return Ok(paused);
                            }
                        }
                    }
                }
            }
            Err(NetworkError::Aborted)
        })
        .await
        .map_err(|_| NetworkError::Timeout(timeout))??;

        // Extract response data
        let status = response_event.response_status_code.unwrap_or(200) as u16;
        let headers: HashMap<String, String> = response_event
            .response_headers
            .as_ref()
            .map(|h| {
                h.iter()
                    .map(|e| (e.name.clone(), e.value.clone()))
                    .collect()
            })
            .unwrap_or_default();

        // Get the response body
        let body = self
            .route
            .get_response_body(&response_event.request_id)
            .await?;

        Ok(FetchedResponse {
            route: self.route,
            request_id: response_event.request_id,
            status,
            headers,
            body,
        })
    }
}

// Allow `route.fetch().await` without calling `.send()`
impl<'a> std::future::IntoFuture for FetchBuilder<'a> {
    type Output = Result<FetchedResponse<'a>, NetworkError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// A response fetched via `route.fetch()`.
#[derive(Debug)]
pub struct FetchedResponse<'a> {
    route: &'a Route,
    /// Request ID for the response-stage paused request.
    request_id: String,
    /// HTTP status code.
    pub status: u16,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Response body (already fetched).
    pub(super) body: Option<Vec<u8>>,
}

impl FetchedResponse<'_> {
    /// Get the response body.
    ///
    /// The body is fetched when `route.fetch()` is called, so this method
    /// returns immediately.
    pub fn body(&self) -> Result<Vec<u8>, NetworkError> {
        self.body
            .clone()
            .ok_or_else(|| NetworkError::InvalidResponse("Response body not available".to_string()))
    }

    /// Get the response body as text.
    pub fn text(&self) -> Result<String, NetworkError> {
        let body = self.body()?;
        String::from_utf8(body)
            .map_err(|e| NetworkError::InvalidResponse(format!("Response is not valid UTF-8: {e}")))
    }

    /// Parse the response body as JSON.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, NetworkError> {
        let text = self.text()?;
        serde_json::from_str(&text)
            .map_err(|e| NetworkError::InvalidResponse(format!("Failed to parse JSON: {e}")))
    }

    /// Continue the response to the page.
    ///
    /// This must be called after inspecting/modifying the response to let
    /// the browser receive it.
    pub async fn fulfill(self) -> Result<(), NetworkError> {
        use base64::Engine;

        // Build response headers
        let response_headers: Vec<HeaderEntry> = self
            .headers
            .iter()
            .map(|(k, v)| HeaderEntry {
                name: k.clone(),
                value: v.clone(),
            })
            .collect();

        // Encode body
        let body = self
            .body
            .map(|b| base64::engine::general_purpose::STANDARD.encode(&b));

        let params = FulfillRequestParams {
            request_id: self.request_id.clone(),
            response_code: i32::from(self.status),
            response_headers: if response_headers.is_empty() {
                None
            } else {
                Some(response_headers)
            },
            binary_response_headers: None,
            body,
            response_phrase: None,
        };

        self.route
            .connection()
            .send_command::<_, serde_json::Value>(
                "Fetch.fulfillRequest",
                Some(params),
                Some(self.route.session_id()),
            )
            .await
            .map_err(NetworkError::from)?;

        Ok(())
    }
}
