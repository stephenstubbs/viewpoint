//! Page emulation features for media and vision deficiency emulation.

use std::sync::Arc;
use tracing::{debug, info, instrument};

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::emulation::{
    MediaFeature, SetDeviceMetricsOverrideParams, SetEmulatedMediaParams,
    SetEmulatedVisionDeficiencyParams, ViewportSize, VisionDeficiency as CdpVisionDeficiency,
};

use super::Page;
use crate::context::{ColorScheme, ForcedColors, ReducedMotion};
use crate::error::PageError;

/// Media type for CSS media emulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// Screen media type.
    Screen,
    /// Print media type.
    Print,
}

impl MediaType {
    /// Convert to CSS media type string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Screen => "screen",
            Self::Print => "print",
        }
    }
}

/// Vision deficiency types for accessibility testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VisionDeficiency {
    /// No vision deficiency emulation (normal vision).
    #[default]
    None,
    /// Achromatopsia - complete color blindness.
    Achromatopsia,
    /// Blurred vision.
    BlurredVision,
    /// Deuteranopia - green-blind (red-green color blindness).
    Deuteranopia,
    /// Protanopia - red-blind (red-green color blindness).
    Protanopia,
    /// Tritanopia - blue-blind (blue-yellow color blindness).
    Tritanopia,
}

impl From<VisionDeficiency> for CdpVisionDeficiency {
    fn from(deficiency: VisionDeficiency) -> Self {
        match deficiency {
            VisionDeficiency::None => CdpVisionDeficiency::None,
            VisionDeficiency::Achromatopsia => CdpVisionDeficiency::Achromatopsia,
            VisionDeficiency::BlurredVision => CdpVisionDeficiency::BlurredVision,
            VisionDeficiency::Deuteranopia => CdpVisionDeficiency::Deuteranopia,
            VisionDeficiency::Protanopia => CdpVisionDeficiency::Protanopia,
            VisionDeficiency::Tritanopia => CdpVisionDeficiency::Tritanopia,
        }
    }
}

/// Builder for emulating CSS media features on a page.
///
/// Use this to test how your application responds to different media preferences
/// like dark mode, print media, reduced motion, and forced colors.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// use viewpoint_core::{ColorScheme, ReducedMotion};
/// use viewpoint_core::page::MediaType;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
///
/// // Emulate dark mode
/// page.emulate_media()
///     .color_scheme(ColorScheme::Dark)
///     .apply()
///     .await.unwrap();
///
/// // Emulate print media
/// page.emulate_media()
///     .media(MediaType::Print)
///     .apply()
///     .await.unwrap();
///
/// // Combine multiple settings
/// page.emulate_media()
///     .color_scheme(ColorScheme::Dark)
///     .reduced_motion(ReducedMotion::Reduce)
///     .apply()
///     .await.unwrap();
///
/// // Clear all media emulation
/// page.emulate_media()
///     .clear()
///     .await.unwrap();
/// # });
/// ```
#[derive(Debug)]
pub struct EmulateMediaBuilder<'a> {
    connection: &'a Arc<CdpConnection>,
    session_id: &'a str,
    media: Option<MediaType>,
    color_scheme: Option<ColorScheme>,
    reduced_motion: Option<ReducedMotion>,
    forced_colors: Option<ForcedColors>,
}

impl<'a> EmulateMediaBuilder<'a> {
    /// Create a new emulate media builder.
    pub(crate) fn new(connection: &'a Arc<CdpConnection>, session_id: &'a str) -> Self {
        Self {
            connection,
            session_id,
            media: None,
            color_scheme: None,
            reduced_motion: None,
            forced_colors: None,
        }
    }

    /// Set the CSS media type (screen or print).
    #[must_use]
    pub fn media(mut self, media: MediaType) -> Self {
        self.media = Some(media);
        self
    }

    /// Set the color scheme preference.
    ///
    /// This affects CSS `prefers-color-scheme` media queries.
    #[must_use]
    pub fn color_scheme(mut self, color_scheme: ColorScheme) -> Self {
        self.color_scheme = Some(color_scheme);
        self
    }

    /// Set the reduced motion preference.
    ///
    /// This affects CSS `prefers-reduced-motion` media queries.
    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: ReducedMotion) -> Self {
        self.reduced_motion = Some(reduced_motion);
        self
    }

    /// Set the forced colors preference.
    ///
    /// This affects CSS `forced-colors` media queries.
    #[must_use]
    pub fn forced_colors(mut self, forced_colors: ForcedColors) -> Self {
        self.forced_colors = Some(forced_colors);
        self
    }

    /// Apply the media emulation settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the CDP command fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn apply(self) -> Result<(), PageError> {
        let mut features = Vec::new();

        if let Some(color_scheme) = self.color_scheme {
            features.push(MediaFeature {
                name: "prefers-color-scheme".to_string(),
                value: match color_scheme {
                    ColorScheme::Light => "light".to_string(),
                    ColorScheme::Dark => "dark".to_string(),
                    ColorScheme::NoPreference => "no-preference".to_string(),
                },
            });
        }

        if let Some(reduced_motion) = self.reduced_motion {
            features.push(MediaFeature {
                name: "prefers-reduced-motion".to_string(),
                value: match reduced_motion {
                    ReducedMotion::Reduce => "reduce".to_string(),
                    ReducedMotion::NoPreference => "no-preference".to_string(),
                },
            });
        }

        if let Some(forced_colors) = self.forced_colors {
            features.push(MediaFeature {
                name: "forced-colors".to_string(),
                value: match forced_colors {
                    ForcedColors::Active => "active".to_string(),
                    ForcedColors::None => "none".to_string(),
                },
            });
        }

        let media = self.media.map(|m| m.as_str().to_string());
        let features_opt = if features.is_empty() {
            None
        } else {
            Some(features)
        };

        debug!(
            media = ?media,
            features_count = features_opt.as_ref().map_or(0, std::vec::Vec::len),
            "Applying media emulation"
        );

        self.connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setEmulatedMedia",
                Some(SetEmulatedMediaParams {
                    media,
                    features: features_opt,
                }),
                Some(self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Clear all media emulation.
    ///
    /// This resets media type and all media features to their defaults.
    ///
    /// # Errors
    ///
    /// Returns an error if the CDP command fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn clear(self) -> Result<(), PageError> {
        debug!("Clearing media emulation");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setEmulatedMedia",
                Some(SetEmulatedMediaParams {
                    media: Some(String::new()), // Empty string clears media type
                    features: Some(Vec::new()), // Empty array clears features
                }),
                Some(self.session_id),
            )
            .await?;

        Ok(())
    }
}

