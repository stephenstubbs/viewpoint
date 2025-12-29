//! Video file I/O operations.
//!
//! This module handles saving, copying, and deleting video files.

use std::path::{Path, PathBuf};

use tracing::info;

use crate::error::PageError;

use super::video::Video;

impl Video {
    /// Save the video to a specific path.
    ///
    /// This copies the recorded video to the specified location.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::Video;
    ///
    /// # async fn example(video: &Video) -> Result<(), viewpoint_core::CoreError> {
    /// video.save_as("./test-results/my-test.webm").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save_as(&self, path: impl AsRef<Path>) -> Result<(), PageError> {
        let current_path = self.path().await?;
        let target_path = path.as_ref();

        // Ensure parent directory exists
        if let Some(parent) = target_path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                PageError::EvaluationFailed(format!("Failed to create directory: {e}"))
            })?;
        }

        // Copy the video file
        tokio::fs::copy(&current_path, target_path)
            .await
            .map_err(|e| PageError::EvaluationFailed(format!("Failed to copy video: {e}")))?;

        // Also copy the frames directory if it exists (for jpeg-sequence format)
        let state = self.state.read().await;
        if let Some(ref video_path) = state.video_path {
            // Read the metadata to find frames dir
            if let Ok(content) = tokio::fs::read_to_string(video_path).await {
                if let Ok(metadata) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(frames_dir) = metadata.get("frames_dir").and_then(|v| v.as_str()) {
                        let source_frames = PathBuf::from(frames_dir);
                        if source_frames.exists() {
                            let target_frames = target_path.with_extension("frames");
                            copy_dir_recursive(&source_frames, &target_frames).await?;
                        }
                    }
                }
            }
        }

        info!("Video saved to {:?}", target_path);
        Ok(())
    }

    /// Delete the recorded video.
    ///
    /// This removes the video file and any associated frame data.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::Video;
    ///
    /// # async fn example(video: &Video) -> Result<(), viewpoint_core::CoreError> {
    /// video.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self) -> Result<(), PageError> {
        let state = self.state.read().await;

        if let Some(ref video_path) = state.video_path {
            // Read the metadata to find frames dir
            if let Ok(content) = tokio::fs::read_to_string(video_path).await {
                if let Ok(metadata) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(frames_dir) = metadata.get("frames_dir").and_then(|v| v.as_str()) {
                        let frames_path = PathBuf::from(frames_dir);
                        if frames_path.exists() {
                            let _ = tokio::fs::remove_dir_all(&frames_path).await;
                        }
                    }
                }
            }

            // Remove the video file
            tokio::fs::remove_file(video_path)
                .await
                .map_err(|e| PageError::EvaluationFailed(format!("Failed to delete video: {e}")))?;

            info!("Video deleted");
        }

        Ok(())
    }
}

/// Recursively copy a directory.
pub(super) async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), PageError> {
    tokio::fs::create_dir_all(dst)
        .await
        .map_err(|e| PageError::EvaluationFailed(format!("Failed to create directory: {e}")))?;

    let mut entries = tokio::fs::read_dir(src)
        .await
        .map_err(|e| PageError::EvaluationFailed(format!("Failed to read directory: {e}")))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| PageError::EvaluationFailed(format!("Failed to read directory entry: {e}")))?
    {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            Box::pin(copy_dir_recursive(&src_path, &dst_path)).await?;
        } else {
            tokio::fs::copy(&src_path, &dst_path)
                .await
                .map_err(|e| PageError::EvaluationFailed(format!("Failed to copy file: {e}")))?;
        }
    }

    Ok(())
}
