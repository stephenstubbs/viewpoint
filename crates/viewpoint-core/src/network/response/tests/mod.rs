use super::*;

#[test]
fn test_response_ok() {
    // Response with 2xx status
    assert!((200..300).contains(&200_u16));
    assert!((200..300).contains(&299_u16));
    assert!(!(200..300).contains(&404_u16));
    assert!(!(200..300).contains(&500_u16));
}

#[test]
fn test_security_details_from_cdp() {
    // Test conversion from CDP SecurityDetails
    let cdp_details = viewpoint_cdp::protocol::network::SecurityDetails {
        protocol: "TLS 1.3".to_string(),
        key_exchange: "X25519".to_string(),
        key_exchange_group: Some("X25519".to_string()),
        cipher: "AES_256_GCM".to_string(),
        mac: Some("AEAD".to_string()),
        subject_name: "*.example.com".to_string(),
        san_list: vec!["example.com".to_string(), "*.example.com".to_string()],
        issuer: "DigiCert SHA2 Extended Validation Server CA".to_string(),
        valid_from: 1609459200.0, // 2021-01-01
        valid_to: 1640995200.0,   // 2022-01-01
    };

    let details = SecurityDetails::from(cdp_details);

    assert_eq!(details.protocol, "TLS 1.3");
    assert_eq!(details.subject_name, "*.example.com");
    assert_eq!(
        details.issuer,
        "DigiCert SHA2 Extended Validation Server CA"
    );
    assert_eq!(details.valid_from, 1609459200.0);
    assert_eq!(details.valid_to, 1640995200.0);
    assert_eq!(details.san_list.len(), 2);
    assert!(details.san_list.contains(&"example.com".to_string()));
    assert!(details.san_list.contains(&"*.example.com".to_string()));
}

#[test]
fn test_security_details_empty_san_list() {
    // Test with empty SAN list
    let cdp_details = viewpoint_cdp::protocol::network::SecurityDetails {
        protocol: "TLS 1.2".to_string(),
        key_exchange: "ECDHE_RSA".to_string(),
        key_exchange_group: None,
        cipher: "AES_128_GCM".to_string(),
        mac: None,
        subject_name: "localhost".to_string(),
        san_list: vec![],
        issuer: "Self-Signed".to_string(),
        valid_from: 0.0,
        valid_to: 0.0,
    };

    let details = SecurityDetails::from(cdp_details);

    assert_eq!(details.protocol, "TLS 1.2");
    assert!(details.san_list.is_empty());
}

#[test]
fn test_security_details_clone() {
    let details = SecurityDetails {
        protocol: "TLS 1.3".to_string(),
        subject_name: "example.com".to_string(),
        issuer: "Test CA".to_string(),
        valid_from: 1000.0,
        valid_to: 2000.0,
        san_list: vec!["example.com".to_string()],
    };

    let cloned = details.clone();
    assert_eq!(cloned.protocol, details.protocol);
    assert_eq!(cloned.subject_name, details.subject_name);
    assert_eq!(cloned.issuer, details.issuer);
}

#[test]
fn test_remote_address_clone() {
    let addr = RemoteAddress {
        ip_address: "192.168.1.1".to_string(),
        port: 443,
    };

    let cloned = addr.clone();
    assert_eq!(cloned.ip_address, "192.168.1.1");
    assert_eq!(cloned.port, 443);
}

#[test]
fn test_remote_address_ipv6() {
    let addr = RemoteAddress {
        ip_address: "::1".to_string(),
        port: 8080,
    };

    assert_eq!(addr.ip_address, "::1");
    assert_eq!(addr.port, 8080);
}

#[test]
fn test_security_details_debug() {
    let details = SecurityDetails {
        protocol: "TLS 1.3".to_string(),
        subject_name: "test.com".to_string(),
        issuer: "CA".to_string(),
        valid_from: 0.0,
        valid_to: 0.0,
        san_list: vec![],
    };

    let debug_str = format!("{:?}", details);
    assert!(debug_str.contains("TLS 1.3"));
    assert!(debug_str.contains("test.com"));
}
