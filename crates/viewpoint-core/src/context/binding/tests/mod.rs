use super::*;

#[tokio::test]
async fn test_context_binding_registry() {
    let registry = ContextBindingRegistry::new();

    // Register a function
    registry.expose_function("test", |_args| async {
        Ok(serde_json::json!(42))
    }).await;

    assert!(registry.has("test").await);
    assert_eq!(registry.get_all().await.len(), 1);

    // Remove the function
    assert!(registry.remove_function("test").await);
    assert!(!registry.has("test").await);
    assert_eq!(registry.get_all().await.len(), 0);
}
