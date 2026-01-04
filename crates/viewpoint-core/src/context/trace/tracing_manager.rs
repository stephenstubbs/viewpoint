//! Tracing manager for recording test execution traces.

use std::sync::Arc;

use chrono::Utc;
use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::tracing as cdp_tracing;

use crate::page::Page;
use crate::error::ContextError;
use crate::network::har::HarPage;

use super::action_handle::ActionHandle;
use super::capture;
use super::network;
use super::sources;
use super::types::{ActionEntry, SourceFileEntry, TracingOptions, TracingState};
use super::writer;

/// Tracing manager for recording test execution traces.
///
/// Traces record screenshots, DOM snapshots, network activity, and action
/// history. They can be viewed using Playwright's Trace Viewer.
///
/// **Note:** At least one page must exist in the context before starting tracing.
/// The tracing state is shared across all `context.tracing()` calls within the
/// same context, so you can call `start()` and `stop()` from separate `tracing()`
/// invocations.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// use viewpoint_core::context::TracingOptions;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
///
/// // Create a page first (required before starting tracing)
/// let page = context.new_page().await.unwrap();
///
/// // Start tracing with screenshots
/// context.tracing().start(
///     TracingOptions::new()
///         .name("my-test")
///         .screenshots(true)
///         .snapshots(true)
/// ).await.unwrap();
///
/// // Perform test actions...
/// page.goto("https://example.com").goto().await.unwrap();
///
/// // Stop and save trace (state persists across tracing() calls)
/// context.tracing().stop("/tmp/trace.zip").await.unwrap();
/// # });
/// ```
pub struct Tracing {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Browser context ID.
    context_id: String,
    /// Pages in this context (used to get session IDs).
    pages: Arc<RwLock<Vec<Page>>>,
    /// Tracing state.
    state: Arc<RwLock<TracingState>>,
}

impl std::fmt::Debug for Tracing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tracing")
            .field("context_id", &self.context_id)
            .finish_non_exhaustive()
    }
}

