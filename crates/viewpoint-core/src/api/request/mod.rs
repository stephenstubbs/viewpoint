//! API request builder for constructing HTTP requests.

use std::sync::Arc;
use std::time::Duration;

use reqwest::multipart::{Form, Part};
use serde::Serialize;

use super::{APIError, APIResponse, HttpMethod};

/// Builder for constructing and sending HTTP requests.
///
/// This builder provides a fluent API for configuring request options
/// like headers, body, query parameters, and timeout.
///
/// # Example
///
/// ```no_run
/// use viewpoint_core::api::{APIRequestContext, APIContextOptions};
///
/// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
/// let api = APIRequestContext::new(APIContextOptions::new()).await?;
///
/// // Simple GET request
/// let response = api.get("https://api.example.com/users").send().await?;
///
/// // POST with JSON body
/// let user = serde_json::json!({ "name": "John" });
/// let response = api.post("https://api.example.com/users")
///     .json(&user)
///     .header("X-Custom", "value")
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct APIRequestBuilder {
    /// The HTTP client.
    client: Arc<reqwest::Client>,
    /// The HTTP method.
    method: HttpMethod,
    /// The request URL.
    url: String,
    /// Base URL to resolve relative URLs against.
    base_url: Option<String>,
    /// Request headers.
    headers: Vec<(String, String)>,
    /// Default headers from context.
    default_headers: Vec<(String, String)>,
    /// Query parameters.
    query_params: Vec<(String, String)>,
    /// Request body.
    body: Option<RequestBody>,
    /// Request timeout.
    timeout: Option<Duration>,
    /// Whether the context is disposed.
    disposed: bool,
}

/// Types of request body.
#[derive(Debug)]
pub(crate) enum RequestBody {
    /// JSON body (serialized).
    Json(Vec<u8>),
    /// Form-urlencoded body.
    Form(Vec<(String, String)>),
    /// Multipart form body.
    Multipart(Vec<MultipartField>),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// Text body.
    Text(String),
}

/// A field in a multipart form.
#[derive(Debug, Clone)]
pub struct MultipartField {
    /// Field name.
    pub name: String,
    /// Field value (for text fields).
    pub value: Option<String>,
    /// File content (for file fields).
    pub file_content: Option<Vec<u8>>,
    /// File name (for file fields).
    pub filename: Option<String>,
    /// Content type.
    pub content_type: Option<String>,
}

impl MultipartField {
    /// Create a new text field.
    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            file_content: None,
            filename: None,
            content_type: None,
        }
    }

    /// Create a new file field.
    pub fn file(
        name: impl Into<String>,
        filename: impl Into<String>,
        content: Vec<u8>,
    ) -> Self {
        Self {
            name: name.into(),
            value: None,
            file_content: Some(content),
            filename: Some(filename.into()),
            content_type: None,
        }
    }

    /// Set the content type for this field.
    #[must_use]
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
}

impl APIRequestBuilder {
    /// Create a new request builder.
    pub(crate) fn new(
        client: Arc<reqwest::Client>,
        method: HttpMethod,
        url: impl Into<String>,
        base_url: Option<String>,
        default_headers: Vec<(String, String)>,
    ) -> Self {
        Self {
            client,
            method,
            url: url.into(),
            base_url,
            headers: Vec::new(),
            default_headers,
            query_params: Vec::new(),
            body: None,
            timeout: None,
            disposed: false,
        }
    }

    /// Mark this builder as using a disposed context.
    pub(crate) fn set_disposed(&mut self) {
        self.disposed = true;
    }

    /// Add a header to the request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://api.example.com/data")
    ///     .header("Authorization", "Bearer token")
    ///     .header("Accept", "application/json")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Add multiple headers to the request.
    #[must_use]
    pub fn headers(mut self, headers: impl IntoIterator<Item = (String, String)>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Add query parameters to the request URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://api.example.com/search")
    ///     .query(&[("q", "rust"), ("page", "1")])
    ///     .send()
    ///     .await?;
    /// // Request URL: https://api.example.com/search?q=rust&page=1
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn query<K, V>(mut self, params: &[(K, V)]) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (key, value) in params {
            self.query_params
                .push((key.as_ref().to_string(), value.as_ref().to_string()));
        }
        self
    }

