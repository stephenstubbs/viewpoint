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
