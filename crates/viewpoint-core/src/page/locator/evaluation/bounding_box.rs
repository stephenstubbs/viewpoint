//! Bounding box methods.

use super::super::Locator;
use super::super::element::BoundingBox;
use crate::error::LocatorError;

impl Locator<'_> {
    /// Get the bounding box of the element.
    ///
    /// Returns the element's position and dimensions relative to the viewport.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let bbox = page.locator("button").bounding_box().await?;
    /// if let Some(box_) = bbox {
    ///     println!("Element at ({}, {}), size {}x{}",
    ///         box_.x, box_.y, box_.width, box_.height);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// - `Some(BoundingBox)` if the element exists and is visible
    /// - `None` if the element exists but has no visible bounding box
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be found.
    pub async fn bounding_box(&self) -> Result<Option<BoundingBox>, LocatorError> {
        let info = self.query_element_info().await?;

        if !info.found {
            return Err(LocatorError::NotFound(format!("{:?}", self.selector)));
        }

        match (info.x, info.y, info.width, info.height) {
            (Some(x), Some(y), Some(width), Some(height)) if width > 0.0 && height > 0.0 => {
                Ok(Some(BoundingBox {
                    x,
                    y,
                    width,
                    height,
                }))
            }
            _ => Ok(None),
        }
    }
}
