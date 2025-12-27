//! Browser domain types.
//!
//! The Browser domain defines methods and events for browser management.

use serde::{Deserialize, Serialize};

/// Browser permission type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionType {
    /// Accessibility events permission.
    AccessibilityEvents,
    /// Audio capture permission.
    AudioCapture,
    /// Background sync permission.
    BackgroundSync,
    /// Background fetch permission.
    BackgroundFetch,
    /// Captured surface control permission.
    CapturedSurfaceControl,
    /// Clipboard read permission.
    ClipboardReadWrite,
    /// Clipboard sanitized write permission.
    ClipboardSanitizedWrite,
    /// Display capture permission.
    DisplayCapture,
    /// Durable storage permission.
    DurableStorage,
    /// Flash permission.
    Flash,
    /// Geolocation permission.
    Geolocation,
    /// Idle detection permission.
    IdleDetection,
    /// Local fonts permission.
    LocalFonts,
    /// MIDI permission.
    Midi,
    /// MIDI sysex permission.
    MidiSysex,
    /// NFC permission.
    Nfc,
    /// Notifications permission.
    Notifications,
    /// Payment handler permission.
    PaymentHandler,
    /// Periodic background sync permission.
    PeriodicBackgroundSync,
    /// Protected media identifier permission.
    ProtectedMediaIdentifier,
    /// Sensors permission.
    Sensors,
    /// Speaker selection permission.
    SpeakerSelection,
    /// Storage access permission.
    StorageAccess,
    /// Top level storage access permission.
    TopLevelStorageAccess,
    /// Video capture permission.
    VideoCapture,
    /// Video capture pan tilt zoom permission.
    VideoCaptureGenericPanTiltZoom,
    /// Wake lock screen permission.
    WakeLockScreen,
    /// Wake lock system permission.
    WakeLockSystem,
    /// Web app installation permission.
    WebAppInstallation,
    /// Window management permission.
    WindowManagement,
}

impl PermissionType {
    /// Get the CDP string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AccessibilityEvents => "accessibilityEvents",
            Self::AudioCapture => "audioCapture",
            Self::BackgroundSync => "backgroundSync",
            Self::BackgroundFetch => "backgroundFetch",
            Self::CapturedSurfaceControl => "capturedSurfaceControl",
            Self::ClipboardReadWrite => "clipboardReadWrite",
            Self::ClipboardSanitizedWrite => "clipboardSanitizedWrite",
            Self::DisplayCapture => "displayCapture",
            Self::DurableStorage => "durableStorage",
            Self::Flash => "flash",
            Self::Geolocation => "geolocation",
            Self::IdleDetection => "idleDetection",
            Self::LocalFonts => "localFonts",
            Self::Midi => "midi",
            Self::MidiSysex => "midiSysex",
            Self::Nfc => "nfc",
            Self::Notifications => "notifications",
            Self::PaymentHandler => "paymentHandler",
            Self::PeriodicBackgroundSync => "periodicBackgroundSync",
            Self::ProtectedMediaIdentifier => "protectedMediaIdentifier",
            Self::Sensors => "sensors",
            Self::SpeakerSelection => "speakerSelection",
            Self::StorageAccess => "storageAccess",
            Self::TopLevelStorageAccess => "topLevelStorageAccess",
            Self::VideoCapture => "videoCapture",
            Self::VideoCaptureGenericPanTiltZoom => "videoCaptureGenericPanTiltZoom",
            Self::WakeLockScreen => "wakeLockScreen",
            Self::WakeLockSystem => "wakeLockSystem",
            Self::WebAppInstallation => "webAppInstallation",
            Self::WindowManagement => "windowManagement",
        }
    }
}

/// Permission setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum PermissionSetting {
    /// Permission is granted.
    #[default]
    Granted,
    /// Permission is denied.
    Denied,
    /// Permission is prompt.
    Prompt,
}


/// Permission descriptor.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor {
    /// Permission name.
    pub name: String,
    /// For "midi" permission, whether sysex is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysex: Option<bool>,
    /// For "push" permission, whether userVisibleOnly is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_visible_only: Option<bool>,
    /// For "clipboard" permission, allow without gesture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_without_gesture: Option<bool>,
    /// For "camera" permission, whether panTiltZoom is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan_tilt_zoom: Option<bool>,
}

impl PermissionDescriptor {
    /// Create a new permission descriptor.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sysex: None,
            user_visible_only: None,
            allow_without_gesture: None,
            pan_tilt_zoom: None,
        }
    }
}

/// Parameters for Browser.grantPermissions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantPermissionsParams {
    /// Permissions to grant.
    pub permissions: Vec<PermissionType>,
    /// Origin to grant permissions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// `BrowserContext` to override permissions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
}

impl GrantPermissionsParams {
    /// Create new grant permissions params.
    pub fn new(permissions: Vec<PermissionType>) -> Self {
        Self {
            permissions,
            origin: None,
            browser_context_id: None,
        }
    }

    /// Set the origin.
    #[must_use]
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    /// Set the browser context ID.
    #[must_use]
    pub fn browser_context_id(mut self, id: impl Into<String>) -> Self {
        self.browser_context_id = Some(id.into());
        self
    }
}

/// Parameters for Browser.resetPermissions.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetPermissionsParams {
    /// `BrowserContext` to reset permissions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
}

impl ResetPermissionsParams {
    /// Create new reset permissions params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the browser context ID.
    #[must_use]
    pub fn browser_context_id(mut self, id: impl Into<String>) -> Self {
        self.browser_context_id = Some(id.into());
        self
    }
}

/// Parameters for Browser.setPermission.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPermissionParams {
    /// Permission descriptor.
    pub permission: PermissionDescriptor,
    /// Permission setting.
    pub setting: PermissionSetting,
    /// Origin to set permission for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// `BrowserContext` to set permission for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_context_id: Option<String>,
}

impl SetPermissionParams {
    /// Create new set permission params.
    pub fn new(permission: PermissionDescriptor, setting: PermissionSetting) -> Self {
        Self {
            permission,
            setting,
            origin: None,
            browser_context_id: None,
        }
    }

    /// Set the origin.
    #[must_use]
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    /// Set the browser context ID.
    #[must_use]
    pub fn browser_context_id(mut self, id: impl Into<String>) -> Self {
        self.browser_context_id = Some(id.into());
        self
    }
}

/// Parameters for Browser.close.
#[derive(Debug, Clone, Serialize, Default)]
pub struct CloseParams {}

/// Parameters for Browser.getVersion.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetVersionParams {}

/// Result for Browser.getVersion.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVersionResult {
    /// Protocol version.
    pub protocol_version: String,
    /// Product name.
    pub product: String,
    /// Product revision.
    pub revision: String,
    /// User-Agent.
    pub user_agent: String,
    /// V8 version.
    pub js_version: String,
}

#[cfg(test)]
mod tests;
