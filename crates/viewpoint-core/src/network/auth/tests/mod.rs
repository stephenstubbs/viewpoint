use super::*;

#[test]
fn test_credentials_origin_matching() {
    // No origin restriction
    let creds = HttpCredentials::new("user", "pass");
    assert!(creds.matches_origin("example.com"));
    assert!(creds.matches_origin("other.com"));

    // With origin restriction
    let creds = HttpCredentials::for_origin("user", "pass", "example.com");
    assert!(creds.matches_origin("example.com"));
    assert!(creds.matches_origin("sub.example.com"));
    assert!(!creds.matches_origin("other.com"));
    assert!(!creds.matches_origin("notexample.com"));
}
