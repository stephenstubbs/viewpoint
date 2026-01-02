#![allow(clippy::uninlined_format_args)]

use super::*;

#[test]
fn test_aria_snapshot_to_yaml() {
    let snapshot = AriaSnapshot::with_role("button")
        .name("Submit")
        .child(AriaSnapshot::with_role("img").name("Icon"));

    let yaml = snapshot.to_yaml();
    assert!(yaml.contains("- button \"Submit\""));
    assert!(yaml.contains("- img \"Icon\""));
}

#[test]
fn test_aria_snapshot_with_attributes() {
    let mut snapshot = AriaSnapshot::with_role("checkbox");
    snapshot.name = Some("Accept terms".to_string());
    snapshot.checked = Some(AriaCheckedState::True);
    snapshot.disabled = Some(true);

    let yaml = snapshot.to_yaml();
    assert!(yaml.contains("[checked]"));
    assert!(yaml.contains("[disabled]"));
}

#[test]
fn test_aria_snapshot_matches() {
    let actual = AriaSnapshot::with_role("button").name("Submit");
    let expected = AriaSnapshot::with_role("button").name("Submit");
    assert!(actual.matches(&expected));
}

#[test]
fn test_aria_snapshot_matches_regex() {
    let actual = AriaSnapshot::with_role("heading").name("Welcome John!");
    let expected = AriaSnapshot::with_role("heading").name("/Welcome .+!/");
    assert!(actual.matches(&expected));
}

// =============================================================================
// Frame Boundary Tests
// =============================================================================

#[test]
fn test_aria_snapshot_frame_boundary_to_yaml() {
    let mut snapshot = AriaSnapshot::with_role("iframe");
    snapshot.name = Some("Payment Frame".to_string());
    snapshot.is_frame = Some(true);
    snapshot.frame_url = Some("https://payment.example.com/widget".to_string());
    snapshot.frame_name = Some("payment-frame".to_string());

    let yaml = snapshot.to_yaml();
    assert!(
        yaml.contains("[frame-boundary]"),
        "YAML should contain [frame-boundary], got: {}",
        yaml
    );
    assert!(
        yaml.contains("[frame-url=\"https://payment.example.com/widget\"]"),
        "YAML should contain frame URL, got: {}",
        yaml
    );
    assert!(
        yaml.contains("[frame-name=\"payment-frame\"]"),
        "YAML should contain frame name, got: {}",
        yaml
    );
}

#[test]
fn test_aria_snapshot_frame_boundary_to_yaml_minimal() {
    // Frame boundary without URL or name
    let mut snapshot = AriaSnapshot::with_role("iframe");
    snapshot.is_frame = Some(true);

    let yaml = snapshot.to_yaml();
    assert!(
        yaml.contains("[frame-boundary]"),
        "YAML should contain [frame-boundary], got: {}",
        yaml
    );
    assert!(
        !yaml.contains("[frame-url"),
        "YAML should not contain frame-url when not set, got: {}",
        yaml
    );
    assert!(
        !yaml.contains("[frame-name"),
        "YAML should not contain frame-name when not set, got: {}",
        yaml
    );
}

#[test]
fn test_aria_snapshot_frame_boundary_from_yaml() {
    let yaml = r#"- iframe "Payment" [frame-boundary] [frame-url="https://example.com"] [frame-name="payment"]"#;
    let snapshot = AriaSnapshot::from_yaml(yaml).expect("Should parse YAML");

    // The root is a wrapper, the actual iframe is the first child
    assert_eq!(snapshot.children.len(), 1);
    let iframe = &snapshot.children[0];

    assert_eq!(iframe.role, Some("iframe".to_string()));
    assert_eq!(iframe.name, Some("Payment".to_string()));
    assert_eq!(iframe.is_frame, Some(true));
    assert_eq!(iframe.frame_url, Some("https://example.com".to_string()));
    assert_eq!(iframe.frame_name, Some("payment".to_string()));
}

