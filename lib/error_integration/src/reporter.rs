//! Error reporting module for structured error handling and diagnostics
//! Created: 2025-01-20 21:39:10
//! Author: isdood

use crate::context::ErrorContext;
use error_core::{Diagnose, DiagnosticReport};
use serde::Serialize;
use std::sync::Arc;
use tracing::{error, warn, info, Level};

/// Configuration for error reporting behavior
#[derive(Debug, Clone)]
pub struct ReporterConfig {
    /// Minimum level for logging errors
    pub min_level: Level,
    /// Whether to include backtraces
    pub include_backtraces: bool,
    /// Whether to collect error statistics
    pub collect_stats: bool,
}

impl Default for ReporterConfig {
    fn default() -> Self {
        Self {
            min_level: Level::ERROR,
            include_backtraces: true,
            collect_stats: true,
        }
    }
}

/// Trait for error reporting implementations
pub trait ErrorReporter: Send + Sync {
    /// Report an error with context
    fn report_error<E>(&self, error: &E, context: &ErrorContext)
    where
    E: std::error::Error + Diagnose + 'static;

    /// Collect diagnostic information
    fn collect_diagnostics<E>(&self, error: &E) -> Vec<DiagnosticReport>
    where
    E: Diagnose;

    /// Get reporter configuration
    fn config(&self) -> &ReporterConfig;
}

/// Default implementation of ErrorReporter
#[derive(Debug, Clone)]
pub struct DefaultErrorReporter {
    config: ReporterConfig,
    #[allow(dead_code)]
    stats: Arc<parking_lot::RwLock<ErrorStats>>,
}

#[derive(Debug, Default, Serialize)]
struct ErrorStats {
    total_errors: usize,
    by_type: std::collections::HashMap<String, usize>,
    by_module: std::collections::HashMap<String, usize>,
}

impl DefaultErrorReporter {
    /// Create a new reporter with custom configuration
    pub fn new(config: ReporterConfig) -> Self {
        Self {
            config,
            stats: Arc::new(parking_lot::RwLock::new(ErrorStats::default())),
        }
    }

    /// Get current error statistics
    pub fn get_stats(&self) -> ErrorStats {
        self.stats.read().clone()
    }
}

impl Default for DefaultErrorReporter {
    fn default() -> Self {
        Self::new(ReporterConfig::default())
    }
}

impl ErrorReporter for DefaultErrorReporter {
    fn report_error<E>(&self, error: &E, context: &ErrorContext)
    where
    E: std::error::Error + Diagnose + 'static
    {
        // Update statistics if enabled
        if self.config.collect_stats {
            let mut stats = self.stats.write();
            stats.total_errors += 1;
            *stats.by_type.entry(std::any::type_name::<E>().to_string()).or_default() += 1;
            if let Some(module) = &context.module_path {
                *stats.by_module.entry(module.clone()).or_default() += 1;
            }
        }

        // Get diagnostic information
        let diagnostic = error.diagnose();

        // Log the error with context
        error!(
            error = %error,
            context = %context,
            diagnostic = ?diagnostic,
            backtrace = self.config.include_backtraces.then(|| std::backtrace::Backtrace::capture()),
               "Error occurred"
        );

        // Log suggestions if available
        if !diagnostic.suggestions.is_empty() {
            info!(
                suggestions = ?diagnostic.suggestions,
                "Suggested fixes available"
            );
        }
    }

    fn collect_diagnostics<E>(&self, error: &E) -> Vec<DiagnosticReport>
    where
    E: Diagnose
    {
        vec![error.diagnose()]
    }

    fn config(&self) -> &ReporterConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error_derive::Diagnose;
    use thiserror::Error;

    #[derive(Debug, Error, Diagnose)]
    #[error("test error")]
    enum TestError {
        #[diagnose(detect = "test condition", suggestion = "try this instead")]
        Basic,
    }

    #[test]
    fn test_error_reporting() {
        let reporter = DefaultErrorReporter::default();
        let error = TestError::Basic;
        let context = ErrorContext::current();

        reporter.report_error(&error, &context);

        let stats = reporter.get_stats();
        assert_eq!(stats.total_errors, 1);
        assert!(stats.by_type.contains_key(std::any::type_name::<TestError>()));
    }

    #[test]
    fn test_custom_config() {
        let config = ReporterConfig {
            min_level: Level::WARN,
            include_backtraces: false,
            collect_stats: true,
        };
        let reporter = DefaultErrorReporter::new(config);
        assert!(!reporter.config().include_backtraces);
    }

    #[test]
    fn test_diagnostics_collection() {
        let reporter = DefaultErrorReporter::default();
        let error = TestError::Basic;

        let diagnostics = reporter.collect_diagnostics(&error);
        assert_eq!(diagnostics.len(), 1);
        assert!(!diagnostics[0].suggestions.is_empty());
    }
}
