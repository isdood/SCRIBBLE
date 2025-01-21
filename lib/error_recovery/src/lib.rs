//! Automated error recovery system for Scribble
//! Created: 2025-01-20 23:45:55
//! Author: isdood

use async_trait::async_trait;
use error_integration::context::ErrorContext;
use std::future::Future;
use std::time::Duration;
use thiserror::Error;

pub mod strategies;
pub mod macros;
pub mod validation;
pub mod analysis;
pub mod reporting;
pub mod core;
pub mod monitor;

pub use core::{RecoveryCore, recover};
pub use monitor::{RecoveryMonitor, RecoveryMetrics, Alert, AlertSeverity};

#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error("Maximum recovery attempts exceeded")]
    MaxAttemptsExceeded,
    #[error("No suitable recovery strategy found")]
    NoStrategyFound,
    #[error("Recovery strategy failed: {0}")]
    StrategyFailed(String),
}

/// Configuration for error recovery strategies
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// Maximum number of recovery attempts
    pub max_attempts: u32,
    /// Delay between recovery attempts
    pub retry_delay: Duration,
    /// Whether to use exponential backoff
    pub use_backoff: bool,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            retry_delay: Duration::from_millis(100),
            use_backoff: true,
        }
    }
}

/// Trait defining recovery strategy behavior
#[async_trait]
pub trait RecoveryStrategy: Send + Sync {
    /// The error type this strategy handles
    type Error: std::error::Error + Send + Sync;

    /// Attempt to recover from an error
    async fn attempt_recovery(
        &self,
        error: &Self::Error,
        context: &ErrorContext,
    ) -> Result<(), RecoveryError>;

    /// Check if this strategy can handle the given error
    fn can_handle(&self, error: &Self::Error) -> bool;

    /// Get the configuration for this strategy
    fn config(&self) -> &RecoveryConfig;
}

/// Recovery manager that coordinates recovery strategies
pub struct RecoveryManager {
    strategies: Vec<Box<dyn RecoveryStrategy<Error = Box<dyn std::error::Error + Send + Sync>>>>,
    config: RecoveryConfig,
}

impl RecoveryManager {
    pub fn new(config: RecoveryConfig) -> Self {
        Self {
            strategies: Vec::new(),
            config,
        }
    }

    pub fn register_strategy<S>(&mut self, strategy: S)
    where
    S: RecoveryStrategy<Error = Box<dyn std::error::Error + Send + Sync>> + 'static,
    {
        self.strategies.push(Box::new(strategy));
    }

    pub async fn attempt_recovery<E, T, F, Fut>(
        &self,
        error: E,
        context: &ErrorContext,
        operation: F,
    ) -> Result<T, E>
    where
    E: std::error::Error + Send + Sync + 'static,
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    {
        let mut attempts = 0;
        let mut last_error = error;
        let boxed_error: Box<dyn std::error::Error + Send + Sync> = Box::new(last_error);

        while attempts < self.config.max_attempts {
            for strategy in &self.strategies {
                if strategy.can_handle(&boxed_error) {
                    match strategy.attempt_recovery(&boxed_error, context).await {
                        Ok(_) => {
                            // Strategy succeeded, try the operation again
                            match operation().await {
                                Ok(result) => return Ok(result),
                                Err(e) => last_error = e,
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }

            attempts += 1;
            if attempts < self.config.max_attempts {
                let delay = if self.config.use_backoff {
                    self.config.retry_delay * (2_u32.pow(attempts) as u32)
                } else {
                    self.config.retry_delay
                };
                tokio::time::sleep(delay).await;
            }
        }

        Err(last_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug, Error)]
    #[error("Test error: {message}")]
    struct TestError {
        message: String,
    }

    struct TestStrategy {
        config: RecoveryConfig,
    }

    #[async_trait]
    impl RecoveryStrategy for TestStrategy {
        type Error = TestError;

        async fn attempt_recovery(
            &self,
            _error: &Self::Error,
            _context: &ErrorContext,
        ) -> Result<(), RecoveryError> {
            Ok(())
        }

        fn can_handle(&self, _error: &Self::Error) -> bool {
            true
        }

        fn config(&self) -> &RecoveryConfig {
            &self.config
        }
    }

    #[tokio::test]
    async fn test_recovery_manager_basic() {
        let context = ErrorContext::new();
        let config = RecoveryConfig::default();
        let manager = RecoveryManager::new(config);

        let result = manager
        .attempt_recovery(
            TestError {
                message: "test".to_string(),
            },
            &context,
            || async { Ok::<_, TestError>(42) },
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_recovery_strategy() {
        let context = ErrorContext::new();
        let config = RecoveryConfig::default();
        let strategy = TestStrategy { config };

        let result = strategy
        .attempt_recovery(
            &TestError {
                message: "test".to_string(),
            },
            &context,
        )
        .await;

        assert!(result.is_ok());
    }
}
