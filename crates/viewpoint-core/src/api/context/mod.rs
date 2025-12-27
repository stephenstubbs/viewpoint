//! API request context for making HTTP requests.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use reqwest::cookie::Jar;
use tracing::{debug, info};

use super::{APIContextOptions, APIError, APIRequestBuilder, HttpMethod};

/// Context for making API requests.
///
/// `APIRequestContext` can be created standalone or from a browser context.
/// When created from a browser context, cookies are shared between the two.
///
/// # Creating a Standalone Context
///
/// ```no_run
/// use viewpoint_core::api::{APIRequestContext, APIContextOptions};
///
/// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
/// let api = APIRequestContext::new(
///     APIContextOptions::new()
///         .base_url("https://api.example.com")
/// ).await?;
///
/// // Make requests
/// let response = api.get("/users").send().await?;
/// # Ok(())
/// # }
/// ```
///
/// # Creating from Browser Context
///
/// ```ignore
/// use viewpoint_core::{Browser, BrowserContext};
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     let browser = Browser::launch().headless(true).await?;
///     let context = browser.new_context().await?;
///
///     // Get API context that shares cookies with browser
///     let api = context.request().await?;
///
///     // API requests will include browser cookies
///     let response = api.get("https://api.example.com/user").send().await?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct APIRequestContext {
    /// The underlying HTTP client.
    client: Arc<reqwest::Client>,
    /// Cookie jar (shared with browser context if applicable).
    cookie_jar: Arc<Jar>,
    /// Context options.
    options: APIContextOptions,
    /// Whether this context has been disposed.
    disposed: Arc<AtomicBool>,
}

impl APIRequestContext {
    /// Create a new standalone API request context.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for the context
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub async fn new(options: APIContextOptions) -> Result<Self, APIError> {
        info!("Creating standalone APIRequestContext");

        let cookie_jar = Arc::new(Jar::default());
        let client = Self::build_client(&options, Arc::clone(&cookie_jar))?;

        Ok(Self {
            client: Arc::new(client),
            cookie_jar,
            options,
            disposed: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Create an API context with a shared cookie jar.
    ///
    /// This is used internally when creating an API context from a browser context.
    #[allow(dead_code)] // Will be used when context.request() is wired up
    pub(crate) async fn with_shared_cookies(
        options: APIContextOptions,
        cookie_jar: Arc<Jar>,
    ) -> Result<Self, APIError> {
        debug!("Creating APIRequestContext with shared cookie jar");

        let client = Self::build_client(&options, Arc::clone(&cookie_jar))?;

        Ok(Self {
            client: Arc::new(client),
            cookie_jar,
            options,
            disposed: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Build the reqwest client with the given options.
    fn build_client(options: &APIContextOptions, cookie_jar: Arc<Jar>) -> Result<reqwest::Client, APIError> {
        let mut builder = reqwest::Client::builder()
            .cookie_provider(cookie_jar);

        // Set timeout if specified
        if let Some(timeout) = options.timeout {
            builder = builder.timeout(timeout);
        }

        // Set user agent if specified
        if let Some(ref user_agent) = options.user_agent {
            builder = builder.user_agent(user_agent);
        }

        // Handle HTTPS errors
        if options.ignore_https_errors {
            builder = builder.danger_accept_invalid_certs(true);
        }

        // Set up proxy if configured
        if let Some(ref proxy_config) = options.proxy {
            let mut proxy = reqwest::Proxy::all(&proxy_config.server)
                .map_err(|e| APIError::BuildError(format!("Invalid proxy URL: {e}")))?;

            if let (Some(username), Some(password)) = (&proxy_config.username, &proxy_config.password) {
                proxy = proxy.basic_auth(username, password);
            }

            builder = builder.proxy(proxy);
        }

        // Set up HTTP credentials for basic auth
        // Note: reqwest doesn't have built-in preemptive basic auth at client level,
        // so we'll handle this via default headers

        builder.build().map_err(|e| APIError::BuildError(e.to_string()))
    }

    /// Get the default headers including any authentication.
    fn default_headers(&self) -> Vec<(String, String)> {
        let mut headers: Vec<(String, String)> = self
            .options
            .extra_http_headers
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Add basic auth header if credentials are configured
        if let Some(ref creds) = self.options.http_credentials {
            use base64::Engine;
            let auth_string = format!("{}:{}", creds.username, creds.password);
            let encoded = base64::engine::general_purpose::STANDARD.encode(auth_string);
            headers.push(("Authorization".to_string(), format!("Basic {encoded}")));
        }

        headers
    }

    /// Create a GET request builder.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.get("https://api.example.com/users")
    ///     .query(&[("page", "1")])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, url: impl Into<String>) -> APIRequestBuilder {
        self.request(HttpMethod::Get, url)
    }

    /// Create a POST request builder.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.post("https://api.example.com/users")
    ///     .json(&serde_json::json!({"name": "John"}))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn post(&self, url: impl Into<String>) -> APIRequestBuilder {
        self.request(HttpMethod::Post, url)
    }

    /// Create a PUT request builder.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.put("https://api.example.com/users/1")
    ///     .json(&serde_json::json!({"name": "John Updated"}))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn put(&self, url: impl Into<String>) -> APIRequestBuilder {
        self.request(HttpMethod::Put, url)
    }

    /// Create a PATCH request builder.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.patch("https://api.example.com/users/1")
    ///     .json(&serde_json::json!({"status": "active"}))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn patch(&self, url: impl Into<String>) -> APIRequestBuilder {
        self.request(HttpMethod::Patch, url)
    }

    /// Create a DELETE request builder.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.delete("https://api.example.com/users/1")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, url: impl Into<String>) -> APIRequestBuilder {
        self.request(HttpMethod::Delete, url)
    }

    /// Create a HEAD request builder.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.head("https://api.example.com/users")
    ///     .send()
    ///     .await?;
    /// println!("Content-Length: {:?}", response.header("content-length"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn head(&self, url: impl Into<String>) -> APIRequestBuilder {
        self.request(HttpMethod::Head, url)
    }

