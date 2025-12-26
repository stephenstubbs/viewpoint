//! Browser launching and management.

mod launcher;

use std::process::Child;
use std::sync::Arc;
use std::time::Duration;

use viewpoint_cdp::protocol::target::{CreateBrowserContextParams, CreateBrowserContextResult};
use viewpoint_cdp::CdpConnection;
use tokio::sync::Mutex;

use crate::context::BrowserContext;
use crate::error::BrowserError;

pub use launcher::BrowserBuilder;

/// Default timeout for browser operations.
#[allow(dead_code)]
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A browser instance connected via CDP.
#[derive(Debug)]
pub struct Browser {
    /// CDP connection to the browser.
    connection: Arc<CdpConnection>,
    /// Browser process (only present if we launched it).
    process: Option<Mutex<Child>>,
    /// Whether the browser was launched by us (vs connected to).
    owned: bool,
}

impl Browser {
    /// Create a browser builder for launching a new browser.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch()
    ///     .headless(true)
    ///     .launch()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn launch() -> BrowserBuilder {
        BrowserBuilder::new()
    }

    /// Connect to an already-running browser via WebSocket URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::connect("ws://localhost:9222/devtools/browser/...").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the connection fails.
    pub async fn connect(ws_url: &str) -> Result<Self, BrowserError> {
        let connection = CdpConnection::connect(ws_url).await?;

        Ok(Self {
            connection: Arc::new(connection),
            process: None,
            owned: false,
        })
    }

    /// Create a browser from an existing connection and process.
    pub(crate) fn from_connection_and_process(
        connection: CdpConnection,
        process: Child,
    ) -> Self {
        Self {
            connection: Arc::new(connection),
            process: Some(Mutex::new(process)),
            owned: true,
        }
    }

    /// Create a new isolated browser context.
    ///
    /// Browser contexts are isolated environments within the browser,
    /// similar to incognito windows. They have their own cookies,
    /// cache, and storage.
    ///
    /// # Errors
    ///
    /// Returns an error if context creation fails.
    pub async fn new_context(&self) -> Result<BrowserContext, BrowserError> {
        let result: CreateBrowserContextResult = self
            .connection
            .send_command(
                "Target.createBrowserContext",
                Some(CreateBrowserContextParams::default()),
                None,
            )
            .await?;

        Ok(BrowserContext::new(
            self.connection.clone(),
            result.browser_context_id,
        ))
    }

    /// Close the browser.
    ///
    /// If this browser was launched by us, the process will be terminated.
    /// If it was connected to, only the WebSocket connection is closed.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    pub async fn close(&self) -> Result<(), BrowserError> {
        // If we own the process, terminate it
        if let Some(ref process) = self.process {
            let mut child = process.lock().await;
            let _ = child.kill();
        }

        Ok(())
    }

    /// Get a reference to the CDP connection.
    pub fn connection(&self) -> &Arc<CdpConnection> {
        &self.connection
    }

    /// Check if this browser was launched by us.
    pub fn is_owned(&self) -> bool {
        self.owned
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        // Try to kill the process if we own it
        if self.owned {
            if let Some(ref process) = self.process {
                // We can't await in drop, so we try to kill synchronously
                if let Ok(mut guard) = process.try_lock() {
                    let _ = guard.kill();
                }
            }
        }
    }
}
