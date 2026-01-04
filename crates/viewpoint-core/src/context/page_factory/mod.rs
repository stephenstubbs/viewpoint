//! Page creation and setup for browser contexts.
//!
//! This module handles the creation and initialization of new pages
//! within a browser context, including enabling CDP domains,
//! applying emulation settings, and setting up video recording.

use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, trace as trace_log};

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::emulation::{
    MediaFeature, SetDeviceMetricsOverrideParams, SetEmulatedMediaParams, SetLocaleOverrideParams,
    SetTimezoneOverrideParams, SetTouchEmulationEnabledParams, SetUserAgentOverrideParams,
};
use viewpoint_cdp::protocol::target_domain::{
    AttachToTargetParams, AttachToTargetResult, CreateTargetParams, CreateTargetResult,
};

use super::PageInfo;
use super::routing;
use super::types::{ColorScheme, ContextOptions, ForcedColors, ReducedMotion, ViewportSize};
use crate::error::ContextError;
use crate::page::Page;

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

/// Create a target and attach to it.
pub(crate) async fn create_and_attach_target(
    connection: &CdpConnection,
    context_id: &str,
) -> Result<(CreateTargetResult, AttachToTargetResult), ContextError> {
    debug!("Creating target via Target.createTarget");
    let create_result: CreateTargetResult = connection
        .send_command(
            "Target.createTarget",
            Some(CreateTargetParams {
                url: "about:blank".to_string(),
                width: None,
                height: None,
                browser_context_id: Some(context_id.to_string()),
                background: None,
                new_window: None,
            }),
            None,
        )
        .await?;

    let target_id = &create_result.target_id;
    debug!(target_id = %target_id, "Target created");

    debug!(target_id = %target_id, "Attaching to target");
    let attach_result: AttachToTargetResult = connection
        .send_command(
            "Target.attachToTarget",
            Some(AttachToTargetParams {
                target_id: target_id.clone(),
                flatten: Some(true),
            }),
            None,
        )
        .await?;

    let session_id = &attach_result.session_id;
    debug!(session_id = %session_id, "Attached to target");

    Ok((create_result, attach_result))
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

/// Create the page instance with optional video recording.
pub(crate) async fn create_page_instance(
    connection: Arc<CdpConnection>,
    create_result: CreateTargetResult,
    attach_result: AttachToTargetResult,
    frame_id: String,
    context_index: usize,
    page_index: usize,
    options: &ContextOptions,
    test_id_attr: String,
    route_registry: Arc<routing::ContextRouteRegistry>,
    http_credentials: Option<crate::network::auth::HttpCredentials>,
    proxy_credentials: Option<crate::network::auth::ProxyCredentials>,
    context_pages: Arc<RwLock<Vec<PageInfo>>>,
) -> Page {
    if let Some(ref video_options) = options.record_video {
        let page = Page::with_video_and_indices(
            connection,
            create_result.target_id,
            attach_result.session_id,
            frame_id,
            context_index,
            page_index,
            video_options.clone(),
        )
        .with_test_id_attribute(test_id_attr)
        .with_context_pages(context_pages)
        .with_context_routes_and_proxy(route_registry, http_credentials.clone(), proxy_credentials)
        .await;

        // Start video recording immediately
        if let Err(e) = page.start_video_recording().await {
            debug!("Failed to start video recording: {}", e);
        }
        page
    } else {
        Page::new_with_indices(
            connection,
            create_result.target_id,
            attach_result.session_id,
            frame_id,
            context_index,
            page_index,
        )
        .with_test_id_attribute(test_id_attr)
        .with_context_pages(context_pages)
        .with_context_routes_and_proxy(route_registry, http_credentials, proxy_credentials)
        .await
    }
}

/// Track the page in the context's pages list.
pub(crate) async fn track_page(
    pages: &RwLock<Vec<PageInfo>>,
    target_id: String,
    session_id: String,
) {
    let mut pages_guard = pages.write().await;
    pages_guard.push(PageInfo {
        target_id,
        session_id,
    });
}
