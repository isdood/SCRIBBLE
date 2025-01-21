//! Core error recovery system implementation
//! Created: 2025-01-21 00:02:36
//! Author: isdood

use crate::{
    analysis::RecoveryAnalyzer,
    reporting::ReportBuilder,
    strategies::{ResourceExhaustionStrategy, TransientErrorStrategy},
    validation::ValidationRegistry,
    RecoveryConfig, RecoveryError, RecoveryManager, RecoveryStrategy,
};
use error_integration::context::ErrorContext;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Core error recovery system that coordinates all components
#[derive(Clone)]
pub struct RecoveryCore {
    manager: Arc<RecoveryManager>,
    analyzer: Arc<RecoveryAnalyzer>,
    validation: Arc<ValidationRegistry>,
    report_builder: Arc<RwLock<ReportBuilder>>,
    context: ErrorContext,
}

impl RecoveryCore {
    /// Creates a new RecoveryCore with default configuration
    pub fn new() -> Self {
        let validation = Arc::new(ValidationRegistry::new());
        let analyzer = Arc::new(RecoveryAnalyzer::new(validation.clone()));

        Self {
            manager: Arc::new(RecoveryManager::new(RecoveryConfig::default())),
            analyzer,
            validation,
            report_builder: Arc::new(RwLock::new(ReportBuilder::new())),
            context: ErrorContext::new(),
        }
    }

    /// Initialize the core system with default strategies
    pub async fn initialize(&self) -> Result<(), RecoveryError> {
        info!(
            time = chrono::Utc::now().to_rfc3339(),
              "Initializing recovery core"
        );

        // Register built-in strategies
        let mut manager = RecoveryManager::new(RecoveryConfig::default());
        manager.register_strategy(TransientErrorStrategy::new());
        manager.register_strategy(ResourceExhaustionStrategy::new());

        // Validate initial configuration
        let validation_results = self.validation
        .validate_strategy(&manager)
        .await;

        if validation_results.iter().any(|r| !r.is_valid) {
            warn!("Validation warnings in core initialization");
        }

        info!("Recovery core initialized successfully");
        Ok(())
    }

    /// Register a custom recovery strategy
    pub fn register_strategy<S>(&mut self, strategy: S) -> Result<(), RecoveryError>
    where
    S: RecoveryStrategy + 'static,
    {
        info!(
            strategy_type = std::any::type_name::<S>(),
              "Registering new recovery strategy"
        );

        let mut manager = RecoveryManager::new(self.manager.config().clone());
        manager.register_strategy(strategy);
        self.manager = Arc::new(manager);
        Ok(())
    }

    /// Attempt to recover from an error
    pub async fn recover<E, T, F, Fut>(
        &self,
        error: E,
        operation: F,
    ) -> Result<T, E>
    where
    E: std::error::Error + Send + Sync + 'static,
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    {
        let start = std::time::Instant::now();

        info!(
            error = %error,
            "Starting recovery attempt"
        );

        let result = self.manager
        .attempt_recovery(error, &self.context, operation)
        .await;

        // Record the recovery attempt
        self.analyzer
        .record_pattern(&error, &self.context, result.is_ok())
        .await;

        let duration = start.elapsed();
        info!(
            success = result.is_ok(),
              duration_ms = duration.as_millis(),
              "Recovery attempt completed"
        );

        result
    }

    /// Generate a report of recovery patterns and analysis
    pub async fn generate_report(&self) -> crate::reporting::RecoveryReport {
        info!("Generating recovery analysis report");

        let analysis = self.analyzer
        .analyze_strategy(&*self.manager)
        .await;

        let mut report_builder = self.report_builder.write().await;
        report_builder.add_analysis_result(analysis);

        let report = report_builder.build();

        info!(
            total_patterns = report.summary.total_patterns,
            success_rate = %format!("{:.2}%", report.summary.overall_success_rate * 100.0),
              "Report generated successfully"
        );

        report
    }
}

impl Default for RecoveryCore {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for one-off recovery attempts
pub async fn recover<E, T, F, Fut>(
    error: E,
    operation: F,
) -> Result<T, E>
where
E: std::error::Error + Send + Sync + 'static,
F: Fn() -> Fut,
Fut: std::future::Future<Output = Result<T, E>>,
{
    let core = RecoveryCore::new();
    core.initialize().await?;
    core.recover(error, operation).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[tokio::test]
    async fn test_recovery_core() {
        let core = RecoveryCore::new();
        core.initialize().await.unwrap();

        let result = core
        .recover(
            IoError::new(ErrorKind::WouldBlock, "test error"),
                 || async { Ok::<_, IoError>(42) },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_report_generation() {
        let core = RecoveryCore::new();
        core.initialize().await.unwrap();

        // Generate some recovery data
        let _result = core
        .recover(
            IoError::new(ErrorKind::WouldBlock, "test error"),
                 || async { Ok::<_, IoError>(42) },
        )
        .await;

        let report = core.generate_report().await;
        assert_eq!(report.summary.total_patterns, 1);
        assert!(report.summary.overall_success_rate > 0.0);
    }

    #[tokio::test]
    async fn test_convenience_recover() {
        let result = recover(
            IoError::new(ErrorKind::WouldBlock, "test error"),
                             || async { Ok::<_, IoError>(42) },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }
}
