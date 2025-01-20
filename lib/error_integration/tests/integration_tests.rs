// lib/error_integration/tests/integration_test.rs
use error_integration::{ErrorReporter, DefaultErrorReporter};
use error_derive::Diagnose;

#[derive(Debug, thiserror::Error, Diagnose)]
#[error_path = "test/integration"]
enum TestError {
    #[error("Invalid operation")]
    #[diagnose(
    detect = "operation not permitted",
    suggestion = "Check permissions",
    quick_fix = "verify_permissions()"
    )]
    InvalidOperation,
}

#[test]
fn test_error_reporting() {
    let reporter = DefaultErrorReporter;
    let error = TestError::InvalidOperation;

    let diagnostics = reporter.collect_diagnostics(&error);
    assert!(!diagnostics.is_empty());

    let report = &diagnostics[0];
    assert!(report.message.contains("operation not permitted"));
}
