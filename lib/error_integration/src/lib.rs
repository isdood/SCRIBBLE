//! Error integration module
//! Created: 2025-01-20 23:23:08
//! Author: isdood

use chrono::{DateTime, Utc};
use error_core::DiagnosticReport;

mod context;
mod reporter;

pub use context::ErrorContext;
pub use reporter::{ErrorReporter, DefaultErrorReporter, ReporterConfig};

/// Error metadata for tracking and reporting
#[derive(Debug)]
pub struct ErrorMetadata {
    /// Context of where/when the error occurred
    context: ErrorContext,
    /// Diagnostic information about the error
    diagnostic: DiagnosticReport,
    /// When this metadata was created
    created: DateTime<Utc>,
}

impl ErrorMetadata {
    /// Creates new error metadata with the given context and diagnostic
    pub fn new(context: ErrorContext, diagnostic: DiagnosticReport) -> Self {
        Self {
            context,
            diagnostic,
            created: Utc::now(),
        }
    }

    /// Returns a reference to the error context
    pub fn context(&self) -> &ErrorContext {
        &self.context
    }

    /// Returns a reference to the diagnostic report
    pub fn diagnostic(&self) -> &DiagnosticReport {
        &self.diagnostic
    }

    /// Returns when this metadata was created
    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error_core::Diagnose;
    use error_derive::Diagnose;
    use thiserror::Error;

    #[derive(Debug, Error, Diagnose)]
    #[error("test error")]
    enum TestError {
        #[diagnose(detect = "test condition", suggestion = "fix condition")]
        Test,
    }

    #[test]
    fn test_error_metadata() {
        let context = ErrorContext::current();
        let error = TestError::Test;
        let diagnostic = error.diagnose();

        let metadata = ErrorMetadata::new(context, diagnostic);
        assert!(metadata.created() <= Utc::now());
    }

    #[test]
    fn test_metadata_getters() {
        let context = ErrorContext::current();
        let error = TestError::Test;
        let diagnostic = error.diagnose();

        // Clone context before moving it into metadata
        let context_clone = context.clone();
        let metadata = ErrorMetadata::new(context, diagnostic);

        assert_eq!(metadata.context().file, context_clone.file);
        assert!(!metadata.diagnostic().suggestions.is_empty());
    }

    #[test]
    fn test_metadata_creation_time() {
        let context = ErrorContext::current();
        let error = TestError::Test;
        let diagnostic = error.diagnose();

        let before = Utc::now();
        let metadata = ErrorMetadata::new(context, diagnostic);
        let after = Utc::now();

        assert!(metadata.created() >= before);
        assert!(metadata.created() <= after);
    }
}
