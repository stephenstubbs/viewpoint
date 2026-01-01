//! Browser launching functionality.

use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use tempfile::TempDir;
use tokio::time::timeout;
use tracing::{debug, info, instrument, trace, warn};
use viewpoint_cdp::CdpConnection;

use super::Browser;
use crate::error::BrowserError;

/// User data directory configuration for browser profiles.
///
/// Controls how the browser manages user data (cookies, localStorage, settings).
/// The default is [`UserDataDir::Temp`], which creates an isolated temporary
/// directory that is automatically cleaned up when the browser closes.
///
/// # Breaking Change
///
/// Prior to this change, browsers used the system default profile (`~/.config/chromium/`)
/// by default. To restore the old behavior, use [`UserDataDir::System`] explicitly:
///
/// ```no_run
/// use viewpoint_core::Browser;
///
/// # async fn example() -> Result<(), viewpoint_core::CoreError> {
/// let browser = Browser::launch()
///     .user_data_dir_system()
///     .launch()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub enum UserDataDir {
    /// Create a unique temporary directory per session.
    ///
    /// This is the default mode. Each browser instance gets its own isolated
    /// profile that is automatically deleted when the browser closes or is dropped.
    /// This prevents conflicts when running multiple browser instances concurrently.
    Temp,

    /// Copy a template profile to a temporary directory.
    ///
    /// The template directory contents are copied to a new temporary directory.
    /// The temporary directory is cleaned up when the browser closes.
    /// The original template directory is unchanged.
    ///
    /// Use this when you need pre-configured settings, extensions, or cookies
    /// as a starting point, but still want isolation between sessions.
    TempFromTemplate(PathBuf),

    /// Use a persistent directory for browser data.
    ///
    /// Browser state (cookies, localStorage, settings) persists in the specified
    /// directory across browser restarts. The directory is NOT cleaned up when
    /// the browser closes.
    ///
    /// Note: Using the same persistent directory for multiple concurrent browser
    /// instances will cause profile lock conflicts.
    Persist(PathBuf),

    /// Use the system default profile.
    ///
    /// On Linux, this is typically `~/.config/chromium/`.
    /// No `--user-data-dir` flag is passed to Chromium.
    ///
    /// **Warning**: This can cause conflicts if another Chromium instance is running,
    /// or if a previous session crashed without proper cleanup. Prefer [`UserDataDir::Temp`]
    /// for automation scenarios.
    System,
}

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
    /// User data directory configuration.
    user_data_dir: UserDataDir,
}

impl Default for BrowserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserBuilder {
    /// Create a new browser builder with default settings.
    ///
    /// By default, the browser uses an isolated temporary directory for user data.
    /// This prevents conflicts when running multiple browser instances and ensures
    /// clean sessions for automation.
    pub fn new() -> Self {
        Self {
            executable_path: None,
            headless: true,
            args: Vec::new(),
            timeout: DEFAULT_LAUNCH_TIMEOUT,
            user_data_dir: UserDataDir::Temp,
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

    /// Set a persistent user data directory for browser profile.
    ///
    /// When set, browser state (cookies, localStorage, settings) persists
    /// in the specified directory across browser restarts. The directory
    /// is NOT cleaned up when the browser closes.
    ///
    /// **Note**: Using the same directory for multiple concurrent browser
    /// instances will cause profile lock conflicts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch()
    ///     .user_data_dir("/path/to/profile")
    ///     .launch()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn user_data_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.user_data_dir = UserDataDir::Persist(path.into());
        self
    }

    /// Use the system default profile directory.
    ///
    /// On Linux, this is typically `~/.config/chromium/`.
    /// No `--user-data-dir` flag is passed to Chromium.
    ///
    /// **Warning**: This can cause conflicts if another Chromium instance is running,
    /// or if a previous session crashed without proper cleanup. Prefer the default
    /// isolated temp profile for automation scenarios.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch()
    ///     .user_data_dir_system()
    ///     .launch()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn user_data_dir_system(mut self) -> Self {
        self.user_data_dir = UserDataDir::System;
        self
    }

