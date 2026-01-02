#![cfg(feature = "integration")]

//! Proxy configuration tests.
//!
//! Tests for browser context proxy configuration.

mod common;

use viewpoint_core::ProxyConfig;

/// Test creating a context with a simple proxy configuration.
#[tokio::test]
async fn test_context_with_proxy() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with proxy configuration
    let context = browser
        .new_context_builder()
        .proxy(ProxyConfig::new("http://proxy.example.com:8080"))
        .build()
        .await
        .expect("Failed to create context with proxy");

    // Verify context was created (proxy is configured at CDP level)
    assert!(!context.is_closed());

    browser.close().await.expect("Failed to close browser");
}

/// Test creating a context with proxy and authentication credentials.
#[tokio::test]
async fn test_context_with_proxy_credentials() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with proxy configuration including credentials
    let context = browser
        .new_context_builder()
        .proxy(
            ProxyConfig::new("http://proxy.example.com:8080")
                .credentials("user", "password"),
        )
        .build()
        .await
        .expect("Failed to create context with proxy credentials");

    // Verify context was created
    assert!(!context.is_closed());

    browser.close().await.expect("Failed to close browser");
}

/// Test creating a context with SOCKS5 proxy.
#[tokio::test]
async fn test_context_with_socks5_proxy() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with SOCKS5 proxy configuration
    let context = browser
        .new_context_builder()
        .proxy(ProxyConfig::new("socks5://proxy.example.com:1080"))
        .build()
        .await
        .expect("Failed to create context with SOCKS5 proxy");

    // Verify context was created
    assert!(!context.is_closed());

    browser.close().await.expect("Failed to close browser");
}

/// Test creating a context with proxy and bypass list.
#[tokio::test]
async fn test_context_with_proxy_bypass() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with proxy configuration including bypass list
    let context = browser
        .new_context_builder()
        .proxy(
            ProxyConfig::new("http://proxy.example.com:8080")
                .bypass("localhost,127.0.0.1,.internal.example.com"),
        )
        .build()
        .await
        .expect("Failed to create context with proxy bypass");

    // Verify context was created
    assert!(!context.is_closed());

    browser.close().await.expect("Failed to close browser");
}

/// Test creating a context with full proxy configuration.
#[tokio::test]
async fn test_context_with_full_proxy_config() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create context with full proxy configuration
    let context = browser
        .new_context_builder()
        .proxy(
            ProxyConfig::new("http://proxy.example.com:8080")
                .credentials("user", "password")
                .bypass("localhost,127.0.0.1"),
        )
        .build()
        .await
        .expect("Failed to create context with full proxy config");

    // Verify context was created and can create pages
    let page = context.new_page().await.expect("Failed to create page");
    assert!(!page.is_closed());

    browser.close().await.expect("Failed to close browser");
}

/// Test creating multiple contexts with different proxy configurations.
#[tokio::test]
async fn test_multiple_contexts_with_different_proxies() {
    common::init_tracing();

    let browser = common::launch_browser().await;

    // Create first context with one proxy
    let context1 = browser
        .new_context_builder()
        .proxy(ProxyConfig::new("http://proxy1.example.com:8080"))
        .build()
        .await
        .expect("Failed to create context1");

    // Create second context with different proxy
    let context2 = browser
        .new_context_builder()
        .proxy(ProxyConfig::new("http://proxy2.example.com:8080"))
        .build()
        .await
        .expect("Failed to create context2");

    // Create third context without proxy
    let context3 = browser
        .new_context()
        .await
        .expect("Failed to create context3");

    // All contexts should be independent
    assert!(!context1.is_closed());
    assert!(!context2.is_closed());
    assert!(!context3.is_closed());

    browser.close().await.expect("Failed to close browser");
}