#[test]
fn test_aria_snapshot_frame_boundary_roundtrip() {
    let mut original = AriaSnapshot::with_role("iframe");
    original.name = Some("Widget Frame".to_string());
    original.is_frame = Some(true);
    original.frame_url = Some("https://widget.example.com".to_string());
    original.frame_name = Some("widget".to_string());

    let yaml = original.to_yaml();
    let parsed = AriaSnapshot::from_yaml(&yaml).expect("Should parse YAML");

    // The parsed snapshot wraps the original in a root
    assert_eq!(parsed.children.len(), 1);
    let roundtripped = &parsed.children[0];

    assert_eq!(roundtripped.role, original.role);
    assert_eq!(roundtripped.name, original.name);
    assert_eq!(roundtripped.is_frame, original.is_frame);
    assert_eq!(roundtripped.frame_url, original.frame_url);
    assert_eq!(roundtripped.frame_name, original.frame_name);
}

#[test]
fn test_aria_snapshot_iframe_refs_field() {
    let mut snapshot = AriaSnapshot::with_role("document");
    snapshot.iframe_refs = vec![
        "frame-1".to_string(),
        "frame-2".to_string(),
        "frame-3".to_string(),
    ];

    // iframe_refs should serialize
    let json = serde_json::to_string(&snapshot).expect("Should serialize");
    assert!(json.contains("iframeRefs"));
    assert!(json.contains("frame-1"));
    assert!(json.contains("frame-2"));
    assert!(json.contains("frame-3"));

    // And deserialize
    let deserialized: AriaSnapshot = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(deserialized.iframe_refs.len(), 3);
    assert_eq!(deserialized.iframe_refs[0], "frame-1");
}

#[test]
fn test_aria_snapshot_iframe_refs_default_empty() {
    // When not provided, iframe_refs should default to empty
    let json = r#"{"role":"document"}"#;
    let snapshot: AriaSnapshot = serde_json::from_str(json).expect("Should deserialize");
    assert!(
        snapshot.iframe_refs.is_empty(),
        "iframe_refs should default to empty"
    );
}

#[test]
fn test_aria_snapshot_frame_boundary_with_children() {
    // A document containing an iframe (which is a frame boundary)
    let mut iframe = AriaSnapshot::with_role("iframe");
    iframe.name = Some("Content Frame".to_string());
    iframe.is_frame = Some(true);
    iframe.frame_url = Some("about:blank".to_string());

    let document = AriaSnapshot::with_role("document")
        .name("Main Page")
        .child(AriaSnapshot::with_role("heading").name("Title"))
        .child(iframe);

    let yaml = document.to_yaml();
    assert!(yaml.contains("- document \"Main Page\""));
    assert!(yaml.contains("- heading \"Title\""));
    assert!(yaml.contains("- iframe \"Content Frame\" [frame-boundary]"));
}

#[test]
fn test_aria_snapshot_frame_url_with_special_chars() {
    let mut snapshot = AriaSnapshot::with_role("iframe");
    snapshot.is_frame = Some(true);
    snapshot.frame_url = Some("https://example.com/path?foo=bar&baz=qux".to_string());

    let yaml = snapshot.to_yaml();
    assert!(
        yaml.contains("https://example.com/path?foo=bar&baz=qux"),
        "URL with special chars should be preserved"
    );
}