    /// Use a template profile copied to a temporary directory.
    ///
    /// The contents of the template directory are copied to a new temporary
    /// directory. This allows starting with pre-configured settings, extensions,
    /// or cookies while maintaining isolation between sessions.
    ///
    /// The temporary directory is automatically cleaned up when the browser
    /// closes or is dropped. The original template directory is unchanged.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// // Create a browser with extensions from a template profile
    /// let browser = Browser::launch()
    ///     .user_data_dir_template_from("/path/to/template-profile")
    ///     .launch()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Loading Extensions
    ///
    /// Extensions can also be loaded at runtime without a template profile:
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch()
    ///     .args(["--load-extension=/path/to/unpacked-extension"])
    ///     .launch()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn user_data_dir_template_from(mut self, template_path: impl Into<PathBuf>) -> Self {
        self.user_data_dir = UserDataDir::TempFromTemplate(template_path.into());
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
    /// - Template directory doesn't exist or can't be copied
    #[instrument(level = "info", skip(self), fields(headless = self.headless, timeout_ms = self.timeout.as_millis()))]
    pub async fn launch(self) -> Result<Browser, BrowserError> {
        info!("Launching browser");

        let executable = self.find_executable()?;
        info!(executable = %executable.display(), "Found Chromium executable");

        // Handle user data directory configuration
        let (user_data_path, temp_dir) = self.prepare_user_data_dir()?;

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

        // Add user data directory if we have one
        if let Some(ref user_data_dir) = user_data_path {
            cmd.arg(format!("--user-data-dir={}", user_data_dir.display()));
            debug!(user_data_dir = %user_data_dir.display(), "Using user data directory");
        } else {
            debug!("Using system default user data directory");
        }

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
        Ok(Browser::from_launch(connection, child, temp_dir))
    }

    /// Prepare the user data directory based on configuration.
    ///
    /// Returns the path to use for `--user-data-dir` (if any) and an optional
    /// `TempDir` handle that should be stored in the `Browser` struct to ensure
    /// cleanup on drop.
    fn prepare_user_data_dir(&self) -> Result<(Option<PathBuf>, Option<TempDir>), BrowserError> {
        match &self.user_data_dir {
            UserDataDir::Temp => {
                // Create a unique temporary directory
                let temp_dir = TempDir::with_prefix("viewpoint-browser-").map_err(|e| {
                    BrowserError::LaunchFailed(format!(
                        "Failed to create temporary user data directory: {e}"
                    ))
                })?;
                let path = temp_dir.path().to_path_buf();
                debug!(path = %path.display(), "Created temporary user data directory");
                Ok((Some(path), Some(temp_dir)))
            }
            UserDataDir::TempFromTemplate(template_path) => {
                // Validate template exists
                if !template_path.exists() {
                    return Err(BrowserError::LaunchFailed(format!(
                        "Template profile directory does not exist: {}",
                        template_path.display()
                    )));
                }
                if !template_path.is_dir() {
                    return Err(BrowserError::LaunchFailed(format!(
                        "Template profile path is not a directory: {}",
                        template_path.display()
                    )));
                }

                // Create temporary directory
                let temp_dir = TempDir::with_prefix("viewpoint-browser-").map_err(|e| {
                    BrowserError::LaunchFailed(format!(
                        "Failed to create temporary user data directory: {e}"
                    ))
                })?;
                let dest_path = temp_dir.path().to_path_buf();

                // Copy template contents to temp directory
                debug!(
                    template = %template_path.display(),
                    dest = %dest_path.display(),
                    "Copying template profile to temporary directory"
                );
                copy_dir_recursive(template_path, &dest_path).map_err(|e| {
                    BrowserError::LaunchFailed(format!(
                        "Failed to copy template profile: {e}"
                    ))
                })?;

                info!(
                    template = %template_path.display(),
                    dest = %dest_path.display(),
                    "Template profile copied to temporary directory"
                );
                Ok((Some(dest_path), Some(temp_dir)))
            }
            UserDataDir::Persist(path) => {
                // Use the specified path, no cleanup
                debug!(path = %path.display(), "Using persistent user data directory");
                Ok((Some(path.clone()), None))
            }
            UserDataDir::System => {
                // No --user-data-dir flag, use system default
                debug!("Using system default user data directory");
                Ok((None, None))
            }
        }
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

/// Recursively copy a directory and its contents.
///
/// This copies files and subdirectories from `src` to `dst`.
/// The destination directory must already exist.
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
