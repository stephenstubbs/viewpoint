use super::*;

#[test]
fn test_navigation_detection_window_is_reasonable() {
    // 150ms provides enough time for CDP events to arrive after an action
    // while still being fast enough for good UX
    assert_eq!(NAVIGATION_DETECTION_WINDOW, Duration::from_millis(150));
}

#[test]
fn test_default_timeout() {
    assert_eq!(DEFAULT_NAVIGATION_TIMEOUT, Duration::from_secs(30));
}