#[test]
fn test_aria_snapshot_frame_name_with_quotes() {
    let mut snapshot = AriaSnapshot::with_role("iframe");
    snapshot.is_frame = Some(true);
    snapshot.frame_name = Some("my \"special\" frame".to_string());

    let yaml = snapshot.to_yaml();
    // Quotes should be escaped
    assert!(
        yaml.contains(r#"[frame-name="my \"special\" frame"]"#),
        "Quotes in frame name should be escaped, got: {}",
        yaml
    );
}

// =============================================================================
// Node Reference Tests
// =============================================================================

#[test]
fn test_aria_snapshot_ref_to_yaml() {
    let mut snapshot = AriaSnapshot::with_role("button");
    snapshot.name = Some("Submit".to_string());
    snapshot.node_ref = Some("e12345".to_string());

    let yaml = snapshot.to_yaml();
    assert!(
        yaml.contains("[ref=e12345]"),
        "YAML should contain [ref=e12345], got: {}",
        yaml
    );
}

#[test]
fn test_aria_snapshot_ref_to_yaml_no_ref() {
    let snapshot = AriaSnapshot::with_role("button").name("Submit");

    let yaml = snapshot.to_yaml();
    assert!(
        !yaml.contains("[ref="),
        "YAML should not contain [ref=] when node_ref is None, got: {}",
        yaml
    );
}

#[test]
fn test_aria_snapshot_ref_from_yaml() {
    let yaml = r#"- button "Submit" [ref=e12345]"#;
    let snapshot = AriaSnapshot::from_yaml(yaml).expect("Should parse YAML");

    // The root is a wrapper, the actual button is the first child
    assert_eq!(snapshot.children.len(), 1);
    let button = &snapshot.children[0];

    assert_eq!(button.role, Some("button".to_string()));
    assert_eq!(button.name, Some("Submit".to_string()));
    assert_eq!(button.node_ref, Some("e12345".to_string()));
}

#[test]
fn test_aria_snapshot_ref_roundtrip() {
    let mut original = AriaSnapshot::with_role("heading");
    original.name = Some("Page Title".to_string());
    original.level = Some(1);
    original.node_ref = Some("e98765".to_string());

    let yaml = original.to_yaml();
    let parsed = AriaSnapshot::from_yaml(&yaml).expect("Should parse YAML");

    // The parsed snapshot wraps the original in a root
    assert_eq!(parsed.children.len(), 1);
    let roundtripped = &parsed.children[0];

    assert_eq!(roundtripped.role, original.role);
    assert_eq!(roundtripped.name, original.name);
    assert_eq!(roundtripped.level, original.level);
    assert_eq!(roundtripped.node_ref, original.node_ref);
}

#[test]
fn test_aria_snapshot_ref_serialization() {
    let mut snapshot = AriaSnapshot::with_role("textbox");
    snapshot.name = Some("Email".to_string());
    snapshot.node_ref = Some("e42".to_string());

    // ref should serialize to JSON as "ref"
    let json = serde_json::to_string(&snapshot).expect("Should serialize");
    assert!(
        json.contains(r#""ref":"e42""#),
        "JSON should contain ref field, got: {}",
        json
    );

    // And deserialize
    let deserialized: AriaSnapshot = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(deserialized.node_ref, Some("e42".to_string()));
}

#[test]
fn test_aria_snapshot_ref_default_none() {
    // When not provided, node_ref should default to None
    let json = r#"{"role":"button","name":"Click me"}"#;
    let snapshot: AriaSnapshot = serde_json::from_str(json).expect("Should deserialize");
    assert!(
        snapshot.node_ref.is_none(),
        "node_ref should default to None"
    );
}

#[test]
fn test_aria_snapshot_ref_with_children() {
    let mut button = AriaSnapshot::with_role("button");
    button.name = Some("Submit".to_string());
    button.node_ref = Some("e100".to_string());

    let mut icon = AriaSnapshot::with_role("img");
    icon.name = Some("Icon".to_string());
    icon.node_ref = Some("e101".to_string());

    button.children.push(icon);

    let yaml = button.to_yaml();
    assert!(
        yaml.contains("[ref=e100]"),
        "Parent should have ref, got: {}",
        yaml
    );
    assert!(
        yaml.contains("[ref=e101]"),
        "Child should have ref, got: {}",
        yaml
    );
}

#[test]
fn test_aria_snapshot_ref_with_other_attributes() {
    let mut snapshot = AriaSnapshot::with_role("checkbox");
    snapshot.name = Some("Accept terms".to_string());
    snapshot.checked = Some(AriaCheckedState::True);
    snapshot.disabled = Some(true);
    snapshot.node_ref = Some("e999".to_string());

    let yaml = snapshot.to_yaml();
    // All attributes should be present
    assert!(
        yaml.contains("[checked]"),
        "Should have [checked], got: {}",
        yaml
    );
    assert!(
        yaml.contains("[disabled]"),
        "Should have [disabled], got: {}",
        yaml
    );
    assert!(
        yaml.contains("[ref=e999]"),
        "Should have [ref=e999], got: {}",
        yaml
    );
}

#[test]
fn test_aria_snapshot_ref_with_frame_boundary() {
    let mut snapshot = AriaSnapshot::with_role("iframe");
    snapshot.name = Some("Widget Frame".to_string());
    snapshot.is_frame = Some(true);
    snapshot.frame_url = Some("https://example.com".to_string());
    snapshot.node_ref = Some("e555".to_string());

    let yaml = snapshot.to_yaml();
    assert!(
        yaml.contains("[frame-boundary]"),
        "Should have [frame-boundary], got: {}",
        yaml
    );
    assert!(
        yaml.contains("[ref=e555]"),
        "Should have [ref=e555], got: {}",
        yaml
    );
}
