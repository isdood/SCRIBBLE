//! Common error recovery strategies implementation
//! Created: 2025-01-20 23:49:15
//! Author: isdood

use crate::{RecoveryConfig, RecoveryError, RecoveryStrategy};
use async_trait::async_trait;
use error_integration::context::ErrorContext;
use std::time::Duration;

/// Strategy for handling transient network errors
#[derive(Debug)]
pub struct TransientErrorStrategy {
    config: RecoveryConfig,
}

impl TransientErrorStrategy {
    pub fn new() -> Self {
        Self {
            config: RecoveryConfig {
                max_attempts: 3,
                retry_delay: Duration::from_millis(500),
                use_backoff: true,
            },
        }
    }
}

#[async_trait]
impl RecoveryStrategy for TransientErrorStrategy {
    type Error = std::io::Error;

    async fn attempt_recovery(
        &self,
        error: &Self::Error,
        context: &ErrorContext,
    ) -> Result<(), RecoveryError> {
        // Only attempt recovery for temporary errors
        if error.kind() == std::io::ErrorKind::WouldBlock ||
            error.kind() == std::io::ErrorKind::TimedOut {
                // Log recovery attempt
                tracing::info!(
                    error = %error,
                    context = ?context,
                    "Attempting recovery for transient error"
                );
                Ok(())
            } else {
                Err(RecoveryError::StrategyFailed(
                    "Error is not transient".to_string(),
                ))
            }
    }

    fn can_handle(&self, error: &Self::Error) -> bool {
        matches!(
            error.kind(),
                 std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
        )
    }

    fn config(&self) -> &RecoveryConfig {
        &self.config
    }
}

/// Strategy for handling resource exhaustion errors
#[derive(Debug)]
pub struct ResourceExhaustionStrategy {
    config: RecoveryConfig,
}

impl ResourceExhaustionStrategy {
    pub fn new() -> Self {
        Self {
            config: RecoveryConfig {
                max_attempts: 5,
                retry_delay: Duration::from_secs(1),
                use_backoff: true,
            },
        }
    }

    async fn attempt_resource_cleanup(&self) -> Result<(), RecoveryError> {
        // Implement resource cleanup logic here
        Ok(())
    }
}

#[async_trait]
impl RecoveryStrategy for ResourceExhaustionStrategy {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn attempt_recovery(
        &self,
        error: &Self::Error,
        context: &ErrorContext,
    ) -> Result<(), RecoveryError> {
        tracing::info!(
            error = %error,
            context = ?context,
            "Attempting recovery for resource exhaustion"
        );

        self.attempt_resource_cleanup().await?;
        Ok(())
    }

    fn can_handle(&self, error: &Self::Error) -> bool {
        error.to_string().contains("resource exhausted") ||
        error.to_string().contains("out of memory")
    }

    fn config(&self) -> &RecoveryConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[tokio::test]
    async fn test_transient_error_strategy() {
        let strategy = TransientErrorStrategy::new();
        let context = ErrorContext::new();

        // Test with a WouldBlock error (should recover)
        let error = IoError::new(ErrorKind::WouldBlock, "would block");
        let result = strategy.attempt_recovery(&error, &context).await;
        assert!(result.is_ok());

        // Test with a non-transient error (should not recover)
        let error = IoError::new(ErrorKind::NotFound, "not found");
        let result = strategy.attempt_recovery(&error, &context).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_resource_exhaustion_strategy() {
        let strategy = ResourceExhaustionStrategy::new();
        let context = ErrorContext::new();

        // Test with a resource exhaustion error
        let error: Box<dyn std::error::Error + Send + Sync> =
        Box::new(std::io::Error::new(
            ErrorKind::Other,
            "resource exhausted: no more memory"
        ));
        let result = strategy.attempt_recovery(&error, &context).await;
        assert!(result.is_ok());
    }
}
