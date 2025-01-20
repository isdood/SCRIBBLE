//! Integration tests for error handling
//! Created: 2025-01-20 22:05:27
//! Author: isdood

use error_core::Diagnose;
use error_derive::Diagnose;
use error_integration::{DefaultErrorReporter, ErrorContext, ErrorReporter};
use thiserror::Error;

#[derive(Debug, Error, Diagnose)]
#[error("integration test error")]
enum TestError {
    #[diagnose(detect = "test failure", suggestion = "fix the test")]
    TestFailure,
}

#[test]
fn test_error_reporting() {
    let reporter = DefaultErrorReporter::default();
    let error = TestError::TestFailure;
    let context = ErrorContext::current();

    reporter.report_error(&error, &context);

    let stats = reporter.get_stats();
    assert_eq!(stats.total_errors, 1);
    assert!(stats.by_type.contains_key(std::any::type_name::<TestError>()));
}
