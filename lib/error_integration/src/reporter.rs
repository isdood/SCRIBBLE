//! Error integration module for structured error handling and reporting
//!
//! Created: 2025-01-20 23:35:51
//! Author: isdood
//!
//! This module provides tools for capturing, contextualizing, and reporting errors
//! in a structured way. It includes:
//!
//! - Error context tracking with file, line, and module information
//! - Structured error reporting with statistics
//! - Integration with diagnostic information
//!
//! # Examples
//!
//! ```
//! use error_integration::{ErrorContext, DefaultErrorReporter, ErrorReporter};
//! use thiserror::Error;
//! use error_derive::Diagnose;
//!
//! #[derive(Debug, Error, Diagnose)]
//! #[error("custom error: {msg}")]
//! struct CustomError {
//!     msg: String,
//! }
//!
//! // Create a reporter
//! let reporter = DefaultErrorReporter::default();
//!
//! // Create an error with context
//! let error = CustomError {
//!     msg: "something went wrong".to_string()
//! };
//! let context = ErrorContext::current()
//!     .with_extra("additional details");
//!
//! // Report the error
//! reporter.report_error(&error, &context);
//!
//! // Get error statistics
//! let stats = reporter.get_stats();
//! assert_eq!(stats.total_errors, 1);
//! ```
//!
//! # Features
//!
//! - Location tracking: Automatically captures file, line, and module information
//! - Timing information: Records when errors occur and tracks elapsed time
//! - Statistics: Maintains error counts by type and module
//! - Context: Supports additional contextual information
//! - Display formatting: Implements Display for human-readable output
//!
//! # Usage
//!
//! The main types you'll work with are:
//!
//! - [`ErrorContext`]: Captures where and when an error occurred
//! - [`ErrorReporter`]: Trait for implementing error reporters
//! - [`DefaultErrorReporter`]: Standard implementation of error reporting
//! - [`ReporterConfig`]: Configuration for error reporters

use crate::context::ErrorContext;
use error_core::{Diagnose, DiagnosticReport};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{error, info, Level};

/// Configuration for error reporting behavior
#[derive(Debug, Clone)]
pub struct ReporterConfig {
    /// Minimum level for logging errors
    pub min_level: Level,
    /// Whether to include backtraces
    pub include_backtraces: bool,
    /// Whether to collect error statistics
    pub collect_stats: bool,
    /// Maximum number of errors to track per type
    pub max_errors_per_type: usize,
}

impl Default for ReporterConfig {
    fn default() -> Self {
        Self {
            min_level: Level::ERROR,
            include_backtraces: true,
            collect_stats: true,
            max_errors_per_type: 1000,
        }
    }
}

/// Error statistics tracking
#[derive(Debug, Clone)]
pub struct ErrorStats {
    /// Total number of errors reported
    pub total_errors: usize,
    /// Errors counted by type
    pub by_type: HashMap<String, usize>,
    /// Errors counted by module
    pub by_module: HashMap<String, usize>,
    /// When the last error occurred
    pub last_error_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for ErrorStats {
    fn default() -> Self {
        Self {
            total_errors: 0,
            by_type: HashMap::new(),
            by_module: HashMap::new(),
            last_error_time: None,
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
#[derive(Debug)]
pub struct DefaultErrorReporter {
    config: ReporterConfig,
    stats: Arc<RwLock<ErrorStats>>,
}

impl DefaultErrorReporter {
    /// Create a new reporter with custom configuration
    pub fn new(config: ReporterConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(ErrorStats::default())),
        }
    }

    /// Get current error statistics
    pub fn get_stats(&self) -> ErrorStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset error statistics
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.write() {
            *stats = ErrorStats::default();
        }
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
            if let Ok(mut stats) = self.stats.write() {
                stats.total_errors += 1;

                let type_name = std::any::type_name::<E>().to_string();
                *stats.by_type.entry(type_name).or_default() += 1;

                if let Some(module) = context.module_path() {
                    *stats.by_module.entry(module.to_string()).or_default() += 1;
                }

                stats.last_error_time = Some(chrono::Utc::now());

                // Cleanup if we exceed max errors per type
                if stats.by_type.len() > self.config.max_errors_per_type {
                    stats.by_type.retain(|_, count| *count > 1);
                }
            }
        }

        // Get diagnostic information
        let diagnostic = error.diagnose();

        // Log the error with context
        error!(
            error = %error,
            context = %context,
            diagnostic = ?diagnostic,
            has_backtrace = self.config.include_backtraces,
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
        #[diagnose(detect = "test condition", suggestion = "fix condition")]
        Test,
    }

    #[test]
    fn test_error_reporting() {
        let reporter = DefaultErrorReporter::default();
        let error = TestError::Test;
        let context = ErrorContext::current();

        reporter.report_error(&error, &context);

        let stats = reporter.get_stats();
        assert_eq!(stats.total_errors, 1);
        assert!(stats.by_type.contains_key(std::any::type_name::<TestError>()));
    }

    #[test]
    fn test_stats_reset() {
        let reporter = DefaultErrorReporter::default();
        let error = TestError::Test;
        let context = ErrorContext::current();

        reporter.report_error(&error, &context);
        assert_eq!(reporter.get_stats().total_errors, 1);

        reporter.reset_stats();
        assert_eq!(reporter.get_stats().total_errors, 0);
    }

    #[test]
    fn test_max_errors_per_type() {
        let config = ReporterConfig {
            max_errors_per_type: 2,
            ..Default::default()
        };
        let reporter = DefaultErrorReporter::new(config);
        let error = TestError::Test;
        let context = ErrorContext::current();

        for _ in 0..5 {
            reporter.report_error(&error, &context);
        }

        let stats = reporter.get_stats();
        assert!(stats.by_type.len() <= 2);
    }
}
