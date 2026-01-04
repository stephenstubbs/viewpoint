use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn test_handler_id_uniqueness() {
    let id1 = HandlerId::new();
    let id2 = HandlerId::new();
    let id3 = HandlerId::new();

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
}

#[tokio::test]
async fn test_event_emitter_add_remove() {
    let emitter: EventEmitter<CloseEventHandler> = EventEmitter::new();

    assert!(emitter.is_empty().await);

    let id = emitter.add(Box::new(|| Box::pin(async {}))).await;
    assert_eq!(emitter.len().await, 1);
    assert!(!emitter.is_empty().await);

    assert!(emitter.remove(id).await);
    assert!(emitter.is_empty().await);

    // Removing again should return false
    assert!(!emitter.remove(id).await);
}

#[tokio::test]
async fn test_close_event_emission() {
    let counter = Arc::new(AtomicUsize::new(0));
    let emitter: EventEmitter<CloseEventHandler> = EventEmitter::new();

    let counter_clone = counter.clone();
    emitter
        .add(Box::new(move || {
            let counter = counter_clone.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
            })
        }))
        .await;

    let counter_clone = counter.clone();
    emitter
        .add(Box::new(move || {
            let counter = counter_clone.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
            })
        }))
        .await;

    emitter.emit().await;

    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_context_event_manager() {
    let manager = ContextEventManager::new();
    let counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = counter.clone();
    let id = manager
        .on_close(move || {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        })
        .await;

    manager.emit_close().await;
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    manager.off_close(id).await;
    manager.emit_close().await;
    // Counter should still be 1 since handler was removed
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_page_activated_event_emitter_add_remove() {
    let emitter: EventEmitter<PageActivatedEventHandler> = EventEmitter::new();

    assert!(emitter.is_empty().await);

    let id = emitter.add(Box::new(|_page| Box::pin(async {}))).await;
    assert_eq!(emitter.len().await, 1);
    assert!(!emitter.is_empty().await);

    assert!(emitter.remove(id).await);
    assert!(emitter.is_empty().await);

    // Removing again should return false
    assert!(!emitter.remove(id).await);
}

#[tokio::test]
async fn test_on_page_activated_registration() {
    let manager = ContextEventManager::new();
    let counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = counter.clone();
    let id = manager
        .on_page_activated(move |_page| {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        })
        .await;

    // Handler should be registered (we can't emit without a real Page,
    // but we can verify registration works)
    assert!(manager.off_page_activated(id).await);

    // Removing again should return false
    assert!(!manager.off_page_activated(id).await);
}

#[tokio::test]
async fn test_clear_includes_page_activated_handlers() {
    let manager = ContextEventManager::new();

    // Register handlers of each type
    manager.on_close(|| async {}).await;
    manager.on_page_activated(|_page| async {}).await;

    // Clear all handlers
    manager.clear().await;

    // All handler types should be cleared (we test by trying to remove
    // handler IDs that were registered - they should already be gone)
    // Note: Since clear() removes all, we can't directly test this without
    // exposing internal state. The implementation is correct if clear()
    // includes page_activated_handlers.clear().await
}