impl Tracing {
    /// Create a new Tracing instance with shared state.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        context_id: String,
        pages: Arc<RwLock<Vec<Page>>>,
        state: Arc<RwLock<TracingState>>,
    ) -> Self {
        Self {
            connection,
            context_id,
            pages,
            state,
        }
    }

    /// Get session IDs from pages.
    async fn get_session_ids(&self) -> Vec<String> {
        let pages = self.pages.read().await;
        pages
            .iter()
            .filter(|p| !p.session_id().is_empty())
            .map(|p| p.session_id().to_string())
            .collect()
    }

    /// Start recording a trace.
    ///
    /// # Requirements
    ///
    /// At least one page must exist in the context before starting tracing.
    /// Create a page with `context.new_page().await` first.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Tracing is already active
    /// - No pages exist in the context
    /// - CDP commands fail
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Browser, TracingOptions};
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// # let browser = Browser::launch().headless(true).launch().await?;
    /// # let context = browser.new_context().await?;
    /// // Create a page first
    /// let page = context.new_page().await?;
    ///
    /// // Then start tracing
    /// context.tracing().start(
    ///     TracingOptions::new()
    ///         .screenshots(true)
    ///         .snapshots(true)
    /// ).await?;
    /// # Ok(())
    /// # }
    #[instrument(level = "info", skip(self, options))]
    pub async fn start(&self, options: TracingOptions) -> Result<(), ContextError> {
        let mut state = self.state.write().await;

        if state.is_recording {
            return Err(ContextError::Internal(
                "Tracing is already active".to_string(),
            ));
        }

        // Validate that at least one page exists
        let session_ids = self.get_session_ids().await;
        if session_ids.is_empty() {
            return Err(ContextError::Internal(
                "Cannot start tracing: no pages in context. Create a page first.".to_string(),
            ));
        }

        info!(
            screenshots = options.screenshots,
            snapshots = options.snapshots,
            "Starting trace"
        );

        // Build categories for Chrome tracing
        let categories = [
            "devtools.timeline",
            "disabled-by-default-devtools.timeline",
            "disabled-by-default-devtools.timeline.frame",
        ];

        // Start tracing on all sessions
        for session_id in session_ids {
            let params = cdp_tracing::StartParams {
                categories: Some(categories.join(",")),
                transfer_mode: Some(cdp_tracing::TransferMode::ReturnAsStream),
                ..Default::default()
            };

            self.connection
                .send_command::<_, serde_json::Value>(
                    "Tracing.start",
                    Some(params),
                    Some(&session_id),
                )
                .await?;

            // Enable network tracking
            self.connection
                .send_command::<_, serde_json::Value>(
                    "Network.enable",
                    Some(serde_json::json!({})),
                    Some(&session_id),
                )
                .await?;
        }

        // Initialize state
        state.is_recording = true;
        state.options = options;
        state.actions.clear();
        state.events.clear();
        state.screenshots.clear();
        state.snapshots.clear();
        state.pending_requests.clear();
        state.network_entries.clear();
        state.har_pages.clear();
        state.source_files.clear();

        // Start network listener
        drop(state); // Release lock before spawning
        network::start_network_listener(
            self.connection.clone(),
            self.state.clone(),
            self.pages.clone(),
        );

        Ok(())
    }

    /// Stop tracing and save the trace to a file.
    ///
    /// The trace is saved as a zip file containing:
    /// - trace.json: The trace data
    /// - resources/: Screenshots and other resources
    /// - network.har: Network activity in HAR format
    ///
    /// # Errors
    ///
    /// Returns an error if tracing is not active or saving the trace fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// # let browser = Browser::launch().headless(true).launch().await?;
    /// # let context = browser.new_context().await?;
    /// context.tracing().stop("trace.zip").await?;
    /// # Ok(())
    /// # }
    #[instrument(level = "info", skip(self), fields(path = %path.as_ref().display()))]
    pub async fn stop(&self, path: impl AsRef<std::path::Path>) -> Result<(), ContextError> {
        let path = path.as_ref();
        let mut state = self.state.write().await;

        if !state.is_recording {
            return Err(ContextError::Internal("Tracing is not active".to_string()));
        }

        info!("Stopping trace and saving");

        // Stop tracing on all sessions
        for session_id in self.get_session_ids().await {
            let _ = self
                .connection
                .send_command::<_, serde_json::Value>("Tracing.end", None::<()>, Some(&session_id))
                .await;
        }

        state.is_recording = false;

        // Write trace file
        writer::write_trace_zip(path, &state)?;

        Ok(())
    }

    /// Stop tracing and discard the trace data.
    ///
    /// Use this when you don't need to save the trace (e.g., test passed).
    ///
    /// # Errors
    ///
    /// Returns an error if tracing is not active.
    #[instrument(level = "info", skip(self))]
    pub async fn stop_discard(&self) -> Result<(), ContextError> {
        let mut state = self.state.write().await;

        if !state.is_recording {
            return Err(ContextError::Internal("Tracing is not active".to_string()));
        }

        info!("Stopping trace and discarding");

        // Stop tracing on all sessions
        for session_id in self.get_session_ids().await {
            let _ = self
                .connection
                .send_command::<_, serde_json::Value>("Tracing.end", None::<()>, Some(&session_id))
                .await;
        }

        // Clear state
        state.is_recording = false;
        state.actions.clear();
        state.events.clear();
        state.screenshots.clear();
        state.snapshots.clear();
        state.pending_requests.clear();
        state.network_entries.clear();
        state.har_pages.clear();
        state.source_files.clear();

        Ok(())
    }

    /// Start a new trace chunk.
    ///
    /// This is useful for long-running tests where you want to save
    /// periodic snapshots.
    ///
    /// # Errors
    ///
    /// Returns an error if tracing is not active.
    #[instrument(level = "debug", skip(self))]
    pub async fn start_chunk(&self) -> Result<(), ContextError> {
        let state = self.state.read().await;

        if !state.is_recording {
            return Err(ContextError::Internal("Tracing is not active".to_string()));
        }

        debug!("Starting new trace chunk");

        // In a full implementation, this would rotate the trace data
        // For now, we just continue recording

        Ok(())
    }

    /// Stop the current trace chunk and save it.
    ///
    /// # Errors
    ///
    /// Returns an error if tracing is not active or saving fails.
    #[instrument(level = "debug", skip(self), fields(path = %path.as_ref().display()))]
    pub async fn stop_chunk(&self, path: impl AsRef<std::path::Path>) -> Result<(), ContextError> {
        let path = path.as_ref();
        let state = self.state.read().await;

        if !state.is_recording {
            return Err(ContextError::Internal("Tracing is not active".to_string()));
        }

        debug!("Stopping trace chunk and saving");

        // Write current state to file
        writer::write_trace_zip(path, &state)?;

        // Note: In a full implementation, we would clear the current chunk
        // and continue recording for the next chunk

        Ok(())
    }

    /// Check if tracing is currently active.
    pub async fn is_recording(&self) -> bool {
        self.state.read().await.is_recording
    }

    /// Add a source file to include in the trace.
    ///
    /// Source files are shown in the Trace Viewer for debugging.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Browser;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// # let browser = Browser::launch().headless(true).launch().await?;
    /// # let context = browser.new_context().await?;
    /// context.tracing().add_source_file(
    ///     "tests/my_test.rs",
    ///     "// test source code"
    /// ).await;
    /// # Ok(())
    /// # }
    pub async fn add_source_file(&self, path: impl Into<String>, content: impl Into<String>) {
        let mut state = self.state.write().await;
        state.source_files.push(SourceFileEntry {
            path: path.into(),
            content: content.into(),
        });
    }

    /// Collect source files from a directory.
    ///
    /// This recursively adds all matching files from the directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - Directory to scan
    /// * `extensions` - File extensions to include (e.g., `["rs", "ts"]`)
    ///
    /// # Errors
    ///
    /// Returns an error if reading files fails.
    pub async fn collect_sources(
        &self,
        dir: impl AsRef<std::path::Path>,
        extensions: &[&str],
    ) -> Result<(), ContextError> {
        let files = sources::collect_sources_from_dir(dir.as_ref(), extensions)?;

        let mut state = self.state.write().await;
        for (path, content) in files {
            state.source_files.push(SourceFileEntry { path, content });
        }

        Ok(())
    }

    /// Record an action in the trace.
    ///
    /// Returns a handle that must be used to complete or fail the action.
    pub(crate) async fn record_action(
        &self,
        action_type: &str,
        selector: Option<&str>,
        page_id: Option<&str>,
    ) -> ActionHandle {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            * 1000.0;

        let action = ActionEntry {
            action_type: action_type.to_string(),
            selector: selector.map(ToString::to_string),
            page_id: page_id.map(ToString::to_string),
            start_time,
            end_time: None,
            result: None,
            value: None,
            url: None,
            screenshot: None,
            snapshot: None,
        };

        let mut state = self.state.write().await;
        let index = state.actions.len();
        state.actions.push(action);

        ActionHandle::new(self.state.clone(), index)
    }

    /// Record a page being created.
    pub(crate) async fn record_page(&self, page_id: &str, title: &str) {
        let mut state = self.state.write().await;
        let started_date_time = Utc::now().to_rfc3339();
        let page = HarPage::new(page_id, title, &started_date_time);
        state.har_pages.push(page);
        state.current_page_id = Some(page_id.to_string());
    }

    /// Capture a screenshot and add it to the trace.
    pub(crate) async fn capture_screenshot(
        &self,
        session_id: &str,
        name: Option<&str>,
    ) -> Result<(), ContextError> {
        capture::capture_screenshot(&self.connection, &self.state, session_id, name).await
    }

    /// Capture a DOM snapshot and add it to the trace.
    pub(crate) async fn capture_dom_snapshot(&self, session_id: &str) -> Result<(), ContextError> {
        capture::capture_dom_snapshot(&self.connection, &self.state, session_id).await
    }

    /// Capture action context (screenshot + snapshot) if enabled.
    pub(crate) async fn capture_action_context(
        &self,
        session_id: &str,
        action_name: Option<&str>,
    ) -> Result<(), ContextError> {
        capture::capture_action_context(&self.connection, &self.state, session_id, action_name)
            .await
    }
}
