use super::*;

#[test]
fn test_navigation_detection_window_is_reasonable() {
    assert_eq!(NAVIGATION_DETECTION_WINDOW, Duration::from_millis(50));
}

#[test]
fn test_default_timeout() {
    assert_eq!(DEFAULT_NAVIGATION_TIMEOUT, Duration::from_secs(30));
}
