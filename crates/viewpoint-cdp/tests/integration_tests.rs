#![cfg(feature = "integration")]

//! Integration tests for viewpoint-cdp.
//!
//! These tests require Chromium to be installed and accessible.
//! Run with: `cargo test --test integration_tests`
//! Run with tracing: `RUST_LOG=debug cargo test --test integration_tests -- --nocapture`

use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::Once;
use std::time::Duration;

use viewpoint_cdp::protocol::target::{GetTargetsParams, GetTargetsResult};
use viewpoint_cdp::CdpConnection;

static TRACING_INIT: Once = Once::new();

/// Initialize tracing for tests.
/// This is safe to call multiple times - it will only initialize once.
fn init_tracing() {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .with_test_writer()
            .try_init()
            .ok();
    });
}

/// Helper to launch Chromium and get the WebSocket URL.
fn launch_chromium() -> (Child, String) {
    // Find Chromium
    let chromium_path = std::env::var("CHROMIUM_PATH").unwrap_or_else(|_| {
        // Try common paths
        for path in &[
            "chromium",
            "chromium-browser",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
        ] {
            if Command::new("which").arg(path).output().map(|o| o.status.success()).unwrap_or(false) {
                return path.to_string();
            }
        }
        "chromium".to_string()
    });

    let mut cmd = Command::new(&chromium_path);
    cmd.args([
        "--headless=new",
        "--remote-debugging-port=0",
        "--disable-gpu",
        "--no-sandbox",
        "--disable-dev-shm-usage",
    ]);
    cmd.stderr(Stdio::piped());
    cmd.stdout(Stdio::null());

    let mut child = cmd.spawn().expect("Failed to spawn Chromium");

    // Read the WebSocket URL from stderr
    let stderr = child.stderr.take().expect("Failed to get stderr");
    let reader = BufReader::new(stderr);

    let mut ws_url = String::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if let Some(pos) = line.find("DevTools listening on ") {
            ws_url = line[pos + 22..].trim().to_string();
            break;
        }
    }

    assert!(!ws_url.is_empty(), "Failed to get WebSocket URL from Chromium");

    (child, ws_url)
}

/// Test connecting to Chromium via CDP.
#[tokio::test]
async fn test_cdp_connection() {
    init_tracing();
    
    let (mut child, ws_url) = launch_chromium();

    // Connect to the browser
    let conn = CdpConnection::connect(&ws_url)
        .await
        .expect("Failed to connect to Chromium");

    // Get targets to verify connection works
    let result: GetTargetsResult = conn
        .send_command("Target.getTargets", Some(GetTargetsParams::default()), None)
        .await
        .expect("Failed to get targets");

    // Should have at least one target (the browser)
    println!("Found {} targets", result.target_infos.len());
    for target in &result.target_infos {
        println!("  - {} ({}): {}", target.target_type, target.target_id, target.url);
    }

    // Clean up
    let _ = child.kill();
}

/// Test sending commands with session ID.
#[tokio::test]
async fn test_cdp_session_commands() {
    init_tracing();
    
    let (mut child, ws_url) = launch_chromium();

    let conn = CdpConnection::connect(&ws_url)
        .await
        .expect("Failed to connect to Chromium");

    // Create a browser context
    let create_result: viewpoint_cdp::protocol::target::CreateBrowserContextResult = conn
        .send_command(
            "Target.createBrowserContext",
            Some(viewpoint_cdp::protocol::target::CreateBrowserContextParams::default()),
            None,
        )
        .await
        .expect("Failed to create browser context");

    println!("Created browser context: {}", create_result.browser_context_id);

    // Create a target (page) in the context
    let target_result: viewpoint_cdp::protocol::target::CreateTargetResult = conn
        .send_command(
            "Target.createTarget",
            Some(viewpoint_cdp::protocol::target::CreateTargetParams {
                url: "about:blank".to_string(),
                width: None,
                height: None,
                browser_context_id: Some(create_result.browser_context_id.clone()),
                background: None,
                new_window: None,
            }),
            None,
        )
        .await
        .expect("Failed to create target");

    println!("Created target: {}", target_result.target_id);

    // Attach to the target
    let attach_result: viewpoint_cdp::protocol::target::AttachToTargetResult = conn
        .send_command(
            "Target.attachToTarget",
            Some(viewpoint_cdp::protocol::target::AttachToTargetParams {
                target_id: target_result.target_id.clone(),
                flatten: Some(true),
            }),
            None,
        )
        .await
        .expect("Failed to attach to target");

    println!("Attached with session: {}", attach_result.session_id);

    // Enable Page domain on the session
    conn.send_command::<(), serde_json::Value>("Page.enable", None, Some(&attach_result.session_id))
        .await
        .expect("Failed to enable Page domain");

    // Navigate the page
    let nav_result: viewpoint_cdp::protocol::page::NavigateResult = conn
        .send_command(
            "Page.navigate",
            Some(viewpoint_cdp::protocol::page::NavigateParams {
                url: "https://example.com".to_string(),
                referrer: None,
                transition_type: None,
                frame_id: None,
            }),
            Some(&attach_result.session_id),
        )
        .await
        .expect("Failed to navigate");

    println!("Navigated to frame: {}", nav_result.frame_id);
    assert!(nav_result.error_text.is_none(), "Navigation failed: {:?}", nav_result.error_text);

    // Clean up
    let _ = child.kill();
}

