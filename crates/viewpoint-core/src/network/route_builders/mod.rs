//! Builder types for route handling.
//!
//! This module provides builders for fulfilling and continuing intercepted
//! network requests. The `FetchBuilder` and `FetchedResponse` types are in
//! the `route_fetch` module.

use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use viewpoint_cdp::protocol::fetch::{ContinueRequestParams, FulfillRequestParams, HeaderEntry};

use super::route::Route;
use super::route_fetch::FetchedResponse;
use crate::error::NetworkError;

/// Builder for fulfilling a request with a custom response.
#[derive(Debug)]
pub struct FulfillBuilder<'a> {
    pub(super) route: &'a Route,
    pub(super) status: u16,
    pub(super) status_text: Option<String>,
    pub(super) headers: Vec<HeaderEntry>,
    pub(super) body: Option<Vec<u8>>,
}

impl<'a> FulfillBuilder<'a> {
    pub(super) fn new(route: &'a Route) -> Self {
        Self {
            route,
            status: 200,
            status_text: None,
            headers: Vec::new(),
            body: None,
        }
    }

    /// Set the HTTP status code.
    #[must_use]
    pub fn status(mut self, code: u16) -> Self {
        self.status = code;
        self
    }

    /// Set the HTTP status text.
    #[must_use]
    pub fn status_text(mut self, text: impl Into<String>) -> Self {
        self.status_text = Some(text.into());
        self
    }

    /// Set a response header.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(HeaderEntry {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    /// Set multiple response headers.
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

    /// Set the Content-Type header.
    #[must_use]
    pub fn content_type(self, mime_type: impl Into<String>) -> Self {
        self.header("Content-Type", mime_type)
    }

    /// Set the response body as text.
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into().into_bytes());
        self
    }

    /// Set the response body as bytes.
    #[must_use]
    pub fn body_bytes(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set the response body as JSON.
    #[must_use]
    pub fn json<T: serde::Serialize>(self, value: &T) -> Self {
        let json = serde_json::to_string(value).unwrap_or_default();
        self.content_type("application/json").body(json)
    }

    /// Set the response body from a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub async fn path(mut self, path: impl AsRef<Path>) -> Result<Self, NetworkError> {
        let body = tokio::fs::read(path.as_ref())
            .await
            .map_err(|e| NetworkError::IoError(e.to_string()))?;
        self.body = Some(body);

        // Try to set content type based on file extension
        if let Some(ext) = path.as_ref().extension().and_then(|e| e.to_str()) {
            let mime_type = match ext.to_lowercase().as_str() {
                "html" | "htm" => "text/html",
                "css" => "text/css",
                "js" => "application/javascript",
                "json" => "application/json",
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                "pdf" => "application/pdf",
                "txt" => "text/plain",
                "xml" => "application/xml",
                _ => "application/octet-stream",
            };
            return Ok(self.content_type(mime_type));
        }

        Ok(self)
    }

    /// Modify an existing response.
    ///
    /// Use this to fulfill with a response that was fetched via `route.fetch()`.
    #[must_use]
    pub fn response(mut self, response: &FetchedResponse<'_>) -> Self {
        self.status = response.status;
        for (name, value) in &response.headers {
            self.headers.push(HeaderEntry {
                name: name.clone(),
                value: value.clone(),
            });
        }
        // Include the body from the fetched response
        if let Some(ref body) = response.body {
            self.body = Some(body.clone());
        }
        self
    }

    /// Send the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the response cannot be sent.
    pub async fn send(self) -> Result<(), NetworkError> {
        use base64::Engine;

        let body = self
            .body
            .map(|b| base64::engine::general_purpose::STANDARD.encode(&b));

        let params = FulfillRequestParams {
            request_id: self.route.request_id().to_string(),
            response_code: i32::from(self.status),
            response_headers: if self.headers.is_empty() {
                None
            } else {
                Some(self.headers)
            },
            binary_response_headers: None,
            body,
            response_phrase: self.status_text,
        };

        self.route.send_fulfill(params).await
    }
}

/// Builder for continuing a request with optional modifications.
#[derive(Debug)]
pub struct ContinueBuilder<'a> {
    pub(super) route: &'a Route,
    pub(super) url: Option<String>,
    pub(super) method: Option<String>,
    pub(super) headers: Vec<HeaderEntry>,
    pub(super) post_data: Option<Vec<u8>>,
}

impl<'a> ContinueBuilder<'a> {
    pub(super) fn new(route: &'a Route) -> Self {
        Self {
            route,
            url: None,
            method: None,
            headers: Vec::new(),
            post_data: None,
        }
    }

    /// Override the request URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Override the request method.
    #[must_use]
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Add or override a request header.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(HeaderEntry {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    /// Set multiple request headers.
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

    /// Override the request POST data.
    ///
    /// The data will be base64-encoded for the CDP command.
    #[must_use]
    pub fn post_data(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.post_data = Some(data.into());
        self
    }

    /// Continue the request (applying any modifications).
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be continued.
    pub async fn send(self) -> Result<(), NetworkError> {
        use base64::Engine;

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
            intercept_response: None,
        };

        self.route.send_continue(params).await
    }
}

// Allow `route.continue_().await` without calling `.send()`
impl ContinueBuilder<'_> {
    /// Await the continue operation.
    pub async fn await_continue(self) -> Result<(), NetworkError> {
        self.send().await
    }
}

impl<'a> std::future::IntoFuture for ContinueBuilder<'a> {
    type Output = Result<(), NetworkError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

// FetchBuilder and FetchedResponse are in route_fetch.rs
