//! Unified CDP event-driven page tracking.
//!
//! This module is the **single source of truth** for all page lifecycle management.
//! It listens for `Target.targetCreated` and `Target.targetDestroyed` CDP events
//! to handle ALL pages uniformly, whether created via `context.new_page()` or
//! opened externally (e.g., `window.open()`, `target="_blank"`, Ctrl+click).
//!
//! ## Design
//!
//! - **All page creation** goes through `Target.targetCreated` event handling
//! - **All page destruction** goes through `Target.targetDestroyed` event handling
//! - `new_page()` uses `wait_for_page()` internally to receive the Page
//! - External pages trigger `on_page` event handlers
//! - No special cases, no deduplication needed

use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, trace};

use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::target_domain::{
    AttachToTargetParams, AttachToTargetResult, TargetCreatedEvent, TargetDestroyedEvent,
    TargetInfoChangedEvent,
};

use super::events::ContextEventManager;
use super::page_factory;
use super::routing::ContextRouteRegistry;
use super::types::ContextOptions;
use crate::page::Page;

/// Start listening for target events on a browser context.
///
/// This spawns a background task that listens for:
/// - `Target.targetCreated` - creates, tracks, and emits events for ALL new pages
/// - `Target.targetDestroyed` - removes pages from tracking
///
/// The listener automatically:
/// - Filters events by context ID (only tracks pages belonging to this context)
/// - Attaches to new targets and enables required CDP domains
/// - Applies emulation settings from context options
/// - Emits `page` events via the context's event manager
pub(crate) fn start_target_event_listener(
    connection: Arc<CdpConnection>,
    context_id: String,
    pages: Arc<RwLock<Vec<Page>>>,
    event_manager: Arc<ContextEventManager>,
    route_registry: Arc<ContextRouteRegistry>,
    options: ContextOptions,
    context_index: usize,
    page_index_counter: Arc<std::sync::atomic::AtomicUsize>,
    test_id_attribute: Arc<RwLock<String>>,
) {
    let mut events = connection.subscribe_events();

    tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            match event.method.as_str() {
                "Target.targetCreated" => {
                    if let Some(params) = &event.params {
                        if let Ok(created_event) =
                            serde_json::from_value::<TargetCreatedEvent>(params.clone())
                        {
                            handle_target_created(
                                &connection,
                                &context_id,
                                &pages,
                                &event_manager,
                                &route_registry,
                                &options,
                                context_index,
                                &page_index_counter,
                                &test_id_attribute,
                                created_event,
                            )
                            .await;
                        }
                    }
                }
                "Target.targetDestroyed" => {
                    if let Some(params) = &event.params {
                        if let Ok(destroyed_event) =
                            serde_json::from_value::<TargetDestroyedEvent>(params.clone())
                        {
                            handle_target_destroyed(&pages, destroyed_event).await;
                        }
                    }
                }
                "Target.targetInfoChanged" => {
                    if let Some(params) = &event.params {
                        if let Ok(changed_event) =
                            serde_json::from_value::<TargetInfoChangedEvent>(params.clone())
                        {
                            handle_target_info_changed(
                                &context_id,
                                &pages,
                                &event_manager,
                                changed_event,
                            )
                            .await;
                        }
                    }
                }
                _ => {}
            }
        }
    });
}