    /// Add a single query parameter.
    #[must_use]
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// Set the request body as JSON.
    ///
    /// This will also set the `Content-Type` header to `application/json`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let user = serde_json::json!({
    ///     "name": "John",
    ///     "email": "john@example.com"
    /// });
    ///
    /// let response = api.post("https://api.example.com/users")
    ///     .json(&user)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn json<T: Serialize>(mut self, data: &T) -> Self {
        match serde_json::to_vec(data) {
            Ok(bytes) => {
                self.body = Some(RequestBody::Json(bytes));
            }
            Err(e) => {
                // Store error to report later when sending
                tracing::error!("Failed to serialize JSON: {}", e);
            }
        }
        self
    }

    /// Set the request body as form-urlencoded data.
    ///
    /// This will also set the `Content-Type` header to `application/x-www-form-urlencoded`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.post("https://api.example.com/login")
    ///     .form(&[("username", "john"), ("password", "secret")])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn form<K, V>(mut self, data: &[(K, V)]) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let form_data: Vec<(String, String)> = data
            .iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
            .collect();
        self.body = Some(RequestBody::Form(form_data));
        self
    }

    /// Set the request body as multipart form data.
    ///
    /// This is used for file uploads.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions, MultipartField};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let file_content = vec![1, 2, 3, 4]; // or std::fs::read("document.pdf")
    ///
    /// let response = api.post("https://api.example.com/upload")
    ///     .multipart(vec![
    ///         MultipartField::text("description", "My document"),
    ///         MultipartField::file("file", "document.pdf", file_content)
    ///             .content_type("application/pdf"),
    ///     ])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn multipart(mut self, fields: Vec<MultipartField>) -> Self {
        self.body = Some(RequestBody::Multipart(fields));
        self
    }

    /// Set the request body as raw bytes.
    #[must_use]
    pub fn body(mut self, data: Vec<u8>) -> Self {
        self.body = Some(RequestBody::Bytes(data));
        self
    }

    /// Set the request body as text.
    #[must_use]
    pub fn text(mut self, data: impl Into<String>) -> Self {
        self.body = Some(RequestBody::Text(data.into()));
        self
    }

    /// Set the request timeout.
    ///
    /// This overrides the default timeout set on the API context.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://slow-api.example.com/data")
    ///     .timeout(Duration::from_secs(60))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Resolve the URL, handling relative URLs with base URL.
    fn resolve_url(&self) -> Result<String, APIError> {
        if self.url.starts_with("http://") || self.url.starts_with("https://") {
            Ok(self.url.clone())
        } else if let Some(ref base) = self.base_url {
            // Resolve relative URL against base
            let base = base.trim_end_matches('/');
            let path = self.url.trim_start_matches('/');
            Ok(format!("{base}/{path}"))
        } else {
            Err(APIError::InvalidUrl(format!(
                "Relative URL '{}' requires a base URL",
                self.url
            )))
        }
    }

    /// Send the request and return the response.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The context has been disposed
    /// - The URL is invalid
    /// - The request fails
    /// - A timeout occurs
    pub async fn send(self) -> Result<APIResponse, APIError> {
        if self.disposed {
            return Err(APIError::Disposed);
        }

        let url = self.resolve_url()?;

        // Build the request
        let mut request_builder = self.client.request(self.method.to_reqwest(), &url);

        // Add default headers first
        for (name, value) in &self.default_headers {
            request_builder = request_builder.header(name.as_str(), value.as_str());
        }

        // Add request-specific headers (override defaults)
        for (name, value) in &self.headers {
            request_builder = request_builder.header(name.as_str(), value.as_str());
        }

        // Add query parameters
        if !self.query_params.is_empty() {
            request_builder = request_builder.query(&self.query_params);
        }

        // Set timeout
        if let Some(timeout) = self.timeout {
            request_builder = request_builder.timeout(timeout);
        }

        // Set body
        match self.body {
            Some(RequestBody::Json(bytes)) => {
                request_builder = request_builder
                    .header("Content-Type", "application/json")
                    .body(bytes);
            }
            Some(RequestBody::Form(data)) => {
                request_builder = request_builder.form(&data);
            }
            Some(RequestBody::Multipart(fields)) => {
                let mut form = Form::new();
                for field in fields {
                    if let Some(value) = field.value {
                        form = form.text(field.name, value);
                    } else if let Some(content) = field.file_content {
                        let mut part = Part::bytes(content);
                        if let Some(filename) = field.filename {
                            part = part.file_name(filename);
                        }
                        if let Some(content_type) = field.content_type {
                            part = part.mime_str(&content_type).map_err(|e| {
                                APIError::BuildError(format!("Invalid content type: {e}"))
                            })?;
                        }
                        form = form.part(field.name, part);
                    }
                }
                request_builder = request_builder.multipart(form);
            }
            Some(RequestBody::Bytes(data)) => {
                request_builder = request_builder.body(data);
            }
            Some(RequestBody::Text(data)) => {
                request_builder = request_builder
                    .header("Content-Type", "text/plain")
                    .body(data);
            }
            None => {}
        }

        // Send the request
        let response = request_builder
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    APIError::Timeout(self.timeout.unwrap_or(Duration::from_secs(30)))
                } else {
                    APIError::Http(e)
                }
            })?;

        Ok(APIResponse::new(response))
    }
}

// Make the builder awaitable for convenience
impl std::future::IntoFuture for APIRequestBuilder {
    type Output = Result<APIResponse, APIError>;
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

// Unit tests moved to tests/api_request_tests.rs
