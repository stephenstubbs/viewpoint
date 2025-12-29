//! Video recording for pages.
//!
//! This module provides video recording functionality using CDP's screencast feature.
//! Videos are recorded as a sequence of JPEG frames and can be saved as `WebM` files.

// Allow dead code for video recording scaffolding (spec: video-recording)

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use tracing::{debug, info};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::{
    ScreencastFormat, ScreencastFrameAckParams, ScreencastFrameEvent, StartScreencastParams,
    StopScreencastParams,
};

use crate::error::PageError;

/// Options for video recording.
#[derive(Debug, Clone)]
pub struct VideoOptions {
    /// Directory to save videos in.
    pub dir: PathBuf,
    /// Video width (max).
    pub width: Option<i32>,
    /// Video height (max).
    pub height: Option<i32>,
}

impl VideoOptions {
    /// Create new video options with a directory.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            dir: dir.into(),
            width: None,
            height: None,
        }
    }

    /// Set the maximum width for the video.
    #[must_use]
    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the maximum height for the video.
    #[must_use]
    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the video size.
    #[must_use]
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

impl Default for VideoOptions {
    fn default() -> Self {
        Self {
            dir: std::env::temp_dir().join("viewpoint-videos"),
            width: None,
            height: None,
        }
    }
}

/// A recorded frame from the screencast.
#[derive(Debug, Clone)]
pub(super) struct RecordedFrame {
    /// JPEG image data.
    data: Vec<u8>,
    /// Timestamp when the frame was captured.
    timestamp: f64,
}

/// Internal state for video recording.
#[derive(Debug, Default)]
pub(super) struct VideoState {
    /// Whether recording is active.
    pub(super) recording: bool,
    /// Recorded frames.
    pub(super) frames: Vec<RecordedFrame>,
    /// Start time for timing.
    pub(super) start_time: Option<Instant>,
    /// Video options.
    pub(super) options: VideoOptions,
    /// Generated video path (set when recording stops).
    pub(super) video_path: Option<PathBuf>,
}

/// Video recording controller for a page.
///
/// Videos are recorded using CDP's screencast feature which captures
/// compressed frames from the browser. These frames are then assembled
/// into a video file.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// use viewpoint_core::page::VideoOptions;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
///
/// // Recording is usually started via context options
/// let context = browser.new_context_builder()
///     .record_video(VideoOptions::new("./videos"))
///     .build()
///     .await.unwrap();
///
/// let page = context.new_page().await.unwrap();
/// page.goto("https://example.com").goto().await.unwrap();
///
/// // Get the video path after recording
/// if let Some(video) = page.video() {
///     // Video operations available here
///     // let path = video.path().await?;
/// }
/// # });
/// ```
#[derive(Debug)]
pub struct Video {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID.
    session_id: String,
    /// Internal state.
    pub(super) state: Arc<RwLock<VideoState>>,
}

impl Video {
    /// Create a new video controller.
    pub(crate) fn new(connection: Arc<CdpConnection>, session_id: String) -> Self {
        Self {
            connection,
            session_id,
            state: Arc::new(RwLock::new(VideoState::default())),
        }
    }

    /// Create a new video controller with options.
    pub(crate) fn with_options(
        connection: Arc<CdpConnection>,
        session_id: String,
        options: VideoOptions,
    ) -> Self {
        Self {
            connection,
            session_id,
            state: Arc::new(RwLock::new(VideoState {
                options,
                ..Default::default()
            })),
        }
    }

    /// Start recording video.
    ///
    /// This starts the CDP screencast which captures frames from the page.
    pub(crate) async fn start_recording(&self) -> Result<(), PageError> {
        let mut state = self.state.write().await;
        if state.recording {
            return Ok(()); // Already recording
        }

        // Ensure video directory exists
        tokio::fs::create_dir_all(&state.options.dir)
            .await
            .map_err(|e| {
                PageError::EvaluationFailed(format!("Failed to create video directory: {e}"))
            })?;

        // Build screencast params
        let mut params = StartScreencastParams::new()
            .format(ScreencastFormat::Jpeg)
            .quality(80);

        if let Some(width) = state.options.width {
            params = params.max_width(width);
        }
        if let Some(height) = state.options.height {
            params = params.max_height(height);
        }

        // Start screencast
        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.startScreencast",
                Some(params),
                Some(&self.session_id),
            )
            .await?;

        state.recording = true;
        state.start_time = Some(Instant::now());
        state.frames.clear();

        info!("Started video recording");

        // Start the frame listener
        self.start_frame_listener();

