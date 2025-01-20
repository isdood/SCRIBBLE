//! Integration tests for error integration
//! Created: 2025-01-20 22:17:33
//! Author: isdood

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

#[test]
fn test_error_context() {
    let context = ErrorContext::current();
    assert!(context.file.contains("integration_tests.rs"));
    assert!(context.module_path.is_some());
}

#[test]
fn test_reporter_config() {
    let reporter = DefaultErrorReporter::default();
    assert!(reporter.config().collect_stats);
    assert!(reporter.config().include_backtraces);
}
