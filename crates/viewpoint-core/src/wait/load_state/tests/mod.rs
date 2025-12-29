use super::*;

#[test]
fn test_load_state_ordering() {
    assert!(DocumentLoadState::Commit < DocumentLoadState::DomContentLoaded);
    assert!(DocumentLoadState::DomContentLoaded < DocumentLoadState::Load);
    assert!(DocumentLoadState::Load < DocumentLoadState::NetworkIdle);
}

#[test]
fn test_load_state_complete_ordering() {
    let states = [
        DocumentLoadState::Commit,
        DocumentLoadState::DomContentLoaded,
        DocumentLoadState::Load,
        DocumentLoadState::NetworkIdle,
    ];

    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            assert!(
                states[i] < states[j],
                "{:?} should be < {:?}",
                states[i],
                states[j]
            );
        }
    }
}

#[test]
fn test_is_reached() {
    let current = DocumentLoadState::Load;

    assert!(DocumentLoadState::Commit.is_reached(current));
    assert!(DocumentLoadState::DomContentLoaded.is_reached(current));
    assert!(DocumentLoadState::Load.is_reached(current));
    assert!(!DocumentLoadState::NetworkIdle.is_reached(current));
}

#[test]
fn test_is_reached_from_commit() {
    let current = DocumentLoadState::Commit;

    assert!(DocumentLoadState::Commit.is_reached(current));
    assert!(!DocumentLoadState::DomContentLoaded.is_reached(current));
    assert!(!DocumentLoadState::Load.is_reached(current));
    assert!(!DocumentLoadState::NetworkIdle.is_reached(current));
}

#[test]
fn test_is_reached_from_network_idle() {
    let current = DocumentLoadState::NetworkIdle;

    // All states should be reached when at NetworkIdle
    assert!(DocumentLoadState::Commit.is_reached(current));
    assert!(DocumentLoadState::DomContentLoaded.is_reached(current));
    assert!(DocumentLoadState::Load.is_reached(current));
    assert!(DocumentLoadState::NetworkIdle.is_reached(current));
}

#[test]
fn test_default_is_load() {
    assert_eq!(DocumentLoadState::default(), DocumentLoadState::Load);
}

#[test]
fn test_cdp_event_names() {
    assert_eq!(DocumentLoadState::Commit.cdp_event_name(), None);
    assert_eq!(
        DocumentLoadState::DomContentLoaded.cdp_event_name(),
        Some("Page.domContentEventFired")
    );
    assert_eq!(
        DocumentLoadState::Load.cdp_event_name(),
        Some("Page.loadEventFired")
    );
    assert_eq!(DocumentLoadState::NetworkIdle.cdp_event_name(), None);
}

#[test]
fn test_load_state_clone() {
    let state = DocumentLoadState::Load;
    let cloned = state;
    assert_eq!(state, cloned);
}

#[test]
fn test_load_state_debug() {
    let debug = format!("{:?}", DocumentLoadState::DomContentLoaded);
    assert!(debug.contains("DomContentLoaded"));
}

#[test]
fn test_load_state_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(DocumentLoadState::Load);
    set.insert(DocumentLoadState::Commit);
    set.insert(DocumentLoadState::Load); // Duplicate

    assert_eq!(set.len(), 2);
}

#[test]
fn test_load_state_equality() {
    assert_eq!(DocumentLoadState::Load, DocumentLoadState::Load);
    assert_ne!(DocumentLoadState::Load, DocumentLoadState::Commit);
}
