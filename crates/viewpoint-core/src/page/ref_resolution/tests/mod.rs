use super::*;

#[test]
fn test_parse_ref_new_format() {
    let parsed = parse_ref("c0p0f0e1").unwrap();
    assert_eq!(parsed.context_index, 0);
    assert_eq!(parsed.page_index, 0);
    assert_eq!(parsed.frame_index, 0);
    assert_eq!(parsed.element_counter, 1);
}

#[test]
fn test_parse_ref_new_format_larger_indices() {
    let parsed = parse_ref("c12p34f56e789").unwrap();
    assert_eq!(parsed.context_index, 12);
    assert_eq!(parsed.page_index, 34);
    assert_eq!(parsed.frame_index, 56);
    assert_eq!(parsed.element_counter, 789);
}

#[test]
fn test_parse_ref_child_frame() {
    let parsed = parse_ref("c0p0f1e5").unwrap();
    assert_eq!(parsed.context_index, 0);
    assert_eq!(parsed.page_index, 0);
    assert_eq!(parsed.frame_index, 1);
    assert_eq!(parsed.element_counter, 5);
}

#[test]
fn test_parse_ref_invalid_format() {
    assert!(parse_ref("invalid").is_err());
    assert!(parse_ref("x0p0f0e1").is_err());
    assert!(parse_ref("c0p0e1").is_err()); // missing frame
    assert!(parse_ref("c0f0e1").is_err()); // missing page
    assert!(parse_ref("").is_err());
}

#[test]
fn test_parse_ref_legacy_format_rejected() {
    // Legacy e{id} format is no longer supported
    assert!(parse_ref("e12345").is_err());
    assert!(parse_ref("e1").is_err());
}

#[test]
fn test_parse_ref_invalid_numbers() {
    assert!(parse_ref("cXp0f0e1").is_err());
    assert!(parse_ref("c0pXf0e1").is_err());
    assert!(parse_ref("c0p0fXe1").is_err());
    assert!(parse_ref("c0p0f0eX").is_err());
}

#[test]
fn test_format_ref() {
    assert_eq!(format_ref(0, 0, 0, 1), "c0p0f0e1");
    assert_eq!(format_ref(1, 2, 3, 4), "c1p2f3e4");
    assert_eq!(format_ref(12, 34, 56, 789), "c12p34f56e789");
}

#[test]
fn test_format_and_parse_roundtrip() {
    let original = format_ref(5, 10, 2, 100);
    let parsed = parse_ref(&original).unwrap();
    assert_eq!(parsed.context_index, 5);
    assert_eq!(parsed.page_index, 10);
    assert_eq!(parsed.frame_index, 2);
    assert_eq!(parsed.element_counter, 100);
}

#[test]
fn test_parsed_ref_new() {
    let parsed = ParsedRef::new(1, 2, 3, 4);
    assert_eq!(parsed.context_index, 1);
    assert_eq!(parsed.page_index, 2);
    assert_eq!(parsed.frame_index, 3);
    assert_eq!(parsed.element_counter, 4);
}

#[test]
fn test_parsed_ref_equality() {
    let a = ParsedRef::new(1, 2, 3, 4);
    let b = ParsedRef::new(1, 2, 3, 4);
    let c = ParsedRef::new(1, 2, 3, 5);
    assert_eq!(a, b);
    assert_ne!(a, c);
}
