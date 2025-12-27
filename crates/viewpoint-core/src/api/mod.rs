//! API testing support for making HTTP requests outside of browser context.
//!
//! This module provides `APIRequestContext` for making HTTP requests directly,
//! without needing a browser. This is useful for:
//! - Setting up test data via API
//! - Verifying backend state after UI actions
//! - Getting authentication tokens
//! - Running API-only tests without browser overhead
//!
//! # Example
//!
//! ```no_run
//! use viewpoint_core::api::{APIRequestContext, APIContextOptions};
//!
//! # async fn example() -> Result<(), viewpoint_core::api::APIError> {
//! // Create a standalone API context
//! let api = APIRequestContext::new(
//!     APIContextOptions::new()
//!         .base_url("https://api.example.com")
//! ).await?;
//!
//! // Make a GET request
//! let response = api.get("/users").send().await?;
//! assert!(response.ok());
//!
//! // Make a POST request with JSON body
//! let user = serde_json::json!({ "name": "John", "email": "john@example.com" });
//! let response = api.post("/users")
//!     .json(&user)
//!     .send()
//!     .await?;
//!
//! // Parse JSON response
//! let created_user: serde_json::Value = response.json().await?;
//! # Ok(())
//! # }
//! ```

mod context;
pub mod cookies;
mod options;
mod request;
mod response;

pub use context::APIRequestContext;
pub use options::{APIContextOptions, CredentialSend, HttpCredentials, ProxyConfig};
pub use request::{APIRequestBuilder, MultipartField};
pub use response::APIResponse;

use std::time::Duration;
use thiserror::Error;

/// Errors that can occur during API operations.
#[derive(Error, Debug)]
pub enum APIError {
    /// HTTP request failed.
    #[error("request failed: {0}")]
    RequestFailed(String),

    /// Request timed out.
    #[error("request timeout after {0:?}")]
    Timeout(Duration),

    /// Failed to build request.
    #[error("failed to build request: {0}")]
    BuildError(String),

    /// Failed to parse response body.
    #[error("failed to parse response: {0}")]
    ParseError(String),

    /// Invalid URL.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    JsonError(String),

    /// Context is disposed.
    #[error("API context is disposed")]
    Disposed,

    /// HTTP client error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

/// HTTP method for API requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    /// GET method.
    Get,
    /// POST method.
    Post,
    /// PUT method.
    Put,
    /// PATCH method.
    Patch,
    /// DELETE method.
    Delete,
    /// HEAD method.
    Head,
}

impl HttpMethod {
    /// Convert to reqwest Method.
    pub fn to_reqwest(&self) -> reqwest::Method {
        match self {
            Self::Get => reqwest::Method::GET,
            Self::Post => reqwest::Method::POST,
            Self::Put => reqwest::Method::PUT,
            Self::Patch => reqwest::Method::PATCH,
            Self::Delete => reqwest::Method::DELETE,
            Self::Head => reqwest::Method::HEAD,
        }
    }
}

#[cfg(test)]
mod tests;
