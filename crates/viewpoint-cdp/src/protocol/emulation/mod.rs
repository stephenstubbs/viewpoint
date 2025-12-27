//! Emulation domain types.
//!
//! The Emulation domain emulates different device metrics and capabilities.

use serde::{Deserialize, Serialize};

/// Screen orientation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenOrientation {
    /// Orientation type.
    #[serde(rename = "type")]
    pub orientation_type: ScreenOrientationType,
    /// Orientation angle.
    pub angle: i32,
}

/// Screen orientation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenOrientationType {
    /// Portrait orientation (primary).
    PortraitPrimary,
    /// Portrait orientation (secondary).
    PortraitSecondary,
    /// Landscape orientation (primary).
    LandscapePrimary,
    /// Landscape orientation (secondary).
    LandscapeSecondary,
}

/// Display feature.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayFeature {
    /// Orientation of the display feature.
    pub orientation: DisplayFeatureOrientation,
    /// The offset from the screen origin in pixels.
    pub offset: i32,
    /// The length of the feature in pixels.
    pub mask_length: i32,
}

/// Display feature orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayFeatureOrientation {
    /// Vertical orientation.
    Vertical,
    /// Horizontal orientation.
    Horizontal,
}

/// Device posture.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePosture {
    /// Current posture type.
    #[serde(rename = "type")]
    pub posture_type: DevicePostureType,
}

/// Device posture type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DevicePostureType {
    /// Continuous posture.
    Continuous,
    /// Folded posture.
    Folded,
}

/// Parameters for Emulation.setDeviceMetricsOverride.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceMetricsOverrideParams {
    /// Overriding width value in pixels (0 disables override).
    pub width: i32,
    /// Overriding height value in pixels (0 disables override).
    pub height: i32,
    /// Overriding device scale factor value (0 disables override).
    pub device_scale_factor: f64,
    /// Whether to emulate mobile device.
    pub mobile: bool,
    /// Scale to apply to resulting view image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Overriding screen width value in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<i32>,
    /// Overriding screen height value in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_height: Option<i32>,
    /// Overriding view X position on screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_x: Option<i32>,
    /// Overriding view Y position on screen in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_y: Option<i32>,
    /// Do not set visible view size, rely upon explicit setVisibleSize call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_set_visible_size: Option<bool>,
    /// Screen orientation override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_orientation: Option<ScreenOrientation>,
    /// The viewport dimensions and scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport>,
    /// Display feature for foldable devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_feature: Option<DisplayFeature>,
    /// Device posture for foldable devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_posture: Option<DevicePosture>,
}

/// Viewport for device metrics override.
#[derive(Debug, Clone, Serialize)]
pub struct Viewport {
    /// X offset in viewport.
    pub x: f64,
    /// Y offset in viewport.
    pub y: f64,
    /// Viewport width.
    pub width: f64,
    /// Viewport height.
    pub height: f64,
    /// Viewport scale.
    pub scale: f64,
}

/// Parameters for Emulation.clearDeviceMetricsOverride (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct ClearDeviceMetricsOverrideParams {}

/// Media type for emulation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// Print media.
    Print,
    /// Screen media.
    Screen,
}

/// Parameters for Emulation.setEmulatedMedia.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedMediaParams {
    /// Media type to emulate. Empty string disables the override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<String>,
    /// Media features to emulate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<MediaFeature>>,
}

/// Media feature for emulation.
#[derive(Debug, Clone, Serialize)]
pub struct MediaFeature {
    /// Feature name.
    pub name: String,
    /// Feature value.
    pub value: String,
}

/// Viewport size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewportSize {
    /// Width in pixels.
    pub width: i32,
    /// Height in pixels.
    pub height: i32,
}

impl ViewportSize {
    /// Create a new viewport size.
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

/// Parameters for Emulation.setTouchEmulationEnabled.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledParams {
    /// Whether touch emulation is enabled.
    pub enabled: bool,
    /// Maximum touch points. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_touch_points: Option<i32>,
}

// =============================================================================
// Geolocation Emulation
// =============================================================================

/// Parameters for Emulation.setGeolocationOverride.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetGeolocationOverrideParams {
    /// Latitude in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Longitude in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Accuracy in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
}

