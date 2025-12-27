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
