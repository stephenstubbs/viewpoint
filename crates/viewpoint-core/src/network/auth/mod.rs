//! HTTP authentication handling.
//!
//! This module provides support for handling HTTP Basic and Digest authentication
//! challenges via the Fetch.authRequired CDP event.

use std::sync::Arc;

use tokio::sync::RwLock;
use viewpoint_cdp::protocol::fetch::{
    AuthChallenge, AuthChallengeResponse, AuthRequiredEvent,
    ContinueWithAuthParams,
};
use viewpoint_cdp::CdpConnection;

use crate::error::NetworkError;

/// HTTP credentials for authentication.
#[derive(Debug, Clone)]
pub struct HttpCredentials {
    /// Username for authentication.
    pub username: String,
    /// Password for authentication.
    pub password: String,
    /// Optional origin to restrict credentials to.
    /// If None, credentials apply to all origins.
    pub origin: Option<String>,
}

impl HttpCredentials {
    /// Create new HTTP credentials.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            origin: None,
        }
    }

    /// Create HTTP credentials restricted to a specific origin.
    pub fn for_origin(
        username: impl Into<String>,
        password: impl Into<String>,
        origin: impl Into<String>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            origin: Some(origin.into()),
        }
    }

    /// Check if these credentials apply to the given challenge origin.
    pub fn matches_origin(&self, challenge_origin: &str) -> bool {
        match &self.origin {
            Some(origin) => {
                // Match if origin matches exactly or is a subdomain
                challenge_origin == origin || challenge_origin.ends_with(&format!(".{origin}"))
            }
            None => true, // No origin restriction - apply to all
        }
    }
}

/// Handler for HTTP authentication challenges.
#[derive(Debug)]
pub struct AuthHandler {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID for CDP commands.
    session_id: String,
    /// Stored credentials.
    credentials: RwLock<Option<HttpCredentials>>,
    /// How many times to retry with credentials before canceling.
    max_retries: u32,
    /// Current retry count per origin.
    retry_counts: RwLock<std::collections::HashMap<String, u32>>,
}

impl AuthHandler {
    /// Create a new auth handler.
    pub fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        Self {
            connection,
            session_id,
            credentials: RwLock::new(None),
            max_retries: 3,
            retry_counts: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Create an auth handler with pre-configured credentials.
    pub fn with_credentials(
        connection: Arc<CdpConnection>,
        session_id: String,
        credentials: HttpCredentials,
    ) -> Self {
        Self {
            connection,
            session_id,
            credentials: RwLock::new(Some(credentials)),
            max_retries: 3,
            retry_counts: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Set HTTP credentials.
    pub async fn set_credentials(&self, credentials: HttpCredentials) {
        let mut creds = self.credentials.write().await;
        *creds = Some(credentials);
    }
    
    /// Set HTTP credentials synchronously (for use during construction).
    /// 
    /// This uses `blocking_write` which should only be called from non-async contexts.
    pub fn set_credentials_sync(&self, credentials: HttpCredentials) {
        // Use try_write to avoid blocking - this is called during construction
        // before any async tasks are running, so it should always succeed.
        if let Ok(mut creds) = self.credentials.try_write() {
            *creds = Some(credentials);
        }
    }

    /// Clear HTTP credentials.
    pub async fn clear_credentials(&self) {
        let mut creds = self.credentials.write().await;
        *creds = None;
    }

    /// Handle an authentication challenge.
    ///
    /// Returns true if the challenge was handled, false if no credentials available.
    ///
    /// # Errors
    ///
    /// Returns an error if the CDP command to continue with authentication fails,
    /// such as when the connection is closed or the browser rejects the request.
    pub async fn handle_auth_challenge(
        &self,
        event: &AuthRequiredEvent,
    ) -> Result<bool, NetworkError> {
        let creds = self.credentials.read().await;

        if let Some(credentials) = &*creds {
            // Check if credentials match the challenge origin
            if !credentials.matches_origin(&event.auth_challenge.origin) {
                tracing::debug!(
                    origin = %event.auth_challenge.origin,
                    "No matching credentials for origin"
                );
                return self.cancel_auth(&event.request_id).await.map(|()| false);
            }

            // Check retry count
            {
                let mut counts = self.retry_counts.write().await;
                let count = counts.entry(event.auth_challenge.origin.clone()).or_insert(0);
                
                if *count >= self.max_retries {
                    tracing::warn!(
                        origin = %event.auth_challenge.origin,
                        retries = self.max_retries,
                        "Max auth retries exceeded, canceling"
                    );
                    return self.cancel_auth(&event.request_id).await.map(|()| false);
                }
                
                *count += 1;
            }

            // Provide credentials based on the authentication scheme
            self.provide_credentials(
                &event.request_id,
                &event.auth_challenge,
                &credentials.username,
                &credentials.password,
            )
            .await?;

            Ok(true)
        } else {
            tracing::debug!(
                origin = %event.auth_challenge.origin,
                scheme = %event.auth_challenge.scheme,
                "No credentials available, deferring to default"
            );
            // No credentials - let browser handle it (show dialog or fail)
            self.default_auth(&event.request_id).await?;
            Ok(false)
        }
    }

    /// Provide credentials for an auth challenge.
    async fn provide_credentials(
        &self,
        request_id: &str,
        challenge: &AuthChallenge,
        username: &str,
        password: &str,
    ) -> Result<(), NetworkError> {
        tracing::debug!(
            origin = %challenge.origin,
            scheme = %challenge.scheme,
            realm = %challenge.realm,
            "Providing credentials for auth challenge"
        );

        self.connection
            .send_command::<_, serde_json::Value>(
                "Fetch.continueWithAuth",
                Some(ContinueWithAuthParams {
                    request_id: request_id.to_string(),
                    auth_challenge_response: AuthChallengeResponse::provide_credentials(
                        username,
                        password,
                    ),
                }),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Cancel authentication.
    async fn cancel_auth(&self, request_id: &str) -> Result<(), NetworkError> {
        tracing::debug!("Canceling auth challenge");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Fetch.continueWithAuth",
                Some(ContinueWithAuthParams {
                    request_id: request_id.to_string(),
                    auth_challenge_response: AuthChallengeResponse::cancel(),
                }),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Use default browser behavior for auth.
    async fn default_auth(&self, request_id: &str) -> Result<(), NetworkError> {
        self.connection
            .send_command::<_, serde_json::Value>(
                "Fetch.continueWithAuth",
                Some(ContinueWithAuthParams {
                    request_id: request_id.to_string(),
                    auth_challenge_response: AuthChallengeResponse::default_response(),
                }),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Reset retry counts (call after successful auth).
    pub async fn reset_retries(&self, origin: &str) {
        let mut counts = self.retry_counts.write().await;
        counts.remove(origin);
    }

    /// Reset all retry counts.
    pub async fn reset_all_retries(&self) {
        let mut counts = self.retry_counts.write().await;
        counts.clear();
    }
}

#[cfg(test)]
mod tests;
