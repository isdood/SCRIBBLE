// lib/error_integration/src/lib.rs
use error_core::{Diagnose, DiagnosticReport};
use tracing::{error, warn, info};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorContext {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

pub trait ErrorReporter {
    fn report_error<E: Diagnose + std::error::Error>(&self, error: &E);
    fn collect_diagnostics<E: Diagnose>(&self, error: &E) -> Vec<DiagnosticReport>;
}

pub struct DefaultErrorReporter;

impl ErrorReporter for DefaultErrorReporter {
    fn report_error<E: Diagnose + std::error::Error>(&self, error: &E) {
        let report = error.diagnose();
        error!(
            error = %error,
            diagnostic = ?report,
            "Error occurred with diagnostic information"
        );
    }

    fn collect_diagnostics<E: Diagnose>(&self, error: &E) -> Vec<DiagnosticReport> {
        vec![error.diagnose()]
    }
}