/// Emulate a vision deficiency on the page (implementation).
///
/// This is useful for accessibility testing to ensure your application
/// is usable by people with various types of color blindness.
///
/// # Errors
///
/// Returns an error if the CDP command fails.
#[instrument(level = "debug", skip(connection))]
async fn emulate_vision_deficiency_impl(
    connection: &Arc<CdpConnection>,
    session_id: &str,
    deficiency: VisionDeficiency,
) -> Result<(), PageError> {
    debug!(?deficiency, "Emulating vision deficiency");

    connection
        .send_command::<_, serde_json::Value>(
            "Emulation.setEmulatedVisionDeficiency",
            Some(SetEmulatedVisionDeficiencyParams::new(deficiency.into())),
            Some(session_id),
        )
        .await?;

    Ok(())
}

// =============================================================================
// Page impl for viewport and emulation methods
// =============================================================================

impl Page {
    // =========================================================================
    // Viewport Methods
    // =========================================================================

    /// Get the current viewport size.
    ///
    /// Returns the width and height in pixels.
    pub fn viewport_size(&self) -> Option<ViewportSize> {
        // This would need to be tracked during page creation
        // For now, return None to indicate it's not set
        None
    }

    /// Set the viewport size.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.set_viewport_size(1280, 720).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "info", skip(self), fields(width = width, height = height))]
    pub async fn set_viewport_size(&self, width: i32, height: i32) -> Result<(), PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        self.connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setDeviceMetricsOverride",
                Some(SetDeviceMetricsOverrideParams {
                    width,
                    height,
                    device_scale_factor: 1.0,
                    mobile: false,
                    scale: None,
                    screen_width: None,
                    screen_height: None,
                    position_x: None,
                    position_y: None,
                    dont_set_visible_size: None,
                    screen_orientation: None,
                    viewport: None,
                    display_feature: None,
                    device_posture: None,
                }),
                Some(&self.session_id),
            )
            .await?;

        info!("Viewport size set to {}x{}", width, height);
        Ok(())
    }

    /// Bring this page to the front (activate the tab).
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the CDP command fails.
    #[instrument(level = "info", skip(self))]
    pub async fn bring_to_front(&self) -> Result<(), PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        self.connection
            .send_command::<_, serde_json::Value>(
                "Page.bringToFront",
                None::<()>,
                Some(&self.session_id),
            )
            .await?;

        info!("Page brought to front");
        Ok(())
    }

    // =========================================================================
    // Emulation Methods
    // =========================================================================

    /// Create a builder for emulating CSS media features.
    ///
    /// Use this to test how your application responds to different media preferences
    /// like dark mode, print media, reduced motion, and forced colors.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Page, page::MediaType};
    /// use viewpoint_core::{ColorScheme, ReducedMotion};
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Emulate dark mode
    /// page.emulate_media()
    ///     .color_scheme(ColorScheme::Dark)
    ///     .apply()
    ///     .await?;
    ///
    /// // Emulate print media
    /// page.emulate_media()
    ///     .media(MediaType::Print)
    ///     .apply()
    ///     .await?;
    ///
    /// // Combine multiple settings
    /// page.emulate_media()
    ///     .color_scheme(ColorScheme::Dark)
    ///     .reduced_motion(ReducedMotion::Reduce)
    ///     .apply()
    ///     .await?;
    ///
    /// // Clear all media emulation
    /// page.emulate_media()
    ///     .clear()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn emulate_media(&self) -> EmulateMediaBuilder<'_> {
        EmulateMediaBuilder::new(&self.connection, &self.session_id)
    }

    /// Emulate a vision deficiency on the page.
    ///
    /// This is useful for accessibility testing to ensure your application
    /// is usable by people with various types of color blindness.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::{Page, page::VisionDeficiency};
    ///
    /// # async fn example(page: Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Emulate deuteranopia (green-blind)
    /// page.emulate_vision_deficiency(VisionDeficiency::Deuteranopia).await?;
    ///
    /// // Clear vision deficiency emulation
    /// page.emulate_vision_deficiency(VisionDeficiency::None).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or the CDP command fails.
    pub async fn emulate_vision_deficiency(
        &self,
        deficiency: VisionDeficiency,
    ) -> Result<(), PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }
        emulate_vision_deficiency_impl(&self.connection, &self.session_id, deficiency).await
    }
}

#[cfg(test)]
mod tests;
