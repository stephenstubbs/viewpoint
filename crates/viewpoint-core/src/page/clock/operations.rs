//! Clock operations for time manipulation.

use std::time::Duration;

use tracing::{debug, instrument};
use viewpoint_js::js;

use super::{Clock, TimeValue};
use crate::error::PageError;

impl Clock<'_> {
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
    /// # Errors
    ///
    /// Returns an error if setting the time fails.
    #[instrument(level = "debug", skip(self, time))]
    pub async fn set_fixed_time(&self, time: impl Into<TimeValue>) -> Result<(), PageError> {
        let time_value = time.into();
        match &time_value {
            TimeValue::Timestamp(ts) => {
                self.evaluate(&js! { window.__viewpointClock.setFixedTime(#{ts}) })
                    .await?;
            }
            TimeValue::IsoString(s) => {
                self.evaluate(&js! { window.__viewpointClock.setFixedTime(#{s}) })
                    .await?;
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
    /// # Errors
    ///
    /// Returns an error if setting the time fails.
    #[instrument(level = "debug", skip(self, time))]
    pub async fn set_system_time(&self, time: impl Into<TimeValue>) -> Result<(), PageError> {
        let time_value = time.into();
        match &time_value {
            TimeValue::Timestamp(ts) => {
                self.evaluate(&js! { window.__viewpointClock.setSystemTime(#{ts}) })
                    .await?;
            }
            TimeValue::IsoString(s) => {
                self.evaluate(&js! { window.__viewpointClock.setSystemTime(#{s}) })
                    .await?;
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
    /// # Errors
    ///
    /// Returns an error if advancing time fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn run_for(&self, duration: Duration) -> Result<u32, PageError> {
        let ms = duration.as_millis();
        let result: f64 = self
            .evaluate_value(&js! { window.__viewpointClock.runFor(#{ms}) })
            .await?;
        debug!(
            duration_ms = ms,
            timers_fired = result as u32,
            "Time advanced"
        );
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
    /// # Errors
    ///
    /// Returns an error if fast-forwarding fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn fast_forward(&self, duration: Duration) -> Result<(), PageError> {
        let ms = duration.as_millis();
        self.evaluate(&js! { window.__viewpointClock.fastForward(#{ms}) })
            .await?;
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
    /// # Errors
    ///
    /// Returns an error if pausing fails.
    #[instrument(level = "debug", skip(self, time))]
    pub async fn pause_at(&self, time: impl Into<TimeValue>) -> Result<(), PageError> {
        let time_value = time.into();
        match &time_value {
            TimeValue::Timestamp(ts) => {
                self.evaluate(&js! { window.__viewpointClock.pauseAt(#{ts}) })
                    .await?;
            }
            TimeValue::IsoString(s) => {
                self.evaluate(&js! { window.__viewpointClock.pauseAt(#{s}) })
                    .await?;
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
    /// # Errors
    ///
    /// Returns an error if resuming fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn resume(&self) -> Result<(), PageError> {
        self.evaluate(js! { window.__viewpointClock.resume() })
            .await?;
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
    /// # Errors
    ///
    /// Returns an error if running timers fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn run_all_timers(&self) -> Result<u32, PageError> {
        let result: f64 = self
            .evaluate_value(js! { window.__viewpointClock.runAllTimers() })
            .await?;
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
    /// # Errors
    ///
    /// Returns an error if running timers fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn run_to_last(&self) -> Result<u32, PageError> {
        let result: f64 = self
            .evaluate_value(js! { window.__viewpointClock.runToLast() })
            .await?;
        debug!(timers_fired = result as u32, "Ran to last timer");
        Ok(result as u32)
    }

    /// Get the number of pending timers.
    ///
    /// This includes setTimeout, setInterval, and requestAnimationFrame callbacks.
    ///
    /// # Errors
    ///
    /// Returns an error if getting the count fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn pending_timer_count(&self) -> Result<u32, PageError> {
        let result: f64 = self
            .evaluate_value(js! { window.__viewpointClock.pendingTimerCount() })
            .await?;
        Ok(result as u32)
    }

    /// Check if clock mocking is installed.
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    pub async fn is_installed(&self) -> Result<bool, PageError> {
        let result: bool = self
            .evaluate_value(
                js! { window.__viewpointClock && window.__viewpointClock.isInstalled() },
            )
            .await
            .unwrap_or(false);
        Ok(result)
    }
}
