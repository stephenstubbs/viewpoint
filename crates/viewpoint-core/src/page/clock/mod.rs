//! Clock mocking for controlling time in browser pages.
//!
//! The Clock API allows you to control time-related functions in JavaScript,
//! including Date, setTimeout, setInterval, requestAnimationFrame, and `performance.now()`.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "integration")]
//! # tokio_test::block_on(async {
//! # use viewpoint_core::Browser;
//! use std::time::Duration;
//! # let browser = Browser::launch().headless(true).launch().await.unwrap();
//! # let context = browser.new_context().await.unwrap();
//! # let page = context.new_page().await.unwrap();
//! # page.goto("about:blank").goto().await.unwrap();
//!
//! // Install clock mocking
//! page.clock().install().await.unwrap();
//!
//! // Set a fixed time
//! page.clock().set_fixed_time("2024-01-01T00:00:00Z").await.unwrap();
//!
//! // Check that Date.now() returns the fixed time
//! use viewpoint_js::js;
//! let time: f64 = page.evaluate(js!{ Date.now() }).await.unwrap();
//!
//! // Advance time by 5 seconds, firing any scheduled timers
//! page.clock().run_for(Duration::from_secs(5)).await.unwrap();
//! # });
//! ```

mod operations;
mod time_value;

pub use time_value::TimeValue;

use std::sync::Arc;

use tracing::{debug, instrument};
use viewpoint_cdp::CdpConnection;
use viewpoint_js::js;

use crate::error::PageError;

use super::clock_script::CLOCK_MOCK_SCRIPT;

/// Clock controller for mocking time in a browser page.
///
/// The Clock API allows you to control JavaScript's time-related functions
/// including Date, setTimeout, setInterval, requestAnimationFrame, and
/// `performance.now()`.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// use std::time::Duration;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
/// # page.goto("about:blank").goto().await.unwrap();
///
/// // Install clock mocking
/// page.clock().install().await.unwrap();
///
/// // Freeze time at a specific moment
/// page.clock().set_fixed_time("2024-01-01T00:00:00Z").await.unwrap();
///
/// // Advance time and fire scheduled timers
/// page.clock().run_for(Duration::from_secs(5)).await.unwrap();
///
/// // Uninstall when done
/// page.clock().uninstall().await.unwrap();
/// # });
/// ```
#[derive(Debug)]
pub struct Clock<'a> {
    connection: &'a Arc<CdpConnection>,
    session_id: &'a str,
    installed: bool,
}

impl<'a> Clock<'a> {
    /// Create a new clock controller for a page.
    pub(crate) fn new(connection: &'a Arc<CdpConnection>, session_id: &'a str) -> Self {
        Self {
            connection,
            session_id,
            installed: false,
        }
    }

    /// Install clock mocking on the page.
    ///
    /// This replaces Date, setTimeout, setInterval, requestAnimationFrame,
    /// and `performance.now()` with mocked versions that can be controlled.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.clock().install().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the clock cannot be installed.
    #[instrument(level = "debug", skip(self))]
    pub async fn install(&mut self) -> Result<(), PageError> {
        // First inject the clock library
        self.inject_clock_library().await?;

        // Then install it
        self.evaluate(js! { window.__viewpointClock.install() })
            .await?;
        self.installed = true;

        debug!("Clock installed");
        Ok(())
    }

    /// Uninstall clock mocking and restore original functions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.clock().uninstall().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the clock cannot be uninstalled.
    #[instrument(level = "debug", skip(self))]
    pub async fn uninstall(&mut self) -> Result<(), PageError> {
        self.evaluate(js! { window.__viewpointClock && window.__viewpointClock.uninstall() })
            .await?;
        self.installed = false;

        debug!("Clock uninstalled");
        Ok(())
    }

    /// Inject the clock mocking library into the page.
    async fn inject_clock_library(&self) -> Result<(), PageError> {
        self.evaluate(CLOCK_MOCK_SCRIPT).await?;
        Ok(())
    }

    /// Evaluate JavaScript and return nothing.
    pub(super) async fn evaluate(&self, expression: &str) -> Result<(), PageError> {
        use viewpoint_cdp::protocol::runtime::EvaluateParams;

        let _: serde_json::Value = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: expression.to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: None,
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Evaluate JavaScript and return a value.
    pub(super) async fn evaluate_value<T: serde::de::DeserializeOwned>(
        &self,
        expression: &str,
    ) -> Result<T, PageError> {
        use viewpoint_cdp::protocol::runtime::EvaluateParams;

        #[derive(serde::Deserialize)]
        struct EvalResult {
            result: ResultValue,
        }

        #[derive(serde::Deserialize)]
        struct ResultValue {
            value: serde_json::Value,
        }

        let result: EvalResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: expression.to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: None,
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(self.session_id),
            )
            .await?;

        serde_json::from_value(result.result.value)
            .map_err(|e| PageError::EvaluationFailed(format!("Failed to deserialize result: {e}")))
    }
}
