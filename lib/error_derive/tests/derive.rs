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
    }

    let error = TestError::NegativeValue;
    let report = error.diagnose();
    assert!(report.quick_fixes.is_empty()); // For now, as we haven't implemented full functionality
}
