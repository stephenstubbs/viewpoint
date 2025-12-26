//! Test configuration.

use std::time::Duration;

/// Configuration for test execution.
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Whether to run the browser in headless mode.
    pub headless: bool,
    /// Default timeout for operations.
    pub timeout: Duration,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            headless: true,
            timeout: Duration::from_secs(30),
        }
    }
}

impl TestConfig {
    /// Create a new test configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a builder for custom configuration.
    pub fn builder() -> TestConfigBuilder {
        TestConfigBuilder::default()
    }
}

/// Builder for `TestConfig`.
#[derive(Debug, Default)]
pub struct TestConfigBuilder {
    headless: Option<bool>,
    timeout: Option<Duration>,
}

impl TestConfigBuilder {
    /// Set whether to run in headless mode.
    #[must_use]
    pub fn headless(mut self, headless: bool) -> Self {
        self.headless = Some(headless);
        self
    }

    /// Set the default timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the configuration.
    pub fn build(self) -> TestConfig {
        TestConfig {
            headless: self.headless.unwrap_or(true),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        }
    }
}
