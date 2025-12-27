//! Route handling for network interception.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use viewpoint_cdp::protocol::fetch::{
    ContinueRequestParams, ErrorReason, FailRequestParams, FulfillRequestParams,
    GetResponseBodyParams, GetResponseBodyResult, HeaderEntry,
};
use viewpoint_cdp::CdpConnection;

use super::request::Request;
use super::route_builders::{ContinueBuilder, FulfillBuilder};
use super::route_fetch::{FetchBuilder, FetchedResponse};
use super::types::AbortError;
use crate::error::NetworkError;

/// The result of a route handler action.
///
/// Route handlers return this to indicate whether they handled the request
/// or want to pass it to the next matching handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteAction {
    /// The request was handled (fulfilled, continued, or aborted).
    Handled,
    /// Pass the request to the next matching handler.
    Fallback,
}

/// A route handler function.
pub type RouteHandler = Box<
    dyn Fn(Route) -> Pin<Box<dyn Future<Output = Result<(), NetworkError>> + Send>>
        + Send
        + Sync,
>;

/// An intercepted network request that can be fulfilled, continued, or aborted.
///
/// When a request matches a route pattern, a `Route` is passed to the handler.
/// The handler must call one of `fulfill()`, `continue_()`, `abort()`, or `fallback()`
/// to resolve the request.
#[derive(Debug, Clone)]
pub struct Route {
    /// The intercepted request.
    request: Request,
    /// CDP request ID for this route.
    request_id: String,
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Whether this route has been handled.
    handled: Arc<Mutex<bool>>,
    /// Response status code (if intercepted at response stage).
    response_status: Option<u16>,
    /// Response headers (if intercepted at response stage).
    response_headers: Option<Vec<HeaderEntry>>,
}

impl Route {
    /// Create a new route.
    pub(crate) fn new(
        request: Request,
        request_id: String,
        connection: Arc<CdpConnection>,
        session_id: String,
        response_status: Option<u16>,
        response_headers: Option<Vec<HeaderEntry>>,
    ) -> Self {
        Self {
            request,
            request_id,
            connection,
            session_id,
            handled: Arc::new(Mutex::new(false)),
            response_status,
            response_headers,
        }
    }

    /// Get the intercepted request.
    pub fn request(&self) -> &Request {
        &self.request
    }

