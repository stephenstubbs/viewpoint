//! Browser process management utilities.
//!
//! This module provides utilities for managing browser child processes,
//! particularly for safe termination and cleanup.

use std::process::Child;
use std::thread;
use std::time::Duration;

use tracing::{info, warn};

/// Kill and reap a child process asynchronously.
///
/// This method:
/// 1. Sends SIGKILL to the process (if still running)
/// 2. Waits for the process to exit and reaps it
///
/// This prevents zombie processes by ensuring `wait()` is called.
pub async fn kill_and_reap_async(child: &mut Child) {
    // Kill the process (ignore errors if already dead)
    let _ = child.kill();

    // Wait for the process to exit and reap it
    // This is the critical step to prevent zombie processes
    match child.wait() {
        Ok(status) => {
            info!(?status, "Browser process reaped successfully");
        }
        Err(e) => {
            warn!(error = %e, "Failed to reap browser process");
        }
    }
}

/// Kill and reap a child process synchronously (for use in Drop).
///
/// This method uses `try_wait()` (non-blocking) with retries since
/// `Drop` cannot be async. It attempts to reap the process a few times
/// with small delays to handle the case where the process hasn't exited
/// immediately after `kill()`.
///
/// # Arguments
///
/// * `child` - The child process to kill and reap
/// * `max_attempts` - Maximum number of `try_wait` attempts
/// * `retry_delay` - Delay between retry attempts
pub fn kill_and_reap_sync(child: &mut Child, max_attempts: u32, retry_delay: Duration) {
    // Kill the process (ignore errors if already dead)
    let _ = child.kill();

    // Try to reap the process with retries
    for attempt in 1..=max_attempts {
        match child.try_wait() {
            Ok(Some(status)) => {
                info!(
                    ?status,
                    attempt, "Browser process reaped successfully in Drop"
                );
                return;
            }
            Ok(None) => {
                // Process still running, wait a bit and retry
                if attempt < max_attempts {
                    thread::sleep(retry_delay);
                }
            }
            Err(e) => {
                warn!(error = %e, "Failed to check browser process status in Drop");
                return;
            }
        }
    }

    // If we get here, the process is still running after all attempts
    warn!(
        max_attempts,
        "Browser process still running after kill, will become zombie until parent exits"
    );
}