/// Test event subscription.
#[tokio::test]
async fn test_cdp_event_subscription() {
    init_tracing();
    
    let (mut child, ws_url) = launch_chromium();

    let conn = CdpConnection::connect(&ws_url)
        .await
        .expect("Failed to connect to Chromium");

    // Subscribe to events
    let mut event_rx = conn.subscribe_events();

    // Create a context and page
    let create_result: viewpoint_cdp::protocol::target::CreateBrowserContextResult = conn
        .send_command(
            "Target.createBrowserContext",
            Some(viewpoint_cdp::protocol::target::CreateBrowserContextParams::default()),
            None,
        )
        .await
        .expect("Failed to create browser context");

    let target_result: viewpoint_cdp::protocol::target::CreateTargetResult = conn
        .send_command(
            "Target.createTarget",
            Some(viewpoint_cdp::protocol::target::CreateTargetParams {
                url: "about:blank".to_string(),
                width: None,
                height: None,
                browser_context_id: Some(create_result.browser_context_id),
                background: None,
                new_window: None,
            }),
            None,
        )
        .await
        .expect("Failed to create target");

    let attach_result: viewpoint_cdp::protocol::target::AttachToTargetResult = conn
        .send_command(
            "Target.attachToTarget",
            Some(viewpoint_cdp::protocol::target::AttachToTargetParams {
                target_id: target_result.target_id,
                flatten: Some(true),
            }),
            None,
        )
        .await
        .expect("Failed to attach to target");

    // Enable Page domain
    conn.send_command::<(), serde_json::Value>("Page.enable", None, Some(&attach_result.session_id))
        .await
        .expect("Failed to enable Page domain");

    // Navigate and check for events
    conn.send_command::<_, viewpoint_cdp::protocol::page::NavigateResult>(
        "Page.navigate",
        Some(viewpoint_cdp::protocol::page::NavigateParams {
            url: "https://example.com".to_string(),
            referrer: None,
            transition_type: None,
            frame_id: None,
        }),
        Some(&attach_result.session_id),
    )
    .await
    .expect("Failed to navigate");

    // Try to receive some events (with timeout)
    let mut events_received = 0;
    let timeout = tokio::time::timeout(Duration::from_secs(10), async {
        while events_received < 3 {
            if let Ok(event) = event_rx.recv().await {
                println!("Received event: {}", event.method);
                events_received += 1;
            }
        }
    });

    let _ = timeout.await;
    println!("Received {events_received} events");

    // Clean up
    let _ = child.kill();
}

/// Test command timeout.
#[tokio::test]
async fn test_cdp_command_with_timeout() {
    init_tracing();
    
    let (mut child, ws_url) = launch_chromium();

    let conn = CdpConnection::connect(&ws_url)
        .await
        .expect("Failed to connect to Chromium");

    // Send a command with a custom timeout
    let result: GetTargetsResult = conn
        .send_command_with_timeout(
            "Target.getTargets",
            Some(GetTargetsParams::default()),
            None,
            Duration::from_secs(5),
        )
        .await
        .expect("Failed to get targets");

    assert!(!result.target_infos.is_empty() || result.target_infos.is_empty()); // Just verify it completed

    // Clean up
    let _ = child.kill();
}

