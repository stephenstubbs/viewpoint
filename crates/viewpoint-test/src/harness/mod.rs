//! Test harness for browser automation tests.

use tracing::{debug, info, instrument, warn};

use crate::config::TestConfig;
use crate::error::TestError;
use viewpoint_core::{Browser, BrowserContext, Page};

/// Test harness that manages browser, context, and page lifecycle.
///
/// The harness provides access to browser automation fixtures and handles
/// cleanup automatically via `Drop`. It supports different scoping levels
/// to balance test isolation against performance.
///
/// # Scoping Levels
///
/// - `TestHarness::new()` - Test-scoped: new browser per test (default)
/// - `TestHarness::from_browser()` - Module-scoped: reuse browser, fresh context/page
/// - `TestHarness::from_context()` - Shared context: reuse context, fresh page only
///
/// # Example
///
/// ```no_run
/// use viewpoint_test::TestHarness;
///
/// #[tokio::test]
/// async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
///     let harness = TestHarness::new().await?;
///     let page = harness.page();
///
///     page.goto("https://example.com").goto().await?;
///
///     Ok(()) // harness drops and cleans up
/// }
/// ```
#[derive(Debug)]
pub struct TestHarness {
    /// The browser instance.
    browser: Option<Browser>,
    /// The browser context.
    context: Option<BrowserContext>,
    /// The page.
    page: Page,
    /// Whether we own the browser (should close on drop).
    owns_browser: bool,
    /// Whether we own the context (should close on drop).
    owns_context: bool,
    /// Test configuration.
    config: TestConfig,
}

impl TestHarness {
    /// Create a new test harness with default configuration.
    ///
    /// This creates a new browser, context, and page for the test.
    /// All resources are owned and will be cleaned up on drop.
    ///
    /// # Errors
    ///
    /// Returns an error if browser launch or page creation fails.
    #[instrument(level = "info", name = "TestHarness::new")]
    pub async fn new() -> Result<Self, TestError> {
        Self::with_config(TestConfig::default()).await
    }

    /// Create a new test harness with custom configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if browser launch or page creation fails.
    #[instrument(level = "info", name = "TestHarness::with_config", skip(config))]
    pub async fn with_config(config: TestConfig) -> Result<Self, TestError> {
        info!(headless = config.headless, "Creating test harness");

        let browser = Browser::launch()
            .headless(config.headless)
            .launch()
            .await
            .map_err(|e| TestError::Setup(format!("Failed to launch browser: {e}")))?;

        debug!("Browser launched");

        let context = browser
            .new_context()
            .await
            .map_err(|e| TestError::Setup(format!("Failed to create context: {e}")))?;

        debug!("Context created");

        let page = context
            .new_page()
            .await
            .map_err(|e| TestError::Setup(format!("Failed to create page: {e}")))?;

        debug!("Page created");

        info!("Test harness ready");

        Ok(Self {
            browser: Some(browser),
            context: Some(context),
            page,
            owns_browser: true,
            owns_context: true,
            config,
        })
    }

    /// Create a test harness builder for custom configuration.
    pub fn builder() -> TestHarnessBuilder {
        TestHarnessBuilder::default()
    }

    /// Create a test harness using an existing browser.
    ///
    /// This creates a new context and page in the provided browser.
    /// The browser will NOT be closed when the harness is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if context or page creation fails.
    #[instrument(level = "info", name = "TestHarness::from_browser", skip(browser))]
    pub async fn from_browser(browser: &Browser) -> Result<Self, TestError> {
        info!("Creating test harness from existing browser");

        let context = browser
            .new_context()
            .await
            .map_err(|e| TestError::Setup(format!("Failed to create context: {e}")))?;

        debug!("Context created");

        let page = context
            .new_page()
            .await
            .map_err(|e| TestError::Setup(format!("Failed to create page: {e}")))?;

        debug!("Page created");

        info!("Test harness ready (browser shared)");

        Ok(Self {
            browser: None, // We don't own the browser
            context: Some(context),
            page,
            owns_browser: false,
            owns_context: true,
            config: TestConfig::default(),
        })
    }

