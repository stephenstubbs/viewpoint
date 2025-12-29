//! Tests for soft assertions.

use crate::expect::soft::{SoftAssertionError, SoftAssertions};

#[test]
fn test_soft_assertions_new() {
    let soft = SoftAssertions::new();
    assert!(soft.passed());
    assert_eq!(soft.failure_count(), 0);
}

#[test]
fn test_soft_assertions_add_error() {
    let soft = SoftAssertions::new();
    soft.add_error(SoftAssertionError::new("test", "failed"));

    assert!(!soft.passed());
    assert_eq!(soft.failure_count(), 1);

    let errors = soft.errors();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].assertion, "test");
    assert_eq!(errors[0].message, "failed");
}

#[test]
fn test_soft_assertions_clear() {
    let soft = SoftAssertions::new();
    soft.add_error(SoftAssertionError::new("test", "failed"));
    assert!(!soft.passed());

    soft.clear();
    assert!(soft.passed());
    assert_eq!(soft.failure_count(), 0);
}

#[test]
fn test_soft_assertions_assert_all_success() {
    let soft = SoftAssertions::new();
    assert!(soft.assert_all().is_ok());
}

#[test]
fn test_soft_assertions_assert_all_failure() {
    let soft = SoftAssertions::new();
    soft.add_error(SoftAssertionError::new("test1", "failed 1"));
    soft.add_error(SoftAssertionError::new("test2", "failed 2"));

    let result = soft.assert_all();
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("2 soft assertion(s) failed"));
    assert!(err.contains("test1"));
    assert!(err.contains("test2"));
}

#[test]
fn test_soft_assertion_error_display() {
    let error = SoftAssertionError::new("to_have_text", "Text mismatch")
        .with_expected("Hello")
        .with_actual("World");

    let display = format!("{}", error);
    assert!(display.contains("to_have_text"));
    assert!(display.contains("Text mismatch"));
    assert!(display.contains("Expected: Hello"));
    assert!(display.contains("Actual: World"));
}
