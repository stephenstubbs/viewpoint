//! Fetch domain types.
//!
//! The Fetch domain allows intercepting network requests, modifying them,
//! and providing custom responses. It's the primary mechanism for request
//! routing and mocking in browser automation.

use serde::{Deserialize, Serialize};

use super::network::{Request, ResourceType};

/// Unique request identifier for the Fetch domain.
pub type RequestId = String;

/// Response HTTP header entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderEntry {
    /// Header name.
    pub name: String,
    /// Header value.
    pub value: String,
}

/// Stage at which to begin intercepting requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum RequestStage {
    /// Intercept before the request is sent.
    #[default]
    Request,
    /// Intercept after the response is received (but before response body is received).
    Response,
}


/// Request pattern for interception.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern {
    /// Wildcards ('*' -> zero or more, '?' -> exactly one) are allowed.
    /// Escape character is backslash. Omitting is equivalent to "*".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_pattern: Option<String>,

    /// If set, only requests for matching resource types will be intercepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceType>,

    /// Stage at which to begin intercepting requests. Default is Request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_stage: Option<RequestStage>,
}

impl RequestPattern {
    /// Create a new request pattern matching all URLs.
    pub fn all() -> Self {
        Self::default()
    }

    /// Create a new request pattern matching the specified URL pattern.
    pub fn url(pattern: impl Into<String>) -> Self {
        Self {
            url_pattern: Some(pattern.into()),
            ..Default::default()
        }
    }

    /// Set the resource type filter.
    #[must_use]
    pub fn with_resource_type(mut self, resource_type: ResourceType) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Set the request stage.
    #[must_use]
    pub fn with_stage(mut self, stage: RequestStage) -> Self {
        self.request_stage = Some(stage);
        self
    }
}

// =============================================================================
// Commands
// =============================================================================

/// Parameters for Fetch.enable.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// If specified, only requests matching any of these patterns will produce
    /// fetchRequested event and will be paused until client's response.
    /// If not set, all requests will be affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<RequestPattern>>,

    /// If true, authRequired events will be issued and requests will be paused
    /// expecting a call to continueWithAuth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle_auth_requests: Option<bool>,
}

/// Parameters for Fetch.disable.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DisableParams {}

/// Parameters for Fetch.continueRequest.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueRequestParams {
    /// An id the client received in requestPaused event.
    pub request_id: RequestId,

    /// If set, the request url will be modified in a way that's not observable by page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// If set, the request method is overridden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// If set, overrides the post data in the request.
    /// (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_data: Option<String>,

    /// If set, overrides the request headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HeaderEntry>>,

    /// If set, overrides response interception behavior for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercept_response: Option<bool>,
}

/// Parameters for Fetch.fulfillRequest.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FulfillRequestParams {
    /// An id the client received in requestPaused event.
    pub request_id: RequestId,

    /// An HTTP response code.
    pub response_code: i32,

    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<HeaderEntry>>,

    /// Alternative way of specifying response headers as a \0-separated
    /// series of name: value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_response_headers: Option<String>,

    /// A response body. If absent, original response body will be used if
    /// the request is intercepted at the response stage and empty body
    /// will be used if the request is intercepted at the request stage.
    /// (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// A textual representation of responseCode.
    /// If absent, a standard phrase matching responseCode is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_phrase: Option<String>,
}

/// Parameters for Fetch.failRequest.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FailRequestParams {
    /// An id the client received in requestPaused event.
    pub request_id: RequestId,

    /// Causes the request to fail with the given reason.
    pub error_reason: ErrorReason,
}

/// Parameters for Fetch.getResponseBody.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyParams {
    /// Identifier for the intercepted request to get body for.
    pub request_id: RequestId,
}

/// Result for Fetch.getResponseBody.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyResult {
    /// Response body.
    pub body: String,

    /// True, if content was sent as base64.
    pub base64_encoded: bool,
}

/// Parameters for Fetch.continueWithAuth.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueWithAuthParams {
    /// An id the client received in authRequired event.
    pub request_id: RequestId,

    /// Response to with an authChallenge.
    pub auth_challenge_response: AuthChallengeResponse,
}

/// Parameters for Fetch.continueResponse (experimental).
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponseParams {
    /// An id the client received in requestPaused event.
    pub request_id: RequestId,

    /// An HTTP response code. If absent, original response code will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,

    /// A textual representation of responseCode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_phrase: Option<String>,

    /// Response headers. If absent, original response headers will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<HeaderEntry>>,

    /// Alternative way of specifying response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_response_headers: Option<String>,
}

// =============================================================================
// Events
// =============================================================================

/// Event: Fetch.requestPaused
///
/// Issued when the domain is enabled and the request URL matches the
/// specified filter. The request is paused until the client responds
/// with one of continueRequest, failRequest or fulfillRequest.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPausedEvent {
    /// Each request the page makes will have a unique id.
    pub request_id: RequestId,

    /// The details of the request.
    pub request: Request,

    /// The id of the frame that initiated the request.
    pub frame_id: String,

    /// How the requested resource will be used.
    pub resource_type: ResourceType,

    /// Response error if intercepted at response stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_error_reason: Option<ErrorReason>,

    /// Response code if intercepted at response stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status_code: Option<i32>,

    /// Response status text if intercepted at response stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status_text: Option<String>,

    /// Response headers if intercepted at the response stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<HeaderEntry>>,

    /// If the intercepted request had a corresponding Network.requestWillBeSent event,
    /// then this networkId will be the same as the requestId in that event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,

    /// If the request is due to a redirect response from the server,
    /// the id of the request that has caused the redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirected_request_id: Option<RequestId>,
}

