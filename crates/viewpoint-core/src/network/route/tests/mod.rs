use super::*;

#[test]
fn test_header_entry() {
    let entry = HeaderEntry {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    };
    assert_eq!(entry.name, "Content-Type");
    assert_eq!(entry.value, "application/json");
}

#[test]
fn test_route_action_variants() {
    // Test that RouteAction has the expected variants
    let handled = RouteAction::Handled;
    let fallback = RouteAction::Fallback;

    assert_eq!(handled, RouteAction::Handled);
    assert_eq!(fallback, RouteAction::Fallback);
    assert_ne!(handled, fallback);
}
