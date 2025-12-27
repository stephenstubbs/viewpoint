use super::*;

#[test]
fn test_no_interpolation() {
    let (segments, has_interp) = parse_interpolations("1 + 2");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    match &segments[0] {
        Segment::Literal(s) => assert_eq!(s, "1 + 2"),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn test_single_interpolation() {
    let (segments, has_interp) = parse_interpolations("document.getElementById(#{id})");
    assert!(has_interp);
    assert_eq!(segments.len(), 3);

    match &segments[0] {
        Segment::Literal(s) => assert_eq!(s, "document.getElementById("),
        _ => panic!("Expected literal"),
    }

    assert!(matches!(&segments[1], Segment::Interpolation(_)));

    match &segments[2] {
        Segment::Literal(s) => assert_eq!(s, ")"),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn test_multiple_interpolations() {
    let (segments, has_interp) = parse_interpolations("[#{x}, #{y}]");
    assert!(has_interp);
    assert_eq!(segments.len(), 5);
}

#[test]
fn test_hash_without_brace() {
    let (segments, has_interp) = parse_interpolations("x # y");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    match &segments[0] {
        Segment::Literal(s) => assert_eq!(s, "x # y"),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn test_nested_braces_in_interpolation() {
    let (segments, has_interp) = parse_interpolations("#{vec![1, 2]}");
    assert!(has_interp);
    // Should have parsed as a single interpolation
    assert!(segments.iter().any(|s| matches!(s, Segment::Interpolation(_))));
}

#[test]
fn test_interpolation_at_start() {
    let (segments, has_interp) = parse_interpolations("#{x} + 1");
    assert!(has_interp);
    assert!(matches!(&segments[0], Segment::Interpolation(_)));
}

#[test]
fn test_interpolation_at_end() {
    let (segments, has_interp) = parse_interpolations("1 + #{x}");
    assert!(has_interp);
    assert!(matches!(segments.last().unwrap(), Segment::Interpolation(_)));
}