impl SetGeolocationOverrideParams {
    /// Create geolocation override with coordinates.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude: Some(latitude),
            longitude: Some(longitude),
            accuracy: Some(0.0), // Default to exact accuracy
        }
    }

    /// Create geolocation override with coordinates and accuracy.
    pub fn with_accuracy(latitude: f64, longitude: f64, accuracy: f64) -> Self {
        Self {
            latitude: Some(latitude),
            longitude: Some(longitude),
            accuracy: Some(accuracy),
        }
    }

    /// Create params that indicate position unavailable.
    pub fn unavailable() -> Self {
        Self::default()
    }
}

/// Parameters for Emulation.clearGeolocationOverride (empty).
#[derive(Debug, Clone, Serialize, Default)]
pub struct ClearGeolocationOverrideParams {}

// =============================================================================
// Timezone Emulation
// =============================================================================

/// Parameters for Emulation.setTimezoneOverride.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTimezoneOverrideParams {
    /// The timezone identifier. If empty, disables the override.
    pub timezone_id: String,
}

impl SetTimezoneOverrideParams {
    /// Create timezone override.
    pub fn new(timezone_id: impl Into<String>) -> Self {
        Self {
            timezone_id: timezone_id.into(),
        }
    }
}

// =============================================================================
// Locale Emulation
// =============================================================================

/// Parameters for Emulation.setLocaleOverride.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLocaleOverrideParams {
    /// ICU locale. Empty string disables the override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl SetLocaleOverrideParams {
    /// Create locale override.
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: Some(locale.into()),
        }
    }

    /// Clear locale override.
    pub fn clear() -> Self {
        Self { locale: None }
    }
}

// =============================================================================
// User Agent Emulation
// =============================================================================

/// Parameters for Emulation.setUserAgentOverride.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideParams {
    /// User agent to use.
    pub user_agent: String,
    /// Browser language to emulate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// The platform navigator.platform should return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// User agent client hints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent_metadata: Option<UserAgentMetadata>,
}

impl SetUserAgentOverrideParams {
    /// Create user agent override.
    pub fn new(user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: user_agent.into(),
            accept_language: None,
            platform: None,
            user_agent_metadata: None,
        }
    }
}

/// User agent metadata.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentMetadata {
    /// Brands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<UserAgentBrandVersion>>,
    /// Full version list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_version_list: Option<Vec<UserAgentBrandVersion>>,
    /// Full version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_version: Option<String>,
    /// Platform.
    pub platform: String,
    /// Platform version.
    pub platform_version: String,
    /// Architecture.
    pub architecture: String,
    /// Model.
    pub model: String,
    /// Mobile.
    pub mobile: bool,
    /// Bitness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitness: Option<String>,
    /// Whether on a `WoW64` machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wow64: Option<bool>,
}

/// User agent brand version.
#[derive(Debug, Clone, Serialize)]
pub struct UserAgentBrandVersion {
    /// Brand.
    pub brand: String,
    /// Version.
    pub version: String,
}

// =============================================================================
// Vision Deficiency Emulation
// =============================================================================

/// Vision deficiency type for emulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum VisionDeficiency {
    /// No vision deficiency emulation.
    #[default]
    None,
    /// Achromatopsia (no color vision).
    Achromatopsia,
    /// Blue blindness.
    BlurredVision,
    /// Deuteranopia (green-blind).
    Deuteranopia,
    /// Protanopia (red-blind).
    Protanopia,
    /// Tritanopia (blue-blind).
    Tritanopia,
}


/// Parameters for Emulation.setEmulatedVisionDeficiency.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedVisionDeficiencyParams {
    /// Vision deficiency type to emulate.
    #[serde(rename = "type")]
    pub deficiency_type: VisionDeficiency,
}

impl SetEmulatedVisionDeficiencyParams {
    /// Create vision deficiency emulation params.
    pub fn new(deficiency_type: VisionDeficiency) -> Self {
        Self { deficiency_type }
    }

    /// Clear vision deficiency emulation.
    pub fn clear() -> Self {
        Self {
            deficiency_type: VisionDeficiency::None,
        }
    }
}
