//! Browser launching functionality.

use std::env;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use tokio::time::timeout;
use tracing::{debug, info, instrument, trace, warn};
use viewpoint_cdp::CdpConnection;

use super::Browser;
use crate::error::BrowserError;

/// Default timeout for browser launch.
const DEFAULT_LAUNCH_TIMEOUT: Duration = Duration::from_secs(30);

/// Common Chromium paths on different platforms.
const CHROMIUM_PATHS: &[&str] = &[
    // Linux
    "chromium",
    "chromium-browser",
    "/usr/bin/chromium",
    "/usr/bin/chromium-browser",
    "/snap/bin/chromium",
    // macOS
    "/Applications/Chromium.app/Contents/MacOS/Chromium",
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    // Windows
    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
];

/// Builder for launching a browser.
#[derive(Debug, Clone)]
pub struct BrowserBuilder {
    /// Path to Chromium executable.
    executable_path: Option<PathBuf>,
    /// Whether to run in headless mode.
    headless: bool,
    /// Additional command line arguments.
    args: Vec<String>,
    /// Launch timeout.
    timeout: Duration,
}

impl Default for BrowserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserBuilder {
    /// Create a new browser builder with default settings.
    pub fn new() -> Self {
        Self {
            executable_path: None,
            headless: true,
            args: Vec::new(),
            timeout: DEFAULT_LAUNCH_TIMEOUT,
        }
    }

    /// Set the path to the Chromium executable.
    ///
    /// If not set, the launcher will search common paths and
    /// check the `CHROMIUM_PATH` environment variable.
    #[must_use]
    pub fn executable_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.executable_path = Some(path.into());
        self
    }

    /// Set whether to run in headless mode.
    ///
    /// Default is `true`.
    #[must_use]
    pub fn headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }

    /// Add additional command line arguments.
    #[must_use]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Set the launch timeout.
    ///
    /// Default is 30 seconds.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Launch the browser.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Chromium is not found
    /// - The process fails to spawn
    /// - The browser doesn't start within the timeout
    #[instrument(level = "info", skip(self), fields(headless = self.headless, timeout_ms = self.timeout.as_millis()))]
    pub async fn launch(self) -> Result<Browser, BrowserError> {
        info!("Launching browser");

        let executable = self.find_executable()?;
        info!(executable = %executable.display(), "Found Chromium executable");

        let mut cmd = Command::new(&executable);

        // Add default arguments
        cmd.arg("--remote-debugging-port=0");

        if self.headless {
            cmd.arg("--headless=new");
            debug!("Running in headless mode");
        } else {
            debug!("Running in headed mode");
        }

        // Add common stability flags
        let stability_args = [
            "--disable-background-networking",
            "--disable-background-timer-throttling",
            "--disable-backgrounding-occluded-windows",
            "--disable-breakpad",
            "--disable-component-extensions-with-background-pages",
            "--disable-component-update",
            "--disable-default-apps",
            "--disable-dev-shm-usage",
            "--disable-extensions",
            "--disable-features=TranslateUI",
            "--disable-hang-monitor",
            "--disable-ipc-flooding-protection",
            "--disable-popup-blocking",
            "--disable-prompt-on-repost",
            "--disable-renderer-backgrounding",
            "--disable-sync",
            "--enable-features=NetworkService,NetworkServiceInProcess",
            "--force-color-profile=srgb",
            "--metrics-recording-only",
            "--no-first-run",
            "--password-store=basic",
            "--use-mock-keychain",
        ];
        cmd.args(stability_args);
        trace!(arg_count = stability_args.len(), "Added stability flags");

        // Add user arguments
        if !self.args.is_empty() {
            cmd.args(&self.args);
            debug!(user_args = ?self.args, "Added user arguments");
        }

        // Capture stderr for the WebSocket URL
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::null());

        info!("Spawning Chromium process");
        let mut child = cmd.spawn().map_err(|e| {
            warn!(error = %e, "Failed to spawn Chromium process");
            BrowserError::LaunchFailed(e.to_string())
        })?;

        let pid = child.id();
        info!(pid = pid, "Chromium process spawned");

        // Read the WebSocket URL from stderr
        debug!("Waiting for DevTools WebSocket URL");
        let ws_url = timeout(self.timeout, Self::read_ws_url(&mut child))
            .await
            .map_err(|_| {
                warn!(
                    timeout_ms = self.timeout.as_millis(),
                    "Browser launch timed out"
                );
                BrowserError::LaunchTimeout(self.timeout)
            })??;

        info!(ws_url = %ws_url, "Got DevTools WebSocket URL");

        // Connect to the browser
        debug!("Connecting to browser via CDP");
        let connection = CdpConnection::connect(&ws_url).await?;

        info!(pid = pid, "Browser launched and connected successfully");
        Ok(Browser::from_connection_and_process(connection, child))
    }

    /// Find the Chromium executable.
    #[instrument(level = "debug", skip(self))]
    fn find_executable(&self) -> Result<PathBuf, BrowserError> {
        // Check if explicitly set
        if let Some(ref path) = self.executable_path {
            debug!(path = %path.display(), "Checking explicit executable path");
            if path.exists() {
                info!(path = %path.display(), "Using explicit executable path");
                return Ok(path.clone());
            }
            warn!(path = %path.display(), "Explicit executable path does not exist");
            return Err(BrowserError::ChromiumNotFound);
        }

        // Check environment variable
        if let Ok(path_str) = env::var("CHROMIUM_PATH") {
            let path = PathBuf::from(&path_str);
            debug!(path = %path.display(), "Checking CHROMIUM_PATH environment variable");
            if path.exists() {
                info!(path = %path.display(), "Using CHROMIUM_PATH");
                return Ok(path);
            }
            warn!(path = %path.display(), "CHROMIUM_PATH does not exist");
        }

        // Search common paths
        debug!("Searching common Chromium paths");
        for path_str in CHROMIUM_PATHS {
            let path = PathBuf::from(path_str);
            if path.exists() {
                info!(path = %path.display(), "Found Chromium at common path");
                return Ok(path);
            }

            // Also try which/where
            if let Ok(output) = Command::new("which").arg(path_str).output() {
                if output.status.success() {
                    let found = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !found.is_empty() {
                        let found_path = PathBuf::from(&found);
                        info!(path = %found_path.display(), "Found Chromium via 'which'");
                        return Ok(found_path);
                    }
                }
            }
        }

        warn!("Chromium not found in any expected location");
        Err(BrowserError::ChromiumNotFound)
    }

    /// Read the WebSocket URL from the browser's stderr.
    async fn read_ws_url(child: &mut Child) -> Result<String, BrowserError> {
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| BrowserError::LaunchFailed("failed to capture stderr".into()))?;

        // Spawn blocking read in a separate task
        let handle = tokio::task::spawn_blocking(move || {
            let reader = BufReader::new(stderr);

            for line in reader.lines() {
                let Ok(line) = line else { continue };

                trace!(line = %line, "Read line from Chromium stderr");

                // Look for "DevTools listening on ws://..."
                if let Some(pos) = line.find("DevTools listening on ") {
                    let url = &line[pos + 22..];
                    return Some(url.trim().to_string());
                }
            }

            None
        });

        handle
            .await
            .map_err(|e| BrowserError::LaunchFailed(e.to_string()))?
            .ok_or(BrowserError::LaunchFailed(
                "failed to find WebSocket URL in browser output".into(),
            ))
    }
}
