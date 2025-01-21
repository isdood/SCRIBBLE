//! Compile-time validation for error recovery strategies
//! Created: 2025-01-20 23:52:14
//! Author: isdood

use crate::{RecoveryError, RecoveryStrategy};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Validation rules for error recovery
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub name: String,
    pub description: String,
    pub severity: ValidationSeverity,
    pub check: Arc<dyn Fn(&dyn RecoveryStrategy) -> ValidationResult + Send + Sync>,
}

/// Severity levels for validation rules
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Result of a validation check
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub message: String,
    pub severity: ValidationSeverity,
}

/// Registry for validation rules
#[derive(Default)]
pub struct ValidationRegistry {
    rules: Arc<RwLock<HashMap<String, ValidationRule>>>,
}

impl ValidationRegistry {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_rule(&self, rule: ValidationRule) {
        let mut rules = self.rules.write().await;
        rules.insert(rule.name.clone(), rule);
    }

    pub async fn validate_strategy(
        &self,
        strategy: &dyn RecoveryStrategy,
    ) -> Vec<ValidationResult> {
        let rules = self.rules.read().await;
        let mut results = Vec::new();

        for rule in rules.values() {
            let result = (rule.check)(strategy);
            results.push(result);
        }

        results
    }
}

/// Common validation rules
pub mod rules {
    use super::*;
    use std::time::Duration;

    pub fn max_attempts_rule() -> ValidationRule {
        ValidationRule {
            name: "max_attempts".to_string(),
            description: "Validates maximum retry attempts".to_string(),
            severity: ValidationSeverity::Error,
            check: Arc::new(|strategy| {
                let config = strategy.config();
                let is_valid = config.max_attempts > 0 && config.max_attempts <= 10;

                ValidationResult {
                    is_valid,
                    message: if is_valid {
                        "Max attempts within acceptable range".to_string()
                    } else {
                        "Max attempts should be between 1 and 10".to_string()
                    },
                    severity: ValidationSeverity::Error,
                }
            }),
        }
    }

    pub fn retry_delay_rule() -> ValidationRule {
        ValidationRule {
            name: "retry_delay".to_string(),
            description: "Validates retry delay duration".to_string(),
            severity: ValidationSeverity::Warning,
            check: Arc::new(|strategy| {
                let config = strategy.config();
                let is_valid = config.retry_delay >= Duration::from_millis(100) &&
                config.retry_delay <= Duration::from_secs(30);

                ValidationResult {
                    is_valid,
                    message: if is_valid {
                        "Retry delay within acceptable range".to_string()
                    } else {
                        "Retry delay should be between 100ms and 30s".to_string()
                    },
                    severity: ValidationSeverity::Warning,
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RecoveryConfig, RecoveryError};
    use async_trait::async_trait;
    use error_integration::context::ErrorContext;
    use std::time::Duration;

    struct TestStrategy {
        config: RecoveryConfig,
    }

    #[async_trait]
    impl RecoveryStrategy for TestStrategy {
        type Error = std::io::Error;

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
    async fn test_validation_registry() {
        let registry = ValidationRegistry::new();

        // Register validation rules
        registry.register_rule(rules::max_attempts_rule()).await;
        registry.register_rule(rules::retry_delay_rule()).await;

        // Test valid configuration
        let valid_strategy = TestStrategy {
            config: RecoveryConfig {
                max_attempts: 3,
                retry_delay: Duration::from_millis(500),
                use_backoff: true,
            },
        };

        let results = registry.validate_strategy(&valid_strategy).await;
        assert!(results.iter().all(|r| r.is_valid));

        // Test invalid configuration
        let invalid_strategy = TestStrategy {
            config: RecoveryConfig {
                max_attempts: 20,
                retry_delay: Duration::from_secs(60),
                use_backoff: true,
            },
        };

        let results = registry.validate_strategy(&invalid_strategy).await;
        assert!(results.iter().any(|r| !r.is_valid));
    }

    #[tokio::test]
    async fn test_validation_rules() {
        let max_attempts_rule = rules::max_attempts_rule();
        let retry_delay_rule = rules::retry_delay_rule();

        let strategy = TestStrategy {
            config: RecoveryConfig::default(),
        };

        let max_attempts_result = (max_attempts_rule.check)(&strategy);
        assert!(max_attempts_result.is_valid);

        let retry_delay_result = (retry_delay_rule.check)(&strategy);
        assert!(retry_delay_result.is_valid);
    }
}