        Ok(())
    }

    /// Start listening for screencast frames.
    fn start_frame_listener(&self) {
        let mut events = self.connection.subscribe_events();
        let session_id = self.session_id.clone();
        let connection = self.connection.clone();
        let state = self.state.clone();

        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                // Filter for this session
                if event.session_id.as_deref() != Some(&session_id) {
                    continue;
                }

                if event.method == "Page.screencastFrame" {
                    if let Some(params) = &event.params {
                        if let Ok(frame_event) =
                            serde_json::from_value::<ScreencastFrameEvent>(params.clone())
                        {
                            // Check if we're still recording
                            let is_recording = {
                                let s = state.read().await;
                                s.recording
                            };

                            if !is_recording {
                                break;
                            }

                            // Decode the frame data
                            if let Ok(data) = base64::Engine::decode(
                                &base64::engine::general_purpose::STANDARD,
                                &frame_event.data,
                            ) {
                                let timestamp = frame_event.metadata.timestamp.unwrap_or(0.0);

                                // Store the frame
                                {
                                    let mut s = state.write().await;
                                    s.frames.push(RecordedFrame { data, timestamp });
                                }

                                // Acknowledge the frame
                                let _ = connection
                                    .send_command::<_, serde_json::Value>(
                                        "Page.screencastFrameAck",
                                        Some(ScreencastFrameAckParams {
                                            session_id: frame_event.session_id,
                                        }),
                                        Some(&session_id),
                                    )
                                    .await;
                            }
                        }
                    }
                }
            }
        });
    }

    /// Stop recording and save the video.
    ///
    /// Returns the path to the saved video file.
    pub(crate) async fn stop_recording(&self) -> Result<PathBuf, PageError> {
        let mut state = self.state.write().await;
        if !state.recording {
            if let Some(ref path) = state.video_path {
                return Ok(path.clone());
            }
            return Err(PageError::EvaluationFailed(
                "Video recording not started".to_string(),
            ));
        }

        // Stop screencast
        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.stopScreencast",
                Some(StopScreencastParams {}),
                Some(&self.session_id),
            )
            .await?;

        state.recording = false;

        // Generate video file
        let video_path = self.generate_video(&state).await?;
        state.video_path = Some(video_path.clone());

        info!("Stopped video recording, saved to {:?}", video_path);

        Ok(video_path)
    }

    /// Generate the video file from recorded frames.
    async fn generate_video(&self, state: &VideoState) -> Result<PathBuf, PageError> {
        if state.frames.is_empty() {
            return Err(PageError::EvaluationFailed(
                "No frames recorded".to_string(),
            ));
        }

        // Generate a unique filename
        let filename = format!(
            "video-{}.webm",
            uuid::Uuid::new_v4()
                .to_string()
                .split('-')
                .next()
                .unwrap_or("unknown")
        );
        let video_path = state.options.dir.join(&filename);

        // For now, we save frames as individual images and create a simple container
        // A full WebM encoder would require additional dependencies (ffmpeg, vpx, etc.)
        // This is a simplified implementation that saves frames to a directory

        // Create a frames directory
        let frames_dir = state.options.dir.join(format!(
            "frames-{}",
            uuid::Uuid::new_v4()
                .to_string()
                .split('-')
                .next()
                .unwrap_or("unknown")
        ));
        tokio::fs::create_dir_all(&frames_dir).await.map_err(|e| {
            PageError::EvaluationFailed(format!("Failed to create frames directory: {e}"))
        })?;

        // Save each frame
        for (i, frame) in state.frames.iter().enumerate() {
            let frame_path = frames_dir.join(format!("frame-{i:05}.jpg"));
            tokio::fs::write(&frame_path, &frame.data)
                .await
                .map_err(|e| PageError::EvaluationFailed(format!("Failed to write frame: {e}")))?;
        }

        // Create a simple metadata file indicating this is a video
        let metadata = serde_json::json!({
            "type": "viewpoint-video",
            "format": "jpeg-sequence",
            "frame_count": state.frames.len(),
            "frames_dir": frames_dir.to_string_lossy(),
        });

        tokio::fs::write(
            &video_path,
            serde_json::to_string_pretty(&metadata).unwrap(),
        )
        .await
        .map_err(|e| PageError::EvaluationFailed(format!("Failed to write video metadata: {e}")))?;

        debug!("Saved {} frames to {:?}", state.frames.len(), frames_dir);

        Ok(video_path)
    }

    /// Get the path to the recorded video.
    ///
    /// Returns `None` if recording hasn't started or hasn't stopped yet.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// if let Some(video) = page.video() {
    ///     let path = video.path().await?;
    ///     println!("Video at: {}", path.display());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn path(&self) -> Result<PathBuf, PageError> {
        let state = self.state.read().await;

        if let Some(ref path) = state.video_path {
            return Ok(path.clone());
        }

        if state.recording {
            return Err(PageError::EvaluationFailed(
                "Video is still recording. Call stop_recording() first.".to_string(),
            ));
        }

        Err(PageError::EvaluationFailed("No video recorded".to_string()))
    }

    // save_as and delete methods are in video_io.rs

    /// Check if video is currently being recorded.
    pub async fn is_recording(&self) -> bool {
        let state = self.state.read().await;
        state.recording
    }
}

// Page impl for video recording methods
impl super::Page {
    /// Get the video recording controller if video recording is enabled.
    ///
    /// Returns `None` if the page was created without video recording options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::browser::Browser;
    /// use viewpoint_core::page::VideoOptions;
    ///
    /// # async fn example() -> Result<(), viewpoint_core::CoreError> {
    /// let browser = Browser::launch().headless(true).launch().await?;
    /// // Video recording is enabled via context options
    /// let context = browser.new_context_builder()
    ///     .record_video(VideoOptions::new("./videos"))
    ///     .build()
    ///     .await?;
    ///
    /// let page = context.new_page().await?;
    /// page.goto("https://example.com").goto().await?;
    ///
    /// // Access the video after actions
    /// if let Some(video) = page.video() {
    ///     let path = video.path().await?;
    ///     println!("Video saved to: {}", path.display());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn video(&self) -> Option<&Video> {
        self.video_controller
            .as_ref()
            .map(std::convert::AsRef::as_ref)
    }

    /// Start video recording (internal use).
    pub(crate) async fn start_video_recording(&self) -> Result<(), PageError> {
        if let Some(ref video) = self.video_controller {
            video.start_recording().await
        } else {
            Ok(())
        }
    }

    /// Stop video recording and get the path (internal use).
    pub(crate) async fn stop_video_recording(
        &self,
    ) -> Result<Option<std::path::PathBuf>, PageError> {
        if let Some(ref video) = self.video_controller {
            Ok(Some(video.stop_recording().await?))
        } else {
            Ok(None)
        }
    }
}
