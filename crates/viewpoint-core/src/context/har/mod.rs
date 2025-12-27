//! Context-level HAR recording.

use std::path::PathBuf;

use tracing::debug;

use crate::error::NetworkError;
use crate::network::{HarRecorder, HarRecordingBuilder, HarRecordingOptions};

use super::BrowserContext;

impl BrowserContext {
    /// Start recording network traffic to a HAR file.
    ///
    /// All network requests and responses will be captured and saved to the
    /// specified path when `close()` is called or when `save_har()` is called.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Basic recording
    /// context.record_har("output.har").await?;
    ///
    /// // Navigate and make requests...
    /// page.goto("https://example.com").await?;
    ///
    /// // HAR is saved automatically on context.close()
    /// context.close().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - HAR recording is already active
    /// - The context is closed
    pub async fn record_har(&self, path: impl Into<PathBuf>) -> Result<HarRecordingBuilder, NetworkError> {
        if self.is_closed() {
            return Err(NetworkError::Aborted);
        }

        let recorder = self.har_recorder.read().await;
        if recorder.is_some() {
            return Err(NetworkError::AlreadyHandled);
        }
        drop(recorder);

        Ok(HarRecordingBuilder::new(path))
    }

    /// Start HAR recording with the given options.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use viewpoint_core::network::HarRecordingBuilder;
    ///
    /// // Record only API requests
    /// context.start_har_recording(
    ///     HarRecordingBuilder::new("api.har")
    ///         .url_filter("**/api/**")
    ///         .build()
    /// ).await?;
    ///
    /// // Omit response content
    /// context.start_har_recording(
    ///     HarRecordingBuilder::new("requests.har")
    ///         .omit_content(true)
    ///         .build()
    /// ).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed or HAR recording is already active.
    pub async fn start_har_recording(
        &self,
        options: HarRecordingOptions,
    ) -> Result<(), NetworkError> {
        if self.is_closed() {
            return Err(NetworkError::Aborted);
        }

        let mut recorder_lock = self.har_recorder.write().await;
        if recorder_lock.is_some() {
            return Err(NetworkError::AlreadyHandled);
        }

        let recorder = HarRecorder::new(options)?;
        *recorder_lock = Some(recorder);
        
        debug!("Started HAR recording");
        Ok(())
    }

    /// Save the current HAR recording to file.
    ///
    /// # Example
    ///
    /// ```ignore
    /// context.record_har("output.har").await?;
    /// // ... do some navigation ...
    /// let path = context.save_har().await?;
    /// println!("HAR saved to: {}", path.display());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No HAR recording is active
    /// - Failed to write the file
    pub async fn save_har(&self) -> Result<PathBuf, NetworkError> {
        let recorder = self.har_recorder.read().await;
        match recorder.as_ref() {
            Some(rec) => rec.save().await,
            None => Err(NetworkError::InvalidResponse(
                "No HAR recording is active".to_string(),
            )),
        }
    }

    /// Stop HAR recording and optionally save to file.
    ///
    /// If `save` is true, the HAR file is saved before stopping.
    ///
    /// # Errors
    ///
    /// Returns an error if saving the HAR file fails.
    pub async fn stop_har_recording(&self, save: bool) -> Result<Option<PathBuf>, NetworkError> {
        let mut recorder_lock = self.har_recorder.write().await;
        if let Some(recorder) = recorder_lock.take() {
            if save {
                let path = recorder.save().await?;
                return Ok(Some(path));
            }
        }
        Ok(None)
    }
}
