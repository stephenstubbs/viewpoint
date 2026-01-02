//! Context-related types.

mod cookies;
mod options;
mod proxy;
mod storage;

use serde::{Deserialize, Serialize};

// Re-export all types
pub use cookies::{Cookie, SameSite};
pub use options::{ContextOptions, ContextOptionsBuilder, StorageStateSource};
pub use proxy::ProxyConfig;
pub use storage::{
    IndexedDbDatabase, IndexedDbEntry, IndexedDbIndex, IndexedDbObjectStore, LocalStorageEntry,
    StorageOrigin, StorageState,
};

// =============================================================================
// Geolocation Types
// =============================================================================

/// Geolocation coordinates for mocking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Geolocation {
    /// Latitude in degrees (-90 to 90).
    pub latitude: f64,
    /// Longitude in degrees (-180 to 180).
    pub longitude: f64,
    /// Accuracy in meters (defaults to 0 for exact).
    #[serde(default)]
    pub accuracy: f64,
}

impl Geolocation {
    /// Create a new geolocation with coordinates.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            accuracy: 0.0,
        }
    }

    /// Create a geolocation with accuracy.
    pub fn with_accuracy(latitude: f64, longitude: f64, accuracy: f64) -> Self {
        Self {
            latitude,
            longitude,
            accuracy,
        }
    }
}

// =============================================================================
// HTTP Credentials Types
// =============================================================================

/// HTTP credentials for Basic/Digest authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpCredentials {
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
    /// Origin to apply credentials to (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

impl HttpCredentials {
    /// Create new HTTP credentials.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            origin: None,
        }
    }

    /// Set the origin.
    #[must_use]
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
}

// =============================================================================
// Permission Types
// =============================================================================

/// Browser permission type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    /// Geolocation access.
    Geolocation,
    /// Notification permission.
    Notifications,
    /// Camera/video capture access.
    Camera,
    /// Microphone/audio capture access.
    Microphone,
    /// Clipboard read access.
    ClipboardRead,
    /// Clipboard write access.
    ClipboardWrite,
    /// Background sync.
    BackgroundSync,
    /// MIDI device access.
    Midi,
    /// MIDI system exclusive access.
    MidiSysex,
    /// Accelerometer sensor access.
    Accelerometer,
    /// Gyroscope sensor access.
    Gyroscope,
    /// Magnetometer sensor access.
    Magnetometer,
    /// Ambient light sensor access.
    AmbientLightSensor,
    /// Payment handler permission.
    PaymentHandler,
    /// Storage access API.
    StorageAccess,
}

impl Permission {
    /// Convert to CDP permission type name.
    pub fn to_cdp_permission(&self) -> viewpoint_cdp::protocol::browser::PermissionType {
        use viewpoint_cdp::protocol::browser::PermissionType;
        match self {
            Self::Geolocation => PermissionType::Geolocation,
            Self::Notifications => PermissionType::Notifications,
            Self::Camera => PermissionType::VideoCapture,
            Self::Microphone => PermissionType::AudioCapture,
            Self::ClipboardRead => PermissionType::ClipboardReadWrite,
            Self::ClipboardWrite => PermissionType::ClipboardSanitizedWrite,
            Self::BackgroundSync => PermissionType::BackgroundSync,
            Self::Midi => PermissionType::Midi,
            Self::MidiSysex => PermissionType::MidiSysex,
            Self::Accelerometer | Self::Gyroscope | Self::Magnetometer => PermissionType::Sensors,
            Self::AmbientLightSensor => PermissionType::Sensors,
            Self::PaymentHandler => PermissionType::PaymentHandler,
            Self::StorageAccess => PermissionType::StorageAccess,
        }
    }
}

// =============================================================================
// Viewport and Media Types
// =============================================================================

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

/// Color scheme preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColorScheme {
    /// Light color scheme.
    Light,
    /// Dark color scheme.
    Dark,
    /// No preference.
    NoPreference,
}

/// Reduced motion preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReducedMotion {
    /// Reduce motion.
    Reduce,
    /// No preference.
    NoPreference,
}

/// Forced colors preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ForcedColors {
    /// Active (forced colors mode).
    Active,
    /// None (no forced colors).
    None,
}

#[cfg(test)]
mod tests;
