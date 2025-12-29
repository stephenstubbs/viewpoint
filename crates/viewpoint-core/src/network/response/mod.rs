//! Network response types.

use std::collections::HashMap;
use std::sync::Arc;

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::network::GetResponseBodyParams;

use super::request::Request;
use crate::error::NetworkError;

/// A network response.
///
/// This type represents an HTTP response and provides access to response details
/// such as status, headers, and body.
#[derive(Debug, Clone)]
pub struct Response {
    /// Response URL.
    url: String,
    /// HTTP status code.
    status: u16,
    /// HTTP status text.
    status_text: String,
    /// Response headers.
    headers: HashMap<String, String>,
    /// MIME type.
    mime_type: String,
    /// Whether response was served from cache.
    from_cache: bool,
    /// Whether response was served from service worker.
    from_service_worker: bool,
    /// Associated request.
    request: Request,
    /// CDP connection for fetching body.
    connection: Arc<CdpConnection>,
    /// Session ID for CDP commands.
    session_id: String,
    /// Network request ID for fetching body.
    request_id: String,
    /// Security details (for HTTPS).
    security_details: Option<SecurityDetails>,
    /// Remote server address.
    remote_address: Option<RemoteAddress>,
}

impl Response {
    /// Create a new response from CDP response data.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        cdp_response: viewpoint_cdp::protocol::network::Response,
        request: Request,
        connection: Arc<CdpConnection>,
        session_id: String,
        request_id: String,
    ) -> Self {
        let remote_address = cdp_response.remote_ip_address.map(|ip| RemoteAddress {
            ip_address: ip,
            port: cdp_response.remote_port.unwrap_or(0) as u16,
        });

        // Convert security details from CDP type
        let security_details = cdp_response.security_details.map(SecurityDetails::from);

        Self {
            url: cdp_response.url,
            status: cdp_response.status as u16,
            status_text: cdp_response.status_text,
            headers: cdp_response.headers,
            mime_type: cdp_response.mime_type,
            from_cache: cdp_response.from_disk_cache.unwrap_or(false),
            from_service_worker: cdp_response.from_service_worker.unwrap_or(false),
            request,
            connection,
            session_id,
            request_id,
            security_details,
            remote_address,
        }
    }

    /// Get the response URL.
    ///
    /// This may differ from the request URL in case of redirects.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the HTTP status code.
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Get the HTTP status text.
    pub fn status_text(&self) -> &str {
        &self.status_text
    }

    /// Check if the response was successful (status 200-299).
    pub fn ok(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Get the response headers.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get a header value by name (case-insensitive).
    pub fn header_value(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// Get all headers asynchronously.
    ///
    /// This may fetch additional headers that weren't available synchronously.
    pub async fn all_headers(&self) -> HashMap<String, String> {
        // For now, just return the cached headers
        self.headers.clone()
    }

    /// Get the MIME type.
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    /// Check if the response was served from cache.
    pub fn from_cache(&self) -> bool {
        self.from_cache
    }

    /// Check if the response was served from a service worker.
    pub fn from_service_worker(&self) -> bool {
        self.from_service_worker
    }

    /// Get the associated request.
    pub fn request(&self) -> &Request {
        &self.request
    }

    /// Get the response body as bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the body cannot be fetched.
    pub async fn body(&self) -> Result<Vec<u8>, NetworkError> {
        let result: viewpoint_cdp::protocol::network::GetResponseBodyResult = self
            .connection
            .send_command(
                "Network.getResponseBody",
                Some(GetResponseBodyParams {
                    request_id: self.request_id.clone(),
                }),
                Some(&self.session_id),
            )
            .await
            .map_err(NetworkError::from)?;

        if result.base64_encoded {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .decode(&result.body)
                .map_err(|e| NetworkError::InvalidResponse(format!("Failed to decode base64: {e}")))
        } else {
            Ok(result.body.into_bytes())
        }
    }

    /// Get the response body as text.
    ///
    /// # Errors
    ///
    /// Returns an error if the body cannot be fetched or is not valid UTF-8.
    pub async fn text(&self) -> Result<String, NetworkError> {
        let body = self.body().await?;
        String::from_utf8(body)
            .map_err(|e| NetworkError::InvalidResponse(format!("Response is not valid UTF-8: {e}")))
    }

    /// Parse the response body as JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if the body cannot be fetched or is not valid JSON.
    pub async fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, NetworkError> {
        let text = self.text().await?;
        serde_json::from_str(&text)
            .map_err(|e| NetworkError::InvalidResponse(format!("Failed to parse JSON: {e}")))
    }

    /// Get security details for HTTPS responses.
    pub fn security_details(&self) -> Option<&SecurityDetails> {
        self.security_details.as_ref()
    }

    /// Get the remote server address.
    pub fn server_addr(&self) -> Option<&RemoteAddress> {
        self.remote_address.as_ref()
    }

    /// Wait for the response body to be fully received.
    pub async fn finished(&self) -> Result<(), NetworkError> {
        // For responses that are already complete, this is a no-op
        // For streaming responses, we would need to wait for loadingFinished event
        Ok(())
    }
}

/// Security details for HTTPS responses.
#[derive(Debug, Clone)]
pub struct SecurityDetails {
    /// TLS protocol name (e.g., "TLS 1.3").
    pub protocol: String,
    /// Certificate subject name.
    pub subject_name: String,
    /// Certificate issuer.
    pub issuer: String,
    /// Certificate valid from (Unix timestamp).
    pub valid_from: f64,
    /// Certificate valid to (Unix timestamp).
    pub valid_to: f64,
    /// Subject Alternative Names.
    pub san_list: Vec<String>,
}

impl From<viewpoint_cdp::protocol::network::SecurityDetails> for SecurityDetails {
    fn from(details: viewpoint_cdp::protocol::network::SecurityDetails) -> Self {
        Self {
            protocol: details.protocol,
            subject_name: details.subject_name,
            issuer: details.issuer,
            valid_from: details.valid_from,
            valid_to: details.valid_to,
            san_list: details.san_list,
        }
    }
}

/// Remote server address.
#[derive(Debug, Clone)]
pub struct RemoteAddress {
    /// IP address.
    pub ip_address: String,
    /// Port number.
    pub port: u16,
}

#[cfg(test)]
mod tests;