    /// Create a request builder with a specific HTTP method.
    ///
    /// This is the underlying method used by `get()`, `post()`, etc.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions, HttpMethod};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// # let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// let response = api.fetch(HttpMethod::Get, "https://api.example.com/users")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fetch(&self, method: HttpMethod, url: impl Into<String>) -> APIRequestBuilder {
        self.request(method, url)
    }

    /// Internal method to create a request builder.
    fn request(&self, method: HttpMethod, url: impl Into<String>) -> APIRequestBuilder {
        let mut builder = APIRequestBuilder::new(
            Arc::clone(&self.client),
            method,
            url,
            self.options.base_url.clone(),
            self.default_headers(),
        );

        // Mark as disposed if the context is disposed
        if self.disposed.load(Ordering::SeqCst) {
            builder.set_disposed();
        }

        builder
    }

    /// Dispose of this API context, releasing resources.
    ///
    /// After calling this method, any new requests will fail with `APIError::Disposed`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use viewpoint_core::api::{APIRequestContext, APIContextOptions};
    /// # async fn example() -> Result<(), viewpoint_core::api::APIError> {
    /// let api = APIRequestContext::new(APIContextOptions::new()).await?;
    /// // ... use the API ...
    /// api.dispose().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn dispose(&self) {
        info!("Disposing APIRequestContext");
        self.disposed.store(true, Ordering::SeqCst);
    }

    /// Check if this context has been disposed.
    pub fn is_disposed(&self) -> bool {
        self.disposed.load(Ordering::SeqCst)
    }

    /// Get the base URL for this context.
    pub fn base_url(&self) -> Option<&str> {
        self.options.base_url.as_deref()
    }

    /// Get access to the cookie jar.
    ///
    /// This can be used to inspect or manually add cookies.
    pub fn cookie_jar(&self) -> &Arc<Jar> {
        &self.cookie_jar
    }
}

impl Clone for APIRequestContext {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            cookie_jar: Arc::clone(&self.cookie_jar),
            options: self.options.clone(),
            disposed: Arc::clone(&self.disposed),
        }
    }
}

#[cfg(test)]
mod tests;
