use super::*;

#[test]
fn test_permission_type_serialization() {
    let perm = PermissionType::Geolocation;
    let json = serde_json::to_string(&perm).unwrap();
    assert_eq!(json, "\"geolocation\"");
}

#[test]
fn test_grant_permissions_params() {
    let params = GrantPermissionsParams::new(vec![
        PermissionType::Geolocation,
        PermissionType::Notifications,
    ])
    .origin("https://example.com")
    .browser_context_id("ctx-123");

    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains("\"geolocation\""));
    assert!(json.contains("\"notifications\""));
    assert!(json.contains("\"origin\":\"https://example.com\""));
}