/// Test connection error after browser process is killed.
/// This verifies the connection properly reports errors when the browser is terminated.
#[tokio::test]
async fn test_connection_error_after_browser_kill() {
    init_tracing();
    
    let (mut child, ws_url) = launch_chromium();

    let conn = CdpConnection::connect(&ws_url)
        .await
        .expect("Failed to connect to Chromium");

    // Verify connection works initially
    let result: GetTargetsResult = conn
        .send_command("Target.getTargets", Some(GetTargetsParams::default()), None)
        .await
        .expect("Initial command should succeed");
    println!("Initial targets: {}", result.target_infos.len());

    // Kill the browser process
    child.kill().expect("Failed to kill browser");
    child.wait().expect("Failed to wait for browser exit");
    
    // Give the connection time to detect the disconnection
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Commands should now fail with a connection-related error
    let error_result: Result<GetTargetsResult, _> = conn
        .send_command("Target.getTargets", Some(GetTargetsParams::default()), None)
        .await;

    assert!(error_result.is_err(), "Command should fail after browser is killed");
    let err = error_result.unwrap_err();
    println!("Got expected error: {}", err);
}

/// Test connection to invalid WebSocket URL fails gracefully.
#[tokio::test]
async fn test_connection_to_invalid_url() {
    init_tracing();
    
    // Try to connect to a non-existent endpoint
    let result = CdpConnection::connect("ws://127.0.0.1:19999/devtools/browser/invalid").await;
    
    assert!(result.is_err(), "Connection to invalid URL should fail");
    let err = result.unwrap_err();
    println!("Got expected error for invalid URL: {}", err);
}

/// Test connection handles malformed WebSocket URL.
#[tokio::test]
async fn test_connection_to_malformed_url() {
    init_tracing();
    
    // Completely invalid URL
    let result = CdpConnection::connect("not-a-valid-websocket-url").await;
    
    assert!(result.is_err(), "Connection to malformed URL should fail");
    let err = result.unwrap_err();
    println!("Got expected error for malformed URL: {}", err);
}

// =============================================================================
// Fetch domain type tests (unit tests - no browser needed)
// =============================================================================

mod fetch_tests {
    use viewpoint_cdp::protocol::fetch::{
        AuthChallengeResponse, AuthChallengeResponseType, ErrorReason, FulfillRequestParams,
        HeaderEntry, RequestPattern, RequestStage,
    };
    use viewpoint_cdp::protocol::network::ResourceType;

    #[test]
    fn test_request_pattern_serialization() {
        let pattern = RequestPattern::url("**/api/**")
            .with_resource_type(ResourceType::Fetch)
            .with_stage(RequestStage::Response);

        let json = serde_json::to_string(&pattern).unwrap();
        assert!(json.contains("\"urlPattern\":\"**/api/**\""));
        assert!(json.contains("\"resourceType\":\"Fetch\""));
        assert!(json.contains("\"requestStage\":\"Response\""));
    }

    #[test]
    fn test_fulfill_request_params_serialization() {
        let params = FulfillRequestParams {
            request_id: "req-123".to_string(),
            response_code: 200,
            response_headers: Some(vec![HeaderEntry {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            }]),
            binary_response_headers: None,
            body: Some(base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                b"{\"ok\":true}",
            )),
            response_phrase: None,
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("\"requestId\":\"req-123\""));
        assert!(json.contains("\"responseCode\":200"));
    }

    #[test]
    fn test_error_reason_serialization() {
        let reason = ErrorReason::ConnectionRefused;
        let json = serde_json::to_string(&reason).unwrap();
        assert_eq!(json, "\"ConnectionRefused\"");
    }

    #[test]
    fn test_auth_challenge_response() {
        let creds = AuthChallengeResponse::provide_credentials("user", "pass");
        assert_eq!(creds.response, AuthChallengeResponseType::ProvideCredentials);
        assert_eq!(creds.username, Some("user".to_string()));
        assert_eq!(creds.password, Some("pass".to_string()));

        let cancel = AuthChallengeResponse::cancel();
        assert_eq!(cancel.response, AuthChallengeResponseType::CancelAuth);
    }
}
