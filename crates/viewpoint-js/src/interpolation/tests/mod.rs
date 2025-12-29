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
fn test_single_value_interpolation() {
    let (segments, has_interp) = parse_interpolations("document.getElementById(#{id})");
    assert!(has_interp);
    assert_eq!(segments.len(), 3);

    match &segments[0] {
        Segment::Literal(s) => assert_eq!(s, "document.getElementById("),
        _ => panic!("Expected literal"),
    }

    assert!(matches!(&segments[1], Segment::ValueInterpolation(_)));

    match &segments[2] {
        Segment::Literal(s) => assert_eq!(s, ")"),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn test_multiple_value_interpolations() {
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
    assert!(segments
        .iter()
        .any(|s| matches!(s, Segment::ValueInterpolation(_))));
}

#[test]
fn test_value_interpolation_at_start() {
    let (segments, has_interp) = parse_interpolations("#{x} + 1");
    assert!(has_interp);
    assert!(matches!(&segments[0], Segment::ValueInterpolation(_)));
}

#[test]
fn test_value_interpolation_at_end() {
    let (segments, has_interp) = parse_interpolations("1 + #{x}");
    assert!(has_interp);
    assert!(matches!(
        segments.last().unwrap(),
        Segment::ValueInterpolation(_)
    ));
}

// Raw interpolation tests (@{expr})

#[test]
fn test_single_raw_interpolation() {
    let (segments, has_interp) = parse_interpolations("Array.from(@{selector_expr})");
    assert!(has_interp);
    assert_eq!(segments.len(), 3);

    match &segments[0] {
        Segment::Literal(s) => assert_eq!(s, "Array.from("),
        _ => panic!("Expected literal"),
    }

    assert!(matches!(&segments[1], Segment::RawInterpolation(_)));

    match &segments[2] {
        Segment::Literal(s) => assert_eq!(s, ")"),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn test_raw_interpolation_at_start() {
    let (segments, has_interp) = parse_interpolations("@{expr}.length");
    assert!(has_interp);
    assert!(matches!(&segments[0], Segment::RawInterpolation(_)));
}

#[test]
fn test_raw_interpolation_at_end() {
    let (segments, has_interp) = parse_interpolations("return @{result}");
    assert!(has_interp);
    assert!(matches!(
        segments.last().unwrap(),
        Segment::RawInterpolation(_)
    ));
}

#[test]
fn test_at_without_brace() {
    let (segments, has_interp) = parse_interpolations("x @ y");
    assert!(!has_interp);
    assert_eq!(segments.len(), 1);
    match &segments[0] {
        Segment::Literal(s) => assert_eq!(s, "x @ y"),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn test_mixed_value_and_raw_interpolation() {
    let (segments, has_interp) =
        parse_interpolations("@{expr}.setAttribute('data-id', #{id})");
    assert!(has_interp);
    // Segments: RawInterpolation, Literal(".setAttribute('data-id', "), ValueInterpolation, Literal(")")
    assert_eq!(segments.len(), 4);

    assert!(matches!(&segments[0], Segment::RawInterpolation(_)));

    match &segments[1] {
        Segment::Literal(s) => assert_eq!(s, ".setAttribute('data-id', "),
        _ => panic!("Expected literal"),
    }

    assert!(matches!(&segments[2], Segment::ValueInterpolation(_)));

    match &segments[3] {
        Segment::Literal(s) => assert_eq!(s, ")"),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn test_multiple_raw_interpolations() {
    let (segments, has_interp) = parse_interpolations("[@{a}, @{b}]");
    assert!(has_interp);
    assert_eq!(segments.len(), 5);

    assert!(matches!(&segments[0], Segment::Literal(_)));
    assert!(matches!(&segments[1], Segment::RawInterpolation(_)));
    assert!(matches!(&segments[2], Segment::Literal(_)));
    assert!(matches!(&segments[3], Segment::RawInterpolation(_)));
    assert!(matches!(&segments[4], Segment::Literal(_)));
}

#[test]
fn test_nested_braces_in_raw_interpolation() {
    let (segments, has_interp) = parse_interpolations("@{func()}");
    assert!(has_interp);
    assert!(segments
        .iter()
        .any(|s| matches!(s, Segment::RawInterpolation(_))));
}