/// Handle a Target.targetCreated event.
///
/// This is the single entry point for ALL page creation. It:
/// 1. Filters by context ID and target type
/// 2. Attaches to the target
/// 3. Enables required CDP domains
/// 4. Applies emulation settings
/// 5. Creates a Page instance
/// 6. Tracks the page
/// 7. Emits the `on_page` event
async fn handle_target_created(
    connection: &Arc<CdpConnection>,
    context_id: &str,
    pages: &Arc<RwLock<Vec<Page>>>,
    event_manager: &Arc<ContextEventManager>,
    route_registry: &Arc<ContextRouteRegistry>,
    options: &ContextOptions,
    context_index: usize,
    page_index_counter: &Arc<std::sync::atomic::AtomicUsize>,
    test_id_attribute: &Arc<RwLock<String>>,
    event: TargetCreatedEvent,
) {
    let info = &event.target_info;

    // Only handle "page" targets
    if info.target_type != "page" {
        trace!(
            target_type = %info.target_type,
            target_id = %info.target_id,
            "Ignoring non-page target"
        );
        return;
    }

    // Filter by context ID
    // For default context (empty string), match targets without a context ID
    // For named contexts, require exact match
    let matches_context = if context_id.is_empty() {
        info.browser_context_id.is_none() || info.browser_context_id.as_deref() == Some("")
    } else {
        info.browser_context_id.as_deref() == Some(context_id)
    };

    if !matches_context {
        trace!(
            target_context = ?info.browser_context_id,
            our_context = %context_id,
            target_id = %info.target_id,
            "Target belongs to different context"
        );
        return;
    }

    // Skip if already attached (this shouldn't happen in the unified model,
    // but we check just in case to avoid errors)
    if info.attached {
        trace!(
            target_id = %info.target_id,
            "Target already attached, skipping"
        );
        return;
    }

    debug!(
        target_id = %info.target_id,
        url = %info.url,
        opener_id = ?info.opener_id,
        "New page detected via Target.targetCreated"
    );

    // Attach to the target
    let attach_result: Result<AttachToTargetResult, _> = connection
        .send_command(
            "Target.attachToTarget",
            Some(AttachToTargetParams {
                target_id: info.target_id.clone(),
                flatten: Some(true),
            }),
            None,
        )
        .await;

    let attach_result = match attach_result {
        Ok(r) => r,
        Err(e) => {
            debug!(
                target_id = %info.target_id,
                error = %e,
                "Failed to attach to target"
            );
            return;
        }
    };

    let session_id = &attach_result.session_id;

    // Enable required CDP domains
    if let Err(e) = page_factory::enable_page_domains(connection, session_id).await {
        debug!(
            target_id = %info.target_id,
            error = %e,
            "Failed to enable page domains"
        );
        return;
    }

    // Apply emulation settings
    if let Err(e) = page_factory::apply_emulation_settings(connection, session_id, options).await {
        debug!(
            target_id = %info.target_id,
            error = %e,
            "Failed to apply emulation settings"
        );
        // Continue anyway - emulation is optional
    }

    // Get the main frame ID
    let frame_id = match page_factory::get_main_frame_id(connection, session_id).await {
        Ok(id) => id,
        Err(e) => {
            debug!(
                target_id = %info.target_id,
                error = %e,
                "Failed to get main frame ID"
            );
            return;
        }
    };

    // Get test ID attribute
    let test_id_attr = test_id_attribute.read().await.clone();

    // Convert credentials
    let http_credentials = page_factory::convert_http_credentials(options);
    let proxy_credentials = page_factory::convert_proxy_credentials(options);

    // Get next page index
    let page_index = page_index_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    // Create the page instance
    let page = if let Some(ref video_options) = options.record_video {
        Page::with_video_and_indices(
            connection.clone(),
            info.target_id.clone(),
            attach_result.session_id.clone(),
            frame_id,
            context_index,
            page_index,
            video_options.clone(),
        )
        .with_test_id_attribute(test_id_attr)
        .with_context_pages(pages.clone())
        .with_context_routes_and_proxy(
            route_registry.clone(),
            http_credentials.clone(),
            proxy_credentials,
        )
        .await
    } else {
        Page::new_with_indices(
            connection.clone(),
            info.target_id.clone(),
            attach_result.session_id.clone(),
            frame_id,
            context_index,
            page_index,
        )
        .with_test_id_attribute(test_id_attr)
        .with_context_pages(pages.clone())
        .with_context_routes_and_proxy(route_registry.clone(), http_credentials, proxy_credentials)
        .await
    };

    // Enable Fetch for context routes
    if let Err(e) = page.enable_fetch_for_context_routes().await {
        debug!(
            target_id = %info.target_id,
            error = %e,
            "Failed to enable Fetch for context routes"
        );
        // Continue anyway
    }

    // Track the page by storing a clone in the pages list
    {
        let mut pages_guard = pages.write().await;
        pages_guard.push(page.clone_internal());
    }

    debug!(
        target_id = %info.target_id,
        session_id = %attach_result.session_id,
        page_index = page_index,
        "Page created and tracked via target events"
    );

    // Emit page event - this notifies all handlers including wait_for_page()
    event_manager.emit_page(page).await;
}

/// Handle a Target.targetDestroyed event.
///
/// This is the single entry point for ALL page destruction.
/// It removes the page from tracking.
async fn handle_target_destroyed(pages: &Arc<RwLock<Vec<Page>>>, event: TargetDestroyedEvent) {
    let mut pages_guard = pages.write().await;
    let initial_len = pages_guard.len();
    pages_guard.retain(|p| p.target_id() != event.target_id);

    if pages_guard.len() < initial_len {
        debug!(
            target_id = %event.target_id,
            "Page removed from tracking via Target.targetDestroyed"
        );
    }
}

/// Handle a Target.targetInfoChanged event.
///
/// This detects when a page becomes the active/foreground tab and emits
/// the `page_activated` event. This allows consumers to react to user-initiated
/// tab switches (e.g., clicking on a tab) or programmatic tab switches
/// (e.g., `page.bring_to_front()`).
async fn handle_target_info_changed(
    context_id: &str,
    pages: &Arc<RwLock<Vec<Page>>>,
    event_manager: &Arc<ContextEventManager>,
    event: TargetInfoChangedEvent,
) {
    let info = &event.target_info;

    // Only handle "page" targets
    if info.target_type != "page" {
        trace!(
            target_type = %info.target_type,
            target_id = %info.target_id,
            "Ignoring targetInfoChanged for non-page target"
        );
        return;
    }

    // Filter by context ID
    // For default context (empty string), match targets without a context ID
    // For named contexts, require exact match
    let matches_context = if context_id.is_empty() {
        info.browser_context_id.is_none() || info.browser_context_id.as_deref() == Some("")
    } else {
        info.browser_context_id.as_deref() == Some(context_id)
    };

    if !matches_context {
        trace!(
            target_context = ?info.browser_context_id,
            our_context = %context_id,
            target_id = %info.target_id,
            "targetInfoChanged for target in different context"
        );
        return;
    }

    // Look up the page from our tracked pages
    let page = {
        let pages_guard = pages.read().await;
        pages_guard
            .iter()
            .find(|p| p.target_id() == info.target_id)
            .map(super::super::page::Page::clone_internal)
    };

    if let Some(page) = page {
        debug!(
            target_id = %info.target_id,
            url = %info.url,
            "Page activated via Target.targetInfoChanged"
        );

        // Emit the page_activated event
        event_manager.emit_page_activated(page).await;
    } else {
        trace!(
            target_id = %info.target_id,
            "targetInfoChanged for untracked page (may be in creation)"
        );
    }
}