    /// Create a test harness using an existing context.
    ///
    /// This creates a new page in the provided context.
    /// Neither the browser nor context will be closed when the harness is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if page creation fails.
    #[instrument(level = "info", name = "TestHarness::from_context", skip(context))]
    pub async fn from_context(context: &BrowserContext) -> Result<Self, TestError> {
        info!("Creating test harness from existing context");

        let page = context
            .new_page()
            .await
            .map_err(|e| TestError::Setup(format!("Failed to create page: {e}")))?;

        debug!("Page created");

        info!("Test harness ready (context shared)");

        Ok(Self {
            browser: None,
            context: None, // We don't own the context
            page,
            owns_browser: false,
            owns_context: false,
            config: TestConfig::default(),
        })
    }

    /// Get a reference to the page.
    pub fn page(&self) -> &Page {
        &self.page
    }

    /// Get a mutable reference to the page.
    pub fn page_mut(&mut self) -> &mut Page {
        &mut self.page
    }

    /// Get a reference to the browser context.
    ///
    /// Returns `None` if this harness was created with `from_context()`.
    pub fn context(&self) -> Option<&BrowserContext> {
        self.context.as_ref()
    }

    /// Get a reference to the browser.
    ///
    /// Returns `None` if this harness was created with `from_browser()` or `from_context()`.
    pub fn browser(&self) -> Option<&Browser> {
        self.browser.as_ref()
    }

    /// Get the test configuration.
    pub fn config(&self) -> &TestConfig {
        &self.config
    }

    /// Create a new page in the same context.
    ///
    /// # Errors
    ///
    /// Returns an error if page creation fails or if no context is available.
    pub async fn new_page(&self) -> Result<Page, TestError> {
        let context = self
            .context
            .as_ref()
            .ok_or_else(|| TestError::Setup("No context available (harness created with from_context)".to_string()))?;

        context
            .new_page()
            .await
            .map_err(|e| TestError::Setup(format!("Failed to create page: {e}")))
    }

    /// Explicitly close all owned resources.
    ///
    /// This is called automatically on drop, but can be called explicitly
    /// to handle cleanup errors.
    ///
    /// # Errors
    ///
    /// Returns an error if cleanup fails.
    #[instrument(level = "info", name = "TestHarness::close", skip(self))]
    pub async fn close(mut self) -> Result<(), TestError> {
        info!(owns_browser = self.owns_browser, owns_context = self.owns_context, "Closing test harness");

        // Close page
        if let Err(e) = self.page.close().await {
            warn!("Failed to close page: {}", e);
        }

        // Close context if we own it
        if self.owns_context {
            if let Some(ref mut context) = self.context {
                if let Err(e) = context.close().await {
                    warn!("Failed to close context: {}", e);
                }
            }
        }

        // Close browser if we own it
        if self.owns_browser {
            if let Some(ref browser) = self.browser {
                if let Err(e) = browser.close().await {
                    warn!("Failed to close browser: {}", e);
                }
            }
        }

        info!("Test harness closed");
        Ok(())
    }
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        // We can't do async cleanup in Drop, so we rely on the underlying
        // types' Drop implementations. Browser::drop will kill the process
        // if we own it.
        debug!(owns_browser = self.owns_browser, owns_context = self.owns_context, "TestHarness dropped");
    }
}

/// Builder for `TestHarness`.
#[derive(Debug, Default)]
pub struct TestHarnessBuilder {
    config: TestConfig,
}

impl TestHarnessBuilder {
    /// Set whether to run in headless mode.
    pub fn headless(mut self, headless: bool) -> Self {
        self.config.headless = headless;
        self
    }

    /// Set the default timeout.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Build and initialize the test harness.
    ///
    /// # Errors
    ///
    /// Returns an error if browser launch or page creation fails.
    pub async fn build(self) -> Result<TestHarness, TestError> {
        TestHarness::with_config(self.config).await
    }
}
