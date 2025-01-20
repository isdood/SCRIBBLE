use error_derive::Diagnose;
use error_core::Diagnose as _;

#[test]
fn test_basic_derive() {
    #[derive(Debug, Diagnose)]
    #[error_path = "test/errors"]
    enum TestError {
        #[diagnose(
        detect = "value < 0",
        suggestion = "Value must be positive",
        quick_fix = "set_positive_value()"
        )]
        NegativeValue,

        // Variant without diagnose attribute
        SimpleError,
    }

    // Test variant with diagnose attribute
    let error = TestError::NegativeValue;
    let report = error.diagnose();
    assert!(!report.quick_fixes.is_empty());
    assert_eq!(report.quick_fixes[0].code, "set_positive_value()");
    assert_eq!(report.suggestions[0], "Value must be positive");
    assert!(report.message.contains("value < 0"));

    // Test variant without diagnose attribute
    let error = TestError::SimpleError;
    let report = error.diagnose();
    assert!(report.quick_fixes.is_empty());
    assert!(report.suggestions.is_empty());

    // Test compile-time checks
    assert!(TestError::check_at_compile_time().is_none());
}

#[test]
fn test_missing_error_path() {
    #[derive(Debug, Diagnose)]
    enum NoPathError {
        #[diagnose(
        detect = "test",
        suggestion = "test",
        quick_fix = "test()"
        )]
        TestError,
    }

    // Should have compile-time error due to missing error_path
    let compile_error = NoPathError::check_at_compile_time().unwrap();
    assert!(compile_error.message.contains("error_path"));
}