    /// Get the request ID.
    pub(super) fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Get the CDP connection.
    pub(super) fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }

    /// Get the session ID.
    pub(super) fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Check if this route has been handled.
    pub async fn is_handled(&self) -> bool {
        *self.handled.lock().await
    }

    /// Check if this route is at the response stage.
    pub fn is_response_stage(&self) -> bool {
        self.response_status.is_some()
    }

    /// Get the response status code (if at response stage).
    pub fn response_status(&self) -> Option<u16> {
        self.response_status
    }

    /// Get the response headers (if at response stage).
    pub fn response_headers(&self) -> Option<&[HeaderEntry]> {
        self.response_headers.as_deref()
    }

    /// Get the response body (if at response stage).
    ///
    /// # Errors
    ///
    /// Returns an error if not at response stage or body cannot be fetched.
    pub async fn response_body(&self) -> Result<Option<Vec<u8>>, NetworkError> {
        if !self.is_response_stage() {
            return Ok(None);
        }
        self.get_response_body(&self.request_id).await
    }

    /// Fulfill the request with a custom response.
    ///
    /// # Example
    ///
    /// ```ignore
    /// route.fulfill()
    ///     .status(200)
    ///     .content_type("application/json")
    ///     .body(r#"{"success": true}"#)
    ///     .send()
    ///     .await?;
    /// ```
    pub fn fulfill(&self) -> FulfillBuilder<'_> {
        FulfillBuilder::new(self)
    }

    /// Continue the request with optional modifications.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Continue unchanged
    /// route.continue_().await?;
    ///
    /// // Modify the request
    /// route.continue_()
    ///     .header("X-Custom", "value")
    ///     .send()
    ///     .await?;
    /// ```
    pub fn continue_(&self) -> ContinueBuilder<'_> {
        ContinueBuilder::new(self)
    }

    /// Abort the request with a generic error.
    ///
    /// # Errors
    ///
    /// Returns an error if the abort fails.
    pub async fn abort(&self) -> Result<(), NetworkError> {
        self.abort_with(AbortError::Failed).await
    }

    /// Abort the request with a specific error.
    ///
    /// # Errors
    ///
    /// Returns an error if the abort fails.
    pub async fn abort_with(&self, error: AbortError) -> Result<(), NetworkError> {
        // Mark as handled
        {
            let mut handled = self.handled.lock().await;
            if *handled {
                return Err(NetworkError::AlreadyHandled);
            }
            *handled = true;
        }

        let error_reason = match error {
            AbortError::Aborted => ErrorReason::Aborted,
            AbortError::AccessDenied => ErrorReason::AccessDenied,
            AbortError::AddressUnreachable => ErrorReason::AddressUnreachable,
            AbortError::BlockedByClient => ErrorReason::BlockedByClient,
            AbortError::BlockedByResponse => ErrorReason::BlockedByResponse,
            AbortError::ConnectionAborted => ErrorReason::ConnectionAborted,
            AbortError::ConnectionClosed => ErrorReason::ConnectionClosed,
            AbortError::ConnectionFailed => ErrorReason::ConnectionFailed,
            AbortError::ConnectionRefused => ErrorReason::ConnectionRefused,
            AbortError::ConnectionReset => ErrorReason::ConnectionReset,
            AbortError::InternetDisconnected => ErrorReason::InternetDisconnected,
            AbortError::NameNotResolved => ErrorReason::NameNotResolved,
            AbortError::TimedOut => ErrorReason::TimedOut,
            AbortError::Failed => ErrorReason::Failed,
        };

        let params = FailRequestParams {
            request_id: self.request_id.clone(),
            error_reason,
        };

        self.connection
            .send_command::<_, serde_json::Value>("Fetch.failRequest", Some(params), Some(&self.session_id))
            .await
            .map_err(NetworkError::from)?;

        Ok(())
    }

    /// Pass this request to the next matching route handler.
    ///
    /// If no other handlers match, the request continues to the server.
    ///
    /// # Errors
    ///
    /// Returns an error if the fallback fails.
    pub async fn fallback(&self) -> Result<(), NetworkError> {
        // For fallback, we just continue the request unchanged
        // The routing system will check for other matching handlers
        let params = ContinueRequestParams {
            request_id: self.request_id.clone(),
            url: None,
            method: None,
            post_data: None,
            headers: None,
            intercept_response: None,
        };

        self.connection
            .send_command::<_, serde_json::Value>("Fetch.continueRequest", Some(params), Some(&self.session_id))
            .await
            .map_err(NetworkError::from)?;

        Ok(())
    }

    /// Fetch the actual response from the server, allowing inspection/modification.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = route.fetch().await?;
    /// println!("Status: {}", response.status);
    ///
    /// // Modify and send to page
    /// route.fulfill()
    ///     .response(&response)
    ///     .header("X-Modified", "true")
    ///     .send()
    ///     .await?;
    /// ```
    pub fn fetch(&self) -> FetchBuilder<'_> {
        FetchBuilder::new(self)
    }

    /// Fetch the response with a timeout.
    pub async fn fetch_with_timeout(&self, timeout: Duration) -> Result<FetchedResponse<'_>, NetworkError> {
        self.fetch().timeout(timeout).send().await
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    /// Send a fulfill request.
    pub(super) async fn send_fulfill(&self, params: FulfillRequestParams) -> Result<(), NetworkError> {
        // Mark as handled
        {
            let mut handled = self.handled.lock().await;
            if *handled {
                return Err(NetworkError::AlreadyHandled);
            }
            *handled = true;
        }

        self.connection
            .send_command::<_, serde_json::Value>("Fetch.fulfillRequest", Some(params), Some(&self.session_id))
            .await
            .map_err(NetworkError::from)?;

        Ok(())
    }

    /// Send a continue request.
    pub(super) async fn send_continue(&self, params: ContinueRequestParams) -> Result<(), NetworkError> {
        // Mark as handled
        {
            let mut handled = self.handled.lock().await;
            if *handled {
                return Err(NetworkError::AlreadyHandled);
            }
            *handled = true;
        }

        self.connection
            .send_command::<_, serde_json::Value>("Fetch.continueRequest", Some(params), Some(&self.session_id))
            .await
            .map_err(NetworkError::from)?;

        Ok(())
    }

    /// Get the response body for a request.
    pub(super) async fn get_response_body(&self, request_id: &str) -> Result<Option<Vec<u8>>, NetworkError> {
        use base64::Engine;

        let result: GetResponseBodyResult = self
            .connection
            .send_command(
                "Fetch.getResponseBody",
                Some(GetResponseBodyParams {
                    request_id: request_id.to_string(),
                }),
                Some(&self.session_id),
            )
            .await
            .map_err(NetworkError::from)?;

        let body = if result.base64_encoded {
            base64::engine::general_purpose::STANDARD
                .decode(&result.body)
                .ok()
        } else {
            Some(result.body.into_bytes())
        };

        Ok(body)
    }
}

#[cfg(test)]
mod tests;
