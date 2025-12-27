use super::*;

#[tokio::test]
async fn test_create_standalone_context() {
    let api = APIRequestContext::new(APIContextOptions::new())
        .await
        .expect("Failed to create API context");

    assert!(!api.is_disposed());
    assert!(api.base_url().is_none());
}

#[tokio::test]
async fn test_context_with_base_url() {
    let api = APIRequestContext::new(
        APIContextOptions::new().base_url("https://api.example.com"),
    )
    .await
    .expect("Failed to create API context");

    assert_eq!(api.base_url(), Some("https://api.example.com"));
}

#[tokio::test]
async fn test_dispose_context() {
    let api = APIRequestContext::new(APIContextOptions::new())
        .await
        .expect("Failed to create API context");

    assert!(!api.is_disposed());
    api.dispose().await;
    assert!(api.is_disposed());
}

#[tokio::test]
async fn test_clone_shares_state() {
    let api = APIRequestContext::new(APIContextOptions::new())
        .await
        .expect("Failed to create API context");

    let api_clone = api.clone();

    api.dispose().await;

    // Clone should also be disposed
    assert!(api_clone.is_disposed());
}
