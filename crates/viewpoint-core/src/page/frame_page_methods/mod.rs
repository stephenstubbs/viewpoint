//! Page methods for frame management.
//!
//! This module contains the frame-related methods on the `Page` struct.

use tracing::instrument;

use super::frame::Frame;
use super::frame_locator::FrameLocator;
use super::Page;
use crate::error::PageError;

// =========================================================================
// Page Frame Methods
// =========================================================================

impl Page {
    /// Create a locator for an iframe element.
    ///
    /// Frame locators allow targeting elements inside iframes. They can be chained
    /// to navigate through nested iframes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Target an element inside an iframe
    /// page.frame_locator("#my-iframe")
    ///     .locator("button")
    ///     .click()
    ///     .await?;
    ///
    /// // Navigate through nested iframes
    /// page.frame_locator("#outer")
    ///     .frame_locator("#inner")
    ///     .locator("input")
    ///     .fill("text")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn frame_locator(&self, selector: impl Into<String>) -> FrameLocator<'_> {
        FrameLocator::new(self, selector)
    }

    /// Get the main frame of the page.
    ///
    /// The main frame is the top-level frame that contains the page content.
    /// All other frames are child frames (iframes) of this frame.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame tree cannot be retrieved.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// let main_frame = page.main_frame().await?;
    /// println!("Main frame URL: {}", main_frame.url());
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self))]
    pub async fn main_frame(&self) -> Result<Frame, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::page::GetFrameTreeResult = self
            .connection
            .send_command("Page.getFrameTree", None::<()>, Some(&self.session_id))
            .await?;

        let frame_info = result.frame_tree.frame;

        Ok(Frame::new(
            self.connection.clone(),
            self.session_id.clone(),
            frame_info.id,
            frame_info.parent_id,
            frame_info.loader_id,
            frame_info.url,
            frame_info.name.unwrap_or_default(),
        ))
    }

    /// Get all frames in the page, including the main frame and all iframes.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame tree cannot be retrieved.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// let frames = page.frames().await?;
    /// for frame in frames {
    ///     println!("Frame: {} - {}", frame.name(), frame.url());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self))]
    pub async fn frames(&self) -> Result<Vec<Frame>, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::page::GetFrameTreeResult = self
            .connection
            .send_command("Page.getFrameTree", None::<()>, Some(&self.session_id))
            .await?;

        let mut frames = Vec::new();
        self.collect_frames(&result.frame_tree, &mut frames);

        Ok(frames)
    }

    /// Collect frames recursively from a frame tree.
    fn collect_frames(
        &self,
        tree: &viewpoint_cdp::protocol::page::FrameTree,
        frames: &mut Vec<Frame>,
    ) {
        let frame_info = &tree.frame;

        frames.push(Frame::new(
            self.connection.clone(),
            self.session_id.clone(),
            frame_info.id.clone(),
            frame_info.parent_id.clone(),
            frame_info.loader_id.clone(),
            frame_info.url.clone(),
            frame_info.name.clone().unwrap_or_default(),
        ));

        if let Some(children) = &tree.child_frames {
            for child in children {
                self.collect_frames(child, frames);
            }
        }
    }

    /// Get a frame by its name attribute.
    ///
    /// Returns `None` if no frame with the given name is found.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame tree cannot be retrieved.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// if let Some(frame) = page.frame("payment-frame").await? {
    ///     frame.goto("https://payment.example.com").await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self), fields(name = %name))]
    pub async fn frame(&self, name: &str) -> Result<Option<Frame>, PageError> {
        let frames = self.frames().await?;

        for frame in frames {
            if frame.name() == name {
                return Ok(Some(frame));
            }
        }

        Ok(None)
    }

    /// Get a frame by URL pattern.
    ///
    /// The pattern can be a glob pattern (e.g., "**/payment/**") or an exact URL.
    /// Returns the first frame whose URL matches the pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame tree cannot be retrieved.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// if let Some(frame) = page.frame_by_url("**/checkout/**").await? {
    ///     println!("Found checkout frame: {}", frame.url());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self), fields(pattern = %pattern))]
    pub async fn frame_by_url(&self, pattern: &str) -> Result<Option<Frame>, PageError> {
        let frames = self.frames().await?;

        // Convert glob pattern to regex
        let regex_pattern = pattern
            .replace("**", ".*")
            .replace('*', "[^/]*")
            .replace('?', ".");

        let regex = regex::Regex::new(&format!("^{regex_pattern}$"))
            .map_err(|e| PageError::EvaluationFailed(format!("Invalid URL pattern: {e}")))?;

        for frame in frames {
            if regex.is_match(&frame.url()) {
                return Ok(Some(frame));
            }
        }

        Ok(None)
    }
}
