//! Page creation and setup for browser contexts.
//!
//! This module provides utilities for page initialization within a browser context,
//! including enabling CDP domains, applying emulation settings, and setting up
//! video recording. These utilities are used by the target_events module which
//! is the single source of truth for page lifecycle management.

use tracing::{debug, trace as trace_log};

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::emulation::{
    MediaFeature, SetDeviceMetricsOverrideParams, SetEmulatedMediaParams, SetLocaleOverrideParams,
    SetTimezoneOverrideParams, SetTouchEmulationEnabledParams, SetUserAgentOverrideParams,
};

use super::types::{ColorScheme, ContextOptions, ForcedColors, ReducedMotion, ViewportSize};
use crate::error::ContextError;

/// Apply CDP domain enabling to a new page session.
pub(crate) async fn enable_page_domains(
    connection: &CdpConnection,
    session_id: &str,
) -> Result<(), ContextError> {
    trace_log!("Enabling Page domain");
    connection
        .send_command::<(), serde_json::Value>("Page.enable", None, Some(session_id))
        .await?;

    trace_log!("Enabling Network domain");
    connection
        .send_command::<(), serde_json::Value>("Network.enable", None, Some(session_id))
        .await?;

    trace_log!("Enabling Runtime domain");
    connection
        .send_command::<(), serde_json::Value>("Runtime.enable", None, Some(session_id))
        .await?;

    trace_log!("Enabling lifecycle events");
    connection
        .send_command::<_, serde_json::Value>(
            "Page.setLifecycleEventsEnabled",
            Some(viewpoint_cdp::protocol::page::SetLifecycleEventsEnabledParams { enabled: true }),
            Some(session_id),
        )
        .await?;

    Ok(())
}

/// Apply emulation settings to a new page session.
pub(crate) async fn apply_emulation_settings(
    connection: &CdpConnection,
    session_id: &str,
    options: &ContextOptions,
) -> Result<(), ContextError> {
    // Apply touch emulation if enabled
    if options.has_touch {
        trace_log!("Enabling touch emulation");
        connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setTouchEmulationEnabled",
                Some(SetTouchEmulationEnabledParams {
                    enabled: true,
                    max_touch_points: Some(5),
                }),
                Some(session_id),
            )
            .await?;
    }

    // Apply locale if set
    if let Some(ref locale) = options.locale {
        trace_log!("Setting locale override");
        connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setLocaleOverride",
                Some(SetLocaleOverrideParams::new(locale)),
                Some(session_id),
            )
            .await?;
    }

    // Apply timezone if set
    if let Some(ref tz) = options.timezone_id {
        trace_log!("Setting timezone override");
        connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setTimezoneOverride",
                Some(SetTimezoneOverrideParams::new(tz)),
                Some(session_id),
            )
            .await?;
    }

    // Apply user agent if set
    if let Some(ref ua) = options.user_agent {
        trace_log!("Setting user agent override");
        connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setUserAgentOverride",
                Some(SetUserAgentOverrideParams::new(ua)),
                Some(session_id),
            )
            .await?;
    }

    // Apply viewport and device metrics if set
    if options.viewport.is_some() || options.device_scale_factor.is_some() {
        let viewport = options.viewport.unwrap_or(ViewportSize::new(1280, 720));
        let scale_factor = options.device_scale_factor.unwrap_or(1.0);

        trace_log!("Setting device metrics override");
        connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setDeviceMetricsOverride",
                Some(SetDeviceMetricsOverrideParams {
                    width: viewport.width,
                    height: viewport.height,
                    device_scale_factor: scale_factor,
                    mobile: options.is_mobile,
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
                Some(session_id),
            )
            .await?;
    }

    // Apply media emulation settings (color scheme, reduced motion, forced colors)
    apply_media_features(connection, session_id, options).await?;

    Ok(())
}

/// Apply media feature emulation settings.
async fn apply_media_features(
    connection: &CdpConnection,
    session_id: &str,
    options: &ContextOptions,
) -> Result<(), ContextError> {
    let mut media_features = Vec::new();

    if let Some(color_scheme) = &options.color_scheme {
        media_features.push(MediaFeature {
            name: "prefers-color-scheme".to_string(),
            value: match color_scheme {
                ColorScheme::Light => "light".to_string(),
                ColorScheme::Dark => "dark".to_string(),
                ColorScheme::NoPreference => "no-preference".to_string(),
            },
        });
    }

    if let Some(reduced_motion) = &options.reduced_motion {
        media_features.push(MediaFeature {
            name: "prefers-reduced-motion".to_string(),
            value: match reduced_motion {
                ReducedMotion::Reduce => "reduce".to_string(),
                ReducedMotion::NoPreference => "no-preference".to_string(),
            },
        });
    }

    if let Some(forced_colors) = &options.forced_colors {
        media_features.push(MediaFeature {
            name: "forced-colors".to_string(),
            value: match forced_colors {
                ForcedColors::Active => "active".to_string(),
                ForcedColors::None => "none".to_string(),
            },
        });
    }

    if !media_features.is_empty() {
        trace_log!("Setting emulated media features");
        connection
            .send_command::<_, serde_json::Value>(
                "Emulation.setEmulatedMedia",
                Some(SetEmulatedMediaParams {
                    media: None,
                    features: Some(media_features),
                }),
                Some(session_id),
            )
            .await?;
    }

    Ok(())
}

/// Get the main frame ID from a page session.
pub(crate) async fn get_main_frame_id(
    connection: &CdpConnection,
    session_id: &str,
) -> Result<String, ContextError> {
    trace_log!("Getting frame tree");
    let frame_tree: viewpoint_cdp::protocol::page::GetFrameTreeResult = connection
        .send_command("Page.getFrameTree", None::<()>, Some(session_id))
        .await?;

    let frame_id = frame_tree.frame_tree.frame.id.clone();
    debug!(frame_id = %frame_id, "Got main frame ID");

    Ok(frame_id)
}

/// Convert context HTTP credentials to network auth credentials.
pub(crate) fn convert_http_credentials(
    options: &ContextOptions,
) -> Option<crate::network::auth::HttpCredentials> {
    options.http_credentials.as_ref().map(|creds| {
        let mut auth_creds = crate::network::auth::HttpCredentials::new(
            creds.username.clone(),
            creds.password.clone(),
        );
        if let Some(ref origin) = creds.origin {
            auth_creds = crate::network::auth::HttpCredentials::for_origin(
                creds.username.clone(),
                creds.password.clone(),
                origin.clone(),
            );
        }
        auth_creds
    })
}

/// Convert context proxy credentials to network auth proxy credentials.
pub(crate) fn convert_proxy_credentials(
    options: &ContextOptions,
) -> Option<crate::network::auth::ProxyCredentials> {
    options
        .proxy
        .as_ref()
        .and_then(|proxy| match (&proxy.username, &proxy.password) {
            (Some(username), Some(password)) => Some(crate::network::auth::ProxyCredentials::new(
                username.clone(),
                password.clone(),
            )),
            _ => None,
        })
}