impl RequestPausedEvent {
    /// Check if this event is at the response stage.
    pub fn is_response_stage(&self) -> bool {
        self.response_error_reason.is_some() || self.response_status_code.is_some()
    }

    /// Check if this event is at the request stage.
    pub fn is_request_stage(&self) -> bool {
        !self.is_response_stage()
    }

    /// Check if this is a redirect response.
    pub fn is_redirect(&self) -> bool {
        if let Some(code) = self.response_status_code {
            matches!(code, 301 | 302 | 303 | 307 | 308)
                && self.response_headers.as_ref().is_some_and(|headers| {
                    headers.iter().any(|h| h.name.eq_ignore_ascii_case("location"))
                })
        } else {
            false
        }
    }
}

/// Event: Fetch.authRequired
///
/// Issued when the domain is enabled with handleAuthRequests set to true.
/// The request is paused until client responds with continueWithAuth.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequiredEvent {
    /// Each request the page makes will have a unique id.
    pub request_id: RequestId,

    /// The details of the request.
    pub request: Request,

    /// The id of the frame that initiated the request.
    pub frame_id: String,

    /// How the requested resource will be used.
    pub resource_type: ResourceType,

    /// Details of the Authorization Challenge encountered.
    pub auth_challenge: AuthChallenge,
}

// =============================================================================
// Types
// =============================================================================

/// Network level fetch failure reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ErrorReason {
    /// Generic failure.
    #[default]
    Failed,
    /// Request was aborted.
    Aborted,
    /// Request timed out.
    TimedOut,
    /// Access was denied.
    AccessDenied,
    /// Connection was closed.
    ConnectionClosed,
    /// Connection was reset.
    ConnectionReset,
    /// Connection was refused.
    ConnectionRefused,
    /// Connection was aborted.
    ConnectionAborted,
    /// Connection failed.
    ConnectionFailed,
    /// Name could not be resolved.
    NameNotResolved,
    /// Internet is disconnected.
    InternetDisconnected,
    /// Address is unreachable.
    AddressUnreachable,
    /// Blocked by client.
    BlockedByClient,
    /// Blocked by response.
    BlockedByResponse,
}


impl ErrorReason {
    /// Get the CDP string representation of this error reason.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Failed => "Failed",
            Self::Aborted => "Aborted",
            Self::TimedOut => "TimedOut",
            Self::AccessDenied => "AccessDenied",
            Self::ConnectionClosed => "ConnectionClosed",
            Self::ConnectionReset => "ConnectionReset",
            Self::ConnectionRefused => "ConnectionRefused",
            Self::ConnectionAborted => "ConnectionAborted",
            Self::ConnectionFailed => "ConnectionFailed",
            Self::NameNotResolved => "NameNotResolved",
            Self::InternetDisconnected => "InternetDisconnected",
            Self::AddressUnreachable => "AddressUnreachable",
            Self::BlockedByClient => "BlockedByClient",
            Self::BlockedByResponse => "BlockedByResponse",
        }
    }
}

/// Authorization challenge for HTTP status code 401 or 407.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallenge {
    /// Source of the authentication challenge.
    pub source: AuthChallengeSource,

    /// Origin of the challenger.
    pub origin: String,

    /// The authentication scheme used, such as basic or digest.
    pub scheme: String,

    /// The realm of the challenge. May be empty.
    pub realm: String,
}

/// Source of the authentication challenge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthChallengeSource {
    /// Server authentication.
    Server,
    /// Proxy authentication.
    Proxy,
}

/// Response to an `AuthChallenge`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallengeResponse {
    /// The decision on what to do in response to the authorization challenge.
    pub response: AuthChallengeResponseType,

    /// The username to provide, possibly empty.
    /// Should only be set if response is `ProvideCredentials`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// The password to provide, possibly empty.
    /// Should only be set if response is `ProvideCredentials`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// The decision on what to do in response to the authorization challenge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthChallengeResponseType {
    /// Defer to the default behavior of the net stack.
    Default,
    /// Cancel the authentication.
    CancelAuth,
    /// Provide credentials.
    ProvideCredentials,
}

impl AuthChallengeResponse {
    /// Create a default response (defer to browser).
    pub fn default_response() -> Self {
        Self {
            response: AuthChallengeResponseType::Default,
            username: None,
            password: None,
        }
    }

    /// Create a cancel response.
    pub fn cancel() -> Self {
        Self {
            response: AuthChallengeResponseType::CancelAuth,
            username: None,
            password: None,
        }
    }

    /// Create a response providing credentials.
    pub fn provide_credentials(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            response: AuthChallengeResponseType::ProvideCredentials,
            username: Some(username.into()),
            password: Some(password.into()),
        }
    }
}

// Unit tests moved to tests/integration_tests.rs
