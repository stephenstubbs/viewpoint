//! Clock mocking for controlling time in browser pages.
//!
//! The Clock API allows you to control time-related functions in JavaScript,
//! including Date, setTimeout, setInterval, requestAnimationFrame, and `performance.now()`.
//!
//! # Example
//!
//! ```ignore
//! use std::time::Duration;
//!
//! // Install clock mocking
//! page.clock().install().await?;
//!
//! // Set a fixed time
//! page.clock().set_fixed_time("2024-01-01T00:00:00Z").await?;
//!
//! // Check that Date.now() returns the fixed time
//! use viewpoint_js::js;
//! let time: f64 = page.evaluate(js!{ Date.now() }).await?;
//!
//! // Advance time by 5 seconds, firing any scheduled timers
//! page.clock().run_for(Duration::from_secs(5)).await?;
//! ```

use std::sync::Arc;
use std::time::Duration;

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
/// ```ignore
/// use std::time::Duration;
///
/// // Install clock mocking
/// page.clock().install().await?;
///
/// // Freeze time at a specific moment
/// page.clock().set_fixed_time("2024-01-01T00:00:00Z").await?;
///
/// // Advance time and fire scheduled timers
/// page.clock().run_for(Duration::from_secs(5)).await?;
///
/// // Uninstall when done
/// page.clock().uninstall().await?;
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
    /// ```ignore
    /// page.clock().install().await?;
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
        self.evaluate(js!{ window.__viewpointClock.install() }).await?;
        self.installed = true;
        
        debug!("Clock installed");
        Ok(())
    }

    /// Uninstall clock mocking and restore original functions.
    ///
    /// # Example
    ///
    /// ```ignore
    /// page.clock().uninstall().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the clock cannot be uninstalled.
    #[instrument(level = "debug", skip(self))]
    pub async fn uninstall(&mut self) -> Result<(), PageError> {
        self.evaluate(js!{ window.__viewpointClock && window.__viewpointClock.uninstall() })
            .await?;
        self.installed = false;
        
        debug!("Clock uninstalled");
        Ok(())
    }

    /// Set a fixed time that doesn't advance.
    ///
    /// All calls to `Date.now()` and new `Date()` will return this time.
    /// Time remains frozen until you call `run_for`, `fast_forward`,
    /// `set_system_time`, or `resume`.
    ///
    /// # Arguments
    ///
    /// * `time` - The time to set, either as an ISO 8601 string (e.g., "2024-01-01T00:00:00Z")
    ///   or a Unix timestamp in milliseconds.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Using ISO string
    /// page.clock().set_fixed_time("2024-01-01T00:00:00Z").await?;
    ///
    /// // Using timestamp
    /// page.clock().set_fixed_time(1704067200000i64).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if setting the time fails.
    #[instrument(level = "debug", skip(self, time))]
    pub async fn set_fixed_time(&self, time: impl Into<TimeValue>) -> Result<(), PageError> {
        let time_value = time.into();
        match &time_value {
            TimeValue::Timestamp(ts) => {
                self.evaluate(&js!{ window.__viewpointClock.setFixedTime(#{ts}) }).await?;
            }
            TimeValue::IsoString(s) => {
                self.evaluate(&js!{ window.__viewpointClock.setFixedTime(#{s}) }).await?;
            }
        }
        debug!(time = ?time_value, "Fixed time set");
        Ok(())
    }

    /// Set the system time that flows normally.
    ///
    /// Time starts from the specified value and advances in real time.
    ///
    /// # Arguments
    ///
    /// * `time` - The starting time, either as an ISO 8601 string or Unix timestamp.
    ///
    /// # Example
    ///
    /// ```ignore
    /// page.clock().set_system_time("2024-01-01T00:00:00Z").await?;
    /// // Time will now flow from 2024-01-01
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if setting the time fails.
    #[instrument(level = "debug", skip(self, time))]
    pub async fn set_system_time(&self, time: impl Into<TimeValue>) -> Result<(), PageError> {
        let time_value = time.into();
        match &time_value {
            TimeValue::Timestamp(ts) => {
                self.evaluate(&js!{ window.__viewpointClock.setSystemTime(#{ts}) }).await?;
            }
            TimeValue::IsoString(s) => {
                self.evaluate(&js!{ window.__viewpointClock.setSystemTime(#{s}) }).await?;
            }
        }
        debug!(time = ?time_value, "System time set");
        Ok(())
    }

    /// Advance time by a duration, firing any scheduled timers.
    ///
    /// This advances the clock and executes any setTimeout/setInterval
    /// callbacks that were scheduled to fire during that period.
    ///
    /// # Arguments
    ///
    /// * `duration` - The amount of time to advance.
    ///
    /// # Returns
    ///
    /// The number of timers that were fired.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Advance 5 seconds, firing any timers scheduled in that period
    /// let fired = page.clock().run_for(Duration::from_secs(5)).await?;
    /// println!("Fired {} timers", fired);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if advancing time fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn run_for(&self, duration: Duration) -> Result<u32, PageError> {
        let ms = duration.as_millis();
        let result: f64 = self.evaluate_value(&js!{ window.__viewpointClock.runFor(#{ms}) }).await?;
        debug!(duration_ms = ms, timers_fired = result as u32, "Time advanced");
        Ok(result as u32)
    }

    /// Fast-forward time without firing timers.
    ///
    /// This advances the clock but does NOT execute scheduled timers.
    /// Use this when you want to jump ahead in time quickly.
    ///
    /// # Arguments
    ///
    /// * `duration` - The amount of time to skip.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Skip ahead 1 hour without firing any timers
    /// page.clock().fast_forward(Duration::from_secs(3600)).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if fast-forwarding fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn fast_forward(&self, duration: Duration) -> Result<(), PageError> {
        let ms = duration.as_millis();
        self.evaluate(&js!{ window.__viewpointClock.fastForward(#{ms}) }).await?;
        debug!(duration_ms = ms, "Time fast-forwarded");
        Ok(())
    }

    /// Pause at a specific time.
    ///
    /// This sets the clock to the specified time and pauses it there.
    ///
    /// # Arguments
    ///
    /// * `time` - The time to pause at, as an ISO string or timestamp.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Pause at noon
    /// page.clock().pause_at("2024-01-01T12:00:00Z").await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if pausing fails.
    #[instrument(level = "debug", skip(self, time))]
    pub async fn pause_at(&self, time: impl Into<TimeValue>) -> Result<(), PageError> {
        let time_value = time.into();
        match &time_value {
            TimeValue::Timestamp(ts) => {
                self.evaluate(&js!{ window.__viewpointClock.pauseAt(#{ts}) }).await?;
            }
            TimeValue::IsoString(s) => {
                self.evaluate(&js!{ window.__viewpointClock.pauseAt(#{s}) }).await?;
            }
        }
        debug!(time = ?time_value, "Clock paused");
        Ok(())
    }

    /// Resume normal time flow.
    ///
    /// After calling this, time will advance normally from the current
    /// mocked time value.
    ///
    /// # Example
    ///
    /// ```ignore
    /// page.clock().resume().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if resuming fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn resume(&self) -> Result<(), PageError> {
        self.evaluate(js!{ window.__viewpointClock.resume() }).await?;
        debug!("Clock resumed");
        Ok(())
    }

    /// Run all pending timers.
    ///
    /// This advances time to execute all scheduled setTimeout and setInterval
    /// callbacks, as well as requestAnimationFrame callbacks.
    ///
    /// # Returns
    ///
    /// The number of timers that were fired.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let fired = page.clock().run_all_timers().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if running timers fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn run_all_timers(&self) -> Result<u32, PageError> {
        let result: f64 = self.evaluate_value(js!{ window.__viewpointClock.runAllTimers() }).await?;
        debug!(timers_fired = result as u32, "All timers executed");
        Ok(result as u32)
    }

    /// Run to the last scheduled timer.
    ///
    /// This advances time to the last scheduled timer and executes all
    /// timers up to that point.
    ///
    /// # Returns
    ///
    /// The number of timers that were fired.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let fired = page.clock().run_to_last().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if running timers fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn run_to_last(&self) -> Result<u32, PageError> {
        let result: f64 = self.evaluate_value(js!{ window.__viewpointClock.runToLast() }).await?;
        debug!(timers_fired = result as u32, "Ran to last timer");
        Ok(result as u32)
    }

    /// Get the number of pending timers.
    ///
    /// This includes setTimeout, setInterval, and requestAnimationFrame callbacks.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = page.clock().pending_timer_count().await?;
    /// println!("{} timers pending", count);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if getting the count fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn pending_timer_count(&self) -> Result<u32, PageError> {
        let result: f64 = self.evaluate_value(js!{ window.__viewpointClock.pendingTimerCount() }).await?;
        Ok(result as u32)
    }

    /// Check if clock mocking is installed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if page.clock().is_installed().await? {
    ///     println!("Clock is mocked");
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    pub async fn is_installed(&self) -> Result<bool, PageError> {
        let result: bool = self.evaluate_value(js!{ window.__viewpointClock && window.__viewpointClock.isInstalled() }).await.unwrap_or(false);
        Ok(result)
    }

    /// Inject the clock mocking library into the page.
    async fn inject_clock_library(&self) -> Result<(), PageError> {
        self.evaluate(CLOCK_MOCK_SCRIPT).await?;
        Ok(())
    }

    /// Evaluate JavaScript and return nothing.
    async fn evaluate(&self, expression: &str) -> Result<(), PageError> {
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
    async fn evaluate_value<T: serde::de::DeserializeOwned>(
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

        serde_json::from_value(result.result.value).map_err(|e| {
            PageError::EvaluationFailed(format!("Failed to deserialize result: {e}"))
        })
    }
}

/// A time value that can be either a timestamp or an ISO string.
#[derive(Debug, Clone)]
pub enum TimeValue {
    /// Unix timestamp in milliseconds.
    Timestamp(i64),
    /// ISO 8601 formatted string.
    IsoString(String),
}

impl From<i64> for TimeValue {
    fn from(ts: i64) -> Self {
        TimeValue::Timestamp(ts)
    }
}

impl From<u64> for TimeValue {
    fn from(ts: u64) -> Self {
        TimeValue::Timestamp(ts as i64)
    }
}

impl From<&str> for TimeValue {
    fn from(s: &str) -> Self {
        TimeValue::IsoString(s.to_string())
    }
}

impl From<String> for TimeValue {
    fn from(s: String) -> Self {
        TimeValue::IsoString(s)
    }
}
