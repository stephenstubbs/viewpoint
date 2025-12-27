//! API response handling.

use std::collections::HashMap;

use bytes::Bytes;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use super::APIError;

/// Response from an API request.
///
/// This struct wraps a reqwest response and provides convenient methods
/// for extracting the response body in various formats.
///
/// # Example
///
/// ```no_run
/// use viewpoint_core::api::{APIRequestContext, APIContextOptions};
///
/// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
/// let api = APIRequestContext::new(APIContextOptions::new()).await?;
/// let response = api.get("https://api.example.com/users").send().await?;
///
/// // Check status
/// if response.ok() {
///     // Parse JSON
///     let data: serde_json::Value = response.json().await?;
///     println!("Got data: {:?}", data);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct APIResponse {
    /// The underlying reqwest response.
    response: reqwest::Response,
}

impl APIResponse {
    /// Create a new API response from a reqwest response.
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self { response }
    }

    /// Get the HTTP status code.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://api.example.com/users").send().await?;
    /// println!("Status: {}", response.status()); // e.g., 200
    /// # Ok(())
    /// # }
    /// ```
    pub fn status(&self) -> u16 {
        self.response.status().as_u16()
    }

    /// Get the HTTP status code as a `reqwest::StatusCode`.
    pub fn status_code(&self) -> reqwest::StatusCode {
        self.response.status()
    }

    /// Check if the response was successful (status code 2xx).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://api.example.com/users").send().await?;
    /// if response.ok() {
    ///     println!("Request succeeded!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn ok(&self) -> bool {
        self.response.status().is_success()
    }

    /// Get the status text (reason phrase).
    pub fn status_text(&self) -> &str {
        self.response
            .status()
            .canonical_reason()
            .unwrap_or("Unknown")
    }

    /// Get the response headers.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://api.example.com/users").send().await?;
    /// let headers = response.headers();
    /// if let Some(content_type) = headers.get("content-type") {
    ///     println!("Content-Type: {:?}", content_type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn headers(&self) -> &HeaderMap {
        self.response.headers()
    }

    /// Get response headers as a `HashMap`.
    pub fn headers_map(&self) -> HashMap<String, String> {
        self.response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|v| (name.as_str().to_string(), v.to_string()))
            })
            .collect()
    }

    /// Get a specific header value.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.response
            .headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
    }

    /// Get the final URL after any redirects.
    pub fn url(&self) -> &str {
        self.response.url().as_str()
    }

    /// Parse the response body as JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if the response body cannot be parsed as JSON.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use serde::Deserialize;
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    ///
    /// #[derive(Deserialize)]
    /// struct User {
    ///     id: i32,
    ///     name: String,
    /// }
    ///
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://api.example.com/users/1").send().await?;
    /// let user: User = response.json().await?;
    /// println!("User: {} (id={})", user.name, user.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn json<T: DeserializeOwned>(self) -> Result<T, APIError> {
        self.response
            .json()
            .await
            .map_err(|e| APIError::JsonError(e.to_string()))
    }

    /// Get the response body as text.
    ///
    /// # Errors
    ///
    /// Returns an error if the response body cannot be read as text.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://example.com").send().await?;
    /// let html = response.text().await?;
    /// println!("HTML length: {} bytes", html.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn text(self) -> Result<String, APIError> {
        self.response
            .text()
            .await
            .map_err(|e| APIError::ParseError(e.to_string()))
    }

    /// Get the response body as raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the response body cannot be read.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://example.com/image.png").send().await?;
    /// let bytes = response.body().await?;
    /// std::fs::write("image.png", &bytes).expect("Failed to write file");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn body(self) -> Result<Bytes, APIError> {
        self.response
            .bytes()
            .await
            .map_err(|e| APIError::ParseError(e.to_string()))
    }

    /// Get the content length if known.
    pub fn content_length(&self) -> Option<u64> {
        self.response.content_length()
    }

    /// Check if the response indicates a redirect.
    pub fn is_redirect(&self) -> bool {
        self.response.status().is_redirection()
    }

    /// Check if the response indicates a client error (4xx).
    pub fn is_client_error(&self) -> bool {
        self.response.status().is_client_error()
    }

    /// Check if the response indicates a server error (5xx).
    pub fn is_server_error(&self) -> bool {
        self.response.status().is_server_error()
    }
}

#[cfg(test)]
mod tests;
